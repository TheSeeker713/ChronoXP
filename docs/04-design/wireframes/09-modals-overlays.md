# Wireframe: Modals & Overlays

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

This document covers all modal dialogs, overlays, and popup interfaces not already documented in other wireframes. Includes confirmation dialogs, pickers, detail views, and transient notifications.

---

## 1. Tag Picker Modal

**Triggered by:** Ctrl+T, [Tag] button in formatting toolbar, or clicking tag pill

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ October 23, 2025                              ✓ Saved (1:23) │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │                                                               │  │
│  │ Today was a reflective day. I spent time thinking about my   │  │
│  │ goals for the next year.                                      │  │
│  │                                                               │  │
│  │ ╔═══════════════════════════════════════════════════════╗    │  │
│  │ ║ Add Tags                                          [×] ║    │  │
│  │ ╠═══════════════════════════════════════════════════════╣    │  │
│  │ ║                                                       ║    │  │
│  │ ║ [🔍 Search or create tags...]                         ║    │  │
│  │ ║                                                       ║    │  │
│  │ ║ Current tags:                                         ║    │  │
│  │ ║ [reflection ×] [goals ×]                              ║    │  │
│  │ ║                                                       ║    │  │
│  │ ║ ─────────────────────────────────────────────────    ║    │  │
│  │ ║                                                       ║    │  │
│  │ ║ Suggested (AI):                                       ║    │  │
│  │ ║ [+ planning] [+ future] [+ introspection]             ║    │  │
│  │ ║                                                       ║    │  │
│  │ ║ Recent:                                               ║    │  │
│  │ ║ [+ work] [+ family] [+ gratitude] [+ anxiety]         ║    │  │
│  │ ║                                                       ║    │  │
│  │ ║ All tags (24):                                        ║    │  │
│  │ ║ [+ achievement] [+ anxiety] [+ books] [+ creativity]  ║    │  │
│  │ ║ [+ dreams] [+ exercise] [+ family] [+ food]           ║    │  │
│  │ ║ [+ friends] [+ goals] [+ gratitude] [+ growth]        ║    │  │
│  │ ║ [+ health] [+ ideas] [+ learning] [+ love]            ║    │  │
│  │ ║ [+ meditation] [+ nature] [+ planning] [+ reading]    ║    │  │
│  │ ║ [+ reflection] [+ relationships] [+ travel] [+ work]  ║    │  │
│  │ ║                                                       ║    │  │
│  │ ║                             [Done] [Cancel]           ║    │  │
│  │ ╚═══════════════════════════════════════════════════════╝    │  │
│  │                                                               │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Interactions:**
- **Search field:** Type to filter existing tags or create new one
- **[+ tag]:** Click to add tag to current entry
- **[tag ×]:** Click × to remove tag from current entry
- **Suggested tags:** AI-generated based on entry content (Pro feature)
- **[Done]:** Apply tags and close modal
- **[Cancel] / Esc:** Close without changes

**Creating a new tag:**
```
┌─────────────────────────────────────────────────────────────────┐
│ [🔍 productivity_____________]  ← User typing                   │
│                                                                 │
│ No matching tags found.                                         │
│ [+ Create "productivity" tag]                                   │
│                                                                 │
│ Similar tags:                                                   │
│ [+ goals] [+ work] [+ planning]                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Mood Selector Dropdown

**Triggered by:** Clicking mood indicator in Canvas

```
┌──────────────────────────────────────────────────────────────┐
│ October 23, 2025                              ✓ Saved (1:23) │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│ Today was a reflective day...                                │
│                                                               │
│ [B] [I] [H1] [List] [Tag] [Mood: 😊 Happy ▼] [Fullscreen]   │
│                            │                                  │
│                            ▼                                  │
│                    ┌──────────────────┐                      │
│                    │ 😊 Happy         │                      │
│                    │ 😔 Sad           │                      │
│                    │ 😰 Anxious       │                      │
│                    │ 😌 Calm          │                      │
│                    │ 😤 Frustrated    │                      │
│                    │ 🎉 Excited       │                      │
│                    │ 😴 Tired         │                      │
│                    │ 🤔 Thoughtful    │                      │
│                    │ 🙂 Neutral       │                      │
│                    │ ─────────────    │                      │
│                    │ [No mood]        │                      │
│                    └──────────────────┘                      │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

**Interactions:**
- **Click mood:** Apply to current entry, close dropdown
- **[No mood]:** Remove mood from entry
- **Keyboard:** Arrow keys to navigate, Enter to select, Esc to cancel

---

## 3. Export Dialog

**Triggered by:** Command Bar → Export, Settings → Export & Import

```
┌──────────────────────────────────────────────────────────────────────┐
│  Export Entries                                                   [×]│
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Format                                                               │
│  ● ChronoXP (.chrxp) — Native format, includes everything            │
│  ○ Markdown (.md) — One file per entry                               │
│  ○ JSON (.json) — Structured data                                    │
│  ○ Plain Text (.txt) — No formatting                                 │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  Include                                                              │
│  ☑ All entries (156 entries)                                         │
│  ☑ Tags                                                               │
│  ☑ Attachments                                                        │
│  ☑ Embeddings (for semantic search)                                  │
│  ☑ XP and artifacts progress                                         │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  Date Range                                                           │
│  ● All time (156 entries)                                            │
│  ○ Past year (104 entries)                                           │
│  ○ Past month (18 entries)                                           │
│  ○ Custom range: [From: 01/01/2024] [To: 10/23/2025]                │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  Security                                                             │
│  ☐ Password-protect export (AES-256-GCM)                             │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  Estimated file size: 1.2 MB                                          │
│                                                                       │
│  [Choose Save Location...] [Export] [Cancel]                         │
└──────────────────────────────────────────────────────────────────────┘
```

**With password protection enabled:**
```
┌──────────────────────────────────────────────────────────────────┐
│  ☑ Password-protect export (AES-256-GCM)                         │
│                                                                   │
│  Export password:                                                 │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │ ••••••••••••••••                                            │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                   │
│  Confirm password:                                                │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │ ••••••••••••••••                                            │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                   │
│  Password strength: ████████░░ Strong                            │
│                                                                   │
│  ⚠️ Keep this password safe! You'll need it to import the       │
│  backup later.                                                    │
└──────────────────────────────────────────────────────────────────┘
```

**Export progress:**
```
┌──────────────────────────────────────────────────────────────────┐
│  Exporting Entries...                                         [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Preparing 156 entries for export...                            │
│  ██████████████████████████░░░░░░░░░░░░░░░ 68% (106 / 156)     │
│                                                                  │
│  • Encrypting entries                                           │
│  • Compressing data                                             │
│  • Writing to file                                              │
│                                                                  │
│  Estimated time remaining: 3 seconds                             │
│                                                                  │
│  [Cancel Export]                                                 │
└──────────────────────────────────────────────────────────────────┘
```

**Export complete:**
```
┌──────────────────────────────────────────────────────────────────┐
│  Export Complete! ✓                                           [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Successfully exported 156 entries to:                          │
│  C:\Users\...\Downloads\chronoxp_backup_2025-10-23.chrxp       │
│                                                                  │
│  File size: 1.2 MB                                              │
│                                                                  │
│  Keep this backup safe—it contains your entire journal history. │
│                                                                  │
│  [Open Folder] [Export Another] [Close]                         │
└──────────────────────────────────────────────────────────────────┘
```

---

## 4. Import Dialog

**Triggered by:** Settings → Export & Import → [Choose File to Import]

```
┌──────────────────────────────────────────────────────────────────────┐
│  Import Entries                                                   [×]│
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Select file(s) to import:                                            │
│  [Choose Files...]                                                    │
│                                                                       │
│  Supported formats:                                                   │
│  • .chrxp (ChronoXP native)                                           │
│  • .json (Day One, Journey, ChronoXP JSON)                            │
│  • .md (Markdown files in folder)                                     │
│  • .txt (Plain text files)                                            │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  Import Mode                                                          │
│  ● Merge with existing entries (keep both)                           │
│  ○ Replace all entries (⚠️ delete current, import new)              │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  Duplicate Handling                                                   │
│  [Skip duplicates ▼]                                                  │
│    • Skip duplicates (based on date + title)                         │
│    • Import all (allow duplicates)                                   │
│    • Ask me for each duplicate                                       │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  [Import] [Cancel]                                                    │
└──────────────────────────────────────────────────────────────────────┘
```

**After file selected:**
```
┌──────────────────────────────────────────────────────────────────┐
│  Import Entries                                               [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  File: chronoxp_backup_2025-10-23.chrxp                         │
│  Found: 243 entries, 18 tags                                    │
│                                                                  │
│  ☑ Password-protected — Enter password:                         │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │ ••••••••••••••••                                            │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  [Import] [Cancel]                                               │
└──────────────────────────────────────────────────────────────────┘
```

**Import progress:**
```
┌──────────────────────────────────────────────────────────────────┐
│  Importing Entries...                                         [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Importing 243 entries...                                        │
│  ████████████████████████░░░░░░░░░░░░░░░░ 68% (165 / 243)       │
│                                                                  │
│  • Decrypting entries                                           │
│  • Converting to ChronoXP format                                │
│  • Checking for duplicates                                      │
│  • Writing to database                                          │
│                                                                  │
│  Estimated time remaining: 15 seconds                            │
│                                                                  │
│  [Cancel Import]                                                 │
└──────────────────────────────────────────────────────────────────┘
```

**Import complete:**
```
┌──────────────────────────────────────────────────────────────────┐
│  Import Complete! ✓                                           [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Successfully imported:                                          │
│  • 243 entries (12 duplicates skipped)                          │
│  • 18 tags                                                       │
│  • 45 XP awarded for migrating your journal!                    │
│                                                                  │
│  Your journal history is now safe in ChronoXP. 💙               │
│                                                                  │
│  [View Library] [Close]                                          │
└──────────────────────────────────────────────────────────────────┘
```

---

## 5. Artifact Detail Modal

**Triggered by:** Clicking artifact in Records Cavern

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ╔════════════════════════════════════════════════════════════════╗ │
│  ║ [Hand-painted artifact closeup:                             [×]║ │
│  ║  Worn leather journal with brass clasp,                        ║ │
│  ║  visible brush strokes, warm lighting]                         ║ │
│  ╠════════════════════════════════════════════════════════════════╣ │
│  ║                                                                 ║ │
│  ║ The Wayfarer's Journal                                          ║ │
│  ║ ────────────────────────────────────────────                   ║ │
│  ║                                                                 ║ │
│  ║ "A well-worn companion for those who document their travels    ║ │
│  ║ through life's terrain. The leather bears the marks of         ║ │
│  ║ countless openings, each page a footprint on the path."        ║ │
│  ║                                                                 ║ │
│  ║ ────────────────────────────────────────────                   ║ │
│  ║                                                                 ║ │
│  ║ Unlocked: October 15, 2025                                      ║ │
│  ║ Condition: Write 10 entries in your first month                ║ │
│  ║                                                                 ║ │
│  ║ Related entries: 12 entries around this time                   ║ │
│  ║ [View Related Entries]                                          ║ │
│  ║                                                                 ║ │
│  ║                                         [< Previous] [Next >]   ║ │
│  ╚════════════════════════════════════════════════════════════════╝ │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Interactions:**
- **[View Related Entries]:** Opens Library filtered to entries from unlock timeframe
- **[< Previous] / [Next >]:** Navigate between unlocked artifacts
- **Click outside / Esc:** Close modal

---

## 6. Quest Prompt Modal

**Triggered by:** Random (gentle frequency), when user hasn't written in 3+ days, or milestone reached

```
┌──────────────────────────────────────────────────────────────────┐
│  📜 Reflection Quest                                          [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  "What made you smile today, even if just for a moment?"        │
│                                                                  │
│  Write a short reflection to earn bonus XP (+10 XP).            │
│                                                                  │
│  [Start Writing] [Maybe Later] [Don't Show Again]               │
└──────────────────────────────────────────────────────────────────┘
```

**Quest completed:**
```
┌──────────────────────────────────────────────────────────────────┐
│  Quest Complete! ✓                                            [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  You reflected on "What made you smile today?"                  │
│                                                                  │
│  Reward: +10 XP                                                  │
│                                                                  │
│  These small reflections build self-awareness over time. 💙     │
│                                                                  │
│  [Close]                                                         │
└──────────────────────────────────────────────────────────────────┘
```

---

## 7. Artifact Unlock Toast (Non-Modal)

**Triggered by:** Earning enough XP to unlock artifact

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ October 23, 2025                              ✓ Saved (1:23) │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │                                                               │  │
│  │ Today was another good day of journaling...                  │  │
│  │                                                               │  │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 🎉 Artifact Unlocked!                                    [×]│   │
│  │ ─────────────────────────────────────────────────────────   │   │
│  │                                                              │   │
│  │ [Artifact thumbnail] The Wayfarer's Journal                  │   │
│  │                                                              │   │
│  │ "A well-worn companion for those who document..."           │   │
│  │                                                              │   │
│  │ [View in Cavern] [Close]                                     │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Auto-dismisses after 5 seconds if not interacted with.**

---

## 8. Unsaved Changes Warning

**Triggered by:** Trying to close app or navigate away with unsaved changes

```
┌──────────────────────────────────────────────────────────────────┐
│  Unsaved Changes                                              [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  You have unsaved changes in your current entry.                │
│                                                                  │
│  [Save and Close] [Don't Save] [Cancel]                         │
└──────────────────────────────────────────────────────────────────┘
```

**Keyboard shortcuts:**
- **Enter / S:** Save and close
- **D:** Don't save
- **Esc / C:** Cancel

---

## 9. Delete Entry Confirmation

**Triggered by:** Delete key in Library, context menu → Delete

```
┌──────────────────────────────────────────────────────────────────┐
│  Delete Entry?                                                [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Are you sure you want to delete this entry?                    │
│                                                                  │
│  Entry: "October 23, 2025 — Today was a reflective day..."     │
│  Tags: reflection, goals                                        │
│  Word count: 342 words                                          │
│                                                                  │
│  This action cannot be undone.                                  │
│                                                                  │
│  [Export First] [Delete] [Cancel]                               │
└──────────────────────────────────────────────────────────────────┘
```

**Bulk delete (multiple entries):**
```
┌──────────────────────────────────────────────────────────────────┐
│  Delete Multiple Entries?                                     [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  You are about to delete 12 entries.                            │
│                                                                  │
│  ⚠️ This action cannot be undone!                               │
│                                                                  │
│  We strongly recommend exporting a backup first.                │
│                                                                  │
│  [Export Backup First] [Delete Anyway] [Cancel]                 │
└──────────────────────────────────────────────────────────────────┘
```

---

## 10. Clear All Data Confirmation

**Triggered by:** Settings → Privacy & Data → [Clear All Data]

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Clear All Data?                                           [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  This will PERMANENTLY DELETE:                                  │
│  • All 156 entries                                              │
│  • All 24 tags                                                  │
│  • All XP and artifacts (842 XP, 12 artifacts unlocked)         │
│  • All settings (except app preferences)                        │
│                                                                  │
│  This action CANNOT be undone!                                  │
│                                                                  │
│  To confirm, type "DELETE ALL" below:                           │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │ ________________________________                            │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  [Export Backup First] [Clear All Data] [Cancel]                │
└──────────────────────────────────────────────────────────────────┘
```

**Validation:**
- **[Clear All Data]** button disabled until user types "DELETE ALL" exactly

---

## 11. Pro Upgrade Modal

**Triggered by:** Clicking locked AI feature, [Upgrade to Pro] buttons

```
┌──────────────────────────────────────────────────────────────────────┐
│  Unlock AI Features                                               [×]│
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ChronoXP AI Concierge — $1.99 one-time                              │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  Unlock three AI companions that run entirely on your device:        │
│                                                                       │
│  💡 Muse                                                              │
│     Gentle writing suggestions when you're stuck or need             │
│     a prompt to continue.                                            │
│                                                                       │
│  🏷️ Archivist                                                         │
│     Automatic tag suggestions based on your entry content,           │
│     making organization effortless.                                  │
│                                                                       │
│  🔍 Concierge                                                         │
│     Semantic search—find entries by meaning, not just keywords.      │
│     ("Show me entries about feeling hopeful")                        │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  ✓ All AI runs locally (no cloud, no tracking)                       │
│  ✓ One-time purchase—yours forever                                   │
│  ✓ Even if you cancel subscriptions later, AI stays unlocked         │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  [Purchase for $1.99] [Learn More] [Maybe Later]                     │
│                                                                       │
│  30-day money-back guarantee, no questions asked.                    │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

**After purchase:**
```
┌──────────────────────────────────────────────────────────────────┐
│  Welcome to Pro! 🎉                                           [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Your AI companions are now unlocked!                           │
│                                                                  │
│  • Muse (writing suggestions)                                   │
│  • Archivist (auto-tagging)                                     │
│  • Concierge (semantic search)                                  │
│                                                                  │
│  Downloading AI models (2.8 GB)...                              │
│  ████████████████████████░░░░░░░░░░░░░░░░ 68% (1.6 GB / 2.8 GB)│
│                                                                  │
│  [Continue Writing] [Cancel Download]                           │
└──────────────────────────────────────────────────────────────────┘
```

---

## 12. Subscription Upgrade Modal (Level 1/2)

**Triggered by:** Clicking locked Cavern features, [Upgrade] buttons

```
┌──────────────────────────────────────────────────────────────────────┐
│  Upgrade Your ChronoXP Experience                                 [×]│
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Extras Level 1 — $3.33/month                                        │
│  ────────────────────────────────────────────────────────────────    │
│  ✓ AI Concierge (Muse, Archivist, Concierge)                         │
│  ✓ Records Cavern (gamification with hand-painted artifacts)         │
│  ✓ Export encryption (password-protect backups)                      │
│  ✓ Priority support (email response within 24 hours)                 │
│                                                                       │
│  Everything Level 2 — $4.44/month                                    │
│  ────────────────────────────────────────────────────────────────    │
│  ✓ Everything in Level 1                                             │
│  ✓ Advanced analytics (writing patterns, mood trends)                │
│  ✓ Theme customization (create your own color schemes)               │
│  ✓ Future features (LAN sync, mobile apps)                           │
│  ✓ Vote on roadmap priorities                                        │
│                                                                       │
│  Annual Package — $11.11/year (79% savings!)                         │
│  ────────────────────────────────────────────────────────────────    │
│  ✓ Everything in Level 2                                             │
│  ✓ Save $42.17/year vs monthly pricing                               │
│                                                                       │
│  Lifetime — $33.30 one-time                                           │
│  ────────────────────────────────────────────────────────────────    │
│  ✓ Everything forever                                                │
│  ✓ Never pay again                                                   │
│                                                                       │
│  ────────────────────────────────────────────────────────────────    │
│                                                                       │
│  [Choose Level 1] [Choose Level 2] [Choose Annual] [Choose Lifetime] │
│                                                                       │
│  [Compare All Tiers] [Maybe Later]                                   │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 13. Keyboard Shortcuts Overlay

**Triggered by:** Ctrl+? or Help → Keyboard Shortcuts

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│  ╔════════════════════════════════════════════════════════════════╗ │
│  ║ Keyboard Shortcuts                                          [×]║ │
│  ╠════════════════════════════════════════════════════════════════╣ │
│  ║                                                                 ║ │
│  ║ Global                                                          ║ │
│  ║ ──────────────────────────────────────────────────────         ║ │
│  ║ Ctrl+K              Command Bar                                ║ │
│  ║ Ctrl+N              New Entry                                  ║ │
│  ║ Ctrl+F              Search Entries                             ║ │
│  ║ Ctrl+S              Save Entry (manual)                        ║ │
│  ║ Ctrl+?              Show Keyboard Shortcuts                    ║ │
│  ║ Esc                 Close Modal / Cancel                       ║ │
│  ║                                                                 ║ │
│  ║ Writing (Canvas)                                                ║ │
│  ║ ──────────────────────────────────────────────────────         ║ │
│  ║ Ctrl+B              Bold                                        ║ │
│  ║ Ctrl+I              Italic                                      ║ │
│  ║ Ctrl+T              Add Tag                                     ║ │
│  ║ Tab                 Accept Muse Suggestion                      ║ │
│  ║ Ctrl+[              Previous Entry                              ║ │
│  ║ Ctrl+]              Next Entry                                  ║ │
│  ║ F11                 Toggle Fullscreen Mode                      ║ │
│  ║                                                                 ║ │
│  ║ Library                                                          ║ │
│  ║ ──────────────────────────────────────────────────────         ║ │
│  ║ Arrow Keys          Navigate Calendar/List                     ║ │
│  ║ Enter               Open Selected Entry                        ║ │
│  ║ Delete              Delete Entry                               ║ │
│  ║ Ctrl+P              Pin Entry                                   ║ │
│  ║ Ctrl+E              Export Entry                               ║ │
│  ║                                                                 ║ │
│  ║ [Print Cheat Sheet] [Close]                                     ║ │
│  ╚════════════════════════════════════════════════════════════════╝ │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 14. Update Available Notification

**Triggered by:** Auto-update check (background)

```
┌──────────────────────────────────────────────────────────────────┐
│  Update Available                                             [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP v1.1.0 is now available!                              │
│                                                                  │
│  What's new:                                                     │
│  • New artifacts: The Cartographer's Compass                    │
│  • Improved search performance (50% faster)                     │
│  • Bug fixes and stability improvements                         │
│                                                                  │
│  [View Release Notes]                                            │
│                                                                  │
│  [Download Update] [Remind Me Later]                            │
└──────────────────────────────────────────────────────────────────┘
```

---

## Accessibility Notes

### Keyboard Navigation
- **Tab:** Navigate between interactive elements
- **Enter:** Activate button / select item
- **Esc:** Close modal (except confirmations requiring explicit action)
- **Arrow keys:** Navigate lists, dropdown menus

### Screen Reader
- **Modal title:** Announced when modal opens ("Tag Picker Modal")
- **Button labels:** Clear actions ("Save and Close", not just "OK")
- **Error states:** Announced immediately ("Passwords do not match")

### Focus Management
- When modal opens, focus goes to first interactive element (search field, button)
- When modal closes, focus returns to trigger element (button that opened modal)
- **Focus trap:** Tab cannot leave modal while open

---

## Performance Targets

- **Modal open:** <50ms (smooth animation)
- **Tag picker search:** <16ms per keystroke (instant feel)
- **Export dialog render:** <100ms
- **Import progress:** Update every 500ms (smooth progress bar)

---

## Developer Notes

### Tech Stack
- **Modals:** React portals with `<dialog>` element (native HTML5)
- **Backdrop:** Semi-transparent overlay with `backdrop-filter: blur(4px)`
- **Animations:** CSS transitions (200ms ease-out)
- **Confirmation dialogs:** Require explicit user action (no auto-dismiss)

### Modal State Management
```typescript
interface ModalState {
  type: 'tag-picker' | 'export' | 'import' | 'confirm' | 'artifact-detail';
  props: Record<string, any>;
  onClose: () => void;
}
```

### Keyboard Trap (Accessibility)
```typescript
// Trap Tab focus within modal
const focusableElements = modal.querySelectorAll('button, input, [tabindex="0"]');
const firstElement = focusableElements[0];
const lastElement = focusableElements[focusableElements.length - 1];

lastElement.addEventListener('keydown', (e) => {
  if (e.key === 'Tab' && !e.shiftKey) {
    e.preventDefault();
    firstElement.focus();
  }
});
```

---

**End of Modals & Overlays Wireframe**
