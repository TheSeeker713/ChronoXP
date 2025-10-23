# UX Flow: F-2 Save & Tag

**Flow ID:** F-2  
**User Goal:** Ensure entry is saved and properly tagged for future retrieval  
**Primary Persona:** All (especially Reflective and Gamer who value organization)

---

## Entry Point
User has written or is writing a journal entry in the Canvas.

---

## Flow Steps

### Step 1: Automatic Saving (No User Action Required)
**Trigger:** User stops typing for 3 seconds  
**System:**
- Autosave writes entry to encrypted SQLite database
- Generates unique entry ID and timestamp
- Status bar updates: "Autosaved just now"

**Visual Feedback:**
```
┌─────────────────────────────────────────────────────┐
│  ...entry text...                                   │
└─────────────────────────────────────────────────────┘
   Word count: 156 | ✓ Autosaved just now
```

**User Confidence:** No need to press Ctrl+S or click "Save" button.

---

### Step 2: Manual Tag Entry (Optional)
**Trigger:** User clicks `[Tags]` button or presses `Ctrl/Cmd+T`  
**System:**
- Non-modal chip input appears below entry
- Autocomplete shows existing tags as user types
- Tags saved immediately on Enter

**Screen:**
```
┌─────────────────────────────────────────────────────┐
│  Today I reflected on my career goals...            │
│                                                     │
│  Tags: [career ×] [reflection ×] [add tag_____]    │
│        ↓ Suggestions: goals, future, planning       │
└─────────────────────────────────────────────────────┘
```

**User Actions:**
- Type tag name → Press `Enter` → Tag added
- Click `×` on tag chip → Tag removed
- Click outside or press `Esc` → Close tag input

**Database:**
```sql
INSERT INTO tags (name) VALUES ('career') ON CONFLICT DO NOTHING;
INSERT INTO entry_tags (entry_id, tag_id) VALUES (123, 5);
```

---

### Step 3: Background Archivist Processing
**Trigger:** Autosave completes  
**System (invisible to user):**
1. **Text analysis:**
   - Extracts mood (e.g., "reflective", "hopeful")
   - Identifies topics (e.g., "career", "goals", "future")
   - Detects entities (e.g., "next year", "manager")
2. **Embedding generation:**
   - Calls local GGUF model to generate vector
   - Stores embedding in `embeddings` table
3. **Auto-tagging:**
   - Suggests additional tags (non-intrusive)
   - User can accept or ignore

**Notification (bottom-right toast, dismissable):**
```
┌─────────────────────────────────┐
│ Archivist noticed these topics: │
│ [+ goals] [+ planning]          │
│ [Accept] [Dismiss]              │
└─────────────────────────────────┘
```

**User Actions:**
- Click `[Accept]` → Tags added
- Click `[Dismiss]` or wait 10s → Toast fades
- Toggle off: Settings → AI → ☐ Enable Archivist auto-tagging

**Database:**
```sql
-- Embedding stored as BLOB
INSERT INTO embeddings (entry_id, vector, model_id, updated_at)
VALUES (123, <BLOB>, 'phi-3-mini-q4', '2025-10-22 15:42:00');
```

---

### Step 4: Date & Time Stamp (Automatic)
**Trigger:** Entry created or modified  
**System:**
- `created_at`: Set on first save
- `updated_at`: Updated on every autosave
- Displayed as human-readable format

**Display:**
```
October 22, 2025 • 3:42 PM
Last edited: 3:45 PM (if within same day)
```

**Database:**
```sql
INSERT INTO entries (id, created_at, updated_at, title, markdown)
VALUES (123, '2025-10-22 15:42:00', '2025-10-22 15:45:00', NULL, '...');
```

---

### Step 5: Word Count Tracking
**Trigger:** Any text change  
**System:**
- Counts words in real-time (debounced)
- Stored in `word_count` column
- Used for XP calculation (threshold: 50 words)

**Display:**
```
Word count: 156 | ✓ Autosaved 2s ago
```

**XP Logic:**
```
if word_count >= 50 and entry_is_new:
    award_xp(10)
```

---

### Step 6: Mood Detection (Background)
**Trigger:** Archivist analysis completes  
**System:**
- Sentiment analysis via local model
- Mood saved as text field (e.g., "reflective", "anxious", "joyful")
- Never shown intrusively; visible in Library details

**Database:**
```sql
UPDATE entries SET mood = 'reflective' WHERE id = 123;
```

**Future use:**
- Mood trends over time (optional analytics)
- Filter entries by mood in Library

---

## Success Criteria
- ✅ Autosave completes within 3 seconds of last keystroke
- ✅ Tags are saved immediately on Enter
- ✅ Archivist suggestions appear within 5 seconds (non-blocking)
- ✅ User can dismiss Archivist suggestions without penalty
- ✅ Entry is never lost, even if app crashes mid-save

---

## Edge Cases

### E1: User adds duplicate tag
- System: Prevents duplicate (case-insensitive)
- Feedback: Tag input shakes, shows "Already added"

### E2: Network is down (offline mode)
- System: Works identically (fully offline)
- No warnings or errors

### E3: User closes app before autosave triggers
- System: Force-saves on window close event
- Ensures no data loss

### E4: Archivist suggests irrelevant tags
- User: Click `[Dismiss]` or ignore
- Future: User can downvote suggestions (trains local model)

### E5: Entry is very long (5000+ words)
- System: Autosave may take >3s
- Status bar: "Saving..." with spinner
- Embedding generation queued for background thread

---

## Accessibility Notes
- **Keyboard-only tag management:**
  - `Ctrl/Cmd+T`: Open tag input
  - Type → `Enter`: Add tag
  - `Backspace` on empty input: Remove last tag
  - `Esc`: Close tag input
- **Screen reader:**
  - "Tag input opened. Type to search or create tag."
  - "Tag 'career' added. 2 tags total."
  - "Archivist suggests: goals, planning. Press Tab to accept, Esc to dismiss."

---

## Microcopy
- **Autosave status:**
  - ✅ "Autosaved just now"
  - ✅ "Autosaved 2 seconds ago"
  - ✅ "Saving..." (if in progress)
- **Tag input placeholder:** "Add a tag..."
- **Archivist toast:** "Archivist noticed these topics:" (not "AI detected" — too technical)

---

## Analytics (Optional, Privacy-Respecting)
If user opts in:
- % of entries with manual tags vs. auto-tags only
- Median time between keystrokes and autosave completion
- % of Archivist suggestions accepted vs. dismissed
- Average word count per entry

**Never logged:** Tag names, entry content, mood values.
