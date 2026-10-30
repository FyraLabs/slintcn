# slintcn

> [!NOTE]
> Fyra slintcn is still a work in progress. many things are missing and will change dramatically. The API is not stable, and the components are not complete. Use it at your own risk.

slintcn is a shadcn-ish component library for the [Slint](https://slint.dev) UI framework, inspired by the now-removed slintcn library by zero-sq


## How it works

slintcn has two parts:

1. **Components** ([`src/components/`](src/components/)): widgets Slint doesn't have (Card, Alert, Dialog, …), imported from `@slintcn/…`. They take their colors from the active [widget style](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/style/)'s [`Palette`](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/palette/), so they fit in with Fluent, Cupertino, Material, Cosmic, Qt, or your own style, in dark and light mode.
2. **The slintcn style** ([`src/styles/slintcn/`](src/styles/slintcn/)): a Slint widget style based on shadcn's look. Select it and Slint's standard widgets (`Button`, `LineEdit`, `CheckBox`, …) from `std-widgets.slint` look like shadcn. Your code uses the normal Slint API, so switching styles is a build setting. It's also a template: copy it to make your own style.

Like shadcn, you don't depend on slintcn as a package. You copy its files into your project and they become yours to edit.

You can use either part on its own:

| You want | Copy |
|---|---|
| slintcn's components with a stock Slint style | `src/components/` |
| The shadcn look everywhere | `src/components/` and `src/styles/slintcn/` |
| Your own style | `src/components/`, plus `src/styles/slintcn/` renamed and edited (see [Making your own style](#making-your-own-style)) |

## Usage

> [!NOTE]
> There's no CLI yet, so setup is manual.

### 1. Copy the files into your project

```
my-app/
  build.rs
  ui/
    main.slint
    slintcn/               copy of src/components/
      card.slint, alert.slint, dialog.slint, …
      styling.slint        the components' colors (from Palette) and radii
    styles/                only if you use the slintcn style
      slintcn/             copy of src/styles/slintcn/
        std-widgets.slint  style entry point
        style-base.slint
        styling.slint      colors and radii
        std/               standard widgets (Button, …)
```

### 2. Configure the build

Rust (`build.rs`):

```rust
use std::{collections::HashMap, path::PathBuf};

fn main() {
    let ui = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("ui");
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(HashMap::from([("slintcn".to_string(), ui.join("slintcn"))]))
        // Only for the slintcn style; leave these two lines out to use a stock style.
        .with_style("slintcn".to_string())
        .with_include_paths(vec![ui.join("styles"), ui.join("styles/slintcn")]);
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
```

- The library path makes `@slintcn/…` imports find the components.
- `with_style` selects the style.
- The include paths let Slint find it. `ui/styles` is where Slint checks that a `slintcn/` style exists; `ui/styles/slintcn` is where `import … from "std-widgets.slint"` is resolved. The `ui/styles/slintcn` include path is what actually applies the style: to switch to another style, remove it as well as changing `with_style`.

`slint-viewer`:

```sh
slint-viewer -Lslintcn=ui/slintcn --auto-reload ui/main.slint     # stock style
slint-viewer -I "$PWD/ui/styles" -I "$PWD/ui/styles/slintcn" -Lslintcn=ui/slintcn --style slintcn --auto-reload ui/main.slint
```

Use absolute include paths. Slint resolves relative include paths from the folder of the importing file, not from your working directory.

C++, Node.js and Python have the same settings (library paths, style, include paths); see Slint's docs for each. Untested so far.

### 3. Editor setup

Zed (`.zed/settings.json`), with the slintcn style:

```json
{
    "lsp": {
        "slint": {
            "settings": {
                "slint": {
                    "includePaths": ["ui/styles", "ui/styles/slintcn", "styles/slintcn", "../styles/slintcn"],
                    "libraryPaths": { "slintcn": "ui/slintcn" },
                    "preview": { "style": "slintcn" }
                }
            }
        }
    }
}
```

The include paths are relative on purpose, so the file works on any machine. Slint resolves them from the importing file's folder: Zed runs the language server from the project root, which covers `"ui/styles"` and `"ui/styles/slintcn"`; `"styles/slintcn"` covers files in `ui/`; `"../styles/slintcn"` covers the components in `ui/slintcn/`. Files in other folders need an extra entry. With a stock style, only `libraryPaths` is needed. Restart the language server after changing this.

VS Code: the same values under `"slint.includePaths"`, `"slint.libraryPaths"` and `"slint.preview.style"` in `.vscode/settings.json`. Untested.

Once Slint 1.19 ships `slint-project.json`, most of this should move there.

### 4. Write your UI

```slint
import { Button, LineEdit } from "std-widgets.slint"; // standard widgets, in the active style
import { Card } from "@slintcn/card.slint";           // slintcn component

export component App inherits Window {
    // …
}
```

(`LineEdit` doesn't exist in the slintcn style yet; see [Components](#components).)

### Dark and light mode

Stock styles follow the system setting, and the components follow the style. The slintcn style starts in dark mode, because Slint doesn't let custom styles read the system setting, so switch it yourself:

```slint
import { Palette } from "std-widgets.slint";

export component App inherits Window {
    init => { Palette.color-scheme = ColorScheme.light; }
}
```

### Updating

There's no automatic upgrade. compare them with the latest `src/components/` and `src/styles/slintcn/` and merge your desired changes.

## Making your own style

1. Copy `src/styles/slintcn/` as a template into your project as `ui/styles/myapp/`. The folder name is the style name.
2. Change the colors and radii in `styling.slint`. `style-base.slint` maps them onto Slint's `Palette`, which the components read, so the components follow your colors too.
3. Change how standard widgets look in `std/`. Keep each widget's API exactly as Slint documents it.
4. Select it with `with_style("myapp")` and the include paths `ui/styles` and `ui/styles/myapp`.

See [HACKING.md](HACKING.md) for how a style is put together.

## Components

### Standard widgets in the slintcn style (std-widgets API)

| Widget | Status |
|---|---|
| `Palette`, `StyleMetrics` | draft |
| `Button` | draft |

Everything else in `std-widgets.slint` is still to do. Until a widget exists, importing it with the slintcn style fails with `No exported type called '…' found in "std-widgets.slint"`.

### Components (`@slintcn/…`, any style)

| Component | Import | Status |
|---|---|---|
| `Alert`, `AlertVariant` | `@slintcn/alert.slint` | draft |
| `AlertDialog` | `@slintcn/alert-dialog.slint` | draft |
| `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `CardFooter` | `@slintcn/card.slint` | draft |
| `Dialog`, `DialogFooter` | `@slintcn/dialog.slint` | draft |
| `ComponentPalette`, `ComponentSizeSettings` (the components' colors and radii) | `@slintcn/styling.slint` | draft |

## Developing slintcn

See [HACKING.md](HACKING.md) for how the style and components work and how to add to them.

- `src/components/`: the component library.
- `src/styles/slintcn/`: the slintcn style.
- `examples/gallery/`: a Rust app that uses both the way a user's app would.

```sh
cargo run -p slintcn-gallery                       # slintcn style
SLINT_STYLE=fluent cargo run -p slintcn-gallery    # any stock style
```

## License

GPL-3.0-or-later. Copying slintcn's files into an app makes that app a derivative work, so it must be GPL-compatible.

Based on the original [slintcn](https://github.com/yfn020/slintcn) by Steve Kwon (MIT).
