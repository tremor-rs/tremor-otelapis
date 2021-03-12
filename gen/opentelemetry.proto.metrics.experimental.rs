#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricConfigRequest {
    /// Required. The resource for which configuration should be returned.
    #[prost(message, optional, tag = "1")]
    pub resource: ::core::option::Option<super::super::resource::v1::Resource>,
    /// Optional. The value of MetricConfigResponse.fingerprint for the last
    /// configuration that the caller received and successfully applied.
    #[prost(bytes = "vec", tag = "2")]
    pub last_known_fingerprint: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricConfigResponse {
    /// Optional. The fingerprint associated with this MetricConfigResponse. Each
    /// change in configs yields a different fingerprint. The resource SHOULD copy
    /// this value to MetricConfigRequest.last_known_fingerprint for the next
    /// configuration request. If there are no changes between fingerprint and
    /// MetricConfigRequest.last_known_fingerprint, then all other fields besides
    /// fingerprint in the response are optional, or the same as the last update if
    /// present.
    ///
    /// The exact mechanics of generating the fingerprint is up to the
    /// implementation. However, a fingerprint must be deterministically determined
    /// by the configurations -- the same configuration will generate the same
    /// fingerprint on any instance of an implementation. Hence using a timestamp is
    /// unacceptable, but a deterministic hash is fine.
    #[prost(bytes = "vec", tag = "1")]
    pub fingerprint: ::prost::alloc::vec::Vec<u8>,
    /// A single metric may match multiple schedules. In such cases, the schedule
    /// that specifies the smallest period is applied.
    ///
    /// Note, for optimization purposes, it is recommended to use as few schedules
    /// as possible to capture all required metric updates. Where you can be
    /// conservative, do take full advantage of the inclusion/exclusion patterns to
    /// capture as much of your targeted metrics.
    #[prost(message, repeated, tag = "2")]
    pub schedules: ::prost::alloc::vec::Vec<metric_config_response::Schedule>,
    /// Optional. The client is suggested to wait this long (in seconds) before
    /// pinging the configuration service again.
    #[prost(int32, tag = "3")]
    pub suggested_wait_time_sec: i32,
}
/// Nested message and enum types in `MetricConfigResponse`.
pub mod metric_config_response {
    /// A Schedule is used to apply a particular scheduling configuration to
    /// a metric. If a metric name matches a schedule's patterns, then the metric
    /// adopts the configuration specified by the schedule.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Schedule {
        /// Metrics with names that match a rule in the inclusion_patterns are
        /// targeted by this schedule. Metrics that match the exclusion_patterns
        /// are not targeted for this schedule, even if they match an inclusion
        /// pattern.
        #[prost(message, repeated, tag = "1")]
        pub exclusion_patterns: ::prost::alloc::vec::Vec<schedule::Pattern>,
        #[prost(message, repeated, tag = "2")]
        pub inclusion_patterns: ::prost::alloc::vec::Vec<schedule::Pattern>,
        /// Describes the collection period for each metric in seconds.
        /// A period of 0 means to not export.
        #[prost(int32, tag = "3")]
        pub period_sec: i32,
    }
    /// Nested message and enum types in `Schedule`.
    pub mod schedule {
        /// A light-weight pattern that can match 1 or more
        /// metrics, for which this schedule will apply. The string is used to
        /// match against metric names. It should not exceed 100k characters.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Pattern {
            #[prost(oneof = "pattern::Match", tags = "1, 2")]
            pub r#match: ::core::option::Option<pattern::Match>,
        }
        /// Nested message and enum types in `Pattern`.
        pub mod pattern {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Match {
                /// matches the metric name exactly
                #[prost(string, tag = "1")]
                Equals(::prost::alloc::string::String),
                /// prefix-matches the metric name
                #[prost(string, tag = "2")]
                StartsWith(::prost::alloc::string::String),
            }
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod metric_config_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " MetricConfig is a service that enables updating metric schedules, trace"]
    #[doc = " parameters, and other configurations on the SDK without having to restart the"]
    #[doc = " instrumented application. The collector can also serve as the configuration"]
    #[doc = " service, acting as a bridge between third-party configuration services and"]
    #[doc = " the SDK, piping updated configs from a third-party source to an instrumented"]
    #[doc = " application."]
    pub struct MetricConfigClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MetricConfigClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn get_metric_config(
            &mut self,
            request: impl tonic::IntoRequest<super::MetricConfigRequest>,
        ) -> Result<tonic::Response<super::MetricConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/opentelemetry.proto.metrics.experimental.MetricConfig/GetMetricConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MetricConfigClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MetricConfigClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MetricConfigClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod metric_config_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MetricConfigServer."]
    #[async_trait]
    pub trait MetricConfig: Send + Sync + 'static {
        async fn get_metric_config(
            &self,
            request: tonic::Request<super::MetricConfigRequest>,
        ) -> Result<tonic::Response<super::MetricConfigResponse>, tonic::Status>;
    }
    #[doc = " MetricConfig is a service that enables updating metric schedules, trace"]
    #[doc = " parameters, and other configurations on the SDK without having to restart the"]
    #[doc = " instrumented application. The collector can also serve as the configuration"]
    #[doc = " service, acting as a bridge between third-party configuration services and"]
    #[doc = " the SDK, piping updated configs from a third-party source to an instrumented"]
    #[doc = " application."]
    #[derive(Debug)]
    pub struct MetricConfigServer<T: MetricConfig> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: MetricConfig> MetricConfigServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for MetricConfigServer<T>
    where
        T: MetricConfig,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/opentelemetry.proto.metrics.experimental.MetricConfig/GetMetricConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetMetricConfigSvc<T: MetricConfig>(pub Arc<T>);
                    impl<T: MetricConfig> tonic::server::UnaryService<super::MetricConfigRequest>
                        for GetMetricConfigSvc<T>
                    {
                        type Response = super::MetricConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MetricConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_metric_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetMetricConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: MetricConfig> Clone for MetricConfigServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: MetricConfig> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
