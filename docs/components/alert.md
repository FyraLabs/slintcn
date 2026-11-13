# Alert

**Import:** [`@slintcn/alert.slint`](../../src/components/alert.slint)

`Alert` is a passive inline notification. It shows an optional icon, a title, a description, and any child elements placed after the description.

```slint
import { Alert, AlertVariant } from "@slintcn/alert.slint";

Alert {
    variant: AlertVariant.destructive;
    title: "Unable to save";
    description: "Check your connection and try again.";
}
```

Empty titles and descriptions are omitted. Longer descriptions wrap, while the title elides. The `destructive` variant changes the text colors.

**API**

| Property | Type | Default |
|---|---|---|
| `variant` | `AlertVariant` | `AlertVariant.default` |
| `icon` | `image` | empty |
| `title` | `string` | `""` |
| `description` | `string` | `""` |

Children added to `Alert` appear below the description.
