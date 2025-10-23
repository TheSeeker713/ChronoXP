# Wireframe: Library Surface

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

The Library is where users browse, filter, and manage all journal entries. It offers three viewing modes: Calendar, Timeline, and Tags.

---

## 1. Library Layout (Calendar View - Default)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Library                       [Calendar] [Timeline] [Tags]          │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ┌────────────────────────────────┐  ┌──────────────────────────┐  │
│  │   November 2025      [<] [>]   │  │ Nov 15, 2025             │  │
│  │ ─────────────────────────────  │  │ ──────────────────────   │  │
│  │ Su Mo Tu We Th Fr Sa           │  │                          │  │
│  │                 1  2            │  │ Reflecting on Travel     │  │
│  │  3  4  5  6  7  8  9            │  │ 🏷️ travel 🏷️ reflection  │  │
│  │ 10 11 12 13 14 [15] 16          │  │                          │  │
│  │ 17 18 19 20 21 22 23            │  │ Today I realized how     │  │
│  │ 24 25 26 27 28 29 30            │  │ much I miss exploring... │  │
│  │                                 │  │                          │  │
│  │ ● Days with entries             │  │ 347 words  |  ⭐ Hopeful │  │
│  │ ○ Days without entries          │  │                          │  │
│  │                                 │  │ [Open Entry]             │  │
│  └────────────────────────────────┘  └──────────────────────────┘  │
│                                                                       │
│  156 entries total  |  Last entry: Today at 2:34 PM                 │
└─────────────────────────────────────────────────────────────────────┘
```

### Left Panel: Calendar

**Components:**
- **Month/Year header:** "November 2025" with [<] [>] navigation buttons
- **Calendar grid:** Standard 7-column layout (Su-Sa)
- **Date indicators:**
  - **● (filled circle):** Days with entries (clickable)
  - **○ (empty circle):** Days without entries (not clickable)
  - **[15] (box):** Today's date (bold)
  - **Multiple entries:** Larger circle or number badge (e.g., "3")

**Interaction:**
- **Click date:** Shows entries from that day in right preview panel
- **Arrow keys:** Navigate calendar dates
- **Enter:** Open selected date's entry
- **Keyboard:** Type "Nov 2024" to jump to specific month

**Legend:**
- Bottom of calendar explains indicators

### Right Panel: Entry Preview

**Components:**
- **Date header:** "Nov 15, 2025"
- **Entry title:** "Reflecting on Travel"
- **Tags:** Display first 3 tags (🏷️ travel, 🏷️ reflection)
- **Excerpt:** First 100 characters of entry
- **Metadata:**
  - Word count (347 words)
  - Mood (⭐ Hopeful)
- **[Open Entry] button:** Opens entry on Canvas

**Multiple entries on same day:**
```
┌──────────────────────────┐
│ Nov 15, 2025 (2 entries) │
│ ──────────────────────   │
│                          │
│ 1. Reflecting on Travel  │
│    🏷️ travel             │
│    Today I realized...   │
│    [Open]                │
│                          │
│ 2. Evening Thoughts      │
│    🏷️ reflection         │
│    As the day winds...   │
│    [Open]                │
└──────────────────────────┘
```

---

## 2. Library Layout (Timeline View)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Library                       [Calendar] [Timeline] [Tags]          │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Today                                                         │  │
│  │ ─────────────────────────────────────────────────────────    │  │
│  │ ┌──────────────────────────────────────────────────────┐    │  │
│  │ │ 2:34 PM  •  Reflecting on Travel                     │    │  │
│  │ │ 🏷️ travel 🏷️ reflection  •  347 words  •  ⭐ Hopeful │    │  │
│  │ │ Today I realized how much I miss exploring new...   │    │  │
│  │ └──────────────────────────────────────────────────────┘    │  │
│  │                                                               │  │
│  │ Yesterday                                                     │  │
│  │ ─────────────────────────────────────────────────────────    │  │
│  │ ┌──────────────────────────────────────────────────────┐    │  │
│  │ │ 8:12 AM  •  Morning Reflection                       │    │  │
│  │ │ 🏷️ morning 🏷️ gratitude  •  201 words  •  😌 Calm   │    │  │
│  │ │ Woke up feeling peaceful. Made coffee and...        │    │  │
│  │ └──────────────────────────────────────────────────────┘    │  │
│  │                                                               │  │
│  │ 3 days ago                                                    │  │
│  │ ─────────────────────────────────────────────────────────    │  │
│  │ ┌──────────────────────────────────────────────────────┐    │  │
│  │ │ 6:45 PM  •  Work Frustrations                        │    │  │
│  │ │ 🏷️ work 🏷️ stress  •  489 words  •  😤 Frustrated   │    │  │
│  │ │ Today's meeting was incredibly draining. I felt...  │    │  │
│  │ └──────────────────────────────────────────────────────┘    │  │
│  │                                                               │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
│  [Load More] (Scroll or click to load older entries)                │
└─────────────────────────────────────────────────────────────────────┘
```

### Timeline View Features

**Date grouping:**
- **Today** — Entries from today
- **Yesterday** — Entries from yesterday
- **3 days ago**, **Last week**, **Last month** — Grouped by recency

**Entry Cards:**
- **Time stamp:** Hour:minute (e.g., "2:34 PM")
- **Title:** Entry title (clickable)
- **Tags:** First 3 tags displayed
- **Metadata:** Word count + mood
- **Excerpt:** First 80 characters
- **Click anywhere:** Opens entry on Canvas

**Infinite scroll:**
- Load 20 entries initially
- Auto-load more as user scrolls down
- [Load More] button at bottom (if scroll not triggered)

**Keyboard navigation:**
- **Arrow down/up:** Navigate between entry cards
- **Enter:** Open selected entry
- **Page Down/Up:** Jump 5 entries at a time

---

## 3. Library Layout (Tags View)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Library                       [Calendar] [Timeline] [Tags]          │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ┌───────────────────────┐  ┌────────────────────────────────────┐ │
│  │ All Tags (24)         │  │ Entries tagged "travel" (12)       │ │
│  │ ──────────────────    │  │ ────────────────────────────────   │ │
│  │                       │  │                                    │ │
│  │ 🏷️ travel (12)    ✓  │  │ ┌────────────────────────────────┐ │ │
│  │ 🏷️ work (34)         │  │ │ Nov 15  •  Reflecting on Travel│ │ │
│  │ 🏷️ reflection (28)   │  │ │ 347 words  •  ⭐ Hopeful       │ │ │
│  │ 🏷️ gratitude (18)    │  │ │ Today I realized how much I... │ │ │
│  │ 🏷️ stress (15)       │  │ └────────────────────────────────┘ │ │
│  │ 🏷️ family (22)       │  │                                    │ │
│  │ 🏷️ health (9)        │  │ ┌────────────────────────────────┐ │ │
│  │ 🏷️ goals (14)        │  │ │ Oct 3  •  Wanderlust Dreams    │ │ │
│  │ 🏷️ creative (11)     │  │ │ 521 words  •  ⭐ Hopeful       │ │ │
│  │ 🏷️ plans (8)         │  │ │ Looking at photos from my...   │ │ │
│  │                       │  │ └────────────────────────────────┘ │ │
│  │ [+ Create New Tag]    │  │                                    │ │
│  │                       │  │ (Showing 12 entries)               │ │
│  └───────────────────────┘  └────────────────────────────────────┘ │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Left Panel: Tag List

**Components:**
- **Tag list:** All tags sorted by frequency (most used first)
- **Tag format:** `🏷️ travel (12)` — Name + count
- **Selected tag:** Checkmark (✓) + highlighted background
- **[+ Create New Tag] button:** Opens tag creation modal

**Interaction:**
- **Click tag:** Filter right panel to show entries with that tag
- **Multiple selection:** Ctrl+Click to select multiple tags (AND filter)
- **Right-click tag:** Context menu (Rename, Change Color, Delete)

**Sorting options:**
```
┌───────────────────────┐
│ Sort by:              │
│ ○ Frequency (12, 9...)│  ← Default
│ ○ Alphabetical (A-Z)  │
│ ○ Recent use          │
│ ○ Color               │
└───────────────────────┘
```

### Right Panel: Filtered Entries

**Components:**
- **Header:** "Entries tagged 'travel' (12)"
- **Entry cards:** Similar to Timeline view
  - Date, title, word count, mood, excerpt
- **Empty state:** If tag has no entries: "No entries with this tag yet."

---

## 4. Filter & Sort Bar

```
┌─────────────────────────────────────────────────────────────────────┐
│  Library                       [Calendar] [Timeline] [Tags]          │
│  ─────────────────────────────────────────────────────────────────  │
│  🔽 Filter  |  🔽 Sort: Newest First  |  [×] travel  [×] work        │
│  ─────────────────────────────────────────────────────────────────  │
└─────────────────────────────────────────────────────────────────────┘
```

### Filter Dropdown

```
┌─────────────────────────┐
│ Filter by:              │
│ ──────────────────────  │
│ ☐ Has mood              │
│ ☐ Has tags              │
│ ☐ Pinned only           │
│ ☐ 100+ words            │
│ ☐ 500+ words            │
│ ☐ 1000+ words           │
│ ──────────────────────  │
│ Date range:             │
│ [Past week ▼]           │
│ ──────────────────────  │
│ [Apply] [Clear]         │
└─────────────────────────┘
```

**Features:**
- **Checkboxes:** Multi-select filters (e.g., "Has mood" + "100+ words")
- **Date range:** Pre-defined (Past week, month, year) or custom range
- **Apply button:** Filters entries in real-time
- **Clear button:** Resets all filters

### Sort Dropdown

```
┌────────────────────────┐
│ ● Newest first         │  ← Default
│ ○ Oldest first         │
│ ○ Most words           │
│ ○ Least words          │
│ ○ A-Z (title)          │
│ ○ Z-A (title)          │
└────────────────────────┘
```

### Active Filters (Pills)

**Display:**
- `[×] travel` — Clickable pill to remove filter
- Multiple filters shown as pills
- **Clear All:** Link to remove all filters at once

---

## 5. Entry Context Menu (Right-Click)

### Right-Click on Entry Card

```
┌──────────────────────┐
│ Open in Canvas       │
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

**Actions:**
- **Open in Canvas:** Loads entry on main writing surface
- **Open in New Window:** Opens entry in separate window (future feature)
- **Edit Tags:** Opens tag picker modal
- **Pin Entry:** Pins to top of Library (shows pin icon 📌)
- **Export This Entry:** Exports single entry as Markdown or .chrxp
- **Duplicate Entry:** Creates copy with "(Copy)" suffix
- **Delete Entry:** Shows confirmation dialog

---

## 6. Pinned Entries Section

```
┌─────────────────────────────────────────────────────────────────────┐
│  Library                       [Calendar] [Timeline] [Tags]          │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  📌 Pinned Entries (2)                                               │
│  ─────────────────────────────────────────────────────────────────  │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Nov 10  •  Goals for 2026  •  📌                             │  │
│  │ 🏷️ goals 🏷️ planning  •  689 words  •  💪 Motivated         │  │
│  │ Next year I want to focus on three main areas...            │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Oct 15  •  Important Reminder  •  📌                         │  │
│  │ 🏷️ reminders  •  134 words  •  😌 Calm                      │  │
│  │ I need to remember to reach out to...                       │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
│  All Entries (154)                                                   │
│  ─────────────────────────────────────────────────────────────────  │
│  (Timeline continues below...)                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Features:**
- **Separate section:** Pinned entries always show at top
- **Pin icon:** 📌 visible on entry card
- **Unpin:** Right-click → "Unpin Entry" or click pin icon
- **Max pins:** No limit (but suggest 5-10 max for usability)

---

## 7. Bulk Selection Mode

```
┌─────────────────────────────────────────────────────────────────────┐
│  Library                [Select] [Cancel]                            │
│  ─────────────────────────────────────────────────────────────────  │
│  ☑ Select All (3 selected)  |  [Tag] [Export] [Delete]              │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ☑ Nov 15  •  Reflecting on Travel                                  │
│  ☐ Nov 14  •  Morning Reflection                                    │
│  ☑ Nov 12  •  Work Frustrations                                     │
│  ☐ Nov 10  •  Goals for 2026                                        │
│  ☑ Nov 8   •  Weekend Plans                                         │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Trigger:**
- **Button:** Top-right [Select] button
- **Keyboard:** `Ctrl+A` (select all entries on current view)

**Features:**
- **Checkboxes:** Appear on each entry card
- **Select All:** Checkbox at top selects/deselects all
- **Bulk actions:**
  - **[Tag]:** Add tag to all selected entries
  - **[Export]:** Export selected entries as .chrxp
  - **[Delete]:** Delete selected entries (with confirmation)
- **Cancel:** Exits selection mode

---

## 8. Empty Library State

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│                                                                       │
│                         📚                                           │
│                    Your Library is empty                             │
│                                                                       │
│         Start journaling to see your entries appear here.            │
│                                                                       │
│                      [+ New Entry]                                   │
│                                                                       │
│                                                                       │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Components:**
- **Icon:** 📚 (large book icon)
- **Message:** Encouraging, non-judgmental
- **Action button:** [+ New Entry] → Opens blank Canvas

---

## 9. Search within Library (Quick Filter)

```
┌─────────────────────────────────────────────────────────────────────┐
│  Library                       [Calendar] [Timeline] [Tags]          │
│  ─────────────────────────────────────────────────────────────────  │
│  🔍 Search entries...                                           [×]  │
│  ─────────────────────────────────────────────────────────────────  │
│  Showing 3 results for "travel"                                      │
│  ─────────────────────────────────────────────────────────────────  │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Nov 15  •  Reflecting on Travel                             │  │
│  │ ...how much I miss exploring new places. There's something  │  │
│  │    about being in a foreign city where everything feels...  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

**Features:**
- **Quick search bar:** Appears at top of Library
- **Real-time filtering:** Filters as you type (keyword search)
- **Highlights:** Matching text highlighted in results
- **Clear button:** [×] clears search and shows all entries
- **Semantic search:** Pro feature (shows "Upgrade to Pro for AI-powered semantic search" hint)

---

## Accessibility Notes

### Keyboard Navigation
- **Tab:** Navigate between view modes (Calendar, Timeline, Tags)
- **Arrow keys:** Navigate calendar dates or entry list
- **Enter:** Open selected entry
- **Ctrl+A:** Select all entries (bulk mode)
- **Delete:** Delete selected entry (with confirmation)

### Screen Reader
- **Calendar dates:** Announced as "November 15, 2 entries" or "November 14, no entries"
- **Entry cards:** Announced as "Entry: Reflecting on Travel, 347 words, mood: Hopeful, tags: travel, reflection"
- **Tags:** Announced as "Tag: travel, 12 entries"

### Color & Contrast
- **Calendar indicators:** Use shapes (circles) + color for redundancy
- **Tag colors:** Optional (tags readable without color)
- **Entry cards:** High contrast borders + text

---

## Performance Targets

- **Initial Library load:** <200ms (first 20 entries)
- **Calendar render:** <100ms
- **Search filter:** <50ms (real-time as user types)
- **Scroll performance:** 60fps (smooth infinite scroll)

---

## Developer Notes

### Tech Stack
- **Calendar:** React-based calendar library (e.g., `react-calendar`)
- **Virtualized list:** Use virtual scrolling for Timeline view (e.g., `react-window`) if 500+ entries
- **Tag management:** SQLite query for tag counts, indexed for speed

### Data Fetching
- **Pagination:** Load 20 entries at a time (Timeline view)
- **Calendar:** Pre-calculate which dates have entries (single SQL query)
- **Tags:** Cache tag list (refresh on entry save)

---

**End of Library Surface Wireframe**
