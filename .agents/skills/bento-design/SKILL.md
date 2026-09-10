---
name: bento-design
description: Design and build modern Bento Grid layouts with modular card surfaces, crisp micro-borders, clean typography, 70/20/10 palette with vivid true orange accent (#ff6b00 / #f97316), and spring-easing micro-animations. Applies to web client UI work under `clients/web/`.
---

# Bento Grid Design System Skill

## Overview
Bento Grid UI creates compartmentalized, modular hierarchy where content lives in structured cards with consistent border-radii (`16px–20px`), subtle translucent border boundaries (`rgba(0,0,0,0.08)` / `rgba(255,255,255,0.08)`), rich card-level elevation, and restrained, energetic True Orange accenting (`#ff6b00` / `#f97316`).

---

## 70/20/10 Color Rule
- **70% Base Canvas**: Deep neutral backdrop (`#f8fafc` / `#fafafa` in light mode).
- **20% Elevated Bento Cards**: Crisp white surfaces (`#ffffff` / `#ffffff`) with subtle 1px border lines and multi-layer soft shadows.
- **10% True Orange Accent**: `#ff6b00` / `#f97316` strictly for active states, primary CTA buttons, badges, key metrics, and hover reveals.

---

## Typography
Distinctive product-forward typography pairing:
- **Display / Headings**: `Outfit` (weights 600, 700, 800)
- **Body / Interface**: `Plus Jakarta Sans` or `Inter` (weights 400, 500, 600)

---

## Micro-Animations
- **Hover Lift**:
  ```css
  transition: transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 0.25s ease, border-color 0.25s ease;
  ```
- **Stagger Entrance**: `animation: cellIn 0.45s cubic-bezier(0.16, 1, 0.3, 1) both;`
