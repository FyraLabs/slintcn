# Dialog

**Import:** [`@slintcn/dialog.slint`](../../src/components/dialog.slint)

`Dialog` is a modal panel with an optional title and description. Add content and a `DialogFooter` as children.

```slint
import { Dialog, DialogFooter } from "@slintcn/dialog.slint";
import { Button, ButtonVariant } from "@slintcn/button.slint";
import { Input } from "@slintcn/input.slint";

export component App inherits Window {
    edit-profile := Dialog {
        title: "Edit profile";
        description: "Make changes and save when you are done.";

        VerticalLayout {
            spacing: 12px;
            Input { placeholder-text: "Name"; }
        }
        DialogFooter {
            Button {
                text: "Cancel";
                variant: ButtonVariant.outline;
                clicked => { edit-profile.close(); }
            }
            Button {
                text: "Save";
                clicked => { save(); edit-profile.close(); }
            }
        }

        dismissed => { }
    }

    Button {
        text: "Edit profile";
        clicked => { edit-profile.show(); }
    }
}
```

`show()` opens the dialog. `close()` closes it after the 200 ms closing transition and does not emit `dismissed`. Escape, a permitted outside click, or the close button closes it and emits `dismissed`.

The component fills its parent and draws its popup within those bounds. Place it under a sized parent, not inside the button that opens it.

**API**

| Property | Type | Default |
|---|---|---|
| `title` | `string` | `""` |
| `description` | `string` | `""` |
| `show-close-button` | `bool` | `true` |
| `dismiss-on-outside-click` | `bool` | `true` |
| `is-open` | `bool` | read-only output |

**Members**

| Name | Description |
|---|---|
| `dismissed()` | Emitted after Escape, outside click, or close-button dismissal |
| `show()` | Opens the dialog |
| `close()` | Closes the dialog without emitting `dismissed` |

`DialogFooter` inherits `HorizontalLayout`, defaults to end alignment and 8px spacing, and accepts ordinary children.
