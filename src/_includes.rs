pub mod k8s.io {
    pub mod apimachinery {
        pub mod pkg {
            pub mod apis {
                pub mod meta {
                    pub mod v1 {
                        include!(concat!(
                            env!("OUT_DIR"),
                            "/clusterautoscaler.cloudprovider.v1.externalgrpc.rs"
                        ));
                    }
                }
            }
        }
    }
}
