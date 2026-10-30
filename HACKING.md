# Hacking on slintcn

This covers how slintcn is put together and how to add components. For using slintcn in an app, see [README.md](README.md).

## Layout

```
slintcn/                  the style and extras; this folder is what users copy
  styling.slint           SlintcnPalette (shadcn colors, dark and light) and SlintcnSizeSettings (radii)
  style-base.slint        StyleMetrics + Palette: Slint's required globals, mapped onto SlintcnPalette
  std-widgets.slint       the style's entry point: re-exports Palette, StyleMetrics and every standard widget
  _x.svg, …               icons used by the style (see Icons in styling.slint)
  std/                    standard widgets (Slint's std-widgets API): button.slint, …
  components/             extras (imported as @slintcn/components/…): card.slint, alert.slint, dialog.slint, …
examples/gallery/         Rust app that uses slintcn/ the way a user's app would
  build.rs                selects the slintcn style, include paths and @slintcn library path
  ui/main.slint           the gallery window
.zed/, .vscode/           editor settings for the Slint language server
```

## How it fits together

```
styling.slint ──► style-base.slint ──► std-widgets.slint ──► app: import { Button } from "std-widgets.slint"
      │                                       ▲
      ├──────────────────────────────► std/button.slint, …
      └──► components/card.slint, … ──────────────────────► app: import { Card } from "@slintcn/components/card.slint"
```

The files follow the layout of Slint's built-in styles (Fluent, Cupertino, Material, Cosmic): style tokens in `styling.slint` as `<Style>Palette` / `<Style>SizeSettings`, mapped onto the required globals in `style-base.slint`.

- **`SlintcnPalette`** holds shadcn's colors under shadcn's names (`primary`, `muted-foreground`, `accent`, `ring`, …), with a dark and a light value for each. It switches on `SlintcnPalette.color-scheme`.
- **`SlintcnSizeSettings`** holds the radii (`radius-sm`, `radius-md`, `radius-lg`, `radius-xl`), derived from shadcn's `--radius`.
- **`StyleMetrics`** and **`Palette`** are the globals Slint requires from every style. The compiler loads them from `style-base.slint` by name. `Palette` maps Slint's slots onto `SlintcnPalette`:
  - `accent-background` → `primary`
  - `control-background` → `secondary`
  - `alternate-background` → `muted`

  `Palette.color-scheme` is two-way bound to `SlintcnPalette.color-scheme`, so apps switch modes with the standard `Palette.color-scheme`.
- **Components read `SlintcnPalette`**, not `Palette`, so they use shadcn's names. Watch out for one clash: Slint's `Palette.accent-foreground` is the text on a primary button, while shadcn's `accent-foreground` is the text on a subtle hover background.
- **Files import each other with relative paths** (`import { SlintcnPalette } from "../styling.slint";` from `std/` or `components/`), so the folder works under any style or library name.
- **`std-widgets.slint`, `style-base.slint` and `styling.slint` stay at the top of the folder.** The compiler looks for the first two by exact path.

### How Slint finds the style

Custom styles aren't in Slint's docs. This is from reading the compiler source (Slint 1.17/1.18):

- `--style slintcn` (or `with_style`, or `SLINT_STYLE`) only checks that `slintcn/std-widgets.slint` exists under some include path.
- `import … from "std-widgets.slint"` is resolved by searching the include paths for `std-widgets.slint` itself, *before* the built-in styles. So the `slintcn/` folder must also be an include path. That's why there are two: the repo root and `slintcn/`.
- That also means the include path, not the style name, is what applies slintcn. With `slintcn/` as an include path, `--style fluent` still renders slintcn. To switch styles, drop the `slintcn/` include path too. The gallery's `build.rs` does this based on `SLINT_STYLE`.
- The compiler loads `Palette` and `StyleMetrics` from `style-base.slint`. If the file is missing, it panics with "can't load style metrics".
- Relative include paths are resolved from the importing file's folder, not the working directory. Use absolute paths in `build.rs` and on the command line. See the comment in `.zed/settings.json` for the editor workaround.
- A custom style can't read the system's dark/light setting (built-in styles use `SlintInternal.color-scheme`, which is off-limits). It can't import Slint's built-in files either.

## Running

```sh
cargo run -p slintcn-gallery
SLINT_STYLE=fluent cargo run -p slintcn-gallery    # same gallery with a built-in style
```

Live preview while editing:

```sh
slint-viewer -I "$PWD" -I "$PWD/slintcn" -Lslintcn=slintcn --style slintcn --auto-reload examples/gallery/ui/main.slint
```

Screenshot, e.g. to check a change without a window:

```sh
slint-viewer -I "$PWD" -I "$PWD/slintcn" -Lslintcn=slintcn --style slintcn --screenshot /tmp/gallery.png examples/gallery/ui/main.slint
```

Use the same Slint version for the tools (`cargo install slint-lsp slint-viewer`) and the gallery's `slint` crate, so the editor and the build agree.

## Adding a standard widget

Standard widgets are the ones in Slint's `std-widgets.slint` (`CheckBox`, `LineEdit`, `Switch`, `Slider`, …).

1. **Copy the API from Slint's docs, exactly.** Same property names, types, directions (`in` / `out` / `in-out`), defaults and callbacks. Apps must compile unchanged when they switch between slintcn and `fluent`. Docs: `https://docs.slint.dev/latest/docs/slint/reference/std-widgets/`. Fluent's source shows the directions the docs leave out.
2. **Take the look from shadcn's source**, not from memory: `apps/v4/registry/new-york-v4/ui/<name>.tsx` in [shadcn-ui/ui](https://github.com/shadcn-ui/ui). Translate the Tailwind classes:
   - `h-9` = 36px, and so on: spacing is 4px per step
   - `text-sm` = 14px, `font-medium` = 500, `font-semibold` = 600
   - `rounded-md` = `SlintcnSizeSettings.radius-md`
   - `bg-primary/90` = `SlintcnPalette.primary.transparentize(0.1)`
   - `dark:` classes become `dark ? … : …`
3. **Look at how Fluent builds it** (`internal/compiler/widgets/fluent/<name>.slint` in [slint-ui/slint](https://github.com/slint-ui/slint), at the tag matching our Slint version) for structure, accessibility properties and keyboard handling. Don't copy code from it: Fluent is `GPL-3.0-only OR` Slint's commercial licenses, which isn't compatible with our `GPL-3.0-or-later`.
4. **Write `slintcn/std/<name>.slint`** following the [conventions](#conventions). Import tokens with `import { SlintcnPalette } from "../styling.slint";`.
5. **Export it** from `slintcn/std-widgets.slint`: `export { CheckBox } from "std/checkbox.slint";`
6. **Add it to the gallery** (`examples/gallery/ui/main.slint`), covering each state: default, disabled, checked, with an icon, and so on.
7. **[Check it](#checking-a-component).**
8. **Update the component table** in README.md.

## Adding an extra

Extras are shadcn components that Slint's standard set doesn't have (Badge, Alert, Dialog, …).

1. Take the look and the part structure from shadcn's `<name>.tsx`. Mirror its exported parts where Slint allows it (`Card`, `CardHeader`, `CardTitle`, …).
2. Pick the API yourself, staying close to shadcn's names (e.g. `variant` as an enum) and consistent with the standard widgets (`enabled`, not `disabled`).
3. Write `slintcn/components/<name>.slint`. Don't export it from `std-widgets.slint`; apps import it from `@slintcn/components/<name>.slint`. If it needs a standard widget, import it from `../std/<name>.slint`.
4. Add it to the gallery, check it, and update README.md.

## Conventions

- Standard widgets have exactly Slint's API, with no extra properties, and apps only reach them through `std-widgets.slint`. The files in `std/` are how the style draws them, not separate components. Something that needs more than the standard API (a destructive button, say) is an extra with its own name.
- Start every `.slint` file with `// SPDX-License-Identifier: GPL-3.0-or-later`. Users copy single files, so each one carries its license.
- Colors come from `SlintcnPalette` and radii from `SlintcnSizeSettings`. Don't hard-code hex values in components; add a token in `styling.slint` instead. Shadows (which shadcn doesn't theme) are the exception.
- Don't make a component inherit `Rectangle` unless app code should be able to change its `background`, `border-radius`, and so on. Put the `Rectangle` inside instead. `Button` and `Card` do this.
- Size from content: `min-width: max(36px, i-layout.min-width);`, with `horizontal-stretch: 0` for controls.
- Name internal elements with an `i-` prefix (`i-background`, `i-touch-area`), like Slint's own widgets.
- Keep the `accessible-*` properties: role, label, enabled, checked, and the default action.
- **Focus rings:** shadcn shows them on keyboard focus only (`:focus-visible`), which Slint doesn't have. Track it in the `FocusScope`:

  ```slint
  focus-gained(reason) => {
      if reason != FocusReason.window-activation {
          root.keyboard-focus = reason == FocusReason.tab-navigation;
      }
  }
  ```

  Draw the ring (3px, `SlintcnPalette.ring.transparentize(0.5)`, no offset) only when `has-focus && keyboard-focus`.
- Transitions: `animate background, border-color { duration: 150ms; easing: cubic-bezier(0.4, 0, 0.2, 1); }`.
- Comments are one line at most, and only where the code can't explain itself.

## Checking a component

- **Compiles:** `slint-viewer --check` with the flags from [Running](#running), and `cargo build`.
- **Looks right in both modes:** take a screenshot in dark mode, then in light. To render light mode, temporarily add `init => { Palette.color-scheme = ColorScheme.light; }` to the window. Compare against ui.shadcn.com side by side.
- **Interactions:** run the gallery and try hover, press, disabled, Tab focus (ring shows) versus a click (no ring), and Space/Enter.
- **API matches Slint's** (standard widgets only): the gallery must also build with a built-in style. Leave out the `slintcn/` include path, or it will still use slintcn:

  ```sh
  SLINT_STYLE=fluent cargo build -p slintcn-gallery
  slint-viewer -I "$PWD" -Lslintcn=slintcn --style fluent --check examples/gallery/ui/main.slint
  ```

## Known gaps

- Under another style, the extras (Card, Alert, Dialog, …) keep their own dark/light setting, `SlintcnPalette.color-scheme`, which defaults to dark. It's only linked to `Palette.color-scheme` when the slintcn style is active.
- Only shadcn's neutral base color. Zinc, slate and stone are still to do.
- The compiler builds tooltips, `TabWidget`, `RadioGroup` and menus from extra parts of a style (`std-widgets-impl.slint` in built-in styles). What a custom style needs to provide for those is untested.
- Shared standard pieces (`VerticalBox`, `HorizontalBox`, `GridBox`, `ListView`, `StandardButton`, …) have to be written from scratch, because a custom style can't import Slint's built-in files.
- The style doesn't set a default font size, so plain `Text` uses the platform default.
- Slint 1.19 adds `slint-project.json`, which should replace most of the include and library path setup.
