// Copyright 2020-2021, The Tremor Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
