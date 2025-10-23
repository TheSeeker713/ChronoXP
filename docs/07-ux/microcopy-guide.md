# Microcopy Guide

**Project:** ChronoXP™ Journal  
**Date:** October 22, 2025

---

## Principle: Invite, Don't Judge

ChronoXP microcopy is **warm, non-judgmental, and neuro-affirming**. We use language that invites users to engage without creating pressure, guilt, or shame.

---

## Voice & Tone

### Voice (Constant)
- **Calm:** Never urgent or aggressive
- **Supportive:** Like a kind friend, not a coach or authority
- **Clear:** Plain language, no jargon
- **Honest:** Transparent about what's happening

### Tone (Adapts by Context)

| Context | Tone | Example |
|---------|------|---------|
| **First launch** | Welcoming | "Welcome to ChronoXP. Start writing whenever you're ready." |
| **Daily use** | Neutral/Calm | "October 22, 2025. Ready to write." |
| **After long gap** | Encouraging (not guilt-tripping) | "Welcome back! No pressure — pick up where you left off." |
| **Error** | Reassuring | "ChronoXP couldn't save your entry. Your work is safe — try restarting the app." |
| **Success** | Quietly celebratory | "Export complete!" |

---

## Core Phrases (Use These, Not Those)

### Starting to Write

✅ **"Want a soft start? Try a one-line check-in."**  
❌ "Write something!" (too demanding)  
❌ "What's on your mind?" (too generic, therapy vibes)  
❌ "Start journaling now." (imperative, stressful)

---

### Autosave Status

✅ **"Autosaved just now"**  
✅ **"Autosaved 2 seconds ago"**  
❌ "Saved successfully" (too formal)  
❌ "Your entry is safe" (implies it might not have been?)

---

### Welcome Back After Gap

✅ **"Welcome back! Take your time."**  
✅ **"No pressure. Pick up where you left off."**  
❌ "You haven't written in 7 days!" (guilt trip)  
❌ "Where have you been?" (judgmental)  
❌ "Your streak is broken." (shame)

---

### Artifact Unlock

✅ **"✨ Artifact unlocked! [View in Cavern] [Later]"**  
❌ "Achievement unlocked!" (gamey, Xbox vibes)  
❌ "Congrats! You did it!" (too performative)  
❌ "Level up!" (not calm)

---

### Quest/Reflection Prompt

✅ **"New reflection prompt available: [Title]. This is optional — no pressure if you're not in the mood."**  
❌ "Complete this quest to earn XP!" (obligation)  
❌ "Don't miss out on this limited-time prompt!" (FOMO)

---

### Search (No Results)

✅ **"No entries found. Try different keywords or a broader topic."**  
❌ "0 results" (too clinical)  
❌ "Nothing matches your search." (sounds final/discouraging)

---

### Export

✅ **"Export your journal as a portable backup."**  
✅ **"Export complete! [Open Folder]"**  
❌ "Create .chrxp archive" (too technical)  
❌ "Backup successful" (too formal)

---

### Errors

✅ **"ChronoXP couldn't save your entry. Your work is safe — try restarting the app."**  
✅ **"This file appears corrupted. Import aborted."**  
❌ "Error 500: Database write failed" (technical, scary)  
❌ "Fatal exception in SQLCipher module" (dev-speak)

---

## Specific UI Elements

### Ghost Placeholder (Canvas)
**Text:** "Want a soft start? Try a one-line check-in."  
**Tone:** Gentle invitation  
**Why:** Reduces blank-page anxiety; offers optional structure

---

### Tag Input Placeholder
**Text:** "Add a tag..."  
**Tone:** Neutral  
**Why:** Clear, concise; doesn't need personality

---

### Command Bar Placeholder
**Text:** "Type to search or run a command..."  
**Tone:** Neutral/Helpful  
**Why:** Explains dual purpose (search + commands)

---

### Calendar (No Entry for Date)
**Text:** (Empty cell — no text)  
**Why:** Absence of entry is not failure; don't highlight gaps  

❌ Avoid: Red X, "No entry", "You skipped this day"

---

### Muse Suggestion (Inline)
**Text:** (Greyed-out suggestion text)  
**Tooltip:** "Press Tab to accept, Esc to dismiss"  
**Tone:** Neutral  
**Why:** Instructive without being pushy

---

### Archivist Auto-Tag Suggestion
**Text:** "Archivist noticed these topics: [+tag1] [+tag2]. [Accept] [Dismiss]"  
**Tone:** Informative, non-intrusive  
**Why:** Frames AI as helpful observer, not authoritative decider

---

### Settings: Simple Mode
**Label:** "Enable Simple Mode"  
**Description:** "Hide XP, artifacts, and quests. Focus on pure journaling."  
**Confirmation (when enabled):** "Simple Mode enabled. You can turn this off anytime."  
**Why:** Clear opt-out, no shame for choosing simplicity

---

### Settings: Reduced Motion
**Label:** "Reduce motion"  
**Description:** "Simplify animations for a calmer experience."  
**Why:** Frames as benefit (calm), not deficit (disability)

---

### First-Run Welcome
**Title:** "Welcome to ChronoXP"  
**Body:** "This is your private journaling space. Everything stays on your device. Start writing whenever you're ready."  
**Buttons:** `[Skip]` | `[Quick Tour]`  
**Tone:** Welcoming, not overwhelming  
**Why:** Emphasizes privacy immediately; no forced onboarding

---

### Notification: Update Available
**Text:** "Version 1.2.0 is available. [Download] [Skip this version] [Remind me later]"  
**Tone:** Informative, user-controlled  
**Why:** No auto-updates; respects user agency

---

## Button Labels

### Primary Actions
✅ **"Export"** (not "Generate Archive")  
✅ **"Save"** (even though autosave is on — for manual force-save)  
✅ **"Close"** (not "Dismiss" or "OK")  
✅ **"Import"** (not "Restore from Backup")

### Secondary Actions
✅ **"Cancel"** (standard)  
✅ **"Maybe Later"** (softer than "No" or "Skip")  
✅ **"Don't Show Again"** (clear opt-out)

### Destructive Actions
✅ **"Delete Entry"** (clear consequence)  
❌ "Remove" (ambiguous)  
❌ "Trash" (casual, not clear if permanent)

**Confirmation:** "Delete this entry? This can't be undone."  
**Buttons:** `[Cancel]` | `[Delete]` (red background)

---

## Error Messages

### Database Error
❌ "SQLite error: table 'entries' does not exist"  
✅ "ChronoXP couldn't load your journal. Try restarting the app. If this persists, contact support."

---

### Export Failure (Disk Full)
❌ "Insufficient storage space (Error 0x80070070)"  
✅ "Not enough disk space. Free up space and try again."

---

### Import Failure (Corrupted File)
❌ "Checksum mismatch: expected a3f2b9c, got 7d8e1f0"  
✅ "This file appears corrupted. Import aborted."

---

### Wrong Password (Encrypted Import)
❌ "Decryption failed: invalid key"  
✅ "Incorrect password. Please try again."

---

### Network Error (Even Though Offline-First)
If we ever add optional cloud sync:  
❌ "HTTP 503: Service Unavailable"  
✅ "Can't connect right now. Your journal is safe and will sync later."

---

## Success Messages

### Entry Saved
✅ **"Autosaved just now"** (status bar, not toast)  
❌ "Entry saved successfully" (too formal)

---

### Export Complete
✅ **"Export complete! [Open Folder]"** (toast)  
❌ "Backup created at C:\Users\..." (too technical)

---

### Import Complete
✅ **"Import successful! 47 entries restored."** (toast)  
❌ "Import operation completed without errors" (robotic)

---

### Artifact Unlocked
✅ **"✨ Artifact unlocked: Memory Jar"** (toast)  
❌ "You earned the Memory Jar artifact!" (too game-like)

---

## Empty States

### No Entries Yet (First Launch)
**Text:** "Your journal is empty. Start writing to fill it with your thoughts."  
**Why:** Neutral, not judgmental

---

### No Search Results
**Text:** "No entries found. Try different keywords or a broader topic."  
**Why:** Helpful, not discouraging

---

### No Tags Yet
**Text:** "No tags yet. Add tags to organize your entries."  
**Why:** Instructive, not empty

---

### No Artifacts Unlocked (Simple Mode Off)
**Text:** "Your Cavern is waiting. Write entries to unlock artifacts."  
**Why:** Inviting, implies progress to come

---

## Help Text / Tooltips

### Tag Input
**Tooltip:** "Press Enter to add a tag. Autocomplete will suggest existing tags."  
**Why:** Clear instructions without cluttering UI

---

### Export Encryption
**Tooltip:** "Encrypt your backup with a password. Remember it — it can't be recovered."  
**Why:** Warns user of consequence (no password recovery)

---

### Muse (AI Suggestion)
**Tooltip:** "Muse suggests the next line. Press Tab to accept, Esc to dismiss."  
**Why:** Explains AI feature without being preachy

---

### XP Counter
**Tooltip:** "XP tracks your journaling journey. Earn XP by writing and reflecting."  
**Why:** Simple explanation for new users

---

## Accessibility Labels (ARIA)

### Save Button (Icon Only)
```html
<button aria-label="Save entry (Ctrl+S)">
  <SaveIcon aria-hidden="true" />
</button>
```

### Search Icon
```html
<button aria-label="Search entries (Ctrl+F)">
  <SearchIcon aria-hidden="true" />
</button>
```

### Close Modal (X Icon)
```html
<button aria-label="Close dialog">
  <XIcon aria-hidden="true" />
</button>
```

**Why:** Screen readers need text labels; visual users see icons.

---

## Do's and Don'ts

### Do's
✅ Use **active voice** ("ChronoXP couldn't save" not "Entry could not be saved")  
✅ Use **positive framing** ("Welcome back!" not "You were gone for 7 days")  
✅ Use **plain language** ("Export your journal" not "Generate .chrxp archive")  
✅ Use **specific numbers** ("47 entries restored" not "Entries restored successfully")  
✅ Use **empathy** ("Your work is safe" when there's an error)

### Don'ts
❌ **No jargon** ("Create backup" not "Serialize database to ZIP")  
❌ **No shame** ("Welcome back!" not "You missed 7 days")  
❌ **No urgency** ("Version available" not "Update NOW!")  
❌ **No false positives** ("Export complete!" only if actually complete)  
❌ **No ALL CAPS** (never, unless acronym like "XP")

---

## Testing Microcopy

### Pre-Launch Checklist
- [ ] Read all microcopy aloud — does it sound natural?
- [ ] Show to non-technical user — do they understand without explanation?
- [ ] Show to neurodivergent tester — does it create stress or calm?
- [ ] Translate to another language (if planned) — does it still make sense?

### A/B Testing (Post-Launch)
- Test: "Welcome back!" vs. "You haven't written in 7 days"
  - Measure: User retention, sentiment survey
- Test: "Export complete!" vs. "Backup created successfully"
  - Measure: Clarity, user satisfaction

---

## Localization Considerations (Future)

### What Translates Easily
- Button labels ("Export", "Cancel", "Save")
- Error messages (as long as plain language)

### What Needs Cultural Adaptation
- **"Soft start"** (colloquialism, may not translate)
- **"Welcome back"** (some cultures prefer formal greetings)
- **Idioms:** Avoid ("Pick up where you left off" is English-specific)

### Translation Rules
1. **Keep it simple:** Short sentences translate better
2. **Avoid slang:** "No pressure" → "Optional" (clearer for non-native speakers)
3. **Test with native speakers:** Don't rely on machine translation

---

## Voice Consistency Across Channels

| Channel | Same Voice? | Notes |
|---------|-------------|-------|
| **In-app UI** | ✅ Primary source | All microcopy must match this guide |
| **Website** | ✅ Yes | Marketing copy should align with product tone |
| **Docs/Help** | ✅ Yes | Technical but still warm |
| **Social media** | ⚠️ Slightly more casual | Can use emojis, humor (but still no guilt trips) |
| **Support emails** | ✅ Yes | Empathetic, clear, non-judgmental |

---

## References & Inspiration
- [MailChimp Voice & Tone](https://styleguide.mailchimp.com/voice-and-tone/)
- [GOV.UK Content Design](https://www.gov.uk/guidance/content-design) (clarity focus)
- [Calm App Microcopy](https://www.calm.com/) (gentle, non-pushy)
- [Headspace Tone](https://www.headspace.com/) (supportive without being patronizing)
