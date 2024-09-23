use crate::opentelemetry::proto::collector::metrics::v1 as base;
use crate::opentelemetry::proto::collector::metrics::v1::metrics_service_server as skel;
use tokio::sync::mpsc::{Receiver, Sender};

use super::{OtelMetricsRequest, OtelMetricsResponse};
/// Asynchronous channel sender
pub type OtelMetricsSender = Sender<base::ExportMetricsServiceRequest>;

/// Asynchronous channel receiver
pub type OtelMetricsReceiver = Receiver<base::ExportMetricsServiceRequest>;

/// Creates a metrics service with the specified asynchronous sender channel
pub struct OtelMetricsServiceForwarder {
    channel: OtelMetricsSender,
}

impl OtelMetricsServiceForwarder {
    /// Creates a metrics service forwarding agent with an asynchronous channel sender
    pub fn with_sender(channel: OtelMetricsSender) -> Self {
        OtelMetricsServiceForwarder { channel }
    }
}

#[tonic::async_trait]
impl skel::MetricsService for OtelMetricsServiceForwarder {
    async fn export(
        &self,
        request: OtelMetricsRequest,
    ) -> Result<OtelMetricsResponse, tonic::Status> {
        match self.channel.send(request.into_inner()).await {
            Ok(_) => Ok(tonic::Response::new(base::ExportMetricsServiceResponse {
                partial_success: Some(base::ExportMetricsPartialSuccess {
                    rejected_data_points: 0,
                    error_message: "Ok".to_string(),
                }),
            })),
            Err(e) => Err(tonic::Status::internal(format!(
                "Metrics gRPC forwarder channel sender failed to dispatch {}",
                e
            ))),
        }
    }
}

/// Creates a tonic service forwarder for open telemetry metrics events
pub fn make_forwarder(
    sender: OtelMetricsSender,
) -> skel::MetricsServiceServer<OtelMetricsServiceForwarder> {
    skel::MetricsServiceServer::new(OtelMetricsServiceForwarder::with_sender(sender))
}
