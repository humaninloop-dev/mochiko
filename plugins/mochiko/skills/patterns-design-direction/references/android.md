# Android platform guidance

> Ported from `pbakaus/impeccable` at `e0881d2de397d5e9761d7b35ff5017d8f5ebf69b`
> (`skill/reference/android.md`, Apache-2.0), itself derived in part from
> `ehmo/platform-design-skills` (MIT); rewritten in mochiko's form. Attribution chain in the
> repository's root `NOTICE`.

Loaded when the design baseline's `Platform` value is `android` or `adaptive`. It covers native
Android apps — Jetpack Compose, Android Views, React Native, Expo, or Flutter shipping to Android
hardware.

On native, the mode narrows what expression may override. Material Design 3 governs structure,
navigation, and interaction in every mode; brand speaks through Material's theming — colour
roles, type scale, shape, motion. A Material-everywhere app that also ships to iPhone still owes
iOS its guarantees there: safe-area insets, Reduce Motion, edge-swipe back.

## The trust test

Would a fluent Android user trust this app, or trip on off-spec components? The commonest tell
is an iOS app in Android's skin: a phone bottom bar copied from iPhone, a back arrow that ignores
system Back, Cupertino-shaped switches and dialogs. Material 3 is the rulebook.

## Layout and structure

- Navigation matched to width: a bottom navigation bar (three to five destinations) on compact
  width, a rail or drawer on expanded width; never a phone bar untouched on a tablet.
- System Back always works — predictive Back gesture and Back button, never trapped or hijacked.
- Edge-to-edge with window insets — status bar, navigation bar, cutout, and keyboard — so
  content never hides behind system chrome.
- A top app bar for screen context; a FAB when the screen has one primary action.

## Touch targets

- 48×48 dp minimum for every target, at least 8 dp apart.

## Typography

- The Material type scale — Display, Headline, Title, Body, Label; map text to roles, never
  hand-pick sizes per screen.
- Roboto is the system face; a brand face is themed in through the type scale.
- `sp` units, never fixed pixels, so type follows the system font-size setting.

## Colour and theming

- Material colour roles (primary, on-primary, surface, surface-variant, secondary-container,
  outline, error); raw hex breaks light, dark, and contrast variants.
- Dynamic Color where it fits, with a static fallback.
- Dark theme is a first-class scheme, never a quick invert.
- Tonal elevation through the standard surface levels; no arbitrary drop shadows.

## Components and motion

- Material components — the four button styles, FAB, switches, chips, snackbars, bottom
  sheets, dialogs, navigation bar, rail, and drawer; never ported iOS controls.
- One FAB, one primary action.
- Snackbars for transient feedback; dialogs only for decisions that must interrupt.
- Material motion patterns (container transform, shared axis, fade through) with standard
  easing; the system Remove-animations setting honoured.

## Verifying the build

- Screenshots come from an emulator or device, never a browser (`adb exec-out screencap -p >
  <path>`), for every device class the app ships to.
- Dark theme and a raised font scale belong in the same pass.
- Emulators give breadth; gestures and performance need hardware — say which produced the
  evidence.
