use crate::opentelemetry::proto::collector::trace::v1 as base;
use crate::opentelemetry::proto::collector::trace::v1::trace_service_server as skel;

/// Alias tonic TraceRequest
pub type OtelTraceRequest = tonic::Request<base::ExportTraceServiceRequest>;

/// Alias tonic TraceResponse
pub type OtelTraceResponse = tonic::Response<base::ExportTraceServiceResponse>;

/// Alias the generated server skeletons
pub use skel::TraceServiceServer;

/// Alias trace callback fn
pub type OnTraceFn =
    dyn Fn(OtelTraceRequest) -> Result<OtelTraceResponse, tonic::Status> + Send + Sync + 'static;

/// GRPC trace service skeleton
pub struct OtelTraceService {
    on_trace: Box<OnTraceFn>,
}

impl OtelTraceService {
    /// Creates a trace service with the specified trace event handler function
    pub fn with_handler(handler: Box<OnTraceFn>) -> Self {
        OtelTraceService { on_trace: handler }
    }
}

/// Creates a tonic service handler for open telemetry trace events
pub fn make_service(handler: Box<OnTraceFn>) -> skel::TraceServiceServer<OtelTraceService> {
    skel::TraceServiceServer::new(OtelTraceService::with_handler(handler))
}

#[tonic::async_trait]
impl skel::TraceService for OtelTraceService {
    async fn export(
        &self,
        request: tonic::Request<base::ExportTraceServiceRequest>,
    ) -> Result<tonic::Response<base::ExportTraceServiceResponse>, tonic::Status> {
        (self.on_trace)(request)
    }
}
