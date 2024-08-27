fn main() -> eyre::Result<()> {
    tonic_build::compile_protos("../proto/grift.proto")?;
    Ok(())
}
