# Wireframe: Easter Eggs & Hidden Features

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

ChronoXP includes playful, delightful surprises for curious users who explore beyond the surface. These easter eggs are never intrusive—they're optional discoveries that reward exploration and add personality to the app.

**Design Philosophy:**
- **Optional:** Never block core functionality
- **Discoverable:** Hints and patterns for curious users
- **Delightful:** Bring joy, not distraction
- **Respectful:** Can be disabled in settings (Simple Mode)

---

## 1. Konami Code: Developer Mode

**Trigger:** Up, Up, Down, Down, Left, Right, Left, Right, B, A, Enter  
(Keyboard: ↑ ↑ ↓ ↓ ← → ← → B A Enter)

**Activation:**
```
┌──────────────────────────────────────────────────────────────────┐
│  🎮 Developer Mode Activated!                                 [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Welcome, fellow code archaeologist! You've unlocked:           │
│                                                                  │
│  ✓ Developer Console (Ctrl+Shift+D)                             │
│  ✓ Database Inspector (view raw SQLite)                         │
│  ✓ Performance Monitor (FPS, memory usage)                      │
│  ✓ Secret "The Developer's Notes" artifact                      │
│                                                                  │
│  [Open Console] [Close]                                          │
└──────────────────────────────────────────────────────────────────┘
```

**Developer Console (Ctrl+Shift+D):**
```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings] [🛠️ Dev Console]│
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Developer Console                                                    │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Database Stats                                                       │
│  • Entries: 156                                                      │
│  • Tags: 24                                                          │
│  • Embeddings: 156 vectors (768 dimensions)                         │
│  • Database size: 12.4 MB                                            │
│  • Vacuum last run: 2024-10-20                                       │
│                                                                       │
│  Performance                                                          │
│  • FPS: 60 (16.7ms avg frame time)                                   │
│  • Memory: 142 MB / 2048 MB allocated                                │
│  • AI model loaded: Phi-3-mini-4k (2.3 GB)                           │
│                                                                       │
│  SQL Query                                                            │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │ SELECT * FROM entries WHERE date > '2025-01-01'                │ │
│  └────────────────────────────────────────────────────────────────┘ │
│  [Execute Query]                                                      │
│                                                                       │
│  Commands                                                             │
│  [Export Database] [Vacuum Database] [View Logs] [Reset XP]          │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**"The Developer's Notes" Artifact:**
- **Appearance:** Hand-painted laptop with glowing screen, coffee mug, scattered papers
- **Flavor text:** "A memento for those who dig beneath the surface—where curiosity meets code, and bugs become features."
- **Unlock condition:** Activate developer mode
- **XP:** 0 (bonus artifact, doesn't count toward progression)

---

## 2. Records Cavern: Hidden Interactions

### 2.1 Clicking the Cosmic Window (Level 5 Cavern)

**Location:** Top-right corner of Level 5 cavern (Master's Sanctum)

**Interaction:**
- **First click:** Window view changes to nighttime (stars, moon)
- **Second click:** Dawn (orange/pink sunrise)
- **Third click:** Midday (bright blue sky)
- **Fourth click:** Sunset (purple/gold)
- **Fifth click:** Back to original (ambient mystical glow)

**Visual feedback:**
- Smooth 2-second crossfade between window states
- No sound (respects quiet journaling environment)

**Toast notification (first time only):**
```
┌────────────────────────────────────────────────────────────┐
│ 🌅 Time flows differently here...                       [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ The cosmic window responds to your curiosity.             │
│ Click again to see what else it reveals.                  │
│                                                            │
│ +5 XP (Secret Explorer)                                    │
└────────────────────────────────────────────────────────────┘
```

---

### 2.2 Clicking Moss Patch (Level 3+ Cavern)

**Location:** Bottom-left corner, near stone floor

**Interaction:**
- **Click:** Tiny mushrooms sprout from moss (hand-painted animation, 1 second)
- **Wait 5 seconds:** Mushrooms emit gentle sparkles
- **Click again:** Mushrooms retract back into moss

**Toast notification (first time only):**
```
┌────────────────────────────────────────────────────────────┐
│ 🍄 Life finds a way...                                   [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ Even in the quiet corners, magic grows.                   │
│ +5 XP (Secret Explorer)                                    │
└────────────────────────────────────────────────────────────┘
```

---

### 2.3 Cat Silhouette (Random Chance)

**Trigger:** 1% chance per cavern visit (random event)

**Interaction:**
- **Appearance:** Black cat silhouette walks across middle of screen (left to right)
- **Duration:** 3 seconds (slow, deliberate walk)
- **Meow:** Soft "meow" sound effect (optional, respects accessibility settings)

**If user clicks the cat:**
```
┌────────────────────────────────────────────────────────────┐
│ 🐈 A Familiar Visitor                                    [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ "I've been watching your progress," purrs the cat.        │
│ "You're doing well."                                       │
│                                                            │
│ The cat vanishes into shadow.                             │
│                                                            │
│ +10 XP (Serendipity)                                       │
└────────────────────────────────────────────────────────────┘
```

**Special artifact unlock (if clicked 3 times across different sessions):**
- **Artifact:** The Shadow Companion (hand-painted black cat with yellow eyes)
- **Flavor text:** "A mysterious feline friend who appears only to those patient enough to notice. Cats, like memories, come and go on their own terms."

---

### 2.4 Bookshelf Secret (Level 4+ Cavern)

**Location:** Bookshelf in background (right side)

**Interaction:**
- **Hover over specific book spine** (third book from left, second shelf): Faint glow
- **Click:** Book slides out slightly, reveals hidden note

**Hidden Note Modal:**
```
┌──────────────────────────────────────────────────────────────────┐
│  📜 A Hidden Note                                             [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  [Hand-drawn sketch of quill pen with handwritten text:]        │
│                                                                  │
│  "To whoever finds this—                                        │
│                                                                  │
│   Your thoughts are not noise.                                  │
│   Your feelings are not too much.                               │
│   Your story matters.                                           │
│                                                                  │
│   Keep writing.                                                 │
│                                                                  │
│   — A fellow traveler"                                          │
│                                                                  │
│  +15 XP (Keeper of Secrets)                                      │
│                                                                  │
│  [Close]                                                         │
└──────────────────────────────────────────────────────────────────┘
```

---

## 3. Canvas: Secret Writing Prompts

### 3.1 Triple-Click Empty Canvas

**Trigger:** Triple-click on empty Canvas (no entry loaded)

**Result:** Random encouraging quote appears in ghost text:
- *"The first word is always the hardest. Start small."*
- *"You don't have to write beautifully. You just have to write."*
- *"What's on your mind right now? Start there."*
- *"Dear [User's name], today..."*
- *"It's okay if this is messy. That's what first drafts are for."*

**Toast notification (first time only):**
```
┌────────────────────────────────────────────────────────────┐
│ ✨ A little encouragement...                             [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ Triple-click anytime for a gentle nudge to get started.   │
│ +5 XP (Secret Explorer)                                    │
└────────────────────────────────────────────────────────────┘
```

---

### 3.2 Type "xyzzy" (Text Adventure Reference)

**Trigger:** Type "xyzzy" (classic text adventure magic word) in Canvas

**Result:** Muse suggestion appears (even if Muse disabled):
```
┌─────────────────────────────────────────────────────────────────┐
│ 💡 Muse suggests:                                               │
│ "Nothing happens. Wait, did you expect something to happen?"    │
│ Press Tab to accept, Esc to dismiss                             │
└─────────────────────────────────────────────────────────────────┘
```

**If accepted:**
- Entry text becomes: "xyzzy\n\nNothing happens. Wait, did you expect something to happen?"
- +5 XP (Adventurer)

---

### 3.3 Type "hello ChronoXP" or "hello Muse"

**Trigger:** Type "hello ChronoXP" or "hello Muse" in Canvas

**Result:** Muse responds personally:
```
┌─────────────────────────────────────────────────────────────────┐
│ 💡 Muse suggests:                                               │
│ "Hello! I'm glad you're here. What would you like to write     │
│ about today?"                                                   │
│ Press Tab to accept, Esc to dismiss                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Library: Calendar Patterns

### 4.1 Perfect Month (Entry Every Day)

**Trigger:** User writes at least one entry every day for a full calendar month

**Result:** Calendar view shows special visual effect:
- **Golden glow** around completed month
- **Confetti animation** (brief, 2 seconds) when viewing that month

**Toast notification:**
```
┌────────────────────────────────────────────────────────────┐
│ 🎉 Perfect Month!                                        [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ You wrote every single day in October 2025. Incredible.   │
│                                                            │
│ +50 XP (Dedication)                                         │
│ 🏆 Unlocked artifact: "The Chronicle's Seal"              │
└────────────────────────────────────────────────────────────┘
```

**"The Chronicle's Seal" Artifact:**
- **Appearance:** Hand-painted wax seal with calendar emblem
- **Flavor text:** "A mark of unwavering commitment. One entry, one day, one month—a testament to showing up for yourself."

---

### 4.2 Clicking Today's Date 7 Times

**Trigger:** Click today's date in Calendar view 7 times rapidly

**Result:** Time-travel mode (whimsical):
```
┌────────────────────────────────────────────────────────────┐
│  ⏰ Time-Travel Mode                                      [×]│
│  ──────────────────────────────────────────────────────     │
│                                                            │
│  Enter a date to jump to:                                  │
│  ┌────────────────────────────────────────────────────┐   │
│  │ MM / DD / YYYY                                      │   │
│  └────────────────────────────────────────────────────┘   │
│                                                            │
│  [Go Back in Time] [Cancel]                                │
└────────────────────────────────────────────────────────────┘
```

**Functionality:** Instantly jumps to that date in Calendar view (useful for reviewing specific past dates quickly)

---

## 5. Search: Secret Queries

### 5.1 Search for "meaning of life"

**Result:** Special search result with Easter egg entry:
```
┌──────────────────────────────────────────────────────────────┐
│  Search Results for "meaning of life"                       │
│  ────────────────────────────────────────────────────────   │
│                                                              │
│  🎁 Secret Message                                           │
│  "The meaning of life is whatever you're willing to write   │
│  down. Start with today."                                   │
│                                                              │
│  +10 XP (Philosopher)                                        │
│                                                              │
│  [New Entry] [Close]                                         │
└──────────────────────────────────────────────────────────────┘
```

---

### 5.2 Search for "42" (Hitchhiker's Guide Reference)

**Result:**
```
┌──────────────────────────────────────────────────────────────┐
│  Search Results for "42"                                     │
│  ────────────────────────────────────────────────────────   │
│                                                              │
│  🌌 The Answer to the Ultimate Question                     │
│  "You've found the answer. Now... what was the question?"   │
│                                                              │
│  +5 XP (Hitchhiker)                                          │
│                                                              │
│  [Don't Panic] [Close]                                       │
└──────────────────────────────────────────────────────────────┘
```

---

## 6. Command Bar: Secret Commands

### 6.1 Type "surprise me"

**Result:** Random action:
- Jumps to random past entry
- Opens random artifact in Cavern
- Generates random writing prompt
- Shows random motivational quote

**Toast notification:**
```
┌────────────────────────────────────────────────────────────┐
│ 🎲 Surprise!                                             [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ Sometimes serendipity is the best guide.                   │
└────────────────────────────────────────────────────────────┘
```

---

### 6.2 Type "make it rain"

**Result:** Brief confetti animation across entire app (2 seconds)

**Toast notification:**
```
┌────────────────────────────────────────────────────────────┐
│ 🎉 Celebration time!                                     [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ Because you deserve to celebrate your progress.           │
│ +5 XP (Party Mode)                                         │
└────────────────────────────────────────────────────────────┘
```

---

### 6.3 Type "unicorn mode"

**Result:** All artifact illustrations temporarily replaced with unicorn variants (ridiculous, playful)

**Modal:**
```
┌────────────────────────────────────────────────────────────┐
│  🦄 Unicorn Mode                                          [×]│
│  ──────────────────────────────────────────────────────     │
│                                                            │
│  Unicorn Mode is now active!                               │
│                                                            │
│  All artifacts have been replaced with... unicorns.        │
│  Because why not?                                          │
│                                                            │
│  [Keep Unicorns] [Revert to Normal]                        │
└────────────────────────────────────────────────────────────┘
```

**Duration:** Until app restart or user clicks [Revert to Normal]

---

## 7. Settings: Hidden Developer Credits

### 7.1 Click Version Number 10 Times

**Location:** Settings → About → Version 1.0.0

**Result:** Developer credits modal:
```
┌──────────────────────────────────────────────────────────────────┐
│  Credits                                                      [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP™ Journal                                               │
│  A labor of love by TheSeeker713                                │
│                                                                  │
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Special Thanks:                                                 │
│  • Every user who trusted us with their private thoughts        │
│  • Open-source contributors who made this possible              │
│  • Coffee, for obvious reasons ☕                                │
│                                                                  │
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Built with:                                                     │
│  • Tauri 2.x (Rust + Web)                                        │
│  • React 18 (TypeScript)                                         │
│  • SQLCipher (Privacy-first database)                           │
│  • llama.cpp (Local AI)                                          │
│  • Love, patience, and way too many late nights 💙              │
│                                                                  │
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  "To everyone journaling their way through life:                │
│   Your words matter. Keep writing."                             │
│                                                                  │
│  — TheSeeker713                                                  │
│                                                                  │
│  [Close]                                                         │
└──────────────────────────────────────────────────────────────────┘
```

---

## 8. XP Milestones: Secret Achievements

### 8.1 Write Entry at 3:33 AM

**Result:**
```
┌────────────────────────────────────────────────────────────┐
│ 🌙 Night Owl                                             [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ You wrote an entry at 3:33 AM. Either you're a dedicated  │
│ journaler or you couldn't sleep. Either way, we see you.  │
│                                                            │
│ +15 XP (Nocturnal Scribe)                                  │
└────────────────────────────────────────────────────────────┘
```

---

### 8.2 Write 1000+ Word Entry

**Result:**
```
┌────────────────────────────────────────────────────────────┐
│ 📜 Epic Entry                                            [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ You wrote 1,243 words. That's a novel-length chapter!     │
│                                                            │
│ +25 XP (Wordsmith)                                         │
│ 🏆 Unlocked artifact: "The Endless Scroll"                │
└────────────────────────────────────────────────────────────┘
```

---

### 8.3 Write Entry on Your Birthday

**Trigger:** User sets birthday in Settings (optional field), writes entry on birthday

**Result:**
```
┌────────────────────────────────────────────────────────────┐
│ 🎂 Happy Birthday!                                       [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ You took time to write today—on your birthday. That's     │
│ a beautiful gift to your future self.                     │
│                                                            │
│ +30 XP (Birthday Reflection)                               │
│ 🏆 Unlocked artifact: "The Candle of Years"               │
└────────────────────────────────────────────────────────────┘
```

---

## 9. First Entry Anniversary

**Trigger:** User writes an entry on the one-year anniversary of their first ChronoXP entry

**Result:**
```
┌────────────────────────────────────────────────────────────┐
│ 🎉 One Year Together!                                    [×]│
│ ──────────────────────────────────────────────────────     │
│                                                            │
│ One year ago today, you wrote your first entry in        │
│ ChronoXP. 365 days of growth, reflection, and courage.    │
│                                                            │
│ Total entries: 243                                         │
│ Total words: 87,452                                        │
│ Days journaled: 189 / 365 (52%)                           │
│                                                            │
│ [View Year in Review] [Keep Writing]                       │
│                                                            │
│ +100 XP (First Anniversary)                                │
│ 🏆 Unlocked artifact: "The Eternal Flame"                 │
└────────────────────────────────────────────────────────────┘
```

**[View Year in Review]:**
- Shows timeline visualization of entries over the year
- Most-used tags (word cloud)
- Mood distribution (pie chart)
- Longest streak (consecutive days)
- Favorite writing time (morning / afternoon / evening / night)

---

## 10. Accessibility: Disabling Easter Eggs

**Location:** Settings → Accessibility → Cognitive Support

```
☑ Minimize UI chrome (hide status bar, fewer buttons)
☐ Disable Easter eggs and surprises (focus mode)
```

**When enabled:**
- No confetti animations
- No secret cat appearances
- No random prompts from triple-click
- Developer mode still works (intentional action)
- Hidden interactions still present but no toasts

**Philosophy:** Easter eggs are joyful, but some users find unpredictability stressful. Respect their needs.

---

## 11. Community Contributions

**Future:** User-submitted easter eggs via GitHub pull requests

**Guidelines:**
- Must be non-intrusive
- Must be skippable
- Must respect Simple Mode / accessibility settings
- Must be delightful, not annoying
- Must not compromise privacy or security

---

## Developer Notes

### Tech Stack
- **Easter egg detection:** Event listeners for specific patterns (keystrokes, clicks)
- **State tracking:** SQLite `easter_eggs` table tracks discovered eggs
- **XP rewards:** Unique XP events (`secret_explorer`, `philosopher`, `hitchhiker`)
- **Confetti animations:** Canvas-based particle system (GPU-accelerated)

### Easter Egg Discovery Tracking
```sql
CREATE TABLE easter_eggs (
  id TEXT PRIMARY KEY,
  discovered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  times_triggered INTEGER DEFAULT 1
);

INSERT INTO easter_eggs (id) VALUES
  ('konami_code'),
  ('cosmic_window'),
  ('moss_patch'),
  ('cat_clicked'),
  ('bookshelf_note'),
  ('triple_click_canvas'),
  ('xyzzy'),
  ('meaning_of_life'),
  ('42'),
  ('surprise_me'),
  ('make_it_rain'),
  ('unicorn_mode'),
  ('night_owl'),
  ('epic_entry'),
  ('birthday'),
  ('first_anniversary');
```

### Performance
- **Easter egg detection:** <1ms overhead (passive listeners)
- **Confetti animation:** 60 FPS (requestAnimationFrame)
- **Cat silhouette:** Preloaded sprite, no render lag

---

## Summary: Easter Eggs List

| Trigger | Result | XP | Artifact |
|---------|--------|----|----|
| Konami Code | Developer Mode | 0 | The Developer's Notes |
| Cosmic Window (click) | Weather changes | 5 | — |
| Moss Patch (click) | Mushrooms sprout | 5 | — |
| Cat (click 3x) | Shadow Companion | 10 | The Shadow Companion |
| Bookshelf Secret | Hidden note | 15 | — |
| Triple-click Canvas | Encouraging quote | 5 | — |
| Type "xyzzy" | Text adventure nod | 5 | — |
| Type "hello Muse" | Personal greeting | 0 | — |
| Perfect Month | Golden glow | 50 | The Chronicle's Seal |
| Click date 7x | Time-travel mode | 0 | — |
| Search "meaning of life" | Secret message | 10 | — |
| Search "42" | Hitchhiker's ref | 5 | — |
| Command: "surprise me" | Random action | 0 | — |
| Command: "make it rain" | Confetti | 5 | — |
| Command: "unicorn mode" | Unicorn artifacts | 0 | — |
| Version 10x click | Developer credits | 0 | — |
| Write at 3:33 AM | Night Owl | 15 | — |
| Write 1000+ words | Epic Entry | 25 | The Endless Scroll |
| Write on birthday | Birthday toast | 30 | The Candle of Years |
| First anniversary | Year in review | 100 | The Eternal Flame |

**Total Discoverable XP:** 290 XP  
**Total Secret Artifacts:** 6 artifacts

---

**End of Easter Eggs & Hidden Features Wireframe**
