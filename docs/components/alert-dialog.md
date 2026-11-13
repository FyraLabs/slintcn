# AlertDialog

**Import:** [`@slintcn/alert-dialog.slint`](../../src/components/alert-dialog.slint)

`AlertDialog` is a fixed confirmation dialog with a cancel action and a confirming action. Copy `dialog.slint` and `button.slint` beside it.

```slint
import { AlertDialog } from "@slintcn/alert-dialog.slint";
import { Button } from "@slintcn/button.slint";

export component App inherits Window {
    confirm := AlertDialog {
        title: "Delete this workspace?";
        description: "This action cannot be undone.";
        action-text: "Delete";
        destructive: true;
        action => { remove-workspace(); }
        cancel => { }
    }

    Button {
        text: "Delete workspace";
        clicked => { confirm.show(); }
    }
}
```

Call `show()` to open it and `close()` to close it. Canceling, pressing Escape, or using the cancel button closes the dialog and emits `cancel`. The action button closes it and emits `action`. Clicking outside and the close button are disabled.

The component fills its parent and positions its popup within those bounds. Place it under a sized parent, not inside the button that opens it.

**API**

| Property | Type | Default |
|---|---|---|
| `title` | `string` | `""` |
| `description` | `string` | `""` |
| `cancel-text` | `string` | `"Cancel"` |
| `action-text` | `string` | `"Continue"` |
| `destructive` | `bool` | `false` |
| `is-open` | `bool` | read-only output |

**Members**

| Name | Description |
|---|---|
| `cancel()` | Emitted after canceling or pressing Escape |
| `action()` | Emitted when the confirming button is used |
| `show()` | Opens the dialog |
| `close()` | Closes the dialog without emitting `cancel` or `action` |
