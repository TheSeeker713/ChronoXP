# UX Flow: F-3 Recall by Meaning

**Flow ID:** F-3  
**User Goal:** Find past journal entries by semantic meaning, not exact keywords  
**Primary Persona:** Reflective Journaler, Productivity Gamer

---

## Entry Point
User wants to find entries about a topic but doesn't remember exact words or dates.

---

## Flow Steps

### Step 1: Open Search
**Trigger:** User presses `Ctrl/Cmd+K` or `Ctrl/Cmd+F`  
**System:**
- Command Bar modal opens with search input focused
- Can also access via Library → Search tab

**Screen (Command Bar):**
```
┌─────────────────────────────────────────────────────┐
│  ⌘K                                           [Esc] │
├─────────────────────────────────────────────────────┤
│  🔍 [Type to search or run a command...____]        │
└─────────────────────────────────────────────────────┘
```

---

### Step 2: Enter Semantic Query
**Trigger:** User types natural language query  
**System:**
- Recognizes search intent (vs. command like "new entry")
- Debounced search (500ms after last keystroke)

**Example Queries:**
- "find entries about family and work"
- "when did I write about burnout?"
- "show me reflections on career goals"

**Screen:**
```
┌─────────────────────────────────────────────────────┐
│  ⌘K                                           [Esc] │
├─────────────────────────────────────────────────────┤
│  🔍 [find entries about family and work_____]       │
└─────────────────────────────────────────────────────┘
```

---

### Step 3: Hybrid Search Execution
**Trigger:** User presses `Enter` or waits 500ms  
**System (background):**

1. **Semantic search:**
   - Generate embedding for query using local model
   - Calculate cosine similarity with all stored embeddings
   - Rank by similarity score

2. **Keyword fallback:**
   - If semantic results < 3, also run keyword search
   - Merge results, remove duplicates

3. **Retrieve entries:**
   - Fetch top 10 results from database
   - Include: date, title (first line), snippet (2-3 lines), tags

**Performance:** < 500ms for typical journal (1000 entries).

---

### Step 4: Display Results
**Trigger:** Search completes  
**System:**
- Shows ranked results in modal overlay
- Highlights matching context (for keyword matches)
- Previews tags for quick scanning

**Screen:**
```
┌─────────────────────────────────────────────────────┐
│  Search: "family and work"                    [Esc] │
├─────────────────────────────────────────────────────┤
│  Results (5 found):                                 │
│                                                     │
│  1. ⭐ Oct 15, 2025 — "Work-life balance thoughts" │
│     ...talked to Mom about **family** expectations  │
│     and how **work** is taking over my life...      │
│     Tags: family, work, stress                      │
│                                                     │
│  2. Sep 30, 2025 — "Dinner with family"            │
│     ...discussed career plans with Dad; **work**    │
│     came up but I wanted to focus on **family**...  │
│     Tags: family, career, dinner                    │
│                                                     │
│  3. Oct 8, 2025 — "Feeling burnt out"              │
│     ...**work** is overwhelming; need more          │
│     **family** time to recharge...                  │
│     Tags: work, burnout, health                     │
│                                                     │
│  [Show more...] [New Search]                        │
└─────────────────────────────────────────────────────┘
```

**Legend:**
- ⭐ = Highest relevance (semantic score > 0.85)
- **Bold** = Keyword match (if hybrid search)

---

### Step 5: Open Result
**Trigger:** User clicks result or presses `Enter` (keyboard navigation with arrow keys)  
**System:**
- Opens selected entry in Journal Canvas
- Scrolls to matching section (if keyword match)
- Search modal stays open (but can be closed with `Esc`)

**Screen (Canvas with result):**
```
┌─────────────────────────────────────────────────────┐
│  [<] October 15, 2025 [>]                           │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Work-life balance thoughts                         │
│                                                     │
│  Today I talked to Mom about family expectations    │
│  and how work is taking over my life. She reminded  │
│  me that career success is meaningless without      │
│  family connections...                              │
│                                                     │
│  Tags: [family] [work] [stress]                     │
└─────────────────────────────────────────────────────┘
```

**User Actions:**
- Read entry
- Edit if needed (autosave triggers)
- Navigate to prev/next result with `[<]` / `[>]`
- Return to search with `Ctrl/Cmd+K`

---

### Step 6: Refine Search (Optional)
**Trigger:** User presses `Ctrl/Cmd+K` again  
**System:**
- Reopens search with previous query
- User can modify and re-run

**Example Refinements:**
- "family and work" → "family stress"
- "burnout" → "burnout and rest"

---

## Success Criteria
- ✅ Semantic search returns correct entries for 5 test prompts (see Test Cases below)
- ✅ Results appear in < 500ms for typical journal (1000 entries)
- ✅ Keyboard-only navigation works (arrow keys, Enter, Esc)
- ✅ Matches are highlighted clearly (bold for keywords)
- ✅ No false positives (irrelevant results) in top 5

---

## Test Cases (Semantic Search Accuracy)

### Test 1: Synonyms
- **Query:** "entries about job stress"
- **Expected:** Should find entries with "work burnout", "career pressure", etc.
- **Success:** Top 3 results contain work-related stress

### Test 2: Multi-Concept
- **Query:** "family and health"
- **Expected:** Entries discussing both concepts together
- **Success:** Top 5 results mention family + health/wellness

### Test 3: Temporal
- **Query:** "when did I write about moving?"
- **Expected:** Entries about relocation, new apartment, etc.
- **Success:** Results sorted by date, contain moving-related content

### Test 4: Emotional
- **Query:** "times I felt anxious"
- **Expected:** Entries with anxiety, worry, nervousness
- **Success:** Mood field or content reflects anxiety

### Test 5: Vague
- **Query:** "important thoughts"
- **Expected:** Should return pinned entries or high word-count entries
- **Success:** Top results are substantial reflections

---

## Edge Cases

### E1: No results found
**Screen:**
```
┌─────────────────────────────────────────────────────┐
│  Search: "quantum physics"                    [Esc] │
├─────────────────────────────────────────────────────┤
│  No entries found.                                  │
│                                                     │
│  Try:                                               │
│  • Different keywords                               │
│  • Broader topic (e.g., "science" instead)          │
│  • Check spelling                                   │
│                                                     │
│  [New Search]                                       │
└─────────────────────────────────────────────────────┘
```

### E2: User searches before Archivist has processed entries
- **System:** Falls back to keyword-only search
- **Notification (first time):** "Tip: Archivist is still indexing your entries. Semantic search will improve soon."

### E3: Very large journal (10,000+ entries)
- **System:** Search may take 1-2 seconds
- **Feedback:** Show loading spinner after 500ms
- **Optimization:** Limit to top 50 results, paginate

### E4: Query is a command, not a search
- **Example:** "new entry"
- **System:** Detects command intent, executes instead of searching
- **Fallback:** If ambiguous, asks "Did you mean: [Search] or [New Entry]?"

---

## Accessibility Notes
- **Keyboard navigation:**
  - `Ctrl/Cmd+K`: Open search
  - Type query, press `Enter`: Run search
  - `Arrow keys`: Navigate results
  - `Enter`: Open selected result
  - `Esc`: Close search
- **Screen reader:**
  - "Search results: 5 found. Result 1 of 5: October 15, 2025. Work-life balance thoughts. Tags: family, work, stress."
  - "Press Enter to open entry."

---

## Microcopy
- **Search placeholder:** "Type to search or run a command..."
- **No results:** "No entries found." (not "0 results" — more human)
- **Loading:** "Searching..." (not "Indexing" — less technical)
- **Tip (first time):** "Tip: Try searching by meaning, not just keywords!"

---

## Analytics (Optional, Privacy-Respecting)
If user opts in:
- Average search query length
- % of searches with 0 results
- % of semantic vs. keyword matches
- Time from query to result click

**Never logged:** Query text, result content, clicked entries.

---

## Future Enhancements (Post-MVP)
- **Filter by date range:** "family entries from last month"
- **Filter by mood:** "anxious entries"
- **Sort options:** Relevance, date (newest), date (oldest)
- **Save searches:** Pin frequent queries (e.g., "work reflections")
