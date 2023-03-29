#[cfg(feature = "headers")]
use wasmi_c_api::generate_headers;

#[cfg(feature = "headers")]
fn main() -> ::std::io::Result<()> {
    generate_headers()
}

#[cfg(not(feature = "headers"))]
fn main() -> ::std::io::Result<()> {
    Ok(())
}