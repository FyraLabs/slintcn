# DropdownMenu

**Import:** [`@slintcn/dropdown-menu.slint`](../../src/components/dropdown-menu.slint)

`DropdownMenu` is a popup menu. It has no trigger component: open it with `show()` from a button or another control.

```slint
import { Button, ButtonVariant } from "@slintcn/button.slint";
import {
    DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuItem,
    DropdownMenuItemVariant, DropdownMenuSeparator,
} from "@slintcn/dropdown-menu.slint";
import { PopupAlign } from "@slintcn/styling.slint";

export component App inherits Window {
    in-out property <bool> sidebar-visible: true;

    actions := DropdownMenu {
        align: PopupAlign.end;
        min-content-width: 192px;

        DropdownMenuItem {
            text: "Rename";
            shortcut: "⌘R";
            activated => { rename(); }
        }
        DropdownMenuCheckboxItem {
            text: "Show sidebar";
            checked <=> root.sidebar-visible;
        }
        DropdownMenuSeparator { }
        DropdownMenuItem {
            text: "Delete";
            variant: DropdownMenuItemVariant.destructive;
            activated => { delete-item(); }
        }
    }

    Button {
        text: "Actions";
        variant: ButtonVariant.outline;
        clicked => { actions.show(); }
    }
}
```

Every enabled item closes the open menu when activated. Disabled items do not. The menu also closes when clicking outside it. There is no controlled `open` property; use `is-open` to observe the popup state and `show()` or `close()` to change it.

Place the menu in a sized container beside its trigger. The menu fills that container and positions its popup from those bounds, so it does not locate the clicked button automatically.

**API**

`DropdownMenu`:

| Property | Type | Default |
|---|---|---|
| `side` | `PopupSide` | `PopupSide.bottom` |
| `align` | `PopupAlign` | `PopupAlign.center` |
| `side-offset` | `length` | `4px` |
| `min-content-width` | `length` | `128px` |
| `is-open` | `bool` | read-only output |

**Members**

| Name | Description |
|---|---|
| `show()` | Opens the menu |
| `close()` | Closes the menu |

**Items**

| Component | Properties | Callback |
|---|---|---|
| `DropdownMenuItem` | `text`, `icon`, `shortcut`, `enabled`, `inset`, `variant` | `activated()` |
| `DropdownMenuCheckboxItem` | `text`, `shortcut`, `enabled`, `checked` | `toggled(checked)` |
| `DropdownMenuRadioItem` | `text`, `shortcut`, `enabled`, `checked` | `activated()` |

`DropdownMenuItemVariant` provides `default` and `destructive`. `inset` aligns an item's text with items that have icons.

A checkbox item flips its own `checked` value. A radio item does not: bind `checked` to application state and update that state in `activated`. `DropdownMenuGroup` only groups layouts and does not enforce radio exclusivity.

`DropdownMenuLabel` accepts `text` and `inset`. `DropdownMenuSeparator` draws a divider.
