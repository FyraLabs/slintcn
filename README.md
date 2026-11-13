# slintcn

> [!NOTE]
> Fyra slintcn is a work in progress. Expect missing components and API changes.

slintcn is a shadcn-ish component library for the [Slint](https://slint.dev) UI framework, inspired by the now-removed slintcn library by zero-sq.

## How it works

slintcn is a collection of Slint components that use standardized styling and palettes, reusable across apps. They are designed to be used a template for your own components that can be freely copied and modified.

## Usage

> [!NOTE]
> A CLI is planned in the future to help import and update the components, but for now you'll need to manually copy them

### 1. Copy templates

Simply copy the templates from `src/components` into your repository. You may select any subset of components you want to use.

### 2. Map the library paths

Rust (`build.rs`):

```rust
use std::{collections::HashMap, path::PathBuf};

fn main() {
    let ui = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("ui");
    // map `@slintcn` to `ui/slintcn`
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(HashMap::from([("slintcn".to_string(), ui.join("slintcn"))]));

    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
```

or with `slint-viewer`:

```sh
slint-viewer -Lslintcn=ui/slintcn --auto-reload ui/main.slint
```

## 3. Editor setup

for the Zed Slint plugin, you can add this to your `.zed/settings.json`:

```json
{
    "lsp": {
        "slint": {
            "settings": {
                "slint": {
                    "libraryPaths": { "slintcn": "ui/slintcn" }
                }
            }
        }
    }
}
```

Restart the language server after changing the setting. VS Code uses `"slint.libraryPaths"` in `.vscode/settings.json` (untested).


## Documentation

See the [component documentation](docs/components/) for all components and usage examples.
