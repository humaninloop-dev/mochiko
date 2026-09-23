# iOS platform guidance

> Ported from `pbakaus/impeccable` at `e0881d2de397d5e9761d7b35ff5017d8f5ebf69b`
> (`skill/reference/ios.md`, Apache-2.0), itself derived in part from
> `ehmo/platform-design-skills` (MIT); rewritten in mochiko's form. Attribution chain in the
> repository's root `NOTICE`.

Loaded when the design baseline's `Platform` value is `ios` or `adaptive`. It covers native iOS
and iPadOS apps — SwiftUI, UIKit, React Native, Expo, or Flutter shipping to Apple hardware.

On native, the mode narrows what expression may override. Apple's Human Interface Guidelines
govern structure, navigation, and interaction in every mode; brand speaks through the layer the
platform leaves open — tint, type, motion, content.

## The trust test

Would a fluent iPhone user trust this app, or pause at off-spec controls? The tell is "ported
from a website": reinvented navigation bars, custom back gestures, web-shaped buttons,
hover-dependent affordances. Default to the platform's components; depart only for a reason the
user would thank you for.

## Layout and structure

- Lay out inside the safe-area insets — nothing under the notch, Dynamic Island, home
  indicator, or rounded corners.
- System navigation: a tab bar for two to five top-level sections (sections, never actions), a
  navigation stack for hierarchy, a sheet for a self-contained task. No custom global nav.
- The left-edge swipe back stays alive; never disable or overlay it.
- Large titles on top-level screens, collapsing inline on scroll; deep detail screens inline.

## Touch targets

- 44×44 pt minimum for every tappable control, with room between neighbours.

## Typography

- Dynamic Type through the system text styles (Large Title to Caption); no hard-coded sizes.
- San Francisco carries body, labels, and controls; a brand face may take display moments.
- 11 pt floor; Body is 17 pt.

## Colour and materials

- Semantic system colours (label, secondaryLabel, systemBackground, separator, tint) — they
  adapt to Dark Mode and increased contrast; raw hex breaks there.
- Dark Mode is a first-class appearance, designed and tested.
- One tint colour drives interactive elements; decoration is not its job.
- System materials for blur behind bars and sheets; no hand-rolled glass.

## Components and controls

- Platform controls — switch, segmented control, stepper, pickers, action sheets, alerts,
  context menus, swipe actions. Reinventing these for flavour is the commonest native slop.
- SF Symbols for icons; no web icon set mixed in.
- Deliberate modality: a sheet for a dismissible sub-task, a full-screen cover for immersion,
  clear Cancel/Done, swipe-to-dismiss honoured unless data loss needs a guard.
- Grouped or inset lists for settings-shaped content; no bespoke card stacks.

## Motion

- System transitions: push slides, sheets rise, dismiss reverses the entrance.
- Reduce Motion honoured: crossfade in place of parallax and large slides.

## Verifying the build

- Screenshots come from the Simulator, never a browser (`xcrun simctl io <udid> screenshot
  <path>`), for every device class the app ships to.
- Dark Mode and a large Dynamic Type size belong in the same pass.
- Simulators give breadth; gestures and performance need hardware — say which produced the
  evidence.
