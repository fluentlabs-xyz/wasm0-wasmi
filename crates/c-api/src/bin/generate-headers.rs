#[cfg(feature = "headers")]
use wasmi_c_api;

// temporarily comment '[lib]' section in Cargo.toml to make it work
#[cfg(feature = "headers")]
pub fn generate_headers() -> std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("./packaged/include/wasmi.h")?
        .generate()
}

#[cfg(feature = "headers")]
fn main() -> std::io::Result<()> {
    generate_headers()
}

#[cfg(not(feature = "headers"))]
fn main() -> ::std::io::Result<()> {
    Ok(())
}