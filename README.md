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

### Checkbox

```slint
import { Checkbox } from "@slintcn/checkbox.slint";

Checkbox {
    accessible-label: "Accept terms and conditions";
    toggled(checked) => { debug("Terms accepted:", checked); }
}
```

Bind `checked` with `<=>` to share state with your app. `toggled(bool)` fires on user activation, not programmatic changes. Use `enabled: false` to disable it and `invalid: true` for validation styling. Place labels and descriptions alongside the checkbox; `accessible-label` gives it a screen-reader label. Mouse, Space/Enter, and accessibility activation toggle it; Tab focus shows a focus ring.

### Switch

```slint
import { Switch, SwitchSize } from "@slintcn/switch.slint";

Switch {
    accessible-label: "Airplane mode";
    checked: false;
    size: SwitchSize.default;
    toggled(checked) => { debug("Airplane mode:", checked); }
}
```

Bind `checked` with `<=>` to share state with your app. `toggled(bool)` fires on user activation, not programmatic changes. Use `SwitchSize.sm` for the smaller switch, `enabled: false` to disable it, and `invalid: true` for the validation ring. The switch supports mouse, Space/Enter, and accessibility activation; Tab focus shows a focus ring.

### Scroll area

```slint
import { ScrollArea } from "@slintcn/scroll-area.slint";
import { SlintcnPalette } from "@slintcn/styling.slint";

ScrollArea {
    width: 350px;
    height: 200px;
    content-width: self.visible-width;
    content-height: content.preferred-height;

    content := VerticalLayout {
        padding: 16px;
        spacing: 12px;
        for i in 50: Text {
            text: "v1.2.0-beta.\{50 - i}";
            color: SlintcnPalette.foreground;
            font-size: 14px;
        }
    }
}
```

Give the scroll area a bounded size. Slint's `Flickable` handles wheel, touchpad, and touch scrolling; the themed 10px overlay scrollbars appear on either axis when needed. `vertical-scrollbar-policy` and `horizontal-scrollbar-policy` accept Slint's `ScrollBarPolicy` values. Hiding a scrollbar does not disable scrolling.

Bind `content-x`/`content-y` to share the scroll position (zero at the start, negative when scrolled). `visible-width`/`visible-height` expose the viewport size, and `scrolled()` reports user scrolling. Tab focuses the viewport with a focus ring; arrows, Page Up/Down, and Home/End scroll it. Keys handled by child controls stay with those controls.

`ScrollBar` is also exported for custom compositions. Set `orientation: Orientation.horizontal` for a horizontal bar, provide `content-size` and `page-size`, and bind its negative-offset `value`. Its thumb supports dragging and clicking the track.

### Field

[Field](https://ui.shadcn.com/docs/components/base/field) composes labels, descriptions, errors, and ordinary controls. `FieldSet` and `FieldLegend` group related fields; `FieldGroup` spaces them. `FieldContent` groups text beside a control.

```slint
import { Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSet } from "@slintcn/field.slint";
import { Input } from "@slintcn/input.slint";

FieldSet {
    accessible-label: "Profile";
    FieldLegend { text: "Profile"; }
    FieldGroup {
        Field {
            name-label := FieldLabel {
                text: "Display name";
                clicked => { name-input.focus(); }
            }
            name-input := Input {
                placeholder-text: "Your name";
                accessible-label: name-label.text;
                accessible-description: name-description.text;
            }
            name-description := FieldDescription {
                text: "This is how your name appears to others.";
            }
        }
    }
}
```

`Field.orientation` accepts `FieldOrientation.vertical` (default), `horizontal`, or `responsive`. Responsive fields become horizontal at 448px. Bind `responsive-width` to the containing `FieldGroup.width` for container-based behavior. It defaults to zero, so responsive fields stay vertical until a container width is supplied. Use `FieldLegendVariant.label` for a smaller legend. `FieldSeparator.text` adds optional text over the dividing line.

`FieldLabel.clicked` connects a label to its control. For a switch or checkbox, call the control's `focus()` and `accessible-action-default()` functions to preserve its `toggled(bool)` callback. The control remains the keyboard target. Bind its `accessible-label` and `accessible-description` explicitly.

Choice cards wrap a `Field` in a `FieldLabel` with `bordered: true`. Bind the label's `selected` to the control's `checked`; place a `FieldTitle` and `FieldDescription` inside `FieldContent`. Set `cross-axis-alignment: start` on horizontal fields with multi-line content to align the control at the top. The label's background click activates the control without covering its own hit area.

Bind `enabled` and `invalid` explicitly on the field, label, and control. `Field` does not propagate these states to descendants. `FieldError.text` displays a validation message. Show it conditionally when its message is nonempty. For multiple errors, remove empty and duplicate messages in app code and join them with newlines into one `FieldError`, which provides a single assertive live region. Include those messages in the control's `accessible-description`. Input's existing `invalid` property does not yet change its border.

For nested groups, override `FieldGroup.spacing` to `16px`; Slint cannot apply ancestor-based styles automatically.

`FieldTitle` is non-interactive. Use it for headings within choice cards, and `FieldLabel` for clickable labels. Controls keep their own state and callbacks; there are no per-control card wrappers.

For wrapped text inside `Field`, use these text components or a named component inheriting `Text`. Inline `Text { wrap: word-wrap; }` can trigger a binding loop in Slint 1.18's dynamic FlexboxLayout.

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
