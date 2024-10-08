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
use tokio::sync::mpsc::{Receiver, Sender};

use super::{OtelLogsRequest, OtelLogsResponse};

/// Asynchronous channel sender
pub type OtelLogsSender = Sender<base::ExportLogsServiceRequest>;

/// Asynchronous channel receiver
pub type OtelLogsReceiver = Receiver<base::ExportLogsServiceRequest>;

/// Logs forwarding agent
pub struct OtelLogsServiceForwarder {
    channel: OtelLogsSender,
}

// Creates a metrics service with the specified asynchronous sender channel
impl OtelLogsServiceForwarder {
    /// Creates a log forwarding agent with an asynchronous channel sender
    pub fn with_sender(channel: OtelLogsSender) -> Self {
        OtelLogsServiceForwarder { channel }
    }
}

#[tonic::async_trait]
impl skel::LogsService for OtelLogsServiceForwarder {
    async fn export(&self, request: OtelLogsRequest) -> Result<OtelLogsResponse, tonic::Status> {
        match self.channel.send(request.into_inner()).await {
            Ok(()) => Ok(tonic::Response::new(base::ExportLogsServiceResponse {
                partial_success: Some(base::ExportLogsPartialSuccess {
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

/// Creates a tonic service handler for open telemetry logs events
pub fn make_forwarder(sender: OtelLogsSender) -> skel::LogsServiceServer<OtelLogsServiceForwarder> {
    skel::LogsServiceServer::new(OtelLogsServiceForwarder::with_sender(sender))
}
