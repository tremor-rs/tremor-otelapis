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

use crate::opentelemetry::proto::collector::logs::v1 as base;
use crate::opentelemetry::proto::collector::logs::v1::logs_service_server as skel;

#[cfg(feature = "channels")]
mod channels;

#[cfg(feature = "channels")]
pub use channels::*;

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
