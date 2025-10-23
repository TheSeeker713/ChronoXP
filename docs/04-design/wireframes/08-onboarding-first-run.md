# Wireframe: Onboarding & First Run Experience

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

The first-run experience introduces new users to ChronoXP, guides them through vault setup, and optionally tours key features. Designed to be welcoming, non-overwhelming, and skippable.

---

## 1. Welcome Screen (First Launch)

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│                                                                       │
│                        [Hand-painted journal                          │
│                         illustration with                             │
│                         quill and candle]                             │
│                                                                       │
│                      Welcome to ChronoXP™                             │
│                                                                       │
│          A private, offline journaling companion that's               │
│              yours to keep—no cloud, no tracking.                     │
│                                                                       │
│                                                                       │
│                       [Get Started]                                   │
│                                                                       │
│                    [Import from Another App]                          │
│                                                                       │
│                                                                       │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Interaction:**
- **[Get Started]:** Proceeds to vault setup (Step 2)
- **[Import from Another App]:** Jumps to import wizard (see Export & Import section)

---

## 2. Vault Setup: Choose Security Level

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  [Step 1 of 3: Set Up Your Vault]                               [×] │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  How would you like to secure your journal?                          │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ ○ Device-Only Encryption (Recommended for most users)       │   │
│  │                                                               │   │
│  │   Your journal is encrypted automatically with a key         │   │
│  │   stored on your device. No password needed.                 │   │
│  │                                                               │   │
│  │   ✓ Quick and convenient                                     │   │
│  │   ✓ Still fully encrypted (AES-256)                          │   │
│  │   ⚠ If someone accesses your device, they can read your     │   │
│  │     journal (like any local files)                           │   │
│  │                                                               │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ ● Password Protection (Extra security)                       │   │
│  │                                                               │   │
│  │   Require a password every time you open ChronoXP.           │   │
│  │                                                               │   │
│  │   ✓ Maximum privacy                                          │   │
│  │   ✓ Protected even if device is compromised                  │   │
│  │   ⚠ If you forget your password, your entries are           │   │
│  │     permanently inaccessible (no recovery!)                  │   │
│  │                                                               │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  You can change this later in Settings → Privacy & Data.             │
│                                                                       │
│  [Back] [Next]                                                        │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Branch Logic:**
- **Device-Only Encryption:** Skip to Step 3 (AI setup)
- **Password Protection:** Proceed to Step 2B (password creation)

---

## 3. Vault Setup: Create Password (If Password Protection Selected)

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  [Step 2 of 3: Create Your Password]                            [×] │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Create a password to protect your journal.                          │
│                                                                       │
│  Password:                                                            │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │ ••••••••••••••••                                                │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                       │
│  Confirm password:                                                    │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │ ••••••••••••••••                                                │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                       │
│  Password strength: ████████░░ Strong                                │
│                                                                       │
│  ⚠️ IMPORTANT: ChronoXP cannot recover your password!               │
│  • Write it down and keep it safe.                                   │
│  • If you forget it, your journal is permanently locked.             │
│  • There is no "forgot password" option—your privacy is absolute.   │
│                                                                       │
│  Password hint (optional, stored unencrypted):                       │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │ Example: "Childhood pet + birth year"                           │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                       │
│  ☐ I understand my password cannot be recovered.                     │
│                                                                       │
│  [Back] [Create Vault]                                                │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Validation:**
- **Password strength:** Real-time feedback (Weak / Moderate / Strong / Very Strong)
- **Match check:** "Passwords do not match" error if mismatch
- **Checkbox required:** [Create Vault] button disabled until checkbox checked

---

## 4. AI Setup: Download Models (Optional)

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  [Step 3 of 3: AI Features (Optional)]                          [×] │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ChronoXP includes AI companions to enhance your journaling:         │
│                                                                       │
│  • Muse: Gentle writing suggestions when you're stuck                │
│  • Archivist: Auto-tags entries for easy searching                   │
│  • Concierge: Natural language search ("show me entries about hope") │
│                                                                       │
│  All AI runs locally on your device. Nothing leaves your computer.   │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ ● Download AI models now (~2.8 GB)                          │   │
│  │                                                               │   │
│  │   ✓ AI features ready immediately                            │   │
│  │   ⚠ Requires 2.8 GB disk space                               │   │
│  │   ⚠ Download may take 5-10 minutes                           │   │
│  │                                                               │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ ○ Skip for now (I'll download later)                         │   │
│  │                                                               │   │
│  │   You can download AI models anytime from Settings →         │   │
│  │   AI Features.                                                │   │
│  │                                                               │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  ℹ️ AI features require a $1.99 one-time purchase (Pro tier).       │
│  Try ChronoXP for free first, then upgrade when you're ready.       │
│                                                                       │
│  [Back] [Download Now] [Skip for Now]                                │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Branch Logic:**
- **[Download Now]:** Show download progress screen (Step 4A)
- **[Skip for Now]:** Jump to feature tour (Step 5)

---

## 5. AI Model Download Progress

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  [Downloading AI Models...]                                      [×] │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Downloading Phi-3-mini-4k-instruct (2.3 GB)                          │
│  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░ 68% (1.6 GB / 2.3 GB)  │
│  Estimated time remaining: 2 minutes                                  │
│                                                                       │
│  Downloading nomic-embed-text-v1.5 (500 MB)                           │
│  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0% (Queued)          │
│                                                                       │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  While you wait...                                                    │
│                                                                       │
│  ✓ ChronoXP is fully offline—no cloud, no tracking                   │
│  ✓ Your entries are encrypted with AES-256                           │
│  ✓ AI runs entirely on your device (private by design)               │
│                                                                       │
│  [Cancel Download]                                                    │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**On Completion:**
```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  [Download Complete!]                                            [×] │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ✓ AI models successfully downloaded (2.8 GB)                        │
│                                                                       │
│  Your AI companions are ready:                                        │
│  • Muse (writing suggestions)                                        │
│  • Archivist (auto-tagging)                                          │
│  • Concierge (semantic search)                                       │
│                                                                       │
│  Ready to start journaling?                                           │
│                                                                       │
│  [Take a Quick Tour] [Start Writing]                                 │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 6. Feature Tour (Interactive Walkthrough)

### Tour Step 1: Canvas / Writing Surface

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ October 23, 2025                              ● Autosaving... │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │                                                               │  │
│  │ Want a soft start? Try a one-line check-in.                  │  │
│  │                                                               │  │
│  │                                                               │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
│  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓  │
│  ┃ 📘 Welcome to ChronoXP!                                      ┃  │
│  ┃ ────────────────────────────────────────────────────────     ┃  │
│  ┃                                                               ┃  │
│  ┃ This is your Canvas—your private writing space.              ┃  │
│  ┃                                                               ┃  │
│  ┃ Just start typing. Your work is saved automatically          ┃  │
│  ┃ as you write, encrypted on your device.                      ┃  │
│  ┃                                                               ┃  │
│  ┃ Press Ctrl+N to start a new entry anytime.                   ┃  │
│  ┃                                                               ┃  │
│  ┃                                          [Next] [Skip Tour]   ┃  │
│  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛  │
│                                                                       │
│  [1 of 5]                                                             │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Tour Step 2: Muse AI Suggestions

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ October 23, 2025                              ✓ Saved (1:23) │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │                                                               │  │
│  │ Today was interesting. I learned something new about myself. │  │
│  │                                                               │  │
│  │ ┌─────────────────────────────────────────────────────────┐ │  │
│  │ │ 💡 Muse suggests: "What did you discover?"              │ │  │
│  │ │ Press Tab to accept, Esc to dismiss                     │ │  │
│  │ └─────────────────────────────────────────────────────────┘ │  │
│  │                                                               │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
│  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓  │
│  ┃ 📘 Meet Muse, Your Writing Companion                         ┃  │
│  ┃ ────────────────────────────────────────────────────────     ┃  │
│  ┃                                                               ┃  │
│  ┃ Muse offers gentle prompts when you pause—never pushy,       ┃  │
│  ┃ just helpful nudges to keep your thoughts flowing.           ┃  │
│  ┃                                                               ┃  │
│  ┃ Press Tab to accept a suggestion, or just keep writing!      ┃  │
│  ┃                                                               ┃  │
│  ┃ (You can disable Muse anytime in Settings.)                  ┃  │
│  ┃                                                               ┃  │
│  ┃                              [Back] [Next] [Skip Tour]        ┃  │
│  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛  │
│                                                                       │
│  [2 of 5]                                                             │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Tour Step 3: Library / Finding Entries

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Library                                    [Calendar] [Timeline] [Tags]│
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  October 2025                                  [🔍 Filter entries...] │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │ Sun  Mon  Tue  Wed  Thu  Fri  Sat                              │ │
│  │                  1    2    3    4    5                          │ │
│  │  6    7    8    9   10   11   12                               │ │
│  │ 13   14   15   16   17   18   19                               │ │
│  │ 20   21   22   23●  24   25   26                               │ │
│  │ 27   28   29   30   31                                         │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                       │
│  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓  │
│  ┃ 📘 The Library: Your Memory Archive                          ┃  │
│  ┃ ────────────────────────────────────────────────────────     ┃  │
│  ┃                                                               ┃  │
│  ┃ Browse your entries by date (Calendar), timeline, or tags.   ┃  │
│  ┃                                                               ┃  │
│  ┃ Press Ctrl+F to search all your entries instantly.           ┃  │
│  ┃                                                               ┃  │
│  ┃ Archivist AI automatically suggests tags as you write!       ┃  │
│  ┃                                                               ┃  │
│  ┃                              [Back] [Next] [Skip Tour]        ┃  │
│  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛  │
│                                                                       │
│  [3 of 5]                                                             │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Tour Step 4: Records Cavern (Gamification)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Records Cavern                                    XP: 0 / 50        │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  [Hand-painted stone chamber illustration:                          │
│   Small cave with single candle, worn journal on wooden desk,        │
│   empty shelves waiting for artifacts]                               │
│                                                                       │
│  Your journey begins...                                               │
│                                                                       │
│  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓  │
│  ┃ 📘 The Records Cavern: Gentle Gamification                   ┃  │
│  ┃ ────────────────────────────────────────────────────────     ┃  │
│  ┃                                                               ┃  │
│  ┃ As you journal, you'll earn XP and unlock hand-painted       ┃  │
│  ┃ artifacts that appear in your growing collection.            ┃  │
│  ┃                                                               ┃  │
│  ┃ This space evolves with you—from a humble study to a         ┃  │
│  ┃ grand sanctum filled with memories.                          ┃  │
│  ┃                                                               ┃  │
│  ┃ Not into gamification? Enable "Simple Mode" in Settings      ┃  │
│  ┃ to hide this tab entirely. Your choice!                      ┃  │
│  ┃                                                               ┃  │
│  ┃                              [Back] [Next] [Skip Tour]        ┃  │
│  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛  │
│                                                                       │
│  [4 of 5]                                                             │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Tour Step 5: Command Bar

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ╔══════════════════════════════════════════════════════════════╗  │
│  ║ Type a command or search entries...                          ║  │
│  ╠══════════════════════════════════════════════════════════════╣  │
│  ║ 📝 New Entry                                      Ctrl+N     ║  │
│  ║ 🔍 Search Entries                                 Ctrl+F     ║  │
│  ║ 📤 Export All Entries                                        ║  │
│  ║ ⚙️ Open Settings                                             ║  │
│  ║ 🎨 Change Theme                                              ║  │
│  ║ ⌨️ View Keyboard Shortcuts                      Ctrl+?     ║  │
│  ╚══════════════════════════════════════════════════════════════╝  │
│                                                                       │
│  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓  │
│  ┃ 📘 Command Bar: Your Keyboard Superpower                     ┃  │
│  ┃ ────────────────────────────────────────────────────────     ┃  │
│  ┃                                                               ┃  │
│  ┃ Press Ctrl+K (or Cmd+K on Mac) to access the Command Bar.   ┃  │
│  ┃                                                               ┃  │
│  ┃ Quickly navigate anywhere, run commands, or search entries   ┃  │
│  ┃ without touching your mouse.                                 ┃  │
│  ┃                                                               ┃  │
│  ┃ Power users, this is your home base. ⚡                      ┃  │
│  ┃                                                               ┃  │
│  ┃                              [Back] [Finish Tour]             ┃  │
│  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛  │
│                                                                       │
│  [5 of 5]                                                             │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Tour Complete

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│                                                                       │
│                        [Confetti illustration]                        │
│                                                                       │
│                      You're All Set! 🎉                               │
│                                                                       │
│                 Your private journal is ready.                        │
│                                                                       │
│                                                                       │
│                       [Start Writing]                                 │
│                                                                       │
│                     [Replay Tour] [Close]                             │
│                                                                       │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 7. Skipping the Tour

If user clicks **[Skip Tour]** at any point:

```
┌──────────────────────────────────────────────────────────────────┐
│  Skip Tour?                                                   [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  You can always replay the tour from Settings → About →        │
│  "Replay Feature Tour".                                         │
│                                                                  │
│  [Continue Tour] [Skip to Canvas]                               │
└──────────────────────────────────────────────────────────────────┘
```

---

## 8. Import from Another App (Alternative Path)

**From Welcome Screen → [Import from Another App]:**

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  [Import from Another App]                                       [×] │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Bring your existing journal entries into ChronoXP.                  │
│                                                                       │
│  Supported formats:                                                   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ • Day One (JSON export)                                      │   │
│  │ • Journey (JSON export)                                      │   │
│  │ • Markdown files (.md folder)                                │   │
│  │ • Plain text files (.txt)                                    │   │
│  │ • ChronoXP (.chrxp backup)                                   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  [Choose Files to Import]                                             │
│                                                                       │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Or start fresh:                                                      │
│  [Create Empty Journal]                                               │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**After file selected:**

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  [Importing Entries...]                                          [×] │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Found: 243 entries from Day One export                               │
│                                                                       │
│  Importing entries...                                                 │
│  ████████████████████████░░░░░░░░░░░░░░░░░░░ 68% (165 / 243)         │
│                                                                       │
│  • Converting entries to ChronoXP format                             │
│  • Preserving tags and timestamps                                    │
│  • Encrypting database                                               │
│                                                                       │
│  Estimated time remaining: 30 seconds                                 │
│                                                                       │
│  [Cancel Import]                                                      │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**On completion:**

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  [Import Complete!]                                              [×] │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ✓ Successfully imported 243 entries                                 │
│  ✓ Preserved 18 tags                                                 │
│  ✓ Database encrypted and ready                                      │
│                                                                       │
│  Your journal history is now safe in ChronoXP.                       │
│                                                                       │
│  [Take a Quick Tour] [Start Writing]                                 │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 9. First Entry Celebration (After User Writes First Entry)

**Triggered after first autosave:**

```
┌──────────────────────────────────────────────────────────────────┐
│  🎉 First Entry Complete!                                     [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  You've started your journaling journey with ChronoXP!          │
│                                                                  │
│  ✓ Entry saved and encrypted                                    │
│  ✓ +5 XP earned (First Entry achievement)                       │
│                                                                  │
│  Keep going—your future self will thank you. 💙                 │
│                                                                  │
│  [View in Library] [Write Another] [Close]                      │
└──────────────────────────────────────────────────────────────────┘
```

---

## 10. Vault Unlock Screen (Subsequent Launches, If Password Protected)

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│                                                                       │
│                                                                       │
│                      [ChronoXP Logo]                                  │
│                                                                       │
│                   Welcome back to ChronoXP™                           │
│                                                                       │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │ Enter your vault password:                                      │ │
│  │ ┌────────────────────────────────────────────────────────────┐ │ │
│  │ │ ••••••••••••                                                │ │ │
│  │ └────────────────────────────────────────────────────────────┘ │ │
│  │                                                                 │ │
│  │ [Unlock]                                                        │ │
│  │                                                                 │ │
│  │ Password hint: Childhood pet + birth year                      │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                       │
│  [Forgot Password?]                                                   │
│                                                                       │
│                                                                       │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**After 3 failed attempts:**

```
┌──────────────────────────────────────────────────────────────────┐
│  Too Many Failed Attempts                                     [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Your password is incorrect. For security, ChronoXP will close. │
│                                                                  │
│  If you've forgotten your password:                             │
│  • Your entries cannot be recovered (encryption is absolute)    │
│  • You can delete the vault and start fresh (deletes all data)  │
│                                                                  │
│  [Try Again] [Delete Vault and Start Over] [Close App]          │
└──────────────────────────────────────────────────────────────────┘
```

---

## 11. Pro Upgrade Prompt (Optional, After 7 Days)

**Gentle prompt after 7 days of use (if Free tier):**

```
┌──────────────────────────────────────────────────────────────────┐
│  Loving ChronoXP? ✨                                          [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  You've written 12 entries in the past week—nice work!          │
│                                                                  │
│  Unlock AI companions for just $1.99 one-time:                  │
│  • Muse: Writing suggestions when you're stuck                  │
│  • Archivist: Auto-tagging for easy organization                │
│  • Concierge: Search by meaning, not just keywords              │
│                                                                  │
│  All AI runs locally. Your privacy stays absolute. 🔒           │
│                                                                  │
│  [Upgrade to Pro ($1.99)] [Maybe Later]                         │
│                                                                  │
│  ☐ Don't show this again                                         │
└──────────────────────────────────────────────────────────────────┘
```

---

## Accessibility Notes

### Keyboard Navigation
- **Tab:** Navigate between buttons/inputs
- **Enter:** Activate button
- **Esc:** Close modal or skip current tour step
- **Arrow keys (tour):** Navigate forward/backward in feature tour

### Screen Reader
- **Tour steps:** Announced as "Step 1 of 5: Canvas"
- **Progress indicators:** "Downloading, 68% complete, 1.6 gigabytes of 2.3 gigabytes"
- **Success states:** "Import complete. 243 entries imported successfully."

### Focus Management
- When onboarding starts, focus goes to first interactive element ([Get Started])
- When tour step opens, focus goes to [Next] button
- When password input shown, focus goes to password field

---

## Performance Targets

- **Welcome screen render:** <100ms
- **Vault creation:** <500ms (without AI download)
- **AI model download:** 5-10 minutes (dependent on network, ~2.8 GB)
- **Import entries:** <5s for 1000 entries (with progress indicator)
- **Tour step transitions:** <50ms (smooth animations)

---

## Developer Notes

### Tech Stack
- **Onboarding state:** Store in SQLite `settings` table (`onboarding_completed: true`)
- **Tour overlay:** React portal with z-index over app content
- **Password validation:** Rust backend (SQLCipher key derivation)
- **AI download:** Tauri HTTP client streaming to disk with progress events

### Onboarding Flags
```sql
CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO settings (key, value) VALUES
  ('onboarding_completed', 'false'),
  ('tour_completed', 'false'),
  ('first_entry_celebrated', 'false'),
  ('pro_prompt_shown', 'false');
```

### Skipping Onboarding (Debug Mode)
```bash
# For development: skip onboarding entirely
echo 'UPDATE settings SET value = "true" WHERE key = "onboarding_completed";' | sqlite3 vault.db
```

---

**End of Onboarding & First Run Wireframe**
