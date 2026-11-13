# Popover

**Import:** [`@slintcn/popover.slint`](../../src/components/popover.slint)

`Popover` shows arbitrary content in a popup beside its trigger. Open it with `show()` and close it with `close()` or an outside click.

```slint
import { Popover, PopoverDescription, PopoverHeader, PopoverTitle } from "@slintcn/popover.slint";
import { Button } from "@slintcn/button.slint";
import { Input } from "@slintcn/input.slint";
import { PopupAlign, PopupSide } from "@slintcn/styling.slint";

export component App inherits Window {
    dimensions := Popover {
        side: PopupSide.right;
        align: PopupAlign.start;
        content-width: 288px;

        PopoverHeader {
            PopoverTitle { text: "Dimensions"; }
            PopoverDescription { text: "Set the dimensions for the layer."; }
        }
        VerticalLayout {
            spacing: 8px;
            Input { placeholder-text: "Width"; }
            Input { placeholder-text: "Height"; }
        }
    }

    Button {
        text: "Dimensions";
        clicked => { dimensions.show(); }
    }
}
```

Place the popover in a sized container beside its trigger. It fills that container and calculates its position from those bounds.

**API**

| Property | Type | Default |
|---|---|---|
| `side` | `PopupSide` | `PopupSide.bottom` |
| `align` | `PopupAlign` | `PopupAlign.center` |
| `side-offset` | `length` | `4px` |
| `content-width` | `length` | `288px` |
| `spacing` | `length` | `16px` |
| `is-open` | `bool` | read-only output |

**Members**

| Name | Description |
|---|---|
| `show()` | Opens the popover |
| `close()` | Closes the popover |

Children are placed in a padded vertical layout. `PopoverHeader` stacks its children with 4px spacing. `PopoverTitle` and `PopoverDescription` inherit `Text` and use the popover's foreground colors.
