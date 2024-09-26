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

use crate::opentelemetry::proto::collector::logs::v1 as logs_base;
use crate::opentelemetry::proto::collector::metrics::v1 as metrics_base;
use crate::opentelemetry::proto::collector::trace::v1 as trace_base;
use tokio::sync::mpsc::{Receiver, Sender};

use super::OpenTelemetryEvents;
/// Alias receiver
pub type OpenTelemetrySender = Sender<OpenTelemetryEvents>;

/// Alias sender
pub type OpenTelemetryReceiver = Receiver<OpenTelemetryEvents>;

/// Creates a logs service with the specified asynchronous sender channel
pub struct LogsServiceForwarder {
    channel: OpenTelemetrySender,
}

impl LogsServiceForwarder {
    /// Creates a logs service forwarding agent
    pub fn with_sender(channel: OpenTelemetrySender) -> Self {
        LogsServiceForwarder { channel }
    }
}

#[tonic::async_trait]
impl crate::logs::LogsService for LogsServiceForwarder {
    async fn export(
        &self,
        request: tonic::Request<logs_base::ExportLogsServiceRequest>,
    ) -> Result<tonic::Response<logs_base::ExportLogsServiceResponse>, tonic::Status> {
        match self.channel.send(OpenTelemetryEvents::from(request)).await {
            Ok(_) => Ok(tonic::Response::new(logs_base::ExportLogsServiceResponse {
                partial_success: Some(logs_base::ExportLogsPartialSuccess {
                    rejected_log_records: 0,
                    error_message: "Ok".to_string(),
                }),
            })),
            Err(e) => Err(tonic::Status::internal(format!(
                "Logs gRPC forwarder channel sender failed to dispatch {}",
                e
            ))),
        }
    }
}

/// Creates a metrics service with the specified asynchronous sender channel
pub struct MetricsServiceForwarder {
    channel: OpenTelemetrySender,
}

impl MetricsServiceForwarder {
    /// Creates a metrics service forwarding agent
    pub fn with_sender(channel: OpenTelemetrySender) -> Self {
        MetricsServiceForwarder { channel }
    }
}

#[tonic::async_trait]
impl crate::metrics::MetricsService for MetricsServiceForwarder {
    async fn export(
        &self,
        request: tonic::Request<metrics_base::ExportMetricsServiceRequest>,
    ) -> Result<tonic::Response<metrics_base::ExportMetricsServiceResponse>, tonic::Status> {
        match self.channel.send(OpenTelemetryEvents::from(request)).await {
            Ok(_) => Ok(tonic::Response::new(
                metrics_base::ExportMetricsServiceResponse {
                    partial_success: Some(metrics_base::ExportMetricsPartialSuccess {
                        rejected_data_points: 0,
                        error_message: "snot".to_string(),
                    }),
                },
            )),
            Err(e) => Err(tonic::Status::internal(format!(
                "Metrics gRPC forwarder channel sender failed to dispatch {}",
                e
            ))),
        }
    }
}

/// Creates a trace service with the specified asynchronous sender channel
pub struct TraceServiceForwarder {
    channel: OpenTelemetrySender,
}

impl TraceServiceForwarder {
    /// Creates a trace service forwarding agent
    pub fn with_sender(channel: OpenTelemetrySender) -> Self {
        TraceServiceForwarder { channel }
    }
}

#[tonic::async_trait]
impl trace_base::trace_service_server::TraceService for TraceServiceForwarder {
    async fn export(
        &self,
        request: tonic::Request<trace_base::ExportTraceServiceRequest>,
    ) -> Result<tonic::Response<trace_base::ExportTraceServiceResponse>, tonic::Status> {
        match self.channel.send(OpenTelemetryEvents::from(request)).await {
            Ok(_) => Ok(tonic::Response::new(
                trace_base::ExportTraceServiceResponse {
                    partial_success: Some(trace_base::ExportTracePartialSuccess {
                        rejected_spans: 0,
                        error_message: "snot".to_string(),
                    }),
                },
            )),
            Err(e) => Err(tonic::Status::internal(format!(
                "Trace gRPC forwarder channel sender failed to dispatch {}",
                e
            ))),
        }
    }
}
