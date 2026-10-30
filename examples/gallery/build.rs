use std::{collections::HashMap, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=SLINT_STYLE");
    let style = std::env::var("SLINT_STYLE").unwrap_or_else(|_| "slintcn".to_string());

    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.join("../..");
    let slintcn = repo_root.join("slintcn");

    let mut include_paths = vec![repo_root];
    if style == "slintcn" {
        // `std-widgets.slint` is resolved from include paths before the built-in styles, so this folder is what applies the style.
        include_paths.push(slintcn.clone());
    }

    let config = slint_build::CompilerConfiguration::new()
        .with_style(style)
        .with_include_paths(include_paths)
        .with_library_paths(HashMap::from([("slintcn".to_string(), slintcn)]));
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
