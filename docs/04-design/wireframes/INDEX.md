# ChronoXP™ Wireframes Index

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

This directory contains comprehensive wireframe documentation for every screen, interaction, and edge case in ChronoXP™ Journal. These wireframes serve as the single source of truth for developers, designers, and stakeholders building the application.

---

## Complete Wireframe Documentation

### Core Application Surfaces

#### [01. App Shell & Global Navigation](./01-app-shell.md)
**What's covered:**
- Window chrome (Windows, macOS, Linux)
- Global navigation bar (Canvas, Library, Cavern, Settings)
- Command Bar (Ctrl/Cmd+K) with fuzzy search
- Keyboard shortcuts overlay (Ctrl/Cmd+?)
- Global status bar (autosave, word count, XP, network)
- Notification toasts (artifacts, quests, errors)
- Context menus (right-click interactions)
- Fullscreen/Zen mode
- Focus indicators (accessibility)

**Why it matters:** The app shell is the foundation—everything else lives inside it.

---

#### [02. Journal Canvas (Writing Surface)](./02-journal-canvas.md)
**What's covered:**
- Main writing interface with ghost placeholder
- Muse AI suggestions (inline prompts)
- Formatting toolbar (Bold, Italic, Headings, Lists)
- Tagging inline (add/remove tags while writing)
- Mood selector (9 pre-defined moods + custom)
- Autosave indicator states (saving, saved, failed)
- Entry navigation (previous/next with Ctrl+[ / ])
- Archivist auto-tagging notifications (AI feature)
- Word count milestones (100, 500, 1000 words)
- Dark mode / low-stimulation theme variations
- Related entries sidebar (semantic links)

**Why it matters:** This is where users spend 80% of their time—must be distraction-free and delightful.

---

#### [03. Library Surface](./03-library-surface.md)
**What's covered:**
- **Calendar View:** Month grid with entry indicators
- **Timeline View:** Chronological list with infinite scroll
- **Tags View:** Browse all tags, filter by tag
- Filter & sort bar (date range, mood, word count)
- Entry preview panel with metadata
- Pinned entries section (always at top)
- Bulk selection mode (select multiple entries)
- Context menu (edit tags, pin, export, delete)
- Quick search bar (keyword filter)
- Empty states (no entries, no tags)

**Why it matters:** The Library is the "home base" for navigating past entries—needs to be fast and intuitive.

---

#### [04. Search Modal](./04-search-modal.md)
**What's covered:**
- Full-screen search overlay (Ctrl/Cmd+F)
- Keyword search (SQLite FTS index)
- Semantic search (AI-powered, Pro feature)
- Advanced filters (date, tags, mood, word count)
- Search history (last 5 searches)
- No results state with suggestions
- Inline preview (expand entry without leaving search)
- Search command syntax (tag:, mood:, words:, etc.)
- Recent searches + quick filters

**Why it matters:** Search is the primary way to "recall by meaning"—a killer feature of ChronoXP.

---

#### [05. Records Cavern](./05-records-cavern.md)
**What's covered:**
- Hand-painted 2D fantasy RPG aesthetic
- Cavern evolution (5 tiers: Humble Study → Master's Sanctum)
- Artifact placement (8 unlockable items + future expansions)
- XP progress bar (wooden plaque, painted UI)
- Artifact interaction (click to examine, modal with closeup)
- XP Log (view all earning actions)
- Artifact unlock toast notifications
- Quest prompts (reflection prompts, optional)
- Pan & zoom controls (explore scene details)
- Fullscreen immersive mode
- Simple Mode (hides Cavern entirely)
- Easter eggs (click window, bookshelf, moss patches)

**Why it matters:** Gamification must feel cozy and rewarding, not stressful—this is the visual heart of that experience.

---

#### [06. Error States & Empty States](./06-error-states.md)
**What's covered:**
- **Empty states:** No entries, no search results, no tags, empty Cavern
- **Error states:**
  - Autosave failed (retry, manual save, export to .txt)
  - Database locked (vault password entry)
  - Database corruption (automatic repair flow)
  - AI model missing (download prompt)
  - Disk space low (free up space warning)
  - Import/export errors
  - Unsaved changes warning
  - Network errors (future LAN sync)
- **Loading states:** App launch, AI model initialization, search progress
- **Confirmation dialogs:** Delete entry, delete multiple, clear all data
- **Recovery flows:** Crash recovery (restore draft), backup restoration

**Why it matters:** Errors are inevitable—graceful handling builds trust and prevents data loss.

---

### Supporting Features

#### [07. Settings Panel](./07-settings-panel.md)
**What's covered:**
- **Appearance settings:** Theme selector (Light, Dark, Low-Stimulation, High-Contrast), font family/size/line height, UI preferences, window behavior
- **AI Features settings:** Muse configuration (frequency, writing style), Archivist (auto-tagging), Concierge (semantic search), model management
- **Privacy & Data:** Vault password management, encryption settings, data storage location, backup creation, privacy controls
- **Keyboard shortcuts:** View/customize all shortcuts, export/import configurations, Vim-style navigation option
- **Accessibility:** High contrast mode, large focus indicators, text zoom, cursor size, motion reduction, screen reader optimization, cognitive support, color blindness simulation
- **Export & Import:** Export dialog (multiple formats), import wizard (Day One, Journey, Markdown), automatic backups (Pro)
- **About:** Version info, license status, system diagnostics, update checker, credits, open-source licenses

**Why it matters:** Settings empower users to customize their experience without overwhelming them with options.

---

#### [08. Onboarding & First Run](./08-onboarding-first-run.md)
**What's covered:**
- Welcome screen (Get Started, Import from Another App)
- Vault setup wizard (device-only vs password protection)
- Password creation flow (with strength meter, recovery warning)
- AI model download (optional, ~2.8 GB, progress indicator)
- Feature tour (5 interactive steps: Canvas, Muse, Library, Cavern, Command Bar)
- Import wizard (Day One, Journey, Markdown, .txt support)
- First entry celebration (confetti, +5 XP)
- Vault unlock screen (subsequent launches with password)
- Pro upgrade prompt (gentle, after 7 days)

**Why it matters:** First impressions set the tone—onboarding must be welcoming, clear, and skippable.

---

#### [09. Modals & Overlays](./09-modals-overlays.md)
**What's covered:**
- **Tag picker modal:** Search/create tags, AI suggestions, recent tags, current tags
- **Mood selector dropdown:** 9 pre-defined moods + [No mood]
- **Export dialog:** Format selection (.chrxp, .md, .json, .txt), date range, encryption option, progress indicator
- **Import dialog:** File picker, format detection, merge vs replace, duplicate handling
- **Artifact detail modal:** Hand-painted closeup, flavor text, unlock date, related entries navigation
- **Quest prompt modal:** Reflection prompts with [Start] [Maybe Later] [Don't Show Again]
- **Artifact unlock toast:** Non-blocking notification with [View in Cavern]
- **Unsaved changes warning:** [Save and Close] [Don't Save] [Cancel]
- **Delete confirmations:** Single entry, bulk delete, clear all data (with "DELETE ALL" typing confirmation)
- **Pro upgrade modal:** Feature breakdown, one-time $1.99 purchase, 30-day guarantee
- **Subscription upgrade modal:** Level 1/2/Annual/Lifetime comparison, pricing
- **Keyboard shortcuts overlay:** Full shortcut reference with [Print Cheat Sheet]
- **Update available notification:** Release notes, download, remind later

**Why it matters:** Modals interrupt flow—they must be purposeful, clear, and easily dismissible.

---

#### [10. Easter Eggs & Hidden Features](./10-easter-eggs.md)
**What's covered:**
- **Konami Code:** Unlock developer mode (Ctrl+Shift+D console, database inspector, secret artifact)
- **Cavern secrets:** Cosmic window (click to change weather), moss patch (mushrooms sprout), cat silhouette (random 1% chance), bookshelf hidden note
- **Canvas secrets:** Triple-click for encouragement, type "xyzzy" (text adventure nod), type "hello Muse" (personal greeting)
- **Library achievements:** Perfect month (entry every day = golden glow + 50 XP), click date 7x (time-travel mode)
- **Search easter eggs:** "meaning of life" (philosophical message), "42" (Hitchhiker's Guide reference)
- **Command Bar secrets:** "surprise me" (random action), "make it rain" (confetti), "unicorn mode" (silly artifact variants)
- **Settings secret:** Click version number 10x (developer credits)
- **XP milestones:** Write at 3:33 AM (Night Owl), 1000+ word entry (Epic Entry), write on birthday (Birthday Reflection), first anniversary (Year in Review)
- **Accessibility:** Option to disable all easter eggs (focus mode)

**Why it matters:** Easter eggs add personality and delight without compromising core functionality—they're love letters to curious users.

---

## Wireframe Reading Guide

### ASCII Art Legend

```
┌─┐  Box borders (UI containers)
│ │  Vertical lines
└─┘  Corners

[Button]  Clickable button
🔍  Icons (use emoji as placeholders)
▌  Cursor position
• ○  Bullets (filled, empty)
⚠️  Warning indicator
✓  Success/checkmark
⟳  Loading spinner
📝  Placeholder icons
```

### Interaction Annotations

- **Click:** Primary mouse action
- **Right-click:** Opens context menu
- **Hover:** Shows tooltip or highlight
- **Drag:** Click + drag to pan/move
- **Keyboard:** Hotkey shortcuts (e.g., `Ctrl+K`)
- **Focus:** Tab to navigate, Enter to activate

---

## Design Principles (Applied Throughout)

### 1. Neuro-Affirming UX
- **Low-stimulation default:** Soft colors, minimal animations
- **Clear hierarchy:** Obvious primary actions
- **No shaming:** Positive language, no "you missed a day"
- **Distraction-free:** Minimal chrome, focus on content

### 2. Keyboard-First Navigation
- Every feature accessible via keyboard
- Tab order logical (top-left → bottom-right)
- Escape always closes modals
- Command Bar (Ctrl+K) as universal launcher

### 3. Privacy-First Design
- No telemetry, no analytics, no network calls
- Offline indicator (📶 Offline) is green (intentional)
- Encrypted vault password never visible
- Export/import local-only (.chrxp format)

### 4. Graceful Degradation
- AI features optional (can be disabled)
- Free tier fully functional (not crippled demo)
- Errors handled with recovery options (never dead ends)

---

## How to Use This Documentation

### For Developers
1. **Start with App Shell:** Understand global navigation and window structure
2. **Read surface-by-surface:** Canvas → Library → Search → Cavern → Settings
3. **Check error states:** Handle edge cases before they ship
4. **Accessibility notes:** Implement keyboard nav, screen reader support, focus management

### For Designers
1. **Visual style:** Hand-painted RPG aesthetic (Skyrim texture work, warm fantasy)
2. **Color palette:** Soft blues, greens, ambers (low-stimulation)
3. **Typography:** Dyslexia-friendly fonts (OpenDyslexic, Atkinson Hyperlegible, Comic Sans)
4. **Animations:** Subtle, respectful of reduced motion preference

### For Product Managers
1. **Feature prioritization:** P0 features in Canvas/Library, P1 in Cavern/Search
2. **User flows:** Follow UX flows in `/docs/04-design/flows/` for happy paths
3. **Edge cases:** Review error states for user support scenarios
4. **Accessibility:** WCAG 2.1 Level AA compliance (see accessibility notes in each wireframe)

---

## Performance Targets (From Wireframes)

| Action | Target | Notes |
|--------|--------|-------|
| App launch (cold) | <400ms | Measured to main window visible |
| Canvas keystroke latency | <16ms | 60fps, instant feedback |
| Autosave | <3s | After last keystroke |
| Command Bar open (Ctrl+K) | <50ms | Instant feel |
| Search (keyword) | <50ms | SQLite FTS indexed |
| Search (semantic, AI) | <500ms | Vector similarity query |
| Library Calendar render | <100ms | Month grid with indicators |
| Cavern render | <200ms | Initial load with artifacts |
| Pan/Zoom (Cavern) | 60fps | Smooth parallax scrolling |

---

## Accessibility Checklist (All Wireframes)

- ✅ **Keyboard navigation:** All features accessible via Tab, Arrow keys, Enter, Esc
- ✅ **Screen reader support:** Semantic HTML, ARIA labels, live regions for toasts
- ✅ **Focus indicators:** 3px blue outline, 2px offset, high contrast
- ✅ **Color contrast:** WCAG AA (4.5:1 for text, 3:1 for UI components)
- ✅ **Reduced motion:** No parallax, no pulse animations, fade only
- ✅ **Text scaling:** UI scales to 200% zoom (WCAG requirement)
- ✅ **No color-only indicators:** Use icons + text, not just color

---

## Cross-Reference: Related Documentation

### Vision & Strategy
- [`/docs/01-vision/vision.md`](../../../01-vision/vision.md) — Core mission and non-negotiables
- [`/docs/01-vision/jtbd.md`](../../../01-vision/jtbd.md) — Jobs to be done

### User Research
- [`/docs/02-research/personas.md`](../../../02-research/personas.md) — 3 target personas

### Product Pillars
- [`/docs/03-product/pillars.md`](../../../03-product/pillars.md) — 4 pillars with feature mapping

### UX Flows
- [`/docs/04-design/flows/f-1-start-writing.md`](../flows/f-1-start-writing.md) — First session UX
- [`/docs/04-design/flows/f-2-save-tag.md`](../flows/f-2-save-tag.md) — Autosave and tagging
- [`/docs/04-design/flows/f-3-recall-meaning.md`](../flows/f-3-recall-meaning.md) — Semantic search
- [`/docs/04-design/flows/f-4-export-packet.md`](../flows/f-4-export-packet.md) — Export .chrxp format
- [`/docs/04-design/flows/f-5-cavern-reward.md`](../flows/f-5-cavern-reward.md) — Artifact unlock

### Technical Architecture
- [`/docs/05-architecture/overview.md`](../../../05-architecture/overview.md) — Tech stack and data flow
- [`/docs/05-architecture/data-model.md`](../../../05-architecture/data-model.md) — Database schema

### Gamification Design
- [`/docs/06-gamification/design.md`](../../../06-gamification/design.md) — XP system, artifacts, ethical design

### UX Foundations
- [`/docs/07-ux/accessibility.md`](../../../07-ux/accessibility.md) — WCAG compliance
- [`/docs/07-ux/keyboard-map.md`](../../../07-ux/keyboard-map.md) — 60+ keyboard shortcuts
- [`/docs/07-ux/microcopy-guide.md`](../../../07-ux/microcopy-guide.md) — Voice, tone, specific phrases

### Business & Monetization
- [`/docs/08-product/monetization.md`](../../../08-product/monetization.md) — Pricing strategy
- [`/docs/08-product/tier-roadmap.md`](../../../08-product/tier-roadmap.md) — Feature roadmap by tier

### Security & Privacy
- [`/docs/09-security/privacy.md`](../../../09-security/privacy.md) — Encryption, threat model, compliance

### Acceptance Criteria
- [`/docs/10-delivery/acceptance.md`](../../../10-delivery/acceptance.md) — 100+ test cases for MVP

---

## Changelog

| Date | Wireframe | Changes |
|------|-----------|---------|
| Oct 23, 2025 | All (Initial) | Created comprehensive wireframe documentation (6 complete docs) |
| Oct 23, 2025 | 06. Error States | Added error handling, empty states, recovery flows |
| Oct 23, 2025 | 05. Records Cavern | Updated visual style to hand-painted RPG aesthetic |
| Oct 23, 2025 | INDEX | Created master navigation document |

---

## Contributing to Wireframes

### Adding New Wireframes
1. Create new `.md` file in `/docs/04-design/wireframes/`
2. Use ASCII art for layouts (see existing wireframes for style)
3. Include: Overview, numbered sections, accessibility notes, performance targets, developer notes
4. Update this INDEX.md with link and summary

### Updating Existing Wireframes
1. Make changes directly in wireframe `.md` file
2. Update **Changelog** section in INDEX.md
3. Cross-check related docs (flows, accessibility, keyboard map) for consistency

---

## Questions & Feedback

**For wireframe clarifications:**
- Developers: Check developer notes section in each wireframe
- Designers: Reference visual style sections and cross-link to design system
- PM/Stakeholders: Start with surface-specific overview sections

**For feature changes:**
- Update wireframe first (source of truth)
- Then update related docs (flows, acceptance criteria, etc.)
- Keep wireframes in sync with implementation

---

**End of Wireframes Index**

*This is the master navigation document. For detailed wireframes, click links above.*
