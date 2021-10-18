fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("======");
    tonic_build::compile_protos("proto/shortlink.proto")?;
    Ok(())
}