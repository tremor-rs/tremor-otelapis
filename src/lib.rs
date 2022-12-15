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

//! The code in the `gen` folder was seeded by [`tonic-build`].
//!
//! The code in the `src` folder extends the generated source with utility
//! code to allow for convenient usage and definition of tonic-based gRPC
//! servers. Specifically, this library is designed for use by the Tremor Project
//! but has no dependencies on tremor and can be used standalone.
//!
//! This library does not provide an API or SDK designed for use as a tracing
//! facility. The official [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust) project
//! is a complete OpenTelemetry SDK designed for that purpose. It uses the same
//! underlying protocol buffer definitions and will be a better target for projects that
//! require OpenTelemetry based observability instrumentation and iteroperability with
//! the wider observability ecosystem through third party crates.
//!
//! This library is designed for system integration and interoperability and is not
//! recommended for use as a tracing SDK or for instrumentation as that is well covered
//! already by the OpenTelemetry Rust crate. For instrumentation, use the official crate.
//!
//! For those projects that need basic interworking, interoperability or integration with
//! OpenTelemetry based systems at a wire level, this project may be useful.
//!
//! ## Example
//! The complete code can be found [here](https://github.com/tremor-rs/tremor-otelapis).
//!
//! Cargo.toml:
//! ```toml
//! [dependencies]
//! tremor-otelapis = { version = "0.1", features = ["otel-all"] }
//! tonic = { version = "0.8.2", features = ["tls"] }
//! prost = "0.11"
//! prost-types = "0.11"
//! tokio = { version = "1.1", features = ["rt-multi-thread", "time", "fs", "macros"] }
//! ```
//!
//! Example OpenTelemetry Log client. Note that clients simply use the generated
//! client stub code from `tonic-build`. This library adds no extra utility or
//! convenience.
//!
//! ```ignore
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let channel = Endpoint::from_static("http://0.0.0.0:4316")
//!         .connect()
//!         .await?;
//!
//!     let mut client = LogsServiceClient::new(channel);
//!
//!     let resource_logs = ResourceLogs {
//!         ...
//!     };
//!
//!     client
//!         .export(ExportLogsServiceRequest {
//!             resource_logs: vec![resource_logs],
//!         })
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Example OpenTelemetry Log Server. Note that we use utility code
//! to expose the server side functionality. We pass through the generated
//! Protocol Buffer message data binding generated code unadorned. The
//! data bindings for protocol buffer messages are generated by `tonic-build`.
//! The `tonic-build` in turns builds on `prost-build`.
//!
//! ```ignore
//! fn on_logs(
//!     request: tonic::Request<ExportLogsServiceRequest>,
//! ) -> Result<tonic::Response<ExportLogsServiceResponse>, tonic::Status> {
//!     println!("Got a request from {:?}", request.remote_addr());
//!     let reply = ExportLogsServiceResponse::default();
//!     Ok(tonic::Response::new(reply))
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let addr = "0.0.0.0:4317".parse()?;
//!     let svc = otelapis::logs::make_service(Box::new(on_logs));
//!     Server::builder().add_service(svc).serve(addr).await?;
//!
//!     Ok(())
//! }
//!
//! [`otelapis`]: https://github.com/open-telemetry/opentelemetry-specification
//! [`tonic-build`]: https://github.com/hyperium/tonic/tree/master/tonic-build
//!

#![deny(warnings)] // this cannot be `forbid`, because the generated code uses `allow`
#![deny(
    clippy::all,
    clippy::unwrap_used,
    clippy::unnecessary_unwrap,
    missing_docs
    // clippy::pedantic - this makes the generated code unhappy
)]

extern crate prost;

mod otelapis;
pub use otelapis::opentelemetry;

/// This modules defines the OpenTelemetry gRPC service definitions introduced
/// in v0.19 which make Otel gRPC responses fallible and transporting error context
/// back to the requestor. In prior versions of Otel, the gRPC responses contained
/// no error context and were always successful.
///
/// The change from infallible to fallible is a major breaking change forcing implementors
/// to course correct their handlers to return errors. This module standardises this handling
/// and provides a simple way to handle the error context in a uniform way as far as use in
/// tremor is concerned.
///
pub mod common {
    use crate::opentelemetry::proto::collector::{
        logs::v1::ExportLogsServiceResponse, metrics::v1::ExportMetricsServiceResponse,
        trace::v1::ExportTraceServiceResponse,
    };

    /// Prior to v0.19, responses were infallible. Since v0.19, they propagate error context.
    /// This struct is a convenience wrapper to make handling the error context easier to
    /// integrate with tremor.
    pub struct FallibleOtelResponse {
        /// Possibly non-zero Count of rejected log records
        pub rejected_logs: i64,
        /// Possibly non-zero count of rejected metrics records
        pub rejected_metrics: i64,
        /// Possibly non-zero count of rejected trace records
        pub rejected_spans: i64,
        /// Possibly empty error message
        pub error_message: String,
    }

    impl FallibleOtelResponse {
        /// Create a new FallibleOtelResponse
        pub fn new(
            rejected_logs: i64,
            rejected_metrics: i64,
            rejected_spans: i64,
            error_message: String,
        ) -> Self {
            Self {
                rejected_logs,
                rejected_metrics,
                rejected_spans,
                error_message,
            }
        }

        /// Checks if errors were reported in the response - if any rejected count is non-zero this will return false
        /// The error message is not included in this check
        pub fn is_ok(&self) -> bool {
            self.rejected_logs == 0 && self.rejected_metrics == 0 && self.rejected_spans == 0
        }
    }

    impl From<ExportLogsServiceResponse> for FallibleOtelResponse {
        fn from(response: ExportLogsServiceResponse) -> Self {
            match response.partial_success {
                Some(disposition) => Self::new(
                    disposition.rejected_log_records,
                    0,
                    0,
                    disposition.error_message,
                ),
                None => Self::new(0, 0, 0, String::new()),
            }
        }
    }

    impl From<ExportMetricsServiceResponse> for FallibleOtelResponse {
        fn from(response: ExportMetricsServiceResponse) -> Self {
            match response.partial_success {
                Some(disposition) => Self::new(
                    0,
                    disposition.rejected_data_points,
                    0,
                    disposition.error_message,
                ),
                None => Self::new(0, 0, 0, String::new()),
            }
        }
    }

    impl From<ExportTraceServiceResponse> for FallibleOtelResponse {
        fn from(response: ExportTraceServiceResponse) -> Self {
            match response.partial_success {
                Some(disposition) => Self::new(
                    0,
                    0,
                    disposition.rejected_spans,
                    disposition.error_message,
                ),
                None => Self::new(0, 0, 0, String::new()),
            }
        }
    }
}

#[cfg(feature = "otel-trace")]
/// This module defines a skeleton implementation of the open telemetry
/// collector tracing service
///
pub mod trace {
    use crate::opentelemetry::proto::collector::trace::v1 as base;
    use crate::opentelemetry::proto::collector::trace::v1::trace_service_server as skel;

    /// Alias tonic TraceRequest
    pub type OtelTraceRequest = tonic::Request<base::ExportTraceServiceRequest>;

    /// Alias tonic TraceResponse
    pub type OtelTraceResponse = tonic::Response<base::ExportTraceServiceResponse>;

    /// Alias the generated server skeletons
    pub use skel::TraceServiceServer;

    /// Alias trace callback fn
    pub type OnTraceFn = dyn Fn(OtelTraceRequest) -> Result<OtelTraceResponse, tonic::Status>
        + Send
        + Sync
        + 'static;

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
}

/// This module defines a skeleton implementation of the open telemetry
/// collector logging service
///
#[cfg(feature = "otel-logs")]
pub mod logs {
    use crate::opentelemetry::proto::collector::logs::v1 as base;
    use crate::opentelemetry::proto::collector::logs::v1::logs_service_server as skel;
    use async_channel::{Receiver, Sender};

    /// Alias tonic request
    pub type OtelLogsRequest = tonic::Request<base::ExportLogsServiceRequest>;

    /// Alias tonic reponse
    pub type OtelLogsResponse = tonic::Response<base::ExportLogsServiceResponse>;

    /// Alias service skeleton
    pub use skel::LogsService;

    /// Alias logs server
    pub use skel::LogsServiceServer;

    /// Alias logs callback fn
    pub type OnLogsFn =
        dyn Fn(OtelLogsRequest) -> Result<OtelLogsResponse, tonic::Status> + Send + Sync + 'static;

    /// GRPC logs service skeleton
    pub struct OtelLogsService {
        on_logs: Box<OnLogsFn>,
    }

    impl OtelLogsService {
        /// Creates a logs service with the specified logs event handler function
        pub fn with_handler(handler: Box<OnLogsFn>) -> Self {
            OtelLogsService { on_logs: handler }
        }
    }

    #[tonic::async_trait]
    impl skel::LogsService for OtelLogsService {
        async fn export(
            &self,
            request: tonic::Request<base::ExportLogsServiceRequest>,
        ) -> Result<tonic::Response<base::ExportLogsServiceResponse>, tonic::Status> {
            (self.on_logs)(request)
        }
    }

    /// Creates a tonic service handler for open telemetry log events
    pub fn make_service(handler: Box<OnLogsFn>) -> skel::LogsServiceServer<OtelLogsService> {
        skel::LogsServiceServer::new(OtelLogsService::with_handler(handler))
    }

    /// Asynchronous channel sender
    pub type OtelLogsSender = Sender<base::ExportLogsServiceRequest>;

    /// Asynchronous channel receiver
    pub type OtelLogsReceiver = Receiver<base::ExportLogsServiceRequest>;

    /// Logs forwarding agent
    pub struct OtelLogsServiceForwarder {
        channel: Sender<base::ExportLogsServiceRequest>,
    }

    // Creates a metrics service with the specified asynchronous sender channel
    impl OtelLogsServiceForwarder {
        /// Creates a log forwarding agent with an asynchronous channel sender
        pub fn with_sender(channel: Sender<base::ExportLogsServiceRequest>) -> Self {
            OtelLogsServiceForwarder { channel }
        }
    }

    #[tonic::async_trait]
    impl skel::LogsService for OtelLogsServiceForwarder {
        async fn export(
            &self,
            request: tonic::Request<base::ExportLogsServiceRequest>,
        ) -> Result<tonic::Response<base::ExportLogsServiceResponse>, tonic::Status> {
            match self.channel.send(request.into_inner()).await {
                Ok(()) => Ok(tonic::Response::new(base::ExportLogsServiceResponse {
                    partial_success: Some(base::ExportLogsPartialSuccess {
                        rejected_log_records: 0,
                        error_message: "snot".to_string(),
                    }),
                })),
                Err(e) => Err(tonic::Status::internal(&format!(
                    "Logs gRPC forwarder channel sender failed to dispatch {}",
                    e
                ))),
            }
        }
    }

    /// Creates a tonic service handler for open telemetry logs events
    pub fn make_forwarder(
        sender: OtelLogsSender,
    ) -> skel::LogsServiceServer<OtelLogsServiceForwarder> {
        skel::LogsServiceServer::new(OtelLogsServiceForwarder::with_sender(sender))
    }
}

/// This module defines a skeleton implementation of the open telemetry
/// collector metrics service
///
#[cfg(feature = "otel-metrics")]
pub mod metrics {
    use crate::opentelemetry::proto::collector::metrics::v1 as base;
    use crate::opentelemetry::proto::collector::metrics::v1::metrics_service_server as skel;
    use async_channel::{Receiver, Sender};

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
    pub fn make_service(
        handler: Box<OnMetricsFn>,
    ) -> skel::MetricsServiceServer<OtelMetricsService> {
        skel::MetricsServiceServer::new(OtelMetricsService::with_handler(handler))
    }

    /// Asynchronous channel sender
    pub type OtelMetricsSender = Sender<base::ExportMetricsServiceRequest>;

    /// Asynchronous channel receiver
    pub type OtelMetricsReceiver = Receiver<base::ExportMetricsServiceRequest>;

    /// Creates a metrics service with the specified asynchronous sender channel
    pub struct OtelMetricsServiceForwarder {
        channel: Sender<base::ExportMetricsServiceRequest>,
    }

    impl OtelMetricsServiceForwarder {
        /// Creates a metrics service forwarding agent with an asynchronous channel sender
        pub fn with_sender(channel: Sender<base::ExportMetricsServiceRequest>) -> Self {
            OtelMetricsServiceForwarder { channel }
        }
    }

    #[tonic::async_trait]
    impl skel::MetricsService for OtelMetricsServiceForwarder {
        async fn export(
            &self,
            request: tonic::Request<base::ExportMetricsServiceRequest>,
        ) -> Result<tonic::Response<base::ExportMetricsServiceResponse>, tonic::Status> {
            match self.channel.send(request.into_inner()).await {
                Ok(_) => Ok(tonic::Response::new(base::ExportMetricsServiceResponse {
                    partial_success: Some(base::ExportMetricsPartialSuccess {
                        rejected_data_points: 0,
                        error_message: "snot".to_string(),
                    }),
                })),
                Err(e) => Err(tonic::Status::internal(&format!(
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
}

/// A unified set of services that provide log, metrics and trace events
#[cfg(feature = "otel-all")]
pub mod all {
    use crate::opentelemetry::proto::collector::logs::v1 as logs_base;
    use crate::opentelemetry::proto::collector::metrics::v1 as metrics_base;
    use crate::opentelemetry::proto::collector::trace::v1 as trace_base;
    use async_channel::{Receiver, Sender};
    use std::net::SocketAddr;
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

    /// Alias receiver
    pub type OpenTelemetrySender = Sender<OpenTelemetryEvents>;

    /// Alias sender
    pub type OpenTelemetryReceiver = Receiver<OpenTelemetryEvents>;

    /// Creates a logs service with the specified asynchronous sender channel
    pub struct LogsServiceForwarder {
        channel: Sender<OpenTelemetryEvents>,
    }

    impl LogsServiceForwarder {
        /// Creates a logs service forwarding agent
        pub fn with_sender(channel: Sender<OpenTelemetryEvents>) -> Self {
            LogsServiceForwarder { channel }
        }
    }

    #[tonic::async_trait]
    impl super::logs::LogsService for LogsServiceForwarder {
        async fn export(
            &self,
            request: tonic::Request<logs_base::ExportLogsServiceRequest>,
        ) -> Result<tonic::Response<logs_base::ExportLogsServiceResponse>, tonic::Status> {
            match self.channel.send(OpenTelemetryEvents::from(request)).await {
                Ok(_) => Ok(tonic::Response::new(logs_base::ExportLogsServiceResponse {
                    partial_success: Some(logs_base::ExportLogsPartialSuccess {
                        rejected_log_records: 0,
                        error_message: "snot".to_string(),
                    }),
                })),
                Err(e) => Err(tonic::Status::internal(&format!(
                    "Logs gRPC forwarder channel sender failed to dispatch {}",
                    e
                ))),
            }
        }
    }

    /// Creates a metrics service with the specified asynchronous sender channel
    pub struct MetricsServiceForwarder {
        channel: Sender<OpenTelemetryEvents>,
    }

    impl MetricsServiceForwarder {
        /// Creates a metrics service forwarding agent
        pub fn with_sender(channel: Sender<OpenTelemetryEvents>) -> Self {
            MetricsServiceForwarder { channel }
        }
    }

    #[tonic::async_trait]
    impl super::metrics::MetricsService for MetricsServiceForwarder {
        async fn export(
            &self,
            request: tonic::Request<metrics_base::ExportMetricsServiceRequest>,
        ) -> Result<tonic::Response<metrics_base::ExportMetricsServiceResponse>, tonic::Status>
        {
            match self.channel.send(OpenTelemetryEvents::from(request)).await {
                Ok(_) => Ok(tonic::Response::new(
                    metrics_base::ExportMetricsServiceResponse {
                        partial_success: Some(metrics_base::ExportMetricsPartialSuccess {
                            rejected_data_points: 0,
                            error_message: "snot".to_string(),
                        }),
                    },
                )),
                Err(e) => Err(tonic::Status::internal(&format!(
                    "Metrics gRPC forwarder channel sender failed to dispatch {}",
                    e
                ))),
            }
        }
    }

    /// Creates a trace service with the specified asynchronous sender channel
    pub struct TraceServiceForwarder {
        channel: Sender<OpenTelemetryEvents>,
    }

    impl TraceServiceForwarder {
        /// Creates a trace service forwarding agent
        pub fn with_sender(channel: Sender<OpenTelemetryEvents>) -> Self {
            TraceServiceForwarder { channel }
        }
    }

    #[tonic::async_trait]
    impl trace_base::trace_service_server::TraceService for TraceServiceForwarder {
        async fn export(
            &self,
            request: tonic::Request<trace_base::ExportTraceServiceRequest>,
        ) -> Result<tonic::Response<trace_base::ExportTraceServiceResponse>, tonic::Status>
        {
            match self.channel.send(OpenTelemetryEvents::from(request)).await {
                Ok(_) => Ok(tonic::Response::new(
                    trace_base::ExportTraceServiceResponse {
                        partial_success: Some(trace_base::ExportTracePartialSuccess {
                            rejected_spans: 0,
                            error_message: "snot".to_string(),
                        }),
                    },
                )),
                Err(e) => Err(tonic::Status::internal(&format!(
                    "Trace gRPC forwarder channel sender failed to dispatch {}",
                    e
                ))),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::opentelemetry::proto::collector::{
        logs::v1::{ExportLogsPartialSuccess, ExportLogsServiceResponse},
        metrics::v1::{ExportMetricsPartialSuccess, ExportMetricsServiceResponse},
        trace::v1::{ExportTracePartialSuccess, ExportTraceServiceResponse},
    };

    use super::common::FallibleOtelResponse;

    #[test]
    pub fn make_fallible_error() {
        let mut e = FallibleOtelResponse::new(0, 0, 0, "snot".to_string());
        assert_eq!(e.error_message, "snot".to_string());
        assert_eq!(e.rejected_logs, 0);
        assert_eq!(e.rejected_metrics, 0);
        assert_eq!(e.rejected_spans, 0);
        assert!(e.is_ok());
        e.rejected_logs = 1;
        assert!(!e.is_ok());
        e.rejected_logs = -1;
        assert!(!e.is_ok());
        e.rejected_logs = 0;
        e.rejected_metrics = 1;
        assert!(!e.is_ok());
        e.rejected_metrics = -1;
        assert!(!e.is_ok());
        e.rejected_metrics = 0;
        e.rejected_spans = 1;
        assert!(!e.is_ok());
        e.rejected_spans = -1;
        assert!(!e.is_ok());
    }

    #[test]
    pub fn fallible_from_log_response() {
        let log = ExportLogsServiceResponse {
            partial_success: Some(ExportLogsPartialSuccess {
                rejected_log_records: 1,
                error_message: "beep".to_string(),
            }),
        };
        let e = FallibleOtelResponse::from(log);
        assert_eq!(e.error_message, "beep".to_string());
        assert_eq!(e.rejected_logs, 1);
        assert_eq!(e.rejected_metrics, 0);
        assert_eq!(e.rejected_spans, 0);
    }

    #[test]
    pub fn fallible_from_metric_response() {
        let metric = ExportMetricsServiceResponse {
            partial_success: Some(ExportMetricsPartialSuccess {
                rejected_data_points: 1,
                error_message: "boop".to_string(),
            }),
        };
        let e = FallibleOtelResponse::from(metric);
        assert_eq!(e.error_message, "boop".to_string());
        assert_eq!(e.rejected_logs, 0);
        assert_eq!(e.rejected_metrics, 1);
        assert_eq!(e.rejected_spans, 0);
    }

    #[test]
    pub fn fallible_from_trace_response() {
        let metric = ExportTraceServiceResponse {
            partial_success: Some(ExportTracePartialSuccess {
                rejected_spans: 1,
                error_message: "fleek".to_string(),
            }),
        };
        let e = FallibleOtelResponse::from(metric);
        assert_eq!(e.error_message, "fleek".to_string());
        assert_eq!(e.rejected_logs, 0);
        assert_eq!(e.rejected_metrics, 0);
        assert_eq!(e.rejected_spans, 1);
    }
}
