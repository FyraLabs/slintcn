# slintcn

> [!NOTE]
> Fyra slintcn is still a work in progress. Many things are missing and will change dramatically. The API is not stable, and the components are not complete. Use it at your own risk.

slintcn is a shadcn-ish component library for the [Slint](https://slint.dev) UI framework, inspired by the now-removed slintcn library by zero-sq.

## How it works

slintcn is a set of components (Button, Card, Alert, Dialog, …) with shadcn's look and API, imported from `@slintcn/…`. They sit alongside Slint's own widgets and don't replace them. Mix them with anything from `std-widgets.slint`, under any Slint style.

Like shadcn, you don't depend on slintcn as a package. You copy its files into your project and edit them freely. To change colors, edit `styling.slint`. To change how a button looks, edit `button.slint`.

The components always use shadcn's colors. They follow the system's dark/light setting (through Slint's [`Palette`](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/globals/palette/)) unless your app picks a mode.

## Usage

> [!NOTE]
> There's no CLI yet, so setup is manual.

### 1. Copy the components into your project

Copy [`src/components/`](src/components/) into your app as `ui/slintcn/`:

```
my-app/
  build.rs
  ui/
    main.slint
    slintcn/
      styling.slint      colors, radii and icons
      _x.svg             icon used by styling.slint
      button.slint, card.slint, alert.slint, dialog.slint, …
```

You can copy single components. Every component needs `styling.slint`. Keep `_x.svg` next to it, since the Dialog's close button uses it. AlertDialog also needs `dialog.slint` and `button.slint`. Checkbox needs `_check.svg` next to `checkbox.slint`. Input needs `button.slint` for its password visibility button. Copy `field.slint` when composing controls with labels, descriptions, or errors.

### 2. Map the `@slintcn` library

Rust (`build.rs`):

```rust
use std::{collections::HashMap, path::PathBuf};

fn main() {
    let ui = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("ui");
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(HashMap::from([("slintcn".to_string(), ui.join("slintcn"))]));
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
```

`slint-viewer`:

```sh
slint-viewer -Lslintcn=ui/slintcn --auto-reload ui/main.slint
```

C++ (`LIBRARY_PATHS` in `slint_target_sources`), Node.js (`libraryPaths`) and Python (`library_paths`) have the same setting; see [Slint's docs](https://docs.slint.dev/latest/docs/slint/guide/language/coding/file/#component-libraries). Untested so far.

### 3. Editor setup

Zed (`.zed/settings.json`):

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

Restart the language server after changing the setting. VS Code takes the same value under `"slint.libraryPaths"` in `.vscode/settings.json` (untested).

### 4. Write your UI

```slint
import { Button, ButtonVariant } from "@slintcn/button.slint";
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from "@slintcn/card.slint";
import { SlintcnPalette } from "@slintcn/styling.slint";

export component App inherits Window {
    background: SlintcnPalette.background;

    Card {
        CardHeader { CardTitle { text: "Hello"; } }
        CardContent { Text { text: "Card content."; color: SlintcnPalette.card-foreground; } }
        CardFooter {
            spacing: 8px;
            Button { text: "Save"; }
            Button { text: "Cancel"; variant: ButtonVariant.outline; }
        }
    }
}
```

Set the window's `background` and the color of plain `Text` yourself, as above. Both come from the app's Slint style by default, not from slintcn.

## Dark and light mode

The components follow Slint's `Palette.color-scheme`, which follows the system. To force a mode:

```slint
import { SlintcnPalette } from "@slintcn/styling.slint";

export component App inherits Window {
    init => { SlintcnPalette.color-scheme = ColorScheme.dark; }
}
```

## Updating

There's no automatic upgrade. Compare your copy with the latest `src/components/` and merge the changes you want.

## Components

| Component | Import | Status |
|---|---|---|
| `Alert`, `AlertVariant` | `@slintcn/alert.slint` | draft |
| `AlertDialog` | `@slintcn/alert-dialog.slint` | draft |
| `Button`, `ButtonVariant`, `ButtonSize` | `@slintcn/button.slint` | draft |
| `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `CardFooter` | `@slintcn/card.slint` | draft |
| `Checkbox` | `@slintcn/checkbox.slint` | draft |
| `Dialog`, `DialogFooter` | `@slintcn/dialog.slint` | draft |
| `SlintcnPalette`, `SlintcnSizeSettings` (colors, dark/light, radii) | `@slintcn/styling.slint` | draft |
| `Field`, `FieldContent`, `FieldLabel`, `FieldTitle`, `FieldDescription`, `FieldError`, `FieldGroup`, `FieldSet`, `FieldLegend`, `FieldSeparator`, `FieldOrientation`, `FieldLegendVariant` | `@slintcn/field.slint` | draft |
| `Input` | `@slintcn/input.slint` | draft |
| `ScrollArea`, `ScrollBar` | `@slintcn/scroll-area.slint` | draft |
| `Switch`, `SwitchSize` | `@slintcn/switch.slint` | draft |

## Developing slintcn

See [HACKING.md](HACKING.md) for how the components work and how to add to them.

- `src/components/`: the component library.
- `examples/gallery/`: a Rust app that uses it the way a user's app would.

```sh
cargo run -p slintcn-gallery
```

The gallery builds with Slint's live preview, MCP server and UI testing support on by default; see [HACKING.md](HACKING.md). Add `--no-default-features` to turn them off.

## License

GPL-3.0-or-later. Copying slintcn's files into an app makes that app a derivative work, so it must be GPL-compatible.

Based on the original [slintcn](https://github.com/yfn020/slintcn) by Steve Kwon (MIT).
