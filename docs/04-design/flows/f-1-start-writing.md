# UX Flow: F-1 Start Writing

**Flow ID:** F-1  
**User Goal:** Begin writing a journal entry with minimal friction  
**Primary Persona:** All (Reflective, Neurospicy, Gamer)

---

## Entry Point
User launches ChronoXP app (desktop shortcut, taskbar, or Spotlight/Start menu).

---

## Flow Steps

### Step 1: App Launch
**Trigger:** User opens ChronoXP  
**System:**
- Cold start < 400ms (performance requirement)
- Loads last open surface (default: Journal Canvas)
- If first launch: brief welcome screen (dismissable, never shown again)

**Screen:**
```
┌─────────────────────────────────────────────────────┐
│  ChronoXP                    [_] [□] [×]            │
├─────────────────────────────────────────────────────┤
│  [Journal] [Library] [Cavern] [Settings]           │
├─────────────────────────────────────────────────────┤
│                                                     │
│  October 22, 2025 • 3:42 PM                        │
│                                                     │
│  █                                                  │ ← Cursor auto-focused
│  Want a soft start? Try a one-line check-in.       │ ← Ghost placeholder
│                                                     │
│                                                     │
└─────────────────────────────────────────────────────┘
   Word count: 0 | Ready to write
```

**User Action:** None required — cursor is already in editor.

---

### Step 2: Start Typing
**Trigger:** User begins typing  
**System:**
- Ghost placeholder disappears on first keystroke
- Text appears in default font (system sans or user preference)
- If Muse is enabled: waits for natural pause (300ms) before showing suggestion

**Screen:**
```
┌─────────────────────────────────────────────────────┐
│  ChronoXP                    [_] [□] [×]            │
├─────────────────────────────────────────────────────┤
│  [Journal] [Library] [Cavern] [Settings]           │
├─────────────────────────────────────────────────────┤
│                                                     │
│  October 22, 2025 • 3:42 PM                        │
│                                                     │
│  Today I'm feeling a bit overwhelmed with work.█   │
│                                                     │
│                                                     │
└─────────────────────────────────────────────────────┘
   Word count: 8 | Typing...
```

**User Action:** Continue typing naturally.

---

### Step 3: Muse Suggestion (Optional)
**Trigger:** User pauses typing for 300ms (if Muse enabled)  
**System:**
- Local AI generates next-line or phrase completion
- Displays as greyed-out inline text (ghost style)

**Screen:**
```
┌─────────────────────────────────────────────────────┐
│  ChronoXP                    [_] [□] [×]            │
├─────────────────────────────────────────────────────┤
│  [Journal] [Library] [Cavern] [Settings]           │
├─────────────────────────────────────────────────────┤
│                                                     │
│  October 22, 2025 • 3:42 PM                        │
│                                                     │
│  Today I'm feeling a bit overwhelmed with work.█   │
│  Maybe I need to prioritize and take breaks.       │ ← Greyed out (Muse)
│                                                     │
└─────────────────────────────────────────────────────┘
   Word count: 8 | Tab to accept suggestion
```

**User Actions:**
- **Accept:** Press `Tab` or `Alt+Enter` → grey text becomes black
- **Ignore:** Keep typing → grey text disappears
- **Reject:** Press `Esc` → grey text disappears, Muse pauses for 30s

---

### Step 4: Continue Writing
**Trigger:** User writes 50+ words  
**System:**
- Autosave triggered 3 seconds after last keystroke
- Word count updates in status bar
- Background: Archivist begins analyzing text (debounced)

**Screen:**
```
┌─────────────────────────────────────────────────────┐
│  ChronoXP                    [_] [□] [×]            │
├─────────────────────────────────────────────────────┤
│  [Journal] [Library] [Cavern] [Settings]           │
├─────────────────────────────────────────────────────┤
│                                                     │
│  October 22, 2025 • 3:42 PM                        │
│                                                     │
│  Today I'm feeling a bit overwhelmed with work.    │
│  I have three deadlines this week, and I'm not     │
│  sure I can meet them all without burning out.     │
│  Maybe I should talk to my manager about           │
│  reprioritizing...█                                 │
│                                                     │
└─────────────────────────────────────────────────────┘
   Word count: 52 | Autosaved 2s ago
```

**User Action:** Continue writing or stop.

---

### Step 5: Add Tags (Optional)
**Trigger:** User clicks `[Tags]` button or types `Ctrl/Cmd+T`  
**System:**
- Non-modal chip input appears below entry
- Autocomplete suggests existing tags as user types

**Screen:**
```
┌─────────────────────────────────────────────────────┐
│  ChronoXP                    [_] [□] [×]            │
├─────────────────────────────────────────────────────┤
│  [Journal] [Library] [Cavern] [Settings]           │
├─────────────────────────────────────────────────────┤
│                                                     │
│  October 22, 2025 • 3:42 PM                        │
│                                                     │
│  Today I'm feeling a bit overwhelmed with work...  │
│                                                     │
│  Tags: [work ×] [stress ×] [_________]             │ ← Chip input
│        ↑ Autocomplete: burnout, deadlines           │
└─────────────────────────────────────────────────────┘
   Word count: 52 | Autosaved 2s ago
```

**User Actions:**
- Type tag name, press `Enter` to add
- Click `×` to remove tag
- Click outside input or press `Esc` to close

---

### Step 6: Background Processing (Invisible to User)
**Trigger:** Autosave completes  
**System:**
- **Archivist extracts:**
  - Mood (e.g., "stressed", "overwhelmed")
  - Topics (e.g., "work", "deadlines", "manager")
  - Entities (e.g., "this week")
- Generates embeddings (vector representation)
- Stores in local DB + vector index
- **XP awarded:** +10 XP (for writing 50+ words)

**No visible change** — happens silently in background.

---

### Step 7: Exit / Continue
**User chooses:**

#### Option A: Continue to next entry
- Press `Ctrl/Cmd+N` → New blank entry
- Or click `[>]` arrow to navigate to next day

#### Option B: Close app
- Click `[×]` or press `Alt+F4` / `Cmd+Q`
- System: Auto-saves before closing
- Toast notification (if enabled): "Entry saved. See you next time!"

---

## Success Criteria
- ✅ Time from launch to first keystroke < 400ms
- ✅ Ghost placeholder disappears on first character
- ✅ Muse suggestion appears within 300ms of pause (if enabled)
- ✅ Autosave triggers within 3s of last keystroke
- ✅ No interruptions or mandatory dialogs
- ✅ Entry is saved even if app crashes (autosave reliability)

---

## Edge Cases

### E1: First-time user (no existing entries)
- Welcome screen: "Welcome to ChronoXP! Start writing whenever you're ready."
- `[Skip]` button (bottom-right)
- After dismissal: never shown again

### E2: Muse suggestion is unhelpful
- User presses `Esc` → Muse pauses for 30 seconds
- User can disable Muse entirely: Settings → AI → ☐ Enable Muse

### E3: User closes app mid-sentence
- Autosave ensures no data loss (last state within 3s is saved)
- On next launch: entry reopens at last cursor position

### E4: User wants to format text
- Toolbar (minimal) at top:
  ```
  [B] [I] [H1▼] [•] [1.] [🔗] [🖼️]
  ```
- Keyboard shortcuts:
  - `Ctrl/Cmd+B`: Bold
  - `Ctrl/Cmd+I`: Italic
  - `Ctrl/Cmd+K`: Insert link
  - `Ctrl/Cmd+Shift+I`: Insert image

---

## Accessibility Notes
- **Keyboard-only:** No mouse required for any step
- **Screen reader:** "Editor ready. Type to begin. Optional placeholder: Want a soft start? Try a one-line check-in."
- **Reduced motion:** Muse text fades in (no slide animation)
- **Focus indicator:** Thick blue outline on editor when focused

---

## Microcopy (See `/docs/07-ux/microcopy-guide.md`)
- **Ghost placeholder:** "Want a soft start? Try a one-line check-in."
  - Tone: Inviting, not pushy
  - Alternatives tested:
    - ❌ "Write something!" (too demanding)
    - ❌ "What's on your mind?" (too generic)
    - ✅ Current (gentle nudge)

---

## Analytics (Optional, Privacy-Respecting)
If user opts in to anonymous usage stats:
- Time to first keystroke (median across users)
- % of users who accept Muse suggestions
- % of entries with tags vs. auto-tagged only
- Word count distribution (median, P95)

**Never logged:** Entry content, search queries, personal data.
