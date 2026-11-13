# ButtonGroup

**Import:** [`@slintcn/button-group.slint`](../../src/components/button-group.slint)

`ButtonGroup` arranges components in a zero-spacing row or column. It handles layout only; give each adjacent child a `group-position` from `styling.slint` so their borders and corners join.

```slint
import { Button, ButtonVariant } from "@slintcn/button.slint";
import { ButtonGroup } from "@slintcn/button-group.slint";
import { GroupPosition } from "@slintcn/styling.slint";

ButtonGroup {
    Button {
        text: "Preview";
        variant: ButtonVariant.outline;
        group-position: GroupPosition.first;
    }
    Button {
        text: "Edit";
        variant: ButtonVariant.outline;
        group-position: GroupPosition.middle;
    }
    Button {
        text: "Share";
        variant: ButtonVariant.outline;
        group-position: GroupPosition.last;
    }
}
```

`GroupPosition.first` joins the right edge, `middle` joins both edges, and `last` joins the left edge. For a vertical group, set `orientation: Orientation.vertical` on `ButtonGroup` and `group-orientation: Orientation.vertical` on every child. The children do not inherit the orientation.

`ButtonGroupText` shows a label and optional icon inside a grouped box. It is commonly used between inputs and buttons:

```slint
import { ButtonGroup, ButtonGroupText } from "@slintcn/button-group.slint";
import { Input } from "@slintcn/input.slint";
import { GroupPosition } from "@slintcn/styling.slint";

ButtonGroup {
    ButtonGroupText {
        text: "https://";
        group-position: GroupPosition.first;
    }
    Input {
        placeholder-text: "example";
        group-position: GroupPosition.middle;
    }
    ButtonGroupText {
        text: ".com";
        group-position: GroupPosition.last;
    }
}
```

**API**

`ButtonGroup` inherits `FlexboxLayout` and adds `orientation: Orientation` (default `horizontal`).

| `ButtonGroupText` property | Type | Default |
|---|---|---|
| `text` | `string` | `""` |
| `icon` | `image` | empty |
| `group-position` | `GroupPosition` | `GroupPosition.none` |
| `group-orientation` | `Orientation` | `Orientation.horizontal` |

`ButtonGroupSeparator` accepts `orientation: Orientation`, defaulting to `Orientation.vertical`. A vertical separator stretches to the group's height; a horizontal one stretches across its width.
