use crate::opentelemetry::proto::collector::metrics::v1 as base;
use crate::opentelemetry::proto::collector::metrics::v1::metrics_service_server as skel;

#[cfg(feature = "channels")]
mod channels;

#[cfg(feature = "channels")]
pub use channels::*;

pub use skel::MetricsService;
pub use skel::MetricsServiceServer;

/// Alias tonic request
pub type OtelMetricsRequest = tonic::Request<base::ExportMetricsServiceRequest>;

/// Alias tonic response
pub type OtelMetricsResponse = tonic::Response<base::ExportMetricsServiceResponse>;

/// Alias metrics callback fn
pub type OnMetricsFn = dyn Fn(OtelMetricsRequest) -> Result<OtelMetricsResponse, tonic::Status>
    + Send
    + Sync
    + 'static;

/// GRPC metrics service skeleton
pub struct OtelMetricsService {
    on_metrics: Box<OnMetricsFn>,
}

impl OtelMetricsService {
    /// Creates a metrics service with the specified metrics event handler function
    pub fn with_handler(handler: Box<OnMetricsFn>) -> Self {
        OtelMetricsService {
            on_metrics: handler,
        }
    }
}

#[tonic::async_trait]
impl skel::MetricsService for OtelMetricsService {
    async fn export(
        &self,
        request: tonic::Request<base::ExportMetricsServiceRequest>,
    ) -> Result<tonic::Response<base::ExportMetricsServiceResponse>, tonic::Status> {
        (self.on_metrics)(request)
    }
}

/// Creates a tonic service handler for open telemetry metrics events
pub fn make_service(handler: Box<OnMetricsFn>) -> skel::MetricsServiceServer<OtelMetricsService> {
    skel::MetricsServiceServer::new(OtelMetricsService::with_handler(handler))
}
