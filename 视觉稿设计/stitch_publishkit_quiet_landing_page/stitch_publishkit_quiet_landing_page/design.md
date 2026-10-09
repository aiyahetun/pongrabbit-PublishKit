---
name: PublishKit
colors:
  surface: '#fbf9f4'
  surface-dim: '#dbdad5'
  surface-bright: '#fbf9f4'
  surface-container-lowest: '#ffffff'
  surface-container-low: '#f5f4ee'
  surface-container: '#efeee8'
  surface-container-high: '#e9e8e3'
  surface-container-highest: '#e4e2dd'
  on-surface: '#1b1c19'
  on-surface-variant: '#4e4638'
  inverse-surface: '#30312d'
  inverse-on-surface: '#f2f1eb'
  outline: '#807667'
  outline-variant: '#d1c5b3'
  surface-tint: '#795916'
  primary: '#795916'
  on-primary: '#ffffff'
  primary-container: '#b8924a'
  on-primary-container: '#412c00'
  inverse-primary: '#ebc073'
  secondary: '#5f5e59'
  on-secondary: '#ffffff'
  secondary-container: '#e5e2db'
  on-secondary-container: '#65645f'
  tertiary: '#435f8b'
  on-tertiary: '#ffffff'
  tertiary-container: '#7d99c9'
  on-tertiary-container: '#0f305a'
  error: '#ba1a1a'
  on-error: '#ffffff'
  error-container: '#ffdad6'
  on-error-container: '#93000a'
  primary-fixed: '#ffdea7'
  primary-fixed-dim: '#ebc073'
  on-primary-fixed: '#271900'
  on-primary-fixed-variant: '#5e4200'
  secondary-fixed: '#e5e2db'
  secondary-fixed-dim: '#c9c6c0'
  on-secondary-fixed: '#1c1c18'
  on-secondary-fixed-variant: '#474742'
  tertiary-fixed: '#d6e3ff'
  tertiary-fixed-dim: '#abc8fa'
  on-tertiary-fixed: '#001b3c'
  on-tertiary-fixed-variant: '#2a4772'
  background: '#fbf9f4'
  on-background: '#1b1c19'
  surface-variant: '#e4e2dd'
typography:
  h1:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
    letterSpacing: -0.02em
  h2:
    fontFamily: Inter
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 28px
    letterSpacing: -0.01em
  h3:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '600'
    lineHeight: 24px
  body-base:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  body-sm:
    fontFamily: Inter
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 18px
  label-xs:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '500'
    lineHeight: 16px
    letterSpacing: 0.01em
  mono:
    fontFamily: Inter
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 18px
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  sidebar_width: 220px
  container_gutter: 24px
  element_gap: 12px
  inner_padding: 16px
  baseline: 4px
---

## Brand & Style
The design system is rooted in the "Calm Productivity" movement, prioritizing cognitive clarity for cross-border marketers. The style is **Modern / Functional**, drawing inspiration from utility-first interfaces like Linear and Notion. 

The aesthetic is "quiet" and "trustworthy," utilizing a warm, paper-like neutral palette to reduce eye strain during long periods of orchestration. It favors high information density and precise utility over decorative flair. The visual narrative is defined by intentional whitespace, subtle borders, and a rigorous adherence to a functional grid.

## Colors
The palette uses a warm-gray foundation to create a professional, grounded atmosphere. 

- **Primary (Gold):** Reserved for primary actions, active states, and "Scheduled" statuses. Use the soft accent (12% opacity) for hover states on transparent buttons.
- **Surface Strategy:** Use `background_app` for the main layout backdrop and sidebar. Use `surface_main` for content cards and data tables to create a clear "layer of work."
- **Typography Colors:** `text_primary` (#1B1C19) for high-contrast reading. `text_secondary` (#4E4638) for descriptions and secondary navigation items. `text_muted` (#8A8278) for metadata and placeholder text.

## Typography
The system relies on **Inter** for its systematic, utilitarian qualities. For CJK characters, the system gracefully falls back to *PingFang SC* or *Microsoft YaHei*.

- **Information Density:** Use `body-sm` (13px) for data-heavy views such as tables and side-panels.
- **Hierarchy:** Reserve `h1` and `h2` for page headers and modal titles. Most UI interactions should occur at the `body-base` (14px) level.
- **Labels:** Use `label-xs` for status badges, tags, and table headers.

## Layout & Spacing
This design system uses a **Fixed-Fluid Hybrid** model. The sidebar remains at a fixed `220px`, while the main content area expands to fill the viewport, utilizing a maximum content width of `1440px` for optimal readability.

- **Sidebar:** Positioned on the left with `#F7F6F3` background and a 1px right border (`#E4E2DD`).
- **Rhythm:** A 4px baseline grid governs all spacing. Use `16px` (4x) for standard internal padding and `12px` (3x) for spacing between related elements.
- **Grid:** In the main content area, use a 12-column fluid grid for dashboard widgets and a simple single-column layout for document or post editing.

## Elevation & Depth
Depth is conveyed primarily through **Tonal Layering** and **Low-Contrast Outlines** rather than heavy shadows.

- **Level 0 (Background):** `#F7F6F3` used for the application shell and sidebar.
- **Level 1 (Surface):** `#FFFFFF` for the primary work area, cards, and tables. Defined by a 1px solid border in `#E4E2DD`.
- **Level 2 (Overlays):** Modals, dropdowns, and context menus. These use a very subtle ambient shadow: `0 4px 12px rgba(0, 0, 0, 0.05)` and a 1px border.
- **Interactions:** Hover states on surface elements should use a subtle shift to `#F9F8F6` rather than a shadow change.

## Shapes
The system uses a **Rounded** (8px) language to soften the utilitarian nature of the tool, making it feel approachable.

- **Standard (8px):** Applied to buttons, input fields, cards, and primary panels.
- **Pill (100px):** Exclusively for status badges and tags to distinguish them from interactive buttons.
- **Small (4px):** Used for nested elements like internal checkboxes or small tooltips.

## Components

### Buttons
- **Primary:** Background `#B8924A`, White text. No border.
- **Secondary:** White background, `#E4E2DD` border, `#1B1C19` text.
- **Ghost:** Transparent background, `#4E4638` text. Use `Accent Soft` on hover.

### Inputs & Selects
- Height: 36px for standard, 32px for compact (table filters).
- Border: 1px `#E4E2DD`. Focus state: 1px `#B8924A` with a subtle 2px outer glow of `Accent Soft`.

### Data Tables
- Header: Sticky, 12px Semibold `#8A8278` text, uppercase.
- Rows: 40px height, 1px bottom border. Hover state uses `#F7F6F3`.
- Zebra: Not default, but used for tables exceeding 20 rows.

### Status Badges
- Style: Pill-shaped with a leading 6px dot.
- Color Logic: The dot and text should use the semantic status color; the background should be a 10% opacity version of that same color.

### Navigation Sidebar
- Items: 32px height. Active state uses a vertical 2px "Gold" bar on the left edge and a text weight of 600.