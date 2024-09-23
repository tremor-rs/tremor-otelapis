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

// We allow the generated code to use a less strict coding still
// than hand maintained code.
#[allow(
    clippy::all,
    clippy::unwrap_used,
    clippy::unnecessary_unwrap,
    clippy::pedantic
)]
#[allow(missing_docs)]

/// Opentelemetry APIs
pub mod opentelemetry {
    /// Protocols
    pub mod proto {
        /// Colector
        pub mod collector {
            /// Logs
            pub mod logs {
                /// v1
                pub mod v1 {
                    #[cfg(any(feature = "opentelemetry-proto-collector-logs-v1",))]
                    tonic::include_proto!("opentelemetry.proto.collector.logs.v1");
                }
            }
            /// Metrics
            pub mod metrics {
                /// v1
                pub mod v1 {
                    #[cfg(any(feature = "opentelemetry-proto-collector-metrics-v1",))]
                    tonic::include_proto!("opentelemetry.proto.collector.metrics.v1");
                }
            }
            /// Traces
            pub mod trace {
                /// v1
                pub mod v1 {
                    #[cfg(any(feature = "opentelemetry-proto-collector-trace-v1",))]
                    tonic::include_proto!("opentelemetry.proto.collector.trace.v1");
                }
            }
        }
        /// Common definitions
        pub mod common {
            /// v1
            pub mod v1 {
                #[cfg(any(
                    feature = "opentelemetry-proto-collector-logs-v1",
                    feature = "opentelemetry-proto-collector-metrics-v1",
                    feature = "opentelemetry-proto-collector-trace-v1",
                    feature = "opentelemetry-proto-common-v1",
                    feature = "opentelemetry-proto-logs-v1",
                    feature = "opentelemetry-proto-metrics-v1",
                    feature = "opentelemetry-proto-resource-v1",
                    feature = "opentelemetry-proto-trace-v1",
                ))]
                tonic::include_proto!("opentelemetry.proto.common.v1");
            }
        }
        /// Logs
        pub mod logs {
            /// v1
            pub mod v1 {
                #[cfg(any(
                    feature = "opentelemetry-proto-collector-logs-v1",
                    feature = "opentelemetry-proto-logs-v1",
                ))]
                tonic::include_proto!("opentelemetry.proto.logs.v1");
            }
        }
        /// Metrics
        pub mod metrics {
            /// v1
            pub mod v1 {
                #[cfg(any(
                    feature = "opentelemetry-proto-collector-metrics-v1",
                    feature = "opentelemetry-proto-metrics-v1",
                ))]
                tonic::include_proto!("opentelemetry.proto.metrics.v1");
            }
        }
        /// resourices
        pub mod resource {
            /// v1
            pub mod v1 {
                #[cfg(any(
                    feature = "opentelemetry-proto-collector-logs-v1",
                    feature = "opentelemetry-proto-collector-metrics-v1",
                    feature = "opentelemetry-proto-collector-trace-v1",
                    feature = "opentelemetry-proto-logs-v1",
                    feature = "opentelemetry-proto-metrics-v1",
                    feature = "opentelemetry-proto-resource-v1",
                    feature = "opentelemetry-proto-trace-v1",
                ))]
                tonic::include_proto!("opentelemetry.proto.resource.v1");
            }
        }
        /// traces
        pub mod trace {
            /// v1
            pub mod v1 {
                #[cfg(any(
                    feature = "opentelemetry-proto-collector-trace-v1",
                    feature = "opentelemetry-proto-trace-v1",
                ))]
                tonic::include_proto!("opentelemetry.proto.trace.v1");
            }
        }
    }
}
