fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/inventoryserviceapi/inventoryserviceapi.generated.proto",
                "proto/inventoryserviceapi/processorderitem/processorderitem.proto",
            ],
            &["."],
        )?;
    println!(
        "cargo:rerun-if-changed=proto/inventoryserviceapi/inventoryserviceapi.generated.proto"
    );
    println!(
        "cargo:rerun-if-changed=proto/inventoryserviceapi/processorderitem/processorderitem.proto"
    );
    Ok(())
}
