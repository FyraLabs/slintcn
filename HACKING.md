# Hacking on slintcn

This explains how slintcn works internally. For using it in an app, see the [README](README.md).

slintcn has two halves that are developed together but used independently:

- `src/components/` is a library of components Slint doesn't ship (Card, Alert, Dialog, …). Apps import them as `@slintcn/card.slint`. They work under any widget style.
- `src/styles/slintcn/` is a Slint widget style that gives Slint's own standard widgets the shadcn look. It doubles as the starting point for anyone writing their own style.

The rule that keeps them independent: components never import anything from `src/styles/`.

`examples/gallery/` is a small Rust app that uses both, the same way a user's app would. Run it with:

```sh
cargo run -p slintcn-gallery
SLINT_STYLE=fluent cargo run -p slintcn-gallery   # or cupertino, material, cosmic
```

## How Slint styles work

Slint's standard widgets (`Button`, `LineEdit`, `CheckBox`, …) aren't one set of widgets with a theme on top. Each style, like Fluent or Cupertino, is its own complete implementation of every widget, written in `.slint`. When an app writes `import { Button } from "std-widgets.slint"`, the compiler picks the `std-widgets.slint` of the selected style. That's why you can't restyle Fluent's Button from the outside: its look is in its code, and the colors it exposes through `Palette` are read-only.

So to give the standard widgets a different look, you write a style. Slint doesn't document custom styles, but the compiler supports them. Here's how it actually resolves one, from reading the compiler source (Slint 1.18):

- Selecting a style (`with_style("slintcn")`, `--style slintcn` or `SLINT_STYLE=slintcn`) only checks that some include path contains `slintcn/std-widgets.slint`.
- `import … from "std-widgets.slint"` is then resolved by searching the include paths for a file named `std-widgets.slint`, before falling back to the built-in styles. So the style folder itself also has to be an include path.
- In other words, the include path is what actually applies a custom style, not its name. If `src/styles/slintcn/` is an include path, you get slintcn even with `--style fluent`. The gallery's `build.rs` handles this by only adding include paths when `src/styles/<SLINT_STYLE>/` exists.
- The compiler separately loads `Palette` and `StyleMetrics` from a file called `style-base.slint`. Without it, compilation panics with "can't load style metrics".
- Relative include paths are resolved from the folder of the file doing the import, not from the working directory. Use absolute paths in `build.rs` and on the command line. Editor settings, which can't use absolute paths if they're to be shared, need one relative entry per folder; see the comment in `.zed/settings.json`.
- A custom style can't import Slint's built-in files, and it can't read the system's dark/light setting. The built-in styles use an internal global for that which isn't accessible from outside.

Because this is undocumented, check it again whenever we upgrade Slint.

## The slintcn style

```
src/styles/slintcn/
  styling.slint       SlintcnPalette, SlintcnSizeSettings
  style-base.slint    StyleMetrics, Palette
  std-widgets.slint   the style's exports
  std/button.slint    …one file per standard widget
```

This follows the layout of Slint's built-in styles. `styling.slint` holds the style's own design tokens. `SlintcnPalette` has shadcn's colors under shadcn's names (`primary`, `muted-foreground`, `ring`, …), each with a dark and a light value, switching on `SlintcnPalette.color-scheme`. `SlintcnSizeSettings` has the radii, derived from shadcn's 10px `--radius`.

`style-base.slint` defines the two globals Slint requires from every style, and fills them in from `SlintcnPalette`. The interesting part is the `Palette` mapping:

| Slint `Palette` | shadcn token |
|---|---|
| `background`, `foreground` | `background`, `foreground` |
| `alternate-background`, `alternate-foreground` | `card`, `card-foreground` |
| `control-background`, `control-foreground` | `secondary`, `secondary-foreground` |
| `accent-background`, `accent-foreground` | `primary`, `primary-foreground` |
| `selection-background`, `selection-foreground` | `primary`, `primary-foreground` |
| `border` | `border` |

`Palette.color-scheme` is two-way bound to `SlintcnPalette.color-scheme`, so apps switch between dark and light with the standard `Palette.color-scheme`, as with any other style.

Beware the naming clash: Slint's `accent-*` means the primary color, while shadcn's `accent` is a subtle hover background. That's why the widgets in `std/` use `SlintcnPalette` directly rather than `Palette`.

`std-widgets.slint` is the list of what the style provides. Every widget in `std/` is exported from there and nowhere else. Apps never import `std/button.slint` directly, and the widgets have exactly the API Slint documents, with nothing extra. That's what lets an app switch between slintcn and Fluent without changing code.

`std-widgets.slint` and `style-base.slint` have to stay at the top of the style folder, since the compiler looks for them by name.

## The components

```
src/components/
  styling.slint       ComponentPalette, ComponentSizeSettings, ComponentIcons
  _x.svg              …icons
  card.slint, alert.slint, dialog.slint, alert-dialog.slint, …
```

Components have to look right under any style, so they take their colors from whichever style is active. `ComponentPalette` in `src/components/styling.slint` imports `Palette` from `"std-widgets.slint"`. That import is resolved like an app's import, so it gets the active style's `Palette`, and the component colors follow it:

- `background`, `foreground` and `border` come straight from `Palette`.
- `card` is `Palette.alternate-background`, Slint's slot for panels. Under slintcn that's shadcn's `card`.
- Roles Slint has no slot for are derived: muted text is `foreground` at 60% opacity, and `destructive` is shadcn's red for the current mode.

So under slintcn the components look like shadcn, and under Cosmic they look like Cosmic panels, in either mode.

Components that need a standard widget import it from `"std-widgets.slint"` too. AlertDialog's buttons are slintcn buttons under slintcn and Fluent buttons under Fluent.

## Writing a standard widget

The API comes from [Slint's docs](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/) and must match exactly: property names, types, `in`/`out`/`in-out`, defaults and callbacks. The docs don't always say whether a property is `in` or `in-out`. Fluent's source (`internal/compiler/widgets/fluent/` in [slint-ui/slint](https://github.com/slint-ui/slint)) does, and it's the best reference for how a widget is structured, how it handles keyboard input and which accessibility properties it sets. Read it, but don't copy from it: it's licensed `GPL-3.0-only` or Slint's commercial licenses, which isn't compatible with our `GPL-3.0-or-later`.

The look comes from shadcn's source, `apps/v4/registry/new-york-v4/ui/<name>.tsx` in [shadcn-ui/ui](https://github.com/shadcn-ui/ui). Translating Tailwind is mostly mechanical: spacing is 4px per step (`h-9` is 36px), `text-sm` is 14px, `font-medium` is weight 500, `rounded-md` is `SlintcnSizeSettings.radius-md`, `bg-primary/90` is `SlintcnPalette.primary.transparentize(0.1)`, and `dark:` variants become a `dark ? … : …` expression.

Put the widget in `std/`, export it from `std-widgets.slint`, and add it to the gallery.

## Writing a component

Here the API is ours to design. Keep shadcn's names and part structure where Slint allows it (`Card`, `CardHeader`, `CardTitle`, …), and match the standard widgets' conventions (`enabled`, not `disabled`). Slint has no equivalent of React context, so a part can't read its parent's props. When a variant has to affect several parts, like Alert's destructive variant recoloring its title and description, use properties on the main component instead of separate parts.

Colors come from `ComponentPalette` and sizes from `ComponentSizeSettings`. If a component needs a color role that isn't there yet, add it to `ComponentPalette` as something derived from `Palette`, so it still works under every style.

## Conventions

- Every `.slint` file starts with `// SPDX-License-Identifier: GPL-3.0-or-later`. Users copy individual files, so each one carries its license.
- No hex colors in widgets or components; add a token to the relevant `styling.slint`. Shadows are the exception, since shadcn doesn't theme them either.
- Don't inherit `Rectangle` unless app code is meant to change the background or radius. Put a `Rectangle` inside instead.
- Internal element ids start with `i-` (`i-background`, `i-touch-area`), like in Slint's own widgets.
- Controls size themselves from their content (`min-width: max(36px, i-layout.min-width)`) and don't stretch (`horizontal-stretch: 0`).
- Keep the `accessible-*` properties.
- shadcn shows focus rings only for keyboard focus. Slint has no `:focus-visible`, so track it from the `FocusScope`'s `focus-gained(reason)`: set a flag when the reason is `tab-navigation`, and draw the ring only while focused with that flag set. `std/button.slint` shows how.
- Transitions are 150ms with `cubic-bezier(0.4, 0, 0.2, 1)`, like Tailwind's default.
- Comments are one line, and only where the code can't explain itself.

## Checking your work

The viewer is the fastest loop. With `--auto-reload` it re-renders on every save:

```sh
slint-viewer -I "$PWD/src/styles" -I "$PWD/src/styles/slintcn" -Lslintcn=src/components --style slintcn --auto-reload examples/gallery/ui/main.slint
```

Replace `--auto-reload` with `--check` to only compile, or with `--screenshot out.png` to render to a file. To see light mode in a screenshot, temporarily add `init => { Palette.color-scheme = ColorScheme.light; }` to the gallery window.

Before calling something done:

- Compare it side by side with ui.shadcn.com, in dark and light mode.
- Run the gallery and try it by hand: hover, press, disabled, Tab focus (ring) versus clicking (no ring), Space and Enter.
- Check that it still works under the stock styles. This catches standard widgets drifting from Slint's API and components that accidentally depend on slintcn. Leave out the slintcn include paths, or you'll still be testing slintcn:

  ```sh
  for s in fluent cupertino material cosmic; do slint-viewer -Lslintcn=src/components --style $s --check examples/gallery/ui/main.slint; done
  SLINT_STYLE=fluent cargo build -p slintcn-gallery
  ```

Keep `slint-viewer` and `slint-lsp` on the same Slint version as the gallery's `slint` crate (`cargo install slint-viewer slint-lsp`), so the editor, the viewer and the build agree.

## Known gaps

- Only shadcn's neutral base color exists. Zinc, slate and stone are still to do.
- The compiler builds tooltips, `TabWidget`, `RadioGroup` and menus from extra parts of a style (built-in styles have a `std-widgets-impl.slint` for this). We don't know yet what a custom style needs to provide for those.
- Widgets the built-in styles share (`VerticalBox`, `HorizontalBox`, `GridBox`, `ListView`, `StandardButton`, …) have to be rewritten, because a custom style can't import Slint's built-in files.
- The slintcn style doesn't set a default font size, so plain `Text` uses the platform default.
- The slintcn style starts in dark mode and can't follow the system setting by itself (see above).
