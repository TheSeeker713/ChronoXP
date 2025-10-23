# Wireframe: Journal Canvas (Writing Surface)

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

The Journal Canvas is the primary writing surface where users create and edit entries. It's designed for minimal distraction with optional AI assistance.

---

## 1. Canvas Layout (Default State)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌────────────────────────────────────────────────────────────┐     │
│  │ [B] [I] [T] [🎨]                 📌 Pin  |  🗓️ Nov 15, 2025  │     │
│  ├────────────────────────────────────────────────────────────┤     │
│  │                                                            │     │
│  │  Untitled Entry                                           │     │
│  │                                                            │     │
│  │  Want a soft start? Try a one-line check-in.             │     │
│  │  ▌                                                         │     │
│  │                                                            │     │
│  │                                                            │     │
│  │                                                            │     │
│  │                                                            │     │
│  │                                                            │     │
│  │                                                            │     │
│  │                                                            │     │
│  │                                                            │     │
│  │                                                            │     │
│  │                                                            │     │
│  └────────────────────────────────────────────────────────────┘     │
│                                                                       │
│  Last saved: just now  |  Word count: 0  |  XP: 250  |  📶 Offline  │
└─────────────────────────────────────────────────────────────────────┘
```

### Components

#### Top Toolbar
- **Formatting:** [B] Bold, [I] Italic, [T] Add Tag, [🎨] Mood
- **Actions (Right):**
  - 📌 Pin — Pin entry to top of Library
  - 🗓️ Date — Shows creation date, click to edit

#### Title Field
- **Placeholder:** "Untitled Entry"
- **Auto-title:** AI suggests title after 50+ words (optional)
- **Editable:** Click to edit, Enter to save and move to content

#### Content Area
- **Ghost Placeholder:** "Want a soft start? Try a one-line check-in."
  - Disappears on first keystroke
  - Non-judgmental, encouraging tone
- **Cursor:** Blinking cursor (▌)
- **Auto-focus:** Cursor auto-focuses on Canvas load

---

## 2. Canvas (Writing in Progress)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌────────────────────────────────────────────────────────────┐     │
│  │ [B] [I] [T] [🎨]                 📌 Pin  |  🗓️ Nov 15, 2025  │     │
│  ├────────────────────────────────────────────────────────────┤     │
│  │                                                            │     │
│  │  Reflecting on Travel                                     │     │
│  │                                                            │     │
│  │  Today I realized how much I miss exploring new          │     │
│  │  places. There's something about being in a foreign      │     │
│  │  city where everything feels fresh and unfamiliar. The   │     │
│  │  street signs in different languages, the smell of       │     │
│  │  unfamiliar food cooking, even the sound of people       │     │
│  │  speaking in accents I can't quite place.                │     │
│  │                                                            │     │
│  │  Maybe it's time to plan another trip. Somewhere with    │     │
│  │  mountains this time. ▌                                   │     │
│  │                                                            │     │
│  │  ┌──────────────────────────────────────────┐ ← Muse     │     │
│  │  │ 💭 Muse suggests:                         │   (AI)     │     │
│  │  │ "Where would you go if you could leave    │            │     │
│  │  │  tomorrow?"                         [Tab] │            │     │
│  │  └──────────────────────────────────────────┘            │     │
│  │                                                            │     │
│  │                                                            │     │
│  └────────────────────────────────────────────────────────────┘     │
│                                                                       │
│  Last saved: 8 seconds ago  |  Word count: 97  |  XP: 250           │
└─────────────────────────────────────────────────────────────────────┘
```

### Muse Inline Suggestion (AI Feature)

**Behavior:**
- **Trigger:** Pause typing for 2-3 seconds
- **Position:** Floating card below current paragraph
- **Content:** AI-generated writing prompt or question
- **Accept:** Press `Tab` key to accept and insert text
- **Dismiss:** Press `Esc` or continue typing (suggestion fades)
- **Frequency:** Max 1 suggestion per 5 minutes (not intrusive)

**Visual Style:**
- Light background card (subtle shadow)
- 💭 Icon (thought bubble) indicates AI suggestion
- Soft entrance animation (fade + slide up)
- Respects reduced motion (fade only)

---

## 3. Canvas with Tags Added

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌────────────────────────────────────────────────────────────┐     │
│  │ [B] [I] [T] [🎨]                 📌 Pin  |  🗓️ Nov 15, 2025  │     │
│  ├────────────────────────────────────────────────────────────┤     │
│  │                                                            │     │
│  │  Reflecting on Travel                                     │     │
│  │  🏷️ travel  🏷️ reflection  🏷️ plans  [+ Add tag]         │     │
│  │                                                            │     │
│  │  Today I realized how much I miss exploring new          │     │
│  │  places. There's something about being in a foreign      │     │
│  │  city where everything feels fresh and unfamiliar...     │     │
│  │  ▌                                                         │     │
│  │                                                            │     │
│  └────────────────────────────────────────────────────────────┘     │
│                                                                       │
│  Last saved: just now  |  Word count: 97  |  XP: 250                │
└─────────────────────────────────────────────────────────────────────┘
```

### Tag Display

**Location:** Below title, above content

**Components:**
- **Tag pills:** Rounded rectangles with soft colors
  - `🏷️ travel` (blue), `🏷️ reflection` (green), `🏷️ plans` (purple)
- **[+ Add tag] button:** Opens tag picker modal
- **Remove tag:** Hover tag → [×] appears → Click to remove

**Interaction:**
- **Click tag:** Filters Library to show all entries with that tag
- **Drag to reorder:** Tags can be reordered (future feature)

---

## 4. Canvas with Mood Selector

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌────────────────────────────────────────────────────────────┐     │
│  │ [B] [I] [T] [🎨 Mood: ⭐ Hopeful]        📌  |  🗓️ Nov 15   │     │
│  ├────────────────────────────────────────────────────────────┤     │
│  │                                                            │     │
│  │  Reflecting on Travel                                     │     │
│  │  🏷️ travel  🏷️ reflection  🏷️ plans  [+ Add tag]         │     │
│  │                                                            │     │
│  │  Today I realized how much I miss exploring new          │     │
│  │  places...                                                │     │
│  │  ▌                                                         │     │
│  │                                                            │     │
│  └────────────────────────────────────────────────────────────┘     │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Mood Selector Dropdown

**Trigger:** Click [🎨] button in toolbar

```
┌───────────────────────┐
│ 😊 Happy              │
│ 😔 Sad                │
│ 😰 Anxious            │
│ 😌 Calm               │
│ ⭐ Hopeful       ✓    │  ← Selected
│ 😤 Frustrated         │
│ 🤔 Reflective         │
│ 💪 Motivated          │
│ 😐 Neutral            │
│ [Clear mood]          │
└───────────────────────┘
```

**Features:**
- **Pre-defined moods:** 9 common moods with emoji icons
- **Clear mood:** Removes mood from entry
- **Visual feedback:** Selected mood shows checkmark
- **Analytics:** Mood data used in advanced analytics (Pro feature)

---

## 5. Formatting Toolbar (Expanded)

### Basic Formatting
```
┌──────────────────────────────────────────────────────────────┐
│ [B] [I] [U] [S] [H1] [H2] [•] [1.] [🔗] [T] [🎨] [⛶]        │
└──────────────────────────────────────────────────────────────┘
```

**Buttons:**
- **B** — Bold (Ctrl+B)
- **I** — Italic (Ctrl+I)
- **U** — Underline (Ctrl+U)
- **S** — Strikethrough (Ctrl+Shift+X)
- **H1** — Heading 1 (Markdown: #)
- **H2** — Heading 2 (Markdown: ##)
- **•** — Bullet list
- **1.** — Numbered list
- **🔗** — Insert link (future)
- **T** — Add tag (Ctrl+T)
- **🎨** — Mood selector
- **⛶** — Fullscreen mode (F11)

**Markdown Support:**
- Users can type Markdown directly
- Live preview (bold shows as **bold**)
- Or use toolbar buttons (WYSIWYG-style)

---

## 6. Autosave Indicator States

### Saving...
```
┌─────────────────────────────────────────────────────────────┐
│  ⟳ Saving...  |  Word count: 97  |  XP: 250                 │
└─────────────────────────────────────────────────────────────┘
```

### Saved
```
┌─────────────────────────────────────────────────────────────┐
│  ✓ Last saved: just now  |  Word count: 97  |  XP: 250     │
└─────────────────────────────────────────────────────────────┘
```

### Save Failed
```
┌─────────────────────────────────────────────────────────────┐
│  ⚠️ Failed to save  [Retry]  |  Word count: 97  |  XP: 250  │
└─────────────────────────────────────────────────────────────┘
```

**Behavior:**
- **Auto-save trigger:** 3 seconds after last keystroke
- **Visual feedback:** Spinner (⟳) → Checkmark (✓)
- **Error handling:** Red warning icon → Retry button → Manual save option

---

## 7. Entry Navigation (Previous/Next)

```
┌─────────────────────────────────────────────────────────────────────┐
│  ┌────────────────────────────────────────────────────────────┐     │
│  │ [<] [>]  Entry 42 of 156          📌 Pin  |  🗓️ Nov 15     │     │
│  ├────────────────────────────────────────────────────────────┤     │
│  │                                                            │     │
│  │  Reflecting on Travel                                     │     │
│  │  🏷️ travel  🏷️ reflection                                 │     │
│  │                                                            │     │
│  │  Today I realized how much I miss exploring new...       │     │
│  │                                                            │     │
│  └────────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

**Navigation Buttons:**
- **[<] Previous** — Load previous entry (Ctrl+[)
- **[>] Next** — Load next entry (Ctrl+])
- **Counter:** "Entry 42 of 156" (shows position in chronological order)

**Keyboard Shortcuts:**
- `Ctrl+[` — Previous entry
- `Ctrl+]` — Next entry
- Arrows disabled (so user can navigate text with arrows)

---

## 8. Empty Canvas (No Entry Selected)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│                                                                       │
│                         📝                                           │
│                   No entry selected                                  │
│                                                                       │
│           Press Ctrl+N to create a new entry, or                     │
│            open an existing entry from the Library.                  │
│                                                                       │
│                      [+ New Entry]                                   │
│                                                                       │
│                                                                       │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Empty State:**
- **Icon:** Large 📝 (journal icon)
- **Text:** Clear instructions
- **Action button:** [+ New Entry] (creates blank entry)

---

## 9. Archivist Auto-Tagging Notification (AI Feature)

```
┌─────────────────────────────────────────────────────────────────────┐
│  ┌────────────────────────────────────────────────────────────┐     │
│  │                                                            │     │
│  │  Reflecting on Travel                                     │     │
│  │  🏷️ travel  🏷️ reflection  [+ Add tag]                    │     │
│  │                                                            │     │
│  │  ┌──────────────────────────────────────────────────┐    │     │
│  │  │ 🤖 Archivist suggests tags:                       │    │     │
│  │  │ • wanderlust  [Add]                               │    │     │
│  │  │ • memories  [Add]                                 │    │     │
│  │  │ [Dismiss]                                         │    │     │
│  │  └──────────────────────────────────────────────────┘    │     │
│  │                                                            │     │
│  │  Today I realized how much I miss exploring new...       │     │
│  │                                                            │     │
│  └────────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

**Archivist Behavior:**
- **Trigger:** After entry saved (runs in background)
- **Notification:** Appears below tags section
- **Suggestions:** 2-3 AI-suggested tags
- **Actions:**
  - [Add] — Adds tag to entry
  - [Dismiss] — Hides suggestion (no penalty)
- **Non-intrusive:** Doesn't interrupt writing

---

## 10. Word Count Milestone Celebration

```
┌─────────────────────────────────────────────────────────────────────┐
│  Last saved: just now  |  Word count: 1000 🎉  |  XP: 260           │
└─────────────────────────────────────────────────────────────────────┘
```

**Milestones:**
- **100 words:** 🎉 (party popper)
- **500 words:** ⭐ (star)
- **1000 words:** 🎉 (party popper)
- **Subsequent 1000s:** No visual (avoids spam)

**Celebration:**
- Icon appears next to word count
- Subtle animation (pulse once)
- No toast or sound (non-intrusive)

---

## 11. Dark Mode / Theme Variations

### Light Mode (Default)
- **Background:** Off-white (#FAFAFA)
- **Text:** Dark gray (#333333)
- **Accent:** Soft blue (#4A90E2)

### Dark Mode
- **Background:** Dark gray (#1E1E1E)
- **Text:** Light gray (#E0E0E0)
- **Accent:** Muted blue (#6BA3E0)

### Low-Stimulation Mode (Neuro-Affirming)
- **Background:** Warm beige (#F5F1E8)
- **Text:** Charcoal (#3C3C3C)
- **Accent:** Earthy green (#6B8E6B)
- **No animations:** Static UI, fade-only transitions

---

## 12. Canvas Sidebar (Optional Future Feature)

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  ┌──────────────────────┐  ┌────────────────────────────────┐       │
│  │ Related Entries:     │  │ Reflecting on Travel           │       │
│  │ ──────────────────── │  │ 🏷️ travel  🏷️ reflection       │       │
│  │ • Trip to Paris      │  │                                │       │
│  │   (2 months ago)     │  │ Today I realized...           │       │
│  │                      │  │                                │       │
│  │ • Wanderlust Dreams  │  │                                │       │
│  │   (6 months ago)     │  └────────────────────────────────┘       │
│  │                      │                                            │
│  │ [See all travel]     │                                            │
│  └──────────────────────┘                                            │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Sidebar Features:**
- **Related entries:** AI-powered semantic similarity
- **Quick navigation:** Click to open related entry
- **Collapsible:** Hide sidebar for distraction-free writing
- **Pro feature only** (requires semantic search)

---

## Accessibility Notes

### Keyboard Navigation
- **Tab order:** Title → Tags → Content → Formatting toolbar
- **Shift+Tab:** Reverse tab order
- **Ctrl+Home:** Jump to title
- **Ctrl+End:** Jump to end of content

### Screen Reader
- **Entry title:** Announced as "Heading level 1"
- **Tags:** Announced as "Button, travel tag"
- **Muse suggestions:** Announced as "AI suggestion: [text]"
- **Autosave:** Status announced ("Saving..." → "Saved")

### Reduced Motion
- **Muse entrance:** Fade only (no slide animation)
- **Tag pills:** No hover animations
- **Word count milestone:** No pulse animation

---

## Performance Targets

- **Initial render:** <100ms (blank entry)
- **Keystroke latency:** <16ms (60fps)
- **Autosave:** <3s after last keystroke
- **Muse suggestion:** <500ms (AI inference)

---

## Developer Notes

### Tech Stack
- **Editor:** Prosemirror or CodeMirror (Markdown support)
- **Autosave:** Debounced save function (3s delay)
- **AI integration:** Rust backend calls llama.cpp, sends suggestion to React

### State Management
- **Current entry ID:** Stored in React context
- **Unsaved changes:** Track dirty state (warn on navigation if unsaved)
- **Draft recovery:** Auto-save to temp file every 10s (crash recovery)

---

**End of Journal Canvas Wireframe**
