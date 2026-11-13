# Field

**Import:** [`@slintcn/field.slint`](../../src/components/field.slint)

`Field` arranges a label, control, description, or error. It does not connect them for you; propagate state and focus explicitly.

```slint
import { Field, FieldContent, FieldDescription, FieldError, FieldLabel, FieldTitle } from "@slintcn/field.slint";
import { Input } from "@slintcn/input.slint";

email-field := Field {
    invalid: true;

    email-label := FieldLabel {
        text: "Email";
        invalid: email-field.invalid;
        clicked => { email.focus(); }
    }
    FieldContent {
        email := Input {
            invalid: email-field.invalid;
            accessible-label: email-label.text;
            accessible-description: email-error.text;
        }
        FieldDescription { text: "We only use this for account notices."; }
        email-error := FieldError { text: "Enter a valid email address."; }
    }
}
```

`Field.invalid` does not alter its children. Bind it to each label and control that should change. `Field.enabled` provides field-level accessibility state; also disable controls and labels individually.

`FieldError` is an assertive live region. Link it to the control with `accessible-description`, as above, so assistive technology can associate the message.

`FieldLabel` does not focus a control by itself. Use its `clicked` callback to focus or activate the control, since Slint has no HTML `for` attribute.

```slint
import { Switch } from "@slintcn/switch.slint";

notifications-label := FieldLabel {
    text: "Notifications";
    clicked => { notifications.focus(); }
}
notifications := Switch {
    checked: true;
    accessible-label: notifications-label.text;
}
```

**API**

| Component | Properties |
|---|---|
| `Field` | `orientation: FieldOrientation = vertical`, `responsive-width: length = 0px`, `enabled: bool = true`, `invalid: bool = false` |
| `FieldContent` | No declared properties; vertical layout |
| `FieldTitle` | `enabled`, `invalid`, inherited `text` |
| `FieldLabel` | `text`, `enabled`, `invalid`, `bordered`, `selected`, `clicked()` |
| `FieldDescription` | `enabled`, inherited `text` |
| `FieldError` | Inherited `text` |
| `FieldGroup` | No declared properties; vertical stack |
| `FieldSet` | No declared properties; group box |
| `FieldLegend` | `text`, `variant: FieldLegendVariant = legend` |
| `FieldSeparator` | `text` |

`FieldOrientation` provides `vertical`, `horizontal`, and `responsive`. Responsive fields become horizontal at the `responsive-width` threshold of 448px. `FieldLegendVariant` provides `legend` and `label`.

A bordered `FieldLabel` creates a selectable box around its children. Set `selected` to apply the active border and background. `FieldLabel.enabled` disables its own click handling only.

`FieldGroup` stacks related fields. `FieldSet` groups fields with the accessibility role `groupbox`, and `FieldLegend` gives the set a heading. `FieldSeparator` accepts optional text for a centered divider label.
