# Styling

The shared tokens live in [`styling.slint`](../../src/components/styling.slint). Every component depends on this file, and the Dialog close icon uses `_x.svg` beside it.

```slint
import { SlintcnPalette, SlintcnSizeSettings } from "@slintcn/styling.slint";

Rectangle {
    background: SlintcnPalette.background;
    border-radius: SlintcnSizeSettings.radius-lg;
}
```

## Color scheme

`SlintcnPalette.color-scheme` follows Slint's `Palette.color-scheme` until an application overrides it.

```slint
export component App inherits Window {
    init => { SlintcnPalette.color-scheme = ColorScheme.dark; }
}
```

The color tokens are read-only outputs. To change a color, edit `styling.slint` in your copy.

| Token | Purpose |
|---|---|
| `background` | Application background |
| `foreground` | Default text |
| `card`, `card-foreground` | Card surfaces and their text |
| `popover`, `popover-foreground` | Popup surfaces and their text |
| `primary`, `primary-foreground` | Primary buttons and matching text |
| `secondary`, `secondary-foreground` | Secondary surfaces and text |
| `muted`, `muted-foreground` | Subdued surfaces and text |
| `accent`, `accent-foreground` | Hover and selected surfaces |
| `destructive` | Validation and destructive actions |
| `border` | Component borders |
| `input` | Input and control borders |
| `ring` | Keyboard focus rings |

## Sizes and positions

`SlintcnSizeSettings` provides read-only length tokens: `radius`, `radius-sm`, `radius-md`, `radius-lg`, and `radius-xl`. Edit their values in source rather than overriding them.

`GroupPosition` provides `none`, `first`, `middle`, and `last` for joined Button, Input, and ButtonGroupText edges. `PopupSide` provides `bottom`, `top`, `left`, and `right`. `PopupAlign` provides `center`, `start`, and `end`.

`SlintcnIcons.x` is the decorative close icon used by Dialog. Import `SlintcnIcons` only when building a custom close control.
