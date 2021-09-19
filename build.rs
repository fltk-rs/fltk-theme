#[cfg(target_os = "macos")]
fn main() {
    let mut build = cc::Build::new();
    build.file("src/cocoa_helper.m");
    build.compile("cocoa");
}

#[cfg(not(target_os = "macos"))]
fn main() {}
