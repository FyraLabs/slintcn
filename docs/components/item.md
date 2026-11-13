# Item

**Import:** [`@slintcn/item.slint`](../../src/components/item.slint)

`Item` provides `default`, `outline`, and `muted` variants, and `default`, `sm`, and `xs` sizes. Compose its content with the exported parts:

```slint
import { Item, ItemActions, ItemContent, ItemDescription, ItemTitle, ItemVariant } from "@slintcn/item.slint";
import { Button, ButtonSize, ButtonVariant } from "@slintcn/button.slint";

Item {
    variant: ItemVariant.outline;

    ItemContent {
        ItemTitle { text: "Basic Item"; }
        ItemDescription { text: "A simple item with title and description."; }
    }
    ItemActions {
        Button { text: "Action"; size: ButtonSize.sm; variant: ButtonVariant.outline; }
    }
}
```

`ItemGroup` stacks items, `ItemSeparator` adds a divider, and `ItemHeader` and `ItemFooter` each occupy a full row. `ItemMedia` accepts an `image` with an `icon` or `image` variant, or custom children. To align media with a description, set `with-description: true` and `cross-axis-self-alignment: start` on the media instance.

Pass the same `ItemSize` to `Item`, `ItemMedia`, `ItemContent`, and `ItemDescription`; parts don't inherit the parent's size. Set `grow: false` on trailing `ItemContent` for metadata such as a duration. Descriptions wrap to two lines and elide longer text.

For an interactive row, set `clickable: true`, `accessible-role: button`, and an `accessible-label`, then handle `clicked`. Bind `accessible-action-default => { self.activate(); }` to enable accessible activation. It supports Tab focus, Space, and Enter. Slint requires a constant accessibility role, so static items default to `list-item` and interactive instances set their own role and action. Buttons inside `ItemActions` receive clicks independently. `enabled: false` disables row activation; disable any child controls separately.
