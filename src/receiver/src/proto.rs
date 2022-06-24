//! Protobuf imports.
//!
//! The imports can be found in this project's OUT_DIR under the 'proto' folder.
//! This is usually found in the 'target' directory.

/// Includes the compiled OpenTelemetry components from the compiled protobuf
/// output. This includes both the component types and the collector component.
///
/// Output is structured as the following to ensure that the references in the
/// imported code are valid:
///
/// ```no_run
/// // Invocation:
/// include_components![foo, bar];
///
/// // Output:
/// pub mod foo {
///     pub mod v1 {
///         // foo definitions...
///     }
/// }
///
/// pub mod bar {
///     pub mod v1 {
///         // bar definitions...
///     }
/// }
///
/// pub mod collector {
///     pub mod foo {
///         pub mod v1 {
///             // collector foo definitions + service...
///         }
///     }
///     
///     pub mod bar {
///         pub mod v1 {
///             // collector bar definitions + service...
///         }
///     }
/// }
/// ```
macro_rules! include_components {
    [$($name:ident),*] => {
        $(
            #[doc = concat!("Definitions for the ", stringify!($name), " component.")]
            pub mod $name {
                /// Version 1 of the OTEL specification
                pub mod v1 {
                    include!(concat!(
                        env!("OUT_DIR"),
                        "/proto/opentelemetry.proto.",
                        stringify!($name),
                        ".v1.rs"
                    ));
                }
            }
        )*

        /// Collector definitions and services
        pub mod collector {
            $(
                #[doc = concat!("Definitions and services for the ", stringify!($name), " collector component.")]
                pub mod $name {
                    /// Version 1 of the OTEL specification
                    pub mod v1 {
                        include!(concat!(
                            env!("OUT_DIR"),
                            "/proto/opentelemetry.proto.collector.",
                            stringify!($name),
                            ".v1.rs"
                        ));
                    }
                }
            )*
        }
    };
}

pub mod common {
    pub mod v1 {
        include!(concat!(
            env!("OUT_DIR"),
            "/proto/opentelemetry.proto.common.v1.rs"
        ));
    }
}

pub mod resource {
    pub mod v1 {
        include!(concat!(
            env!("OUT_DIR"),
            "/proto/opentelemetry.proto.resource.v1.rs"
        ));
    }
}

// Include all the OpenTelemetry components
include_components![trace, metrics, logs];
