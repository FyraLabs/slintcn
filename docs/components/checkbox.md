# Checkbox

**Import:** [`@slintcn/checkbox.slint`](../../src/components/checkbox.slint)

`Checkbox` owns a boolean `checked` value and toggles it with a pointer, Space, Return, or its accessible default action.

```slint
import { Checkbox } from "@slintcn/checkbox.slint";

terms := Checkbox {
    accessible-label: "Accept the terms";
    accessible-description: "Required to continue.";
    toggled(checked) => {
        if checked { enable-submit(); }
    }
}
```

Set `invalid: true` to show a destructive border and focus ring. Disabled checkboxes neither change state nor emit `toggled`.

Assigning `checked` directly changes the value without emitting `toggled`. Call `toggle()` when the callback must run:

```slint
import { Label } from "@slintcn/label.slint";

Label {
    text: "Accept the terms";
    clicked => { terms.toggle(); }
}
```

**API**

| Property | Type | Default |
|---|---|---|
| `checked` | `bool` | `false` |
| `enabled` | `bool` | `true` |
| `invalid` | `bool` | `false` |
| `has-focus` | `bool` | read-only output |

**Members**

| Name | Description |
|---|---|
| `toggled(checked)` | Runs after the value changes through user or accessible activation |
| `toggle()` | Flips the value and emits `toggled` when enabled |
