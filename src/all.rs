use crate::opentelemetry::proto::collector::logs::v1 as logs_base;
use crate::opentelemetry::proto::collector::metrics::v1 as metrics_base;
use crate::opentelemetry::proto::collector::trace::v1 as trace_base;

#[cfg(feature = "channels")]
pub use channels::*;
use std::net::SocketAddr;
#[cfg(feature = "channels")]
mod channels;
//    use tonic::transport::Server;

/// Enumeration of protocol buffer messages that are sendable/receivable
pub enum OpenTelemetryEvents {
    /// A logs export request
    Logs(logs_base::ExportLogsServiceRequest, Option<SocketAddr>),
    /// A metrics export request
    Metrics(
        metrics_base::ExportMetricsServiceRequest,
        Option<SocketAddr>,
    ),
    /// A trace export request
    Trace(trace_base::ExportTraceServiceRequest, Option<SocketAddr>),
}
impl From<tonic::Request<logs_base::ExportLogsServiceRequest>> for OpenTelemetryEvents {
    fn from(req: tonic::Request<logs_base::ExportLogsServiceRequest>) -> Self {
        let remote = req.remote_addr();
        Self::Logs(req.into_inner(), remote)
    }
}
impl From<tonic::Request<metrics_base::ExportMetricsServiceRequest>> for OpenTelemetryEvents {
    fn from(req: tonic::Request<metrics_base::ExportMetricsServiceRequest>) -> Self {
        let remote = req.remote_addr();
        Self::Metrics(req.into_inner(), remote)
    }
}
impl From<tonic::Request<trace_base::ExportTraceServiceRequest>> for OpenTelemetryEvents {
    fn from(req: tonic::Request<trace_base::ExportTraceServiceRequest>) -> Self {
        let remote = req.remote_addr();
        Self::Trace(req.into_inner(), remote)
    }
}
