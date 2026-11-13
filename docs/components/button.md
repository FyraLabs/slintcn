# Button

**Import:** [`@slintcn/button.slint`](../../src/components/button.slint)

`Button` supports text, an optional icon, six visual variants, and four sizes.

```slint
import { Button, ButtonSize, ButtonVariant } from "@slintcn/button.slint";

VerticalLayout {
    spacing: 8px;
    alignment: start;

    Button {
        text: "Create project";
        clicked => { create-project(); }
    }
    Button {
        text: "Delete";
        variant: ButtonVariant.destructive;
        clicked => { delete-project(); }
    }
    Button {
        text: "Settings";
        icon: @image-url("settings.svg");
        variant: ButtonVariant.outline;
        size: ButtonSize.sm;
    }
}
```

The `link` variant underlines nonempty text on hover. The `icon` size has no horizontal padding, so give icon-only buttons an `accessible-label`.

```slint
Button {
    icon: @image-url("settings.svg");
    size: ButtonSize.icon;
    accessible-label: "Settings";
}
```

**API**

| Property | Type | Default |
|---|---|---|
| `text` | `string` | `""` |
| `icon` | `image` | empty |
| `variant` | `ButtonVariant` | `ButtonVariant.default` |
| `size` | `ButtonSize` | `ButtonSize.default` |
| `enabled` | `bool` | `true` |
| `group-position` | `GroupPosition` | `GroupPosition.none` |
| `group-orientation` | `Orientation` | `Orientation.horizontal` |
| `has-focus` | `bool` | read-only output |
| `pressed` | `bool` | read-only output |

`ButtonVariant` provides `default`, `destructive`, `outline`, `secondary`, `ghost`, and `link`. `ButtonSize` provides `default`, `sm`, `lg`, and `icon`.

**Callback**

`clicked()` runs on pointer activation, the accessible default action, Space, or Return.

Use the [ButtonGroup](button-group.md) layout and `group-position` to join adjacent buttons.
