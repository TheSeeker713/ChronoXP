# Information Architecture

**Project:** ChronoXP™ Journal  
**Date:** October 22, 2025

---

## Overview

ChronoXP's information architecture is designed around **five top-level surfaces**, accessible via a **global Command Bar** and **full keyboard navigation**. The structure prioritizes **writing first**, with supporting features one keystroke away.

---

## Top-Level Surfaces

### 1. Journal Canvas
**Primary surface for writing and editing entries.**

#### Layout
```
┌─────────────────────────────────────────────────────┐
│  [<] [Today] [>]     [Templates ▼] [Tags] [⋮]      │ ← Toolbar (minimal)
├─────────────────────────────────────────────────────┤
│                                                     │
│  October 22, 2025 • 3:42 PM                        │ ← Auto-timestamp
│                                                     │
│  [Cursor here — start typing...]                   │ ← Focus on load
│                                                     │
│                                                     │
│                                                     │
│                                                     │
│                                                     │
│                                                     │
│                                                     │
│                                                     │
└─────────────────────────────────────────────────────┘
   Word count: 0 | Autosaved just now               ← Status bar
```

#### Features
- **Rich text editor:**
  - Headings (H1-H3)
  - Bold, italic, strikethrough
  - Bulleted/numbered lists
  - Inline images (drag-and-drop or paste)
  - Block quotes
  - Code blocks (for technical journalers)
- **Autosave:** Every 3 seconds after typing stops
- **Ghost placeholder:** "Want a soft start? Try a one-line check-in." (disappears on first keystroke)
- **Inline AI (Muse):** Greyed-out next-line suggestion (if enabled); `Tab` to accept
- **Templates dropdown:** Quick access to starter prompts (e.g., "Daily Check-In", "Gratitude", "Free Write")
- **Tags:** Non-modal chip input; autocomplete from existing tags
- **Navigation:**
  - `[<]` / `[>]` arrows: Previous/Next entry by date
  - `Ctrl/Cmd+N`: New entry
  - `Ctrl/Cmd+S`: Force save (though autosave is always on)

---

### 2. Library
**Browse, search, and organize past entries.**

#### Layout
```
┌─────────────────────────────────────────────────────┐
│  Library                           [Search 🔍]      │
├─────────────────────────────────────────────────────┤
│  [Calendar] [Tags] [All Entries]                   │ ← Tabs
├─────────────────────────────────────────────────────┤
│                                                     │
│  📅 Calendar View                                   │
│  ┌───────────────────────────────┐                 │
│  │  Oct 2025                     │                 │
│  │  S  M  T  W  T  F  S          │                 │
│  │           1  2  3  4          │                 │
│  │  5  6  7  •  •  • 11          │ ← Dots = entries
│  │  • 13 14 15 16 17 18          │
│  │ 19 20 21 ● 23 24 25           │ ← ● = today
│  └───────────────────────────────┘                 │
│                                                     │
│  Recent Entries:                                    │
│  • Oct 22 — "Thoughts on..." (tags: work, ideas)   │
│  • Oct 21 — "Morning pages" (tags: morning)        │
│  • Oct 18 — "Reflection on..." (tags: personal)    │
└─────────────────────────────────────────────────────┘
```

#### Features
- **Calendar view:**
  - Visual heatmap (subtle dots, not guilt-inducing streaks)
  - Click a date to open that day's entry
- **Tags view:**
  - Cloud or list of all tags
  - Click to filter entries by tag
- **All Entries:**
  - Chronological list (newest first)
  - Search bar: keyword or semantic query
  - Preview: title + first 2 lines + tags
- **Search:**
  - Hybrid: keyword + semantic (vector similarity)
  - Results ranked by relevance
  - Highlight matching terms

---

### 3. Records Cavern
**Visual progress space with unlockable artifacts.**

#### Layout (MVP — Simple 2.5D Scene)
```
┌─────────────────────────────────────────────────────┐
│  Records Cavern                     [Hide] [XP: 230]│
├─────────────────────────────────────────────────────┤
│                                                     │
│         🏔️                                          │
│       🪴  📚  🕯️                                     │ ← Artifacts
│      ┌─────────┐                                    │
│      │  Desk   │                                    │
│      └─────────┘                                    │
│                                                     │
│  Progress to next unlock: ████████░░ 80%            │
│  Next: "Memory Jar" (50 XP away)                    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

#### Features
- **2.5D isometric room:**
  - Parallax scrolling (respects reduced motion)
  - Warm, cozy lighting
- **Artifacts:**
  - Unlocked as XP thresholds are met
  - Click to see flavor text (e.g., "A well-worn journal from a distant traveler")
  - No gameplay impact — purely aesthetic
- **Progress bar:**
  - Shows XP toward next unlock
  - No countdown timers or deadlines
- **Toggle:**
  - `[Hide]` button collapses Cavern (replaces with "Show Cavern" in toolbar)
  - "Simple Mode" in Settings disables entirely

---

### 4. Search (Global)
**Accessible via Command Bar or dedicated shortcut.**

#### Layout (Modal Overlay)
```
┌─────────────────────────────────────────────────────┐
│  Search                                       [Esc] │
├─────────────────────────────────────────────────────┤
│  🔍 [find entries about family and work______]      │
├─────────────────────────────────────────────────────┤
│  Results (semantic):                                │
│  1. Oct 15, 2025 — "Work-life balance..."          │
│     ...talked to Mom about family expectations...   │
│     Tags: family, work, stress                      │
│                                                     │
│  2. Sep 30, 2025 — "Dinner with family"            │
│     ...discussed career plans; Dad gave advice...   │
│     Tags: family, career                            │
│                                                     │
│  3. Oct 8, 2025 — "Burnout thoughts"               │
│     ...work is overwhelming; need family time...    │
│     Tags: work, burnout                             │
└─────────────────────────────────────────────────────┘
```

#### Features
- **Natural language queries:**
  - Semantic search via local embeddings
  - Fallback to keyword if vector search fails
- **Ranked results:**
  - Relevance score (hidden, but used for ordering)
  - Preview: date, title, snippet, tags
- **Navigation:**
  - `Enter` on result opens entry in Canvas
  - `Esc` closes search
  - `Ctrl/Cmd+F`: Open search from anywhere

---

### 5. Settings
**Preferences, AI configuration, privacy, and export.**

#### Layout (Sidebar Navigation)
```
┌────────────┬────────────────────────────────────────┐
│ Settings   │  Appearance                            │
│            │                                        │
│ Appearance │  Theme: [Light ▼] [Dark] [Custom]     │
│ Editor     │  Font: [System ▼] [OpenDyslexic] ...  │
│ AI         │  Font size: [16px ▼]                   │
│ Privacy    │  Line spacing: [1.5x ▼]                │
│ Keyboard   │                                        │
│ Export     │  ☑ Reduced motion                      │
│ About      │  ☑ Simple Mode (hide gamification)    │
│            │                                        │
│            │  [Save Changes]                        │
└────────────┴────────────────────────────────────────┘
```

#### Sections

##### Appearance
- Theme (Light, Dark, High Contrast)
- Font family and size
- Line spacing
- Reduced motion toggle
- Simple Mode toggle

##### Editor
- Default template on new entry
- Autosave interval (default: 3 seconds)
- Word count display

##### AI
- Enable/disable Muse (inline suggestions)
- Enable/disable Archivist (auto-tagging)
- Model selection (if multiple GGUF models downloaded)
- Model download manager

##### Privacy
- Enable/disable anonymous usage stats
- Vault password (optional second encryption layer)
- Clear search history

##### Keyboard
- View/customize keyboard shortcuts
- Enable/disable specific shortcuts

##### Export
- Export all data as `.chrxp`
- Import from `.chrxp`
- Export format options (encrypted vs. plain)

##### About
- App version
- Check for updates
- Credits and licenses
- Link to documentation

---

## Navigation Model

### Primary Navigation (Top-Level)
Users can switch between surfaces via:

1. **Toolbar tabs** (visible at top):
   ```
   [Journal] [Library] [Cavern] [Settings]
   ```

2. **Keyboard shortcuts:**
   - `Ctrl/Cmd+1`: Journal Canvas
   - `Ctrl/Cmd+2`: Library
   - `Ctrl/Cmd+3`: Records Cavern
   - `Ctrl/Cmd+,`: Settings
   - `Ctrl/Cmd+K`: Command Bar (search + commands)
   - `Ctrl/Cmd+F`: Global search

3. **Command Bar** (⌘/Ctrl+K):
   - Type natural language or command:
     - "new entry" → Creates new entry in Canvas
     - "search for X" → Opens search with query
     - "show cavern" → Switches to Cavern
     - "settings" → Opens Settings

### Secondary Navigation (Within Surfaces)
- **Journal Canvas:** `[<]` / `[>]` arrows, date picker
- **Library:** Tabs (Calendar, Tags, All Entries), search bar
- **Cavern:** Click artifacts for details
- **Settings:** Sidebar navigation

---

## Global Elements (Always Accessible)

### 1. Command Bar (⌘/Ctrl+K)
- **Appears as modal overlay**
- **Functions:**
  - Natural language search
  - Quick navigation ("go to settings", "open cavern")
  - Commands ("export data", "new entry", "change theme")
  - Concierge queries ("summarize this week")

### 2. Status Bar (Bottom of Window)
- Word count (if in Canvas)
- Autosave status ("Autosaved 2s ago")
- Sync status (future: if LAN sync enabled)
- Current date/time

### 3. Notifications (Non-Intrusive)
- **Toast messages** (bottom-right):
  - "Entry saved"
  - "Artifact unlocked!"
  - "Export complete"
- **Dismissable** with `Esc` or click

---

## Information Flow Examples

### Flow 1: Start Writing
```
1. Launch app → Cursor auto-focused in Canvas
2. Ghost placeholder: "Want a soft start?..."
3. User types → Placeholder disappears
4. Autosave after 3s → Status bar: "Autosaved just now"
5. User adds tag → Non-modal chip input
6. Background: Archivist extracts mood/topics
```

### Flow 2: Find Past Entry
```
1. User presses Ctrl/Cmd+K → Command Bar opens
2. User types: "find entries about work stress"
3. Semantic search runs → Results appear in <500ms
4. User presses Enter on result → Opens entry in Canvas
```

### Flow 3: Unlock Artifact
```
1. User writes entry → XP awarded (background)
2. XP reaches threshold → Toast: "Artifact unlocked!"
3. User clicks toast → Switches to Cavern
4. New artifact (e.g., "Ancient Scroll") appears
5. User clicks artifact → Flavor text modal
```

### Flow 4: Export Data
```
1. User: Ctrl/Cmd+, → Opens Settings
2. Click "Export" in sidebar
3. Click "Export as .chrxp"
4. Choose encryption (optional password)
5. Save dialog → Select location
6. Toast: "Export complete: journal_2025-10-22.chrxp"
```

---

## Mobile / Responsive Considerations (Future)
ChronoXP is initially **desktop-only** (Windows, macOS, Linux via Tauri). However, IA is designed to adapt:

- **Mobile:**
  - Bottom tab bar (Journal, Library, Cavern, Settings)
  - Swipe gestures (left/right for prev/next entry)
  - Simplified toolbar (fewer buttons)
- **Tablet:**
  - Split view (Library + Canvas side-by-side)
  - External keyboard support

---

## IA Design Principles

1. **Writing is always one click/keystroke away**
2. **No more than 3 levels deep** (e.g., Settings → Privacy → Vault Password)
3. **Every surface accessible via keyboard**
4. **Search is global, not buried**
5. **Visual hierarchy: Content > Chrome**

---

## IA Validation Checklist

Before MVP launch:
- [ ] Can navigate entire app with keyboard only?
- [ ] Can create, save, search, and export without mouse?
- [ ] Command Bar recognizes 20+ common queries?
- [ ] All surfaces load in <400ms?
- [ ] Zero dead-end states (every screen has "back" or "Esc")?
- [ ] Tooltips on all icons (for first-time users)?

