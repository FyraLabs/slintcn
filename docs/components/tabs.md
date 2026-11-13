# Tabs

**Import:** [`@slintcn/tabs.slint`](../../src/components/tabs.slint)

Tabs provide `default` and `line` list variants, horizontal and vertical orientation, optional icons, and disabled entries. Set `orientation: Orientation.vertical` on both `Tabs` and `TabsList` for vertical tabs. The parent owns the selected value and passes it to both the list and its content:

```slint
import { Tabs, TabsContent, TabsList } from "@slintcn/tabs.slint";

Tabs {
    TabsList {
        navigation-group: "settings";
        selected-value: root.selected-value;
        tabs: [
            { text: "General", value: "general", icon: @image-url("general.svg") },
            { text: "Account", value: "account" },
            { text: "Advanced", value: "advanced", disabled: true },
        ];
        changed(value) => { root.selected-value = value; }
    }

    TabsContent {
        value: "general";
        selected-value: root.selected-value;
        Text { text: "General settings"; }
    }
    TabsContent {
        value: "account";
        selected-value: root.selected-value;
        Text { text: "Account settings"; }
    }
}
```

Each tab list needs a unique `navigation-group`. Arrow keys move focus, skipping disabled entries, while Space or Enter activates the focused tab. Set `activate-on-focus: true` on `TabsList` to select tabs as they receive focus. `TabEntry.disabled` defaults to false.

`TabsContent` hides its children when unselected. Slint cannot conditionally forward arbitrary children between components, so an application must wrap the entire panel in a condition to unmount expensive content and reset its state:

```slint
if root.selected-value == "general": TabsContent {
    value: "general";
    selected-value: root.selected-value;
    ExpensiveGeneralSettings {}
}
```
