# slintcn

slintcn is a shadcn-ish component library for the [Slint](https://slint.dev) UI framework.

> **Draft.** slintcn is being rewritten from scratch. Only the theme, `Button`, `Alert` and `Card` exist so far.

## How it works

slintcn has two parts, shipped together in one folder:

1. **A Slint widget style.** With the `slintcn` style selected, Slint's standard widgets (`Button`, `LineEdit`, `CheckBox`, …) from `std-widgets.slint` get the shadcn look. Your code uses the normal Slint API, so switching back to `fluent` or `material` is a one-line change.
2. **Extra components** that Slint doesn't have (Card, Badge, Dialog, …), imported from `@slintcn/…`.

Like shadcn, you don't depend on slintcn as a package. You copy its files into your project and they become yours to edit. To change colors, edit `styling.slint`. To change how a button looks, edit `button.slint`.

## Usage

There's no CLI yet, so setup is manual.

### 1. Copy the style into your project

Copy the [`slintcn/`](slintcn/) folder into your app's `ui/` folder:

```
my-app/
  build.rs
  ui/
    main.slint
    slintcn/
      std-widgets.slint
      style-base.slint
      styling.slint
      …
```

The folder name is the style name. To rename the style (e.g. `myapp`), rename the folder and use that name everywhere below.

### 2. Select the style when compiling

Rust (`build.rs`):

```rust
use std::{collections::HashMap, path::PathBuf};

fn main() {
    let ui = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("ui");
    let slintcn = ui.join("slintcn");
    let config = slint_build::CompilerConfiguration::new()
        .with_style("slintcn".to_string())
        .with_include_paths(vec![ui.clone(), slintcn.clone()])
        .with_library_paths(HashMap::from([("slintcn".to_string(), slintcn)]));
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
```

All three settings are needed:

- `with_style` selects the style.
- The include paths let Slint find it. `ui` is where Slint checks that a `slintcn/` style exists; `ui/slintcn` is where `import … from "std-widgets.slint"` is resolved.
- The library path makes `@slintcn/…` imports work for the extra components.

`slint-viewer`:

```sh
slint-viewer -I "$PWD/ui" -I "$PWD/ui/slintcn" -Lslintcn=ui/slintcn --style slintcn --auto-reload ui/main.slint
```

Use absolute include paths. Slint resolves relative include paths from the folder of the importing file, not from your working directory.

C++, Node.js and Python have the same three settings (style, include paths, library paths); see Slint's docs for each. Untested so far.

### 3. Editor setup

Zed (`.zed/settings.json`):

```json
{
    "lsp": {
        "slint": {
            "settings": {
                "slint": {
                    "includePaths": ["ui", "ui/slintcn", "slintcn"],
                    "libraryPaths": { "slintcn": "ui/slintcn" },
                    "preview": { "style": "slintcn" }
                }
            }
        }
    }
}
```

These include paths are relative on purpose and work for `.slint` files directly in `ui/`. Zed runs the language server from the project root, which covers `"ui"` and `"ui/slintcn"`; `"slintcn"` covers imports from files in `ui/`. Files in other folders need an extra relative entry. Restart the language server after changing this.

VS Code: the same values under `"slint.includePaths"`, `"slint.libraryPaths"` and `"slint.preview.style"` in `.vscode/settings.json`. Untested.

Once Slint 1.19 ships `slint-project.json`, most of this should move there.

### 4. Write your UI

```slint
import { Button, LineEdit } from "std-widgets.slint"; // standard widgets, slintcn look
import { Card } from "@slintcn/card.slint";           // slintcn extra

export component App inherits Window {
    // …
}
```

(`LineEdit` doesn't exist yet; see [Components](#components).)

### Dark and light mode

The style starts in dark mode. Slint doesn't let custom styles read the system's dark/light setting, so switch it yourself:

```slint
import { Palette } from "std-widgets.slint";

export component App inherits Window {
    init => { Palette.color-scheme = ColorScheme.light; }
}
```

### Updating

There's no automatic upgrade. The files are yours, so compare them with the latest `slintcn/` and merge what you want.

## Components

### Standard widgets (std-widgets API)

| Widget | Status |
|---|---|
| `Palette`, `StyleMetrics` | draft |
| `Button` | draft |

Everything else in `std-widgets.slint` is still to do. Until a widget exists, importing it fails with `No exported type called '…' found in "std-widgets.slint"`.

### Extras (`@slintcn/…`)

| Component | Import | Status |
|---|---|---|
| `SlintcnPalette` (shadcn colors: `primary`, `muted`, `ring`, …), `SlintcnSizeSettings` (`radius-md`, …) | `@slintcn/styling.slint` | draft |
| `Alert`, `AlertVariant` | `@slintcn/alert.slint` | draft |
| `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `CardFooter` | `@slintcn/card.slint` | draft |

## Developing slintcn

See [HACKING.md](HACKING.md) for how the style works and how to add components.

- `slintcn/`: the style and components (what users copy).
- `examples/gallery/`: a Rust app that uses `slintcn/` the way a user's app would.

```sh
cargo run -p slintcn-gallery
slint-viewer -I "$PWD" -I "$PWD/slintcn" -Lslintcn=slintcn --style slintcn --auto-reload examples/gallery/ui/main.slint
```

## License

GPL-3.0-or-later. Copying slintcn's files into an app makes that app a derivative work, so it must be GPL-compatible.

Based on the original [slintcn](https://github.com/yfn020/slintcn) by Steve Kwon (MIT).
