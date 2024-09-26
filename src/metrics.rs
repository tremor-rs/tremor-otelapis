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
