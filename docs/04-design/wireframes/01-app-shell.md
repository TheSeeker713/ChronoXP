# Wireframe: App Shell & Global Navigation

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

The app shell provides the foundational structure for all screens. It includes the window chrome, global navigation, command bar, and persistent UI elements.

---

## 1. Window Chrome (Desktop)

### Layout
```
┌─────────────────────────────────────────────────────────────────────┐
│ ChronoXP                           [_] [□] [×]  │ (Windows/Linux)   │
│ 🔵 🟡 🔴                         ChronoXP        │ (macOS)           │
└─────────────────────────────────────────────────────────────────────┘
```

**Components:**
- **App title:** "ChronoXP" centered or left-aligned (based on OS)
- **Window controls:** Minimize, maximize/restore, close (native OS style)
- **Drag region:** Top bar is draggable to move window
- **Traffic lights (macOS):** Red/yellow/green buttons (left side)

**States:**
- **Focused:** Full opacity, native colors
- **Unfocused:** Reduced opacity (50%), grayed out
- **Fullscreen:** Hide window chrome entirely (immersive writing mode)

---

## 2. Global Navigation Bar

### Layout
```
┌─────────────────────────────────────────────────────────────────────┐
│  [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
└─────────────────────────────────────────────────────────────────────┘
```

**Position:** Top of app window, below title bar (or integrated if custom chrome)

**Components:**

#### Primary Navigation (Left)
- **📝 Canvas** — Main writing surface (default view)
  - Hotkey: `Ctrl/Cmd+1`
  - Active state: Underline or highlight
- **📚 Library** — Browse all entries
  - Hotkey: `Ctrl/Cmd+2`
- **🏛️ Cavern** — Records Cavern (gamification)
  - Hotkey: `Ctrl/Cmd+3`
  - Hidden if Simple Mode enabled
- **⚙️ Settings** — App settings
  - Hotkey: `Ctrl/Cmd+,`

#### Secondary Actions (Right)
- **🔍 Search icon** — Opens search modal
  - Hotkey: `Ctrl/Cmd+F`
- **Ctrl+K badge** — Visual reminder for command bar
  - Clicking opens command bar
  - Subtle pulsing animation on first launch (onboarding hint)

**Visual Style:**
- **Minimalist:** Icons + text labels (or icons only if space constrained)
- **Low-stimulation colors:** Soft grays, blues, no bright highlights
- **Active indicator:** Subtle underline or background fill (not bold/flashy)
- **Hover state:** Gentle color shift + cursor change

---

## 3. Command Bar (Ctrl/Cmd+K)

### Overlay Appearance
```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 🔍 Search for entries or run a command...              │     │
│     └─────────────────────────────────────────────────────────┘     │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 📝 New Entry                                     Ctrl+N │     │
│     │ 🔍 Search Entries                                Ctrl+F │     │
│     │ 📚 Go to Library                                 Ctrl+2 │     │
│     │ 🏛️ Open Records Cavern                          Ctrl+3 │     │
│     │ 🏷️  Add Tag to Current Entry                    Ctrl+T │     │
│     │ 📤 Export Entries                                       │     │
│     │ ⚙️  Open Settings                                Ctrl+, │     │
│     └─────────────────────────────────────────────────────────┘     │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Trigger:** `Ctrl/Cmd+K` (global hotkey)

**Layout:**
- **Centered overlay:** Darkened backdrop (80% opacity black)
- **Search input:** Large, prominent text field
  - Placeholder: "Search for entries or run a command..."
  - Focus: Automatically focused on open
- **Results dropdown:** Appears below input
  - Max height: 400px (scrollable if more results)
  - Keyboard navigation: Arrow keys + Enter

**Command Categories:**

#### 1. Navigation Commands
- `New Entry` → Creates blank entry on Canvas
- `Go to Library` → Opens Library surface
- `Open Records Cavern` → Opens Cavern (hidden if Simple Mode)
- `Open Settings` → Opens Settings panel

#### 2. Action Commands
- `Add Tag to Current Entry` → Opens tag picker modal
- `Export Entries` → Opens export dialog
- `Import .chrxp File` → Opens file picker
- `Toggle Simple Mode` → Instantly hides/shows gamification

#### 3. Search Commands (Fuzzy Match)
- User types "travel" → Shows entries with "travel" in title/content
- User types "last week" → Shows entries from past 7 days
- User types "tagged work" → Shows entries tagged "work"

#### 4. AI Commands (Pro Feature)
- `Ask Concierge: [natural language query]` → AI searches entries
- `Generate Muse Prompt` → AI suggests writing prompt
- `Suggest Tags` → AI suggests tags for current entry

**Keyboard Behavior:**
- **Arrow keys:** Navigate results
- **Enter:** Execute selected command
- **Esc:** Close command bar
- **Tab:** Cycle through command categories (if implemented)

**States:**
- **Empty state:** Show all commands (no search query)
- **Searching:** Filter commands + entries in real-time
- **No results:** "No matching commands or entries found."

---

## 4. Keyboard Shortcuts Overlay (?)

### Trigger
- **Hotkey:** `Ctrl/Cmd+?` or `F1`
- **Location:** Settings → Keyboard (link to open overlay)

### Layout
```
┌──────────────────────────────────────────────────────────────────────┐
│  Keyboard Shortcuts                                            [×]   │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Global                                                               │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│  Ctrl+K              Command Bar                                      │
│  Ctrl+N              New Entry                                        │
│  Ctrl+F              Search Entries                                   │
│  Ctrl+S              Save Entry (manual save)                         │
│  Esc                 Close Modal / Cancel                             │
│                                                                       │
│  Canvas (Writing)                                                     │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│  Ctrl+B              Bold                                             │
│  Ctrl+I              Italic                                           │
│  Ctrl+T              Add Tag                                          │
│  Tab                 Accept Muse Suggestion (AI)                      │
│  Ctrl+[              Previous Entry                                   │
│  Ctrl+]              Next Entry                                       │
│                                                                       │
│  Library                                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│  Arrow Keys          Navigate Calendar / List                        │
│  Enter               Open Selected Entry                              │
│  Delete              Delete Entry (with confirmation)                 │
│                                                                       │
│  [Print] [Copy to Clipboard]                           [OK]          │
└──────────────────────────────────────────────────────────────────────┘
```

**Features:**
- **Searchable:** Type to filter shortcuts (e.g., "tag" highlights tag-related shortcuts)
- **Organized by context:** Global, Canvas, Library, Search, Cavern, Settings
- **Print button:** Generates printable PDF cheatsheet
- **Copy button:** Copies all shortcuts to clipboard (plain text)

---

## 5. Global Status Bar (Bottom)

### Layout
```
┌─────────────────────────────────────────────────────────────────────┐
│  Last saved: 2 minutes ago  |  Word count: 347  |  XP: 250  |  📶 Offline │
└─────────────────────────────────────────────────────────────────────┘
```

**Position:** Bottom of app window (always visible)

**Components (Left to Right):**

1. **Autosave indicator**
   - Text: "Last saved: [time] ago" (e.g., "2 minutes ago", "just now")
   - Icon: ✓ (checkmark) when saved, ⟳ (spinner) when saving
   - Error state: ⚠️ "Failed to save" (red text) with retry button

2. **Word count**
   - Current entry word count (live updates as user types)
   - Hidden if no entry open

3. **XP counter** (Pro feature, optional)
   - Text: "XP: [number]"
   - Hidden if Simple Mode enabled
   - Click to open Cavern

4. **Network status**
   - Icon: 📶 "Offline" (green) — Always shows offline (privacy feature)
   - Future: If LAN sync enabled, shows "Syncing..." or "Synced"

**Visual Style:**
- Small text, subtle colors (doesn't distract from writing)
- Separators: Thin vertical lines (|) between sections
- Hover: Slightly brighter colors, tooltip for more info

---

## 6. Notification Toasts

### Layout (Top-Right Corner)
```
┌─────────────────────────────────────────────────────────────────────┐
│                                                    ┌─────────────┐   │
│                                                    │ ✨ Artifact │   │
│                                                    │  unlocked!  │   │
│                                                    │   [View]    │   │
│                                                    └─────────────┘   │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Types of Toasts:**

#### 1. Artifact Unlock (Gamification)
- **Icon:** ✨ (sparkle)
- **Text:** "Artifact unlocked! [Artifact Name]"
- **Action button:** [View] → Opens Cavern with artifact highlighted
- **Auto-dismiss:** 5 seconds (or user clicks [×])
- **Sound:** Soft chime (optional, user-configurable)

#### 2. Quest Prompt (Reflection)
- **Icon:** 💭 (thought bubble)
- **Text:** "New reflection prompt: [Quest Name]"
- **Action buttons:** [View] [Dismiss] [Don't Show Again]
- **Auto-dismiss:** Never (user must interact)

#### 3. Welcome Back Bonus
- **Icon:** 👋 (waving hand)
- **Text:** "Welcome back! +5 XP for returning after 3 days."
- **No action button**
- **Auto-dismiss:** 3 seconds

#### 4. Error Notifications
- **Icon:** ⚠️ (warning)
- **Text:** "Failed to save entry. Retry?"
- **Action buttons:** [Retry] [Dismiss]
- **Auto-dismiss:** Never (critical error)

#### 5. Export Success
- **Icon:** ✓ (checkmark)
- **Text:** "Exported 42 entries to journal-backup.chrxp"
- **Action button:** [Open Folder]
- **Auto-dismiss:** 4 seconds

**Visual Style:**
- **Background:** Semi-transparent dark card (shadow + blur)
- **Animation:** Slide in from right, fade out on dismiss
- **Stacking:** Multiple toasts stack vertically (max 3 visible)
- **Respects reduced motion:** No slide animation, just fade in/out

---

## 7. Context Menu (Right-Click)

### Example: Right-Click on Entry (Library View)
```
┌──────────────────────┐
│ Open                 │
│ Open in New Window   │
│ ──────────────────── │
│ Edit Tags            │
│ Pin Entry            │
│ ──────────────────── │
│ Export This Entry    │
│ Duplicate Entry      │
│ ──────────────────── │
│ Delete Entry         │
└──────────────────────┘
```

**Context-Specific Menus:**

#### Canvas (Writing Area)
- Cut, Copy, Paste
- Bold, Italic (if text selected)
- Add Link (future feature)
- Add Tag

#### Library (Entry List)
- Open, Open in New Window
- Edit Tags, Pin/Unpin
- Export, Duplicate
- Delete

#### Cavern (Artifact)
- View Details
- Link to Related Entries
- (No delete option — artifacts permanent)

**Keyboard Navigation:**
- Arrow keys to navigate
- Enter to select
- Esc to close

---

## 8. Global Loading States

### Initial App Launch
```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│                          ░░░░░░░░░░░░░░░░                            │
│                         ChronoXP™ Journal                            │
│                                                                       │
│                          [Loading...]                                │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Components:**
- **Logo:** Hand-painted ChronoXP logo (centered)
- **Loading text:** "Loading..." or "Unlocking vault..."
- **No progress bar:** Simple, calm loading state
- **Duration:** <400ms (per performance target)

### Database Unlock (If Vault Password Set)
```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│                          🔐 Unlock Vault                             │
│                                                                       │
│              Enter your vault password to continue.                  │
│                                                                       │
│              ┌──────────────────────────────────────┐                │
│              │ ••••••••••••                          │                │
│              └──────────────────────────────────────┘                │
│                                                                       │
│                       [Unlock] [Exit App]                            │
│                                                                       │
│              Forgot password? See Settings → Security                │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Security:**
- Password field obscured (•••)
- No "show password" toggle (security best practice)
- Max 3 failed attempts → Show recovery instructions
- No biometric unlock in MVP (future feature)

---

## 9. Fullscreen / Zen Mode

### Trigger
- **Hotkey:** `F11` or `Ctrl/Cmd+Shift+F`
- **Button:** Canvas toolbar → [⛶ Fullscreen] icon

### Fullscreen Layout
```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│                                                                       │
│                                                                       │
│         Today I realized something important...                      │
│         ▌                                                            │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                                                                       │
│                          [Press Esc to exit fullscreen]              │
└─────────────────────────────────────────────────────────────────────┘
```

**Features:**
- **Minimal UI:** Only text editor visible (no navigation, status bar)
- **Centered text:** Content centered on screen (max width 800px)
- **Ambient background:** Soft gradient or subtle texture
- **Exit hint:** Fades in after 5 seconds of inactivity, fades out on typing
- **Autosave:** Still active in background

---

## 10. Accessibility: Focus Indicators

### Visual Style
```
┌──────────────────────────────────┐
│                                  │
│  ┌─────────────────────────┐    │  ← 3px blue outline
│  │██ Focused Button        │    │    2px offset
│  └─────────────────────────┘    │    High contrast
│                                  │
└──────────────────────────────────┘
```

**Requirements:**
- **Thickness:** 3px solid outline
- **Color:** High-contrast blue (#0066CC) or user-selected accent color
- **Offset:** 2px from element edge (prevents overlap)
- **Shape:** Rounded corners match element shape
- **Visibility:** Always visible (not just on Tab key press)

---

## 11. Responsive Breakpoints (Future)

While MVP is desktop-only, future mobile/tablet versions:

| Breakpoint | Width | Layout Changes |
|------------|-------|----------------|
| Desktop | >1024px | Full navigation bar, sidebars visible |
| Tablet | 768-1023px | Collapsible sidebars, icons-only navigation |
| Mobile | <767px | Bottom navigation bar, single-column layout |

---

## Accessibility Notes

### Screen Reader Support
- **Semantic HTML:** Use `<nav>`, `<main>`, `<aside>`, `<button>` (not `<div>` clickable)
- **ARIA labels:** All icons have `aria-label` (e.g., "Search entries")
- **Skip links:** "Skip to main content" link at top (invisible until focused)

### Keyboard Navigation
- **Tab order:** Logical flow (top-left → bottom-right)
- **Focus traps:** Modals trap focus (can't Tab out, must close)
- **Escape closes:** All overlays/modals close with Esc key

### Color & Contrast
- **WCAG AA compliance:** 4.5:1 contrast ratio for text
- **No color-only indicators:** Use icons + text, not just color

---

## Developer Notes

### Implementation Tech
- **Tauri window:** Custom chrome (hide native, draw own) or native chrome (simpler)
- **React Router:** Handle navigation between surfaces (Canvas, Library, etc.)
- **Command bar:** Fuzzy search library (e.g., `fuse.js`)
- **Toasts:** React notification library (e.g., `react-hot-toast`)

### Performance Targets
- **Command bar open:** <50ms (instant feel)
- **Navigation switch:** <100ms (smooth transition)
- **Toast animation:** 60fps (no jank)

---

**End of App Shell Wireframe**
