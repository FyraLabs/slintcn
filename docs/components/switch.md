# Switch

**Import:** [`@slintcn/switch.slint`](../../src/components/switch.slint)

`Switch` is a two-state toggle. It owns `checked` and emits `toggled` after changing it.

```slint
import { Switch, SwitchSize } from "@slintcn/switch.slint";

notifications := Switch {
    checked: true;
    size: SwitchSize.sm;
    accessible-label: "Enable notifications";
    toggled(checked) => {
        if checked { subscribe(); }
    }
}
```

A parent can initialize the switch or bind the value:

```slint
Switch {
    checked <=> root.dark-mode;
    accessible-label: "Dark mode";
}
```

Disabled switches neither change state nor emit `toggled`. `invalid: true` adds a destructive ring. The switch toggles with a pointer, Space, or Return, and shows a focus ring for keyboard focus.

**API**

| Property | Type | Default |
|---|---|---|
| `checked` | `bool` | `false` |
| `enabled` | `bool` | `true` |
| `invalid` | `bool` | `false` |
| `size` | `SwitchSize` | `SwitchSize.default` |
| `has-focus` | `bool` | read-only output |
| `pressed` | `bool` | read-only output |

**Callback**

`toggled(checked)` runs after the value changes, with the new value.

`SwitchSize` provides `default` and `sm`.
