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
            Some(disposition) => {
                Self::new(0, 0, disposition.rejected_spans, disposition.error_message)
            }
            None => Self::new(0, 0, 0, String::new()),
        }
    }
}
