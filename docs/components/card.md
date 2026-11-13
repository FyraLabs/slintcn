# Card

**Import:** [`@slintcn/card.slint`](../../src/components/card.slint)

`Card` groups related content with a background, border, and shadow. Compose it with `CardHeader`, `CardContent`, and `CardFooter`.

```slint
import { Card, CardContent, CardFooter, CardHeader, CardSize } from "@slintcn/card.slint";
import { Button, ButtonVariant } from "@slintcn/button.slint";
import { SlintcnPalette } from "@slintcn/styling.slint";

Card {
    CardHeader {
        title: "Team plan";
        description: "Everything your team needs to ship.";
    }
    CardContent {
        Text {
            text: "Unlimited projects and priority support.";
            color: SlintcnPalette.muted-foreground;
        }
    }
    CardFooter {
        spacing: 8px;
        alignment: end;
        Button { text: "Choose plan"; }
        Button { text: "Compare"; variant: ButtonVariant.outline; }
    }
}
```

Sections do not inherit `Card.size`. Pass the same `CardSize` to `Card`, `CardHeader`, `CardContent`, and `CardFooter` when using `CardSize.sm`.

```slint
Card {
    size: CardSize.sm;
    CardHeader {
        size: CardSize.sm;
        title: "Small card";
    }
    CardContent {
        size: CardSize.sm;
        Text { text: "Compact spacing."; }
    }
}
```

Set `Card.image` to show a rounded cover image above the header. An image removes the card's top padding. `CardHeader` children appear as actions to the right of its title. `divider: true` adds a border below a header or above a footer.

**API**

| Component | Properties |
|---|---|
| `Card` | `size: CardSize = default`, `image: image = empty` |
| `CardHeader` | `title`, `description`, `size`, `divider: bool = false` |
| `CardTitle` | `text`, `font-size: length = 16px` |
| `CardDescription` | `text` |
| `CardContent` | `size` |
| `CardFooter` | `size`, `orientation = horizontal`, `spacing: length = 0px`, `alignment = start`, `divider: bool = false` |

`CardSize` provides `default` and `sm`. `CardContent` inherits `VerticalLayout`. `CardFooter` uses a nonwrapping flex layout.
