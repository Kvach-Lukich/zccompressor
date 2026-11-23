fn main() {
    #[cfg(target_os = "windows")]
    compile_windows_resources();
}

#[cfg(target_os = "windows")]
fn compile_windows_resources() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon(r"assets\zc.ico");
    res.set("ProductName", "Zero-cross compressor");
    res.set("FileDescription", "Audio compressor with zero-crossing and dynamic range analysis");
    res.compile().expect("Failed to compile Windows resources");
}