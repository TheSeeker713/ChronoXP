# Wireframe: Search Modal

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

The Search Modal provides keyword and semantic search capabilities. It opens as a full-screen overlay accessible from any surface.

---

## 1. Search Modal (Initial State)

```
┌─────────────────────────────────────────────────────────────────────┐
│                        [Background dimmed 80%]                        │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 🔍  Search your journal...                           [×]│     │
│     └─────────────────────────────────────────────────────────┘     │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │                                                         │     │
│     │  Recent searches:                                       │     │
│     │  • travel experiences                                   │     │
│     │  • work stress                                          │     │
│     │  • goals for 2026                                       │     │
│     │                                                         │     │
│     │  Quick filters:                                         │     │
│     │  [This week] [Last month] [Has mood] [Tagged]          │     │
│     │                                                         │     │
│     │  Tips:                                                  │     │
│     │  • Type keywords to search titles and content          │     │
│     │  • Use quotes for exact phrases: "mountain trip"       │     │
│     │  • Pro: Use semantic search to find by meaning         │     │
│     │                                                         │     │
│     └─────────────────────────────────────────────────────────┘     │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Trigger:**
- **Hotkey:** `Ctrl/Cmd+F`
- **Button:** 🔍 icon in navigation bar
- **Command Bar:** "Search Entries" command

**Components:**
- **Search input:** Large, auto-focused text field
- **[×] Close button:** Closes modal (or press Esc)
- **Recent searches:** Last 5 searches (clickable to re-run)
- **Quick filters:** One-click common filters
- **Tips section:** Search syntax hints

---

## 2. Search Results (Keyword Search)

```
┌─────────────────────────────────────────────────────────────────────┐
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 🔍  travel                                           [×]│     │
│     └─────────────────────────────────────────────────────────┘     │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 12 results for "travel"  |  [Keyword] [Semantic (Pro)]  │     │
│     │ ─────────────────────────────────────────────────────   │     │
│     │                                                         │     │
│     │ ┌─────────────────────────────────────────────────┐   │     │
│     │ │ Nov 15, 2025  •  Reflecting on Travel           │   │     │
│     │ │ 🏷️ travel 🏷️ reflection  •  347 words           │   │     │
│     │ │ ──────────────────────────────────────────────   │   │     │
│     │ │ Today I realized how much I miss exploring new  │   │     │
│     │ │ places. There's something about being in a      │   │     │
│     │ │ foreign city where everything feels fresh...    │   │     │
│     │ │ [Open]                                           │   │     │
│     │ └─────────────────────────────────────────────────┘   │     │
│     │                                                         │     │
│     │ ┌─────────────────────────────────────────────────┐   │     │
│     │ │ Oct 3, 2025  •  Wanderlust Dreams                │   │     │
│     │ │ 🏷️ travel 🏷️ dreams  •  521 words                │   │     │
│     │ │ ──────────────────────────────────────────────   │   │     │
│     │ │ Looking at photos from my trip to Japan last    │   │     │
│     │ │ year. The cherry blossoms, the temples, the...  │   │     │
│     │ │ [Open]                                           │   │     │
│     │ └─────────────────────────────────────────────────┘   │     │
│     │                                                         │     │
│     │ (Showing 2 of 12 results)                              │     │
│     └─────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

**Features:**
- **Result count:** "12 results for 'travel'"
- **Search mode toggle:** [Keyword] [Semantic (Pro)]
- **Result cards:**
  - Date, title, tags, word count
  - Excerpt with search term highlighted (bold or background color)
  - [Open] button to view full entry
- **Scrollable:** Results scroll if more than 5
- **Keyboard navigation:** Arrow keys to navigate results, Enter to open

---

## 3. Semantic Search (Pro Feature)

```
┌─────────────────────────────────────────────────────────────────────┐
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 🔍  times I felt lonely and needed connection       [×]│     │
│     └─────────────────────────────────────────────────────────┘     │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 8 results (by meaning)  |  [Keyword] [Semantic] ✓       │     │
│     │ ─────────────────────────────────────────────────────   │     │
│     │                                                         │     │
│     │ ┌─────────────────────────────────────────────────┐   │     │
│     │ │ Nov 2, 2025  •  Feeling Isolated                 │   │     │
│     │ │ 🏷️ loneliness 🏷️ social  •  423 words            │   │     │
│     │ │ ──────────────────────────────────────────────   │   │     │
│     │ │ It's been weeks since I had a real conversation │   │     │
│     │ │ with anyone. I miss the feeling of being        │   │     │
│     │ │ understood by someone who really gets me...     │   │     │
│     │ │ [Open]  •  Relevance: 95%                        │   │     │
│     │ └─────────────────────────────────────────────────┘   │     │
│     │                                                         │     │
│     │ ┌─────────────────────────────────────────────────┐   │     │
│     │ │ Sep 14, 2025  •  Missing Old Friends             │   │     │
│     │ │ 🏷️ friends 🏷️ nostalgia  •  298 words            │   │     │
│     │ │ ──────────────────────────────────────────────   │   │     │
│     │ │ Scrolling through photos from college days.     │   │     │
│     │ │ We used to be so close, but now we barely...   │   │     │
│     │ │ [Open]  •  Relevance: 87%                        │   │     │
│     │ └─────────────────────────────────────────────────┘   │     │
│     │                                                         │     │
│     └─────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

**Semantic Search Features:**
- **Natural language query:** User types concept/feeling, not exact keywords
- **Relevance score:** Shows % match (AI-calculated)
- **No exact match needed:** Finds entries about loneliness even if word "lonely" not used
- **Results sorted by relevance:** Most semantically similar first
- **[Semantic] toggle:** Active state indicator

**Locked state (Free tier):**
```
┌─────────────────────────────────────────────────────────────┐
│ 🔍  times I felt lonely                                  [×]│
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│ 🔒 Semantic Search is a Pro Feature                         │
│                                                             │
│ Upgrade to Pro to search by meaning, not just keywords.    │
│                                                             │
│ [Upgrade to Pro ($1.99)]  [Use Keyword Search Instead]     │
└─────────────────────────────────────────────────────────────┘
```

---

## 4. Search Filters (Advanced)

```
┌─────────────────────────────────────────────────────────────────────┐
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 🔍  work                                             [×]│     │
│     │ [🔽 Filters]                                            │     │
│     └─────────────────────────────────────────────────────────┘     │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ ┌─────────────────────┐                                 │     │
│     │ │ Filters:            │  24 results for "work"          │     │
│     │ │ ──────────────────  │  ────────────────────────────   │     │
│     │ │ Date range:         │                                 │     │
│     │ │ [Past year ▼]       │  ┌──────────────────────────┐  │     │
│     │ │                     │  │ Nov 12  •  Work Meeting  │  │     │
│     │ │ Tags:               │  │ 🏷️ work 🏷️ stress        │  │     │
│     │ │ ☑ work              │  │ Today's meeting was...   │  │     │
│     │ │ ☐ stress            │  │ [Open]                   │  │     │
│     │ │ ☐ goals             │  └──────────────────────────┘  │     │
│     │ │                     │                                 │     │
│     │ │ Mood:               │  ┌──────────────────────────┐  │     │
│     │ │ ☐ 😊 Happy          │  │ Nov 5  •  Work Goals     │  │     │
│     │ │ ☑ 😤 Frustrated     │  │ 🏷️ work 🏷️ goals        │  │     │
│     │ │ ☐ 😰 Anxious        │  │ Need to finish the...    │  │     │
│     │ │                     │  │ [Open]                   │  │     │
│     │ │ Word count:         │  └──────────────────────────┘  │     │
│     │ │ [100 - 1000]        │                                 │     │
│     │ │                     │  (Showing 2 of 24)              │     │
│     │ │ [Apply] [Clear]     │                                 │     │
│     │ └─────────────────────┘                                 │     │
│     └─────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

**Filter Options:**
- **Date range:** Pre-defined or custom calendar picker
- **Tags:** Multi-select checkboxes
- **Mood:** Select multiple moods
- **Word count:** Slider range (min-max)
- **[Apply]:** Applies filters
- **[Clear]:** Resets all filters

---

## 5. No Results State

```
┌─────────────────────────────────────────────────────────────────────┐
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 🔍  quantum physics                                  [×]│     │
│     └─────────────────────────────────────────────────────────┘     │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │                                                         │     │
│     │                     🔍                                  │     │
│     │           No results for "quantum physics"             │     │
│     │                                                         │     │
│     │  Suggestions:                                          │     │
│     │  • Check your spelling                                 │     │
│     │  • Try different keywords                              │     │
│     │  • Use semantic search to find by meaning (Pro)       │     │
│     │  • Browse all entries in the Library                  │     │
│     │                                                         │     │
│     │              [Try Semantic Search]  [Go to Library]    │     │
│     │                                                         │     │
│     └─────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 6. Search Shortcuts & Commands

### Command Syntax

**Date filters:**
- `today` — Entries from today
- `yesterday` — Entries from yesterday
- `this week` — Entries from past 7 days
- `this month` — Entries from current month

**Tag filters:**
- `tag:work` — Entries tagged "work"
- `tag:work tag:stress` — Entries with BOTH tags (AND)

**Mood filters:**
- `mood:happy` — Entries with "Happy" mood
- `mood:anxious OR mood:stressed` — Multiple moods (OR)

**Word count:**
- `words:100+` — Entries with 100+ words
- `words:500-1000` — Entries between 500-1000 words

**Combined:**
- `travel tag:adventure this year` — Keyword "travel" + tag "adventure" + from this year

### Example Queries

```
┌─────────────────────────────────────────────────────────┐
│ Query: tag:work this month words:200+                   │
│ Result: 8 entries tagged "work" from this month with    │
│         200+ words                                      │
└─────────────────────────────────────────────────────────┘
```

---

## 7. Search History

```
┌─────────────────────────────────────────────────────────────────────┐
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 🔍                                                   [×]│     │
│     └─────────────────────────────────────────────────────────┘     │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ Recent searches:                      [Clear History]   │     │
│     │ ─────────────────────────────────────────────────────   │     │
│     │                                                         │     │
│     │ • times I felt lonely and needed connection (Nov 15)   │     │
│     │ • work stress (Nov 14)                                 │     │
│     │ • travel experiences (Nov 12)                          │     │
│     │ • goals for 2026 (Nov 10)                              │     │
│     │ • family gatherings (Nov 5)                            │     │
│     │                                                         │     │
│     └─────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

**Features:**
- **Persistent:** Saved between sessions (SQLite table)
- **Clickable:** Click to re-run search
- **Date stamp:** Shows when search was performed
- **[Clear History]:** Deletes all search history

---

## 8. Inline Search Results Preview

```
┌─────────────────────────────────────────────────────────────────────┐
│     │ ┌─────────────────────────────────────────────────┐   │     │
│     │ │ Nov 15, 2025  •  Reflecting on Travel           │   │     │
│     │ │ 🏷️ travel 🏷️ reflection  •  347 words           │   │     │
│     │ │ ──────────────────────────────────────────────   │   │     │
│     │ │ Today I realized how much I miss exploring new  │   │     │
│     │ │ places. There's something about being in a      │   │     │
│     │ │ foreign city where everything feels fresh and   │   │     │
│     │ │ unfamiliar. The street signs in different...    │   │     │
│     │ │                                                  │   │     │
│     │ │ [Open Full Entry]  [Preview ▼]                  │   │     │
│     │ └─────────────────────────────────────────────────┘   │     │
│     │                                                         │     │
│     │ ┌─────────────────────────────────────────────────┐   │     │
│     │ │ ▼ Preview                               [Close] │   │     │
│     │ │ ──────────────────────────────────────────────   │   │     │
│     │ │ Today I realized how much I miss exploring new  │   │     │
│     │ │ places. There's something about being in a      │   │     │
│     │ │ foreign city where everything feels fresh and   │   │     │
│     │ │ unfamiliar. The street signs in different       │   │     │
│     │ │ languages, the smell of unfamiliar food...      │   │     │
│     │ │                                                  │   │     │
│     │ │ Maybe it's time to plan another trip. Somewhere │   │     │
│     │ │ with mountains this time.                        │   │     │
│     │ │                                                  │   │     │
│     │ │ (Full text preview, read-only)                  │   │     │
│     │ └─────────────────────────────────────────────────┘   │     │
└─────────────────────────────────────────────────────────────────────┘
```

**Preview Feature:**
- **[Preview ▼] button:** Expands inline preview
- **Read-only:** Can't edit from preview (must open full entry)
- **Scroll:** If entry is long, preview is scrollable
- **[Close]:** Collapses preview back to excerpt

---

## Accessibility Notes

### Keyboard Navigation
- **Tab:** Focus search input
- **Arrow down/up:** Navigate results
- **Enter:** Open selected result
- **Ctrl+F:** Focus search input (global hotkey)
- **Esc:** Close search modal

### Screen Reader
- **Result count:** Announced as "12 results for travel"
- **Results:** Announced as "Result 1 of 12: Reflecting on Travel, November 15, 2025"
- **Filters:** Announced as "Filter applied: Tagged work, mood frustrated"

### Color & Contrast
- **Highlighted search terms:** Use bold + background color (not color alone)
- **Result cards:** High contrast borders
- **Modal backdrop:** 80% opacity (dimmed but not black)

---

## Performance Targets

- **Search query:** <50ms (keyword search, indexed)
- **Semantic search:** <500ms (AI vector search)
- **Results render:** <100ms (first 10 results)
- **Typing latency:** <16ms (60fps, instant feedback)

---

## Developer Notes

### Tech Stack
- **Keyword search:** SQLite FTS (Full-Text Search) index
- **Semantic search:** sqlite-vec extension (vector similarity)
- **Search syntax parser:** Custom parser for commands (tag:, mood:, etc.)
- **Highlighting:** HTML `<mark>` tag for search term highlights

### Optimization
- **Index entries:** Create FTS index on title + content
- **Cache results:** Cache last 5 searches (avoid redundant queries)
- **Debounce typing:** Wait 300ms after last keystroke before searching

---

**End of Search Modal Wireframe**
