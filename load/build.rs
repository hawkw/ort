fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = &["../proto/ort.proto"];
    let dirs = &["../proto"];

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(files, dirs)?;

    // recompile protobufs only if any of the proto files changes.
    for file in files {
        println!("cargo:rerun-if-changed={}", file);
    }

    Ok(())
}
