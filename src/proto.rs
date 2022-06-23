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
