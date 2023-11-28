extern crate prost_build;

fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.include_file("_includes.rs");
    prost_build.compile_protos(
        &["src/k8s.io/autoscaler/cluster-autoscaler/cloudprovider/externalgrpc/protos/externalgrpc.proto"],
        &["src/"],
    )
    .unwrap();
}
