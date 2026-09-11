---
name: bento-frontend-qa
description: Use when auditing, inspecting, or validating web frontend visual quality, typography, spacing rhythm, dynamic component states, text clipping, and Bento Grid design compliance.
---

# Bento Frontend Quality Control (QC) Skill

## Overview

Frontend Quality Control (QC) ensures that web interfaces are visually wowed, spatially coherent, responsive, accessible, and strictly compliant with the **Bento Grid Design System** ([bento-design](file://.agents/skills/bento-design/SKILL.md)).

Visual design quality cannot be guessed from code alone. QC requires combined **computed CSS DOM auditing** and **visual snapshot verification** using Playwright MCP tools.

---

## When to Use

Use this skill whenever:
- Reviewing or completing frontend UI work under `clients/web/` or any web component suite.
- Auditing Bento Grid layout structure, cards, micro-borders, typography, and accent colors.
- Validating interactive states (hover, focus, active, disabled, loading, empty, error).
- Verifying responsive layout behavior across desktop, tablet, and mobile viewports.
- Running visual regression checks or verifying design system compliance before pull requests/merges.

Do NOT use for backend API verification, database migration testing, or non-visual unit tests.

---

## Bento Design System Audit Checklist

Every audited surface MUST comply with the core Bento design parameters:

### 1. 70 / 20 / 10 Palette Rule
- **70% Base Canvas**: Deep neutral backdrop (`#f8fafc`, `#fafafa`, or dark canvas `#0f172a` / `#090d16`).
- **20% Elevated Bento Cards**: Pure card surfaces (`#ffffff` or dark card surface `#1e293b`) featuring:
  - `border-radius`: `16px` to `20px` (or `rem` equivalent `1rem` - `1.25rem`).
  - `border`: Subtle 1px translucent border (`rgba(0,0,0,0.08)` light mode / `rgba(255,255,255,0.08)` dark mode).
  - `box-shadow`: Multi-layer soft elevation shadow (e.g. `0 4px 20px -2px rgba(0,0,0,0.05)`).
- **10% True Orange Accent**: Strict usage of `#ff6b00` or `#f97316`.
  - **Allowed**: Primary CTAs, active navigation indicators, key metric highlights, status badges, focus outlines, hover reveal borders.
  - **Forbidden**: Large background flood areas, secondary container cards, passive body text.

### 2. Typography Hierarchy & Font Stack
- **Display & Headings (`h1`–`h4`, hero stats)**: `Outfit`, sans-serif (Font weights: `600`, `700`, `800`).
- **Body, Inputs, & UI Labels**: `Plus Jakarta Sans`, `Inter`, or system sans-serif (Font weights: `400`, `500`, `600`).
- **Monospace / Code / Timetable slots**: `JetBrains Mono`, `Fira Code`, or `ui-monospace`.
- **Line Heights**: Headings `1.1` to `1.25`, Body text `1.5` to `1.6`.
- **Text Contrast**: Text contrast ratio MUST meet WCAG AA standards (minimum `4.5:1` for normal text, `3:1` for large text).

### 3. 8px Spatial Rhythm & Spacing Grid
- **Padding & Margins**: All container paddings and gaps MUST follow an 8px grid multiplier (`8px`, `16px`, `24px`, `32px`, `48px`). Exception: `4px` for tight inline micro-badges.
- **Card Gap**: Bento grid gaps MUST be uniform (`16px` or `24px`).
- **No Arbitrary Magic Numbers**: Reject arbitrary offsets like `margin-top: 13px` or `left: 7px`.

### 4. Interactive States & Micro-Animations
- **Hover Lift**: Bento cards and interactive surfaces must have smooth spring-easing hover transitions:
  ```css
  transition: transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 0.25s ease, border-color 0.25s ease;
  ```
- **Hover Behavior**: Card lifts up slightly (`translateY(-2px)` to `translateY(-4px)`), border shifts subtly to True Orange translucent glow (`rgba(255,107,0,0.3)`).
- **Focus States**: Visible focus indicator with `2px` True Orange ring offset.
- **Layout Shift (CLS)**: Zero sudden Cumulative Layout Shift when hovering or loading dynamically.

---

## Playwright MCP QC Inspection Workflow

Follow this step-by-step audit process when performing frontend quality control:

```
+-----------------------------------------------------------------------+
| 1. Set Viewports (Desktop: 1440x900, Tablet: 768x1024, Mobile: 375x812)|
+-----------------------------------------------------------------------+
                                   |
                                   v
+-----------------------------------------------------------------------+
| 2. Navigate & Capture Baseline Screenshot (browser_take_screenshot)   |
+-----------------------------------------------------------------------+
                                   |
                                   v
+-----------------------------------------------------------------------+
| 3. DOM & CSS Computed Style Audit (browser_evaluate script)           |
+-----------------------------------------------------------------------+
                                   |
                                   v
+-----------------------------------------------------------------------+
| 4. Audit Interactive States (browser_hover, browser_click, focus)    |
+-----------------------------------------------------------------------+
                                   |
                                   v
+-----------------------------------------------------------------------+
| 5. Capture State Screenshots & Log Findings                           |
+-----------------------------------------------------------------------+
```

### Step 1: Configure Viewport Dimensions
Set the browser resolution before taking snapshots to test responsiveness:
- **Desktop**: `1440` x `900`
- **Tablet**: `768` x `1024`
- **Mobile**: `375` x `812`

### Step 2: Capture Visual Baseline Screenshots
Use Playwright MCP `browser_take_screenshot` to visually inspect the overall page layout, card alignments, typography scale, and color distribution.

> [!TIP]
> **Fallback (Environment Binary Fallback)**: If Playwright MCP tools fail to launch or download driver binaries due to network/CDN restrictions, use system Chromium headless directly to capture visual snapshots:
> ```bash
> chromium --headless=new --no-sandbox --disable-gpu --window-size=1440,900 --screenshot=snapshot.png http://localhost:5174
> ```

### Step 3: Execute Automated Computed Style Audits
Run browser evaluation scripts (`browser_evaluate`) to query live computed styles across all Bento elements.

#### Audit Script: Bento Card System Verification
```javascript
() => {
  const cards = Array.from(document.querySelectorAll('.bento-card, [class*="card"], [class*="bento"]'));
  return cards.map((card, i) => {
    const cs = window.getComputedStyle(card);
    return {
      index: i,
      className: card.className,
      borderRadius: cs.borderRadius,
      borderColor: cs.borderColor,
      backgroundColor: cs.backgroundColor,
      boxShadow: cs.boxShadow !== 'none',
      padding: cs.padding
    };
  });
}
```

#### Audit Script: Typography & Font Family Audit
```javascript
() => {
  const headings = Array.from(document.querySelectorAll('h1, h2, h3, h4, .heading'));
  return headings.map(h => ({
    tag: h.tagName,
    text: h.innerText.slice(0, 30),
    fontFamily: window.getComputedStyle(h).fontFamily,
    fontWeight: window.getComputedStyle(h).fontWeight,
    fontSize: window.getComputedStyle(h).fontSize,
    color: window.getComputedStyle(h).color
  }));
}
```

#### Audit Script: 8px Spacing Grid Compliance Check
```javascript
() => {
  const elements = Array.from(document.querySelectorAll('.bento-card, header, main, section, nav'));
  const violations = [];
  elements.forEach(el => {
    const cs = window.getComputedStyle(el);
    ['paddingTop', 'paddingBottom', 'paddingLeft', 'paddingRight', 'gap'].forEach(prop => {
      const val = parseInt(cs[prop], 10);
      if (val > 0 && val % 4 !== 0) {
        violations.push({ element: el.tagName + '.' + el.className, prop, value: val });
      }
    });
  });
  return violations;
}
```

### Step 4: Audit Dynamic Component States & Button Layouts
Verify that the UI gracefully handles all interactive and edge-case states:
1. **Default State**: Clean card separation, readable text, clear visually prioritized CTA.
2. **Card Action Buttons & Text Clipping**: Multi-button rows inside Bento cards (e.g. Kanban action buttons) MUST use responsive flex wrapping (`flex-wrap: wrap`) and fluid flex basis (`flex: 1 1 auto`). Single-line fixed flex row layout is **FORBIDDEN** when button labels can exceed container width.
3. **Hover State**: Trigger `browser_hover` on cards, buttons, and nav items; capture screenshot; check transform lift and accent glow.
4. **Active/Pressed State**: Button press down visual feedback (`transform: scale(0.98)`).
5. **Focus State**: Keyboard navigation via `browser_press_key` (`Tab`); verify focus rings are distinct and not clipped.
6. **Loading & Skeleton States**: Verify loading shimmer indicators use smooth pulse animations and match card dimensions.
7. **Empty & Error States**: Verify empty data placeholders are centered with clear iconography and secondary messaging.

---

## QC Defect Severity Matrix

| Severity | Defect Trigger | Action Required |
|---|---|---|
| **CRITICAL** | Overlapping text/cards, clipped button labels, broken layout on mobile/tablet, unhandled contrast failure (< 3.0:1), broken primary CTAs. | Block build/release immediately; fix layout & structure. |
| **MAJOR** | Wrong font stack (missing `Outfit` or `Plus Jakarta Sans` fallbacks), missing hover states, cards without border-radius or micro-borders, true orange used as backdrop flood. | Fix before merge; align with Bento design tokens. |
| **MINOR** | Spacing off 8px grid by 1–3px, subtle shadow inconsistency, line-height slightly tight. | Adjust CSS variables and spacing utilities. |

---

## Common Rationalizations & Red Flags

| Rationalization / Excuse | Reality |
|---|---|
| *"It looks fine in code without taking a screenshot."* | Code inspection cannot detect layout overlap, font rendering bugs, or unexpected z-index clipping. Always capture visual snapshots via Playwright or system Chromium headless. |
| *"The button label is only slightly cut off at the edge, it's acceptable."* | Text clipping degrades professional UI quality and accessibility. Card action button rows must use `flex-wrap: wrap` and `flex: 1 1 auto`. |
| *"Mobile viewport testing isn't necessary for internal dashboard tools."* | Responsive failure ruins usability on laptops with low scaling or narrow split windows. Test all 3 standard viewports. |
| *"Hover states are an unnecessary detail."* | Micro-interactions elevate a modern app from basic MVP to premium status. Hover lifts are mandatory for Bento cards. |
| *"Hardcoding `margin: 11px` was faster than using design tokens."* | Magic numbers ruin spatial rhythm across components. Stick strictly to 4px/8px increments. |

### Red Flags - STOP & Fix Immediately
- Missing translucent border boundaries (`rgba(0,0,0,0.08)` / `rgba(255,255,255,0.08)`) on white/dark cards.
- True Orange (`#ff6b00`) covering >10% of the visual layout surface.
- Absence of `Outfit` font stack on section titles or key headers.
- Interactive elements without visible `:focus-visible` styles.
