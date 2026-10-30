use std::{collections::HashMap, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.join("../..");
    let slintcn = repo_root.join("slintcn");
    let config = slint_build::CompilerConfiguration::new()
        .with_style("slintcn".to_string())
        // The style name is looked up as `<path>/slintcn/std-widgets.slint`, but imports resolve against the folder itself.
        .with_include_paths(vec![repo_root, slintcn.clone()])
        .with_library_paths(HashMap::from([("slintcn".to_string(), slintcn)]));
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
