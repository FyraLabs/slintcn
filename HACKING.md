# Hacking on slintcn

This explains how slintcn works internally. For using it in an app, see the [README](README.md).

slintcn is a component library in `src/components/`. Apps import it as `@slintcn/<name>.slint` through a Slint [library path](https://docs.slint.dev/latest/docs/slint/guide/language/coding/file/#component-libraries). It has its own widgets with shadcn's look and API. It replaces nothing in `std-widgets.slint`, and it looks the same under every Slint style. Slint's [Material component library](https://github.com/slint-ui/slint/tree/master/ui-libraries/material) is built the same way and is a good reference.

`examples/gallery/` is a small Rust app that uses the library the way a user's app would:

```sh
cargo run -p slintcn-gallery
```

The gallery builds with three of Slint's development features on by default:

- `live-preview`: the running window reloads when a `.slint` file in the gallery or in `src/components/` changes. See [Slint's live preview docs](https://docs.slint.dev/latest/docs/slint/guide/tooling/live-preview/).
- `mcp`: an MCP server (see [AI coding assistants](https://docs.slint.dev/latest/docs/slint/guide/tooling/ai-coding-assistants/)) that lets AI agents read the element tree, click and take screenshots. It only starts when `SLINT_MCP_PORT` is set.
- `system-testing`: support for [Slint's UI testing](https://testing.slint.dev/). It only connects when a test sets `SLINT_TEST_SERVER`.

MCP and system testing need debug info in the compiled UI. `.cargo/config.toml` sets `SLINT_EMIT_DEBUG_INFO=1` for every cargo command, including `cargo run`. With live preview, the UI is compiled inside the running app, so the variable also has to be set when the app starts. `cargo run` does that, and so do the Zed debug configurations in `.zed/debug.json`.

```sh
cargo run -p slintcn-gallery                                    # all three on
SLINT_MCP_PORT=9315 cargo run -p slintcn-gallery                # with the MCP server
cargo run -p slintcn-gallery --no-default-features              # normal ahead-of-time build
```

Live preview reacts to edits that write into the file. Saves that write a temporary file and rename it over the original weren't picked up in testing with Slint 1.18.1. If the window doesn't reload after a save, `touch` the file.

## Layout

```
src/components/
  styling.slint       SlintcnPalette, SlintcnSizeSettings, SlintcnIcons
  _x.svg              icons used by the components
  button.slint        one file per component, with its parts and enums
  card.slint
  alert.slint
  dialog.slint
  alert-dialog.slint
```

Users copy individual files, so each component file depends only on `styling.slint` and, rarely, a sibling (AlertDialog builds on Dialog and Button). Files import each other with plain relative paths (`"styling.slint"`), so the folder works under any library name.

## Colors and dark mode

`SlintcnPalette` has shadcn's colors under shadcn's names (`primary`, `muted-foreground`, `accent`, `ring`, …), with a dark and a light value for each. These are shadcn's neutral theme, converted from the oklch values in `apps/v4/registry/themes.ts` in shadcn's repo.

`SlintcnPalette.color-scheme` picks between them. Its default is bound to Slint's own `Palette.color-scheme`, which the built-in styles set from the system's dark/light setting, so the components follow the system with no setup. When the platform reports no preference, the components use light. An app can assign `SlintcnPalette.color-scheme` to take control; the assignment replaces the binding.

`SlintcnSizeSettings` holds the radii, derived from shadcn's 10px `--radius` the same way Tailwind does (`radius-md` is 8px, `radius-xl` is 14px).

## Translating a shadcn component

The look and the API come from shadcn's source, `apps/v4/registry/new-york-v4/ui/<name>.tsx` in [shadcn-ui/ui](https://github.com/shadcn-ui/ui). Work from the source, not from memory or screenshots.

Translating the Tailwind classes is mostly mechanical:

- Spacing is 4px per step: `h-9` is 36px, `px-4` is 16px, `gap-2` is 8px.
- `text-sm` is 14px, `text-lg` is 18px; `font-medium` is weight 500, `font-semibold` 600.
- `rounded-md` is `SlintcnSizeSettings.radius-md`.
- `bg-primary` is `SlintcnPalette.primary`; `bg-primary/90` is `SlintcnPalette.primary.transparentize(0.1)`.
- `dark:` classes become a `dark ? … : …` expression on `SlintcnPalette.color-scheme == ColorScheme.dark`.
- `hover:` classes read the `TouchArea`'s `has-hover`.

shadcn's `variant` and `size` props become Slint enums (`ButtonVariant`, `ButtonSize`) exported next to the component. Where shadcn composes parts (`Card`, `CardHeader`, `CardTitle`, …), mirror them as separate components when they only lay out their children. Slint has no equivalent of React context, so a part can't read its parent's props. When a variant has to change several parts at once, like Alert's destructive variant recoloring its title, description and icon, use properties on the main component instead (`title`, `description`, `icon`).

Overlays (Dialog, AlertDialog) use a `PopupWindow` that covers the whole window. The popup is positioned relative to its parent, so place these components directly in the `Window` and not inside a layout. They set `width: 100%; height: 100%` because a popup sized from `parent` came out as 0×0 in testing. Code outside a `PopupWindow` can't reach elements inside it ([slint-ui/slint#4438](https://github.com/slint-ui/slint/issues/4438)), so state flows in through the component's properties and out through callbacks.

For how a Slint widget is structured internally, how it handles the keyboard and which accessibility properties it sets, Slint's own widgets (`internal/compiler/widgets/fluent/` in [slint-ui/slint](https://github.com/slint-ui/slint)) are the best reference. Read them, but don't copy from them: they're licensed `GPL-3.0-only` or Slint's commercial licenses, which isn't compatible with our `GPL-3.0-or-later`.

## Conventions

- Every `.slint` file starts with `// SPDX-License-Identifier: GPL-3.0-or-later`. Users copy individual files, so each one carries its license.
- Colors come from `SlintcnPalette`; add a token to `styling.slint` instead of writing a hex value. Three exceptions, because shadcn hard-codes them too: shadows, the dialog's black backdrop, and white text on a destructive button.
- Don't inherit `Rectangle` unless app code is meant to change the background or radius. Put a `Rectangle` inside instead.
- Internal element ids start with `i-` (`i-background`, `i-touch-area`), like in Slint's own widgets.
- Controls size themselves from their content (a `min-width` based on the inner layout) and don't stretch (`horizontal-stretch: 0`).
- Disabling is `enabled: false`, like Slint's widgets, not `disabled`.
- Keep the `accessible-*` properties.
- shadcn shows focus rings only for keyboard focus. Slint has no `:focus-visible`, so track it from the `FocusScope`'s `focus-gained(reason)`: set a flag when the reason is `tab-navigation`, and draw the ring only while focused with that flag set. `button.slint` shows how.
- Transitions are 150ms with `cubic-bezier(0.4, 0, 0.2, 1)`, like Tailwind's default.
- Comments are one line, and only where the code can't explain itself.

## Checking your work

The gallery's live preview reloads the real app on every save. The viewer is lighter and doesn't need a Rust build. With `--auto-reload` it re-renders on every save:

```sh
slint-viewer -Lslintcn=src/components --auto-reload examples/gallery/ui/main.slint
```

Replace `--auto-reload` with `--check` to only compile, or with `--screenshot out.png` to render to a file. To force a mode in a screenshot, temporarily add `init => { SlintcnPalette.color-scheme = ColorScheme.light; }` to the gallery window.

Before calling something done:

- Add it to the gallery, covering every variant and state.
- Compare it side by side with ui.shadcn.com, in dark and light mode.
- Run the gallery and try it by hand: hover, press, disabled, Tab focus (ring) versus clicking (no ring), Space and Enter.

Keep `slint-viewer` and `slint-lsp` on the same Slint version as the gallery's `slint` crate (`cargo install slint-viewer slint-lsp`), so the editor, the viewer and the build agree.

## Known gaps

- Only shadcn's neutral base color exists. Zinc, slate and stone are still to do.
- There's no shared type scale yet; font sizes are written directly in each component.
- Plain `Text` and the window background come from the app's Slint style, so apps set `background: SlintcnPalette.background` on their window and a `color` on plain `Text` themselves. A `SlintcnWindow`, like Material's `MaterialWindow`, would remove that step.
