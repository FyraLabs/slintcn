use std::{collections::HashMap, path::PathBuf};

fn main() {
    if std::env::var_os("CARGO_FEATURE_LIVE_PREVIEW").is_some() {
        unsafe { std::env::set_var("SLINT_LIVE_PREVIEW", "1") };
    }

    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let components = manifest_dir.join("../../src/components");
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(HashMap::from([("slintcn".to_string(), components)]));
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
