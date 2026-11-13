# Label

**Import:** [`@slintcn/label.slint`](../../src/components/label.slint)

`Label` uses 14px medium text and supports additional children. Connect its `clicked` callback to the control, and give the control an `accessible-label`; Slint has no HTML `for` attribute.

```slint
import { Checkbox } from "@slintcn/checkbox.slint";
import { Label } from "@slintcn/label.slint";

HorizontalLayout {
    spacing: 8px;
    alignment: start;
    cross-axis-alignment: center;

    terms := Checkbox { accessible-label: "Accept terms"; }
    Label {
        text: "Accept terms";
        clicked => { terms.checked = !terms.checked; }
    }
}
```

Set `enabled: false` on both the label and the control to disable them.
