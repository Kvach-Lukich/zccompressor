fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon(r"assets\zc.ico");
        res.set("ProductName", "Zero-cross compressor");
        res.set("FileDescription", "Audio compressor with zero-crossing and dynamic range analysis");
        res.compile().expect("Failed to compile Windows resources");
    }
}