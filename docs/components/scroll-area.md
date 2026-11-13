# ScrollArea

**Import:** [`@slintcn/scroll-area.slint`](../../src/components/scroll-area.slint)

`ScrollArea` places children in a scrolling viewport with optional vertical and horizontal scrollbars. It provides no content size of its own, so set both dimensions from the content.

```slint
import { ScrollArea } from "@slintcn/scroll-area.slint";

Rectangle {
    width: 320px;
    height: 240px;

    ScrollArea {
        content-width: max(self.visible-width, content.min-width);
        content-height: content.preferred-height;

        content := VerticalLayout {
            padding: 16px;
            alignment: start;
            spacing: 8px;
        }
    }
}
```

`content-width` and `content-height` determine the scrollable range. A dimension equal to the visible extent does not scroll. The gallery uses `max(self.visible-width, content.min-width)` for width so content does not overflow horizontally, and the content's preferred height for vertical scrolling.

Set `vertical-scrollbar-policy` and `horizontal-scrollbar-policy` to `ScrollBarPolicy.always`, `as-needed`, or `hidden`. Mouse dragging is disabled by default; set `mouse-drag-pan-enabled: true` to allow it. Arrow keys, Page Up, Page Down, Home, and End also scroll when the area has focus.

**API**

| Property | Type | Default |
|---|---|---|
| `content-width` | `length` | `0px` |
| `content-height` | `length` | `0px` |
| `content-x` | `length` | `0px` |
| `content-y` | `length` | `0px` |
| `mouse-drag-pan-enabled` | `bool` | `false` |
| `vertical-scrollbar-policy` | `ScrollBarPolicy` | `as-needed` |
| `horizontal-scrollbar-policy` | `ScrollBarPolicy` | `as-needed` |
| `visible-width` | `length` | read-only output |
| `visible-height` | `length` | read-only output |
| `has-focus` | `bool` | read-only output |

**Callback**

`scrolled()` runs when the visible position changes.

`ScrollBar` is exported for custom scroll layouts. It accepts `orientation`, `policy`, `enabled`, `content-size`, `page-size`, and two-way `value`, and emits `scrolled()`. Most applications should use `ScrollArea`.
