---
trigger: model_decision
description: shadcn-svelte (Svelte 5 Runes) and Tailwind CSS v4 component architecture and styling rules for clients/web/
---

# shadcn-svelte & Tailwind CSS v4 Development Rules

## 1. Scope
These rules apply to **all `**/*.svelte` files** and UI styling under `clients/web/`.

## 2. Core Styling Mandates
- **Zero Vanilla `<style>` Blocks**: Never write custom component `<style>` blocks in `.svelte` files. All styling must be achieved using Tailwind CSS utility classes and shadcn-svelte component primitives.
- **shadcn-svelte Component-First**: Whenever building interactive elements, use official primitives from `$lib/components/ui/*`:
  - Actions: `$lib/components/ui/button` (`Button`), `$lib/components/ui/dropdown-menu` (`DropdownMenu`)
  - Layout & Bento Cells: `$lib/components/ui/card` (`Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `CardFooter`)
  - Modals & Overlays: `$lib/components/ui/dialog` (`Dialog`, `DialogContent`, `DialogHeader`, `DialogTitle`, `DialogFooter`), `$lib/components/ui/sheet` (`Sheet`)
  - Forms: `$lib/components/ui/input` (`Input`), `$lib/components/ui/label` (`Label`), `$lib/components/ui/select` (`Select`)
  - Data: `$lib/components/ui/table` (`Table`, `TableHeader`, `TableBody`, `TableRow`, `TableHead`, `TableCell`), `$lib/components/ui/badge` (`Badge`)
  - Navigation: `$lib/components/ui/tabs` (`Tabs`, `TabsList`, `TabsTrigger`, `TabsContent`)

## 3. Bento Grid Design System & Palette (70/20/10 Rule)
- **70% Neutral Canvas**: Background surfaces use `bg-[#f8fafc]` / `bg-slate-50` (or `bg-background` in dark mode).
- **20% Elevated White Bento Cards**: Cards use `bg-white dark:bg-card rounded-[18px] border border-black/[0.08] dark:border-white/[0.08] shadow-sm hover:shadow-md transition-all duration-200`.
- **10% True Orange Accent**:
  - Primary buttons and active highlights strictly use `#ff6b00` / `#f97316` (`bg-primary text-primary-foreground hover:bg-primary/90` or `text-[#ff6b00]`).
  - Badge active states: `bg-[#ff6b00]/10 text-[#ff6b00] border-[#ff6b00]/25`.

## 4. Svelte 5 Runes & Bits UI Conventions
- Components use Svelte 5 runes: `$props()`, `$state()`, `$derived()`, and snippet slots `{@render children?.()}`.
- Combine conditional classes using `cn(...)` from `$lib/utils.ts`.

## 5. Strict Internationalization (i18n)
- Never hardcode user-visible text in components. Always use `$tStore('key')` or `translate(locale, 'key')`.
