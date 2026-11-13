# Input

**Import:** [`@slintcn/input.slint`](../../src/components/input.slint)

`Input` is a single-line text field with an optional placeholder, password visibility control, and grouped borders. Copy `button.slint` beside it because the password toggle uses `Button`.

```slint
import { Input } from "@slintcn/input.slint";

email := Input {
    placeholder-text: "name@example.com";
    input-type: InputType.text;
    accessible-label: "Email address";
    edited(text) => { }
    accepted(text) => { }
}
```

Set `input-type: InputType.password` to show a Show/Hide button. `password-visible` reflects and controls whether the text is concealed. The button does not modify `text`.

```slint
Input {
    input-type: InputType.password;
    accessible-label: "Password";
    accepted(text) => { sign-in(text); }
}
```

`invalid` is part of the API but currently adds no visual treatment. Use a [Field](field.md) error and style around the input when validation needs to be visible.

`read-only: true` prevents editing. The outer component exposes no accessibility role; its label and description are forwarded to the internal text input.

**API**

| Property | Type | Default |
|---|---|---|
| `text` | `string` | `""` |
| `placeholder-text` | `string` | `""` |
| `enabled` | `bool` | `true` |
| `read-only` | `bool` | `false` |
| `input-type` | `InputType` | `InputType.text` |
| `input-method-hints` | `InputMethodHints` | derived from `input-type` |
| `password-visible` | `bool` | `false` |
| `invalid` | `bool` | `false` |
| `group-position` | `GroupPosition` | `GroupPosition.none` |
| `group-orientation` | `Orientation` | `Orientation.horizontal` |
| `has-focus` | `bool` | read-only output |

**Callbacks**

| Name | Description |
|---|---|
| `edited(text)` | Runs whenever the text changes |
| `accepted(text)` | Runs when the field is accepted |
