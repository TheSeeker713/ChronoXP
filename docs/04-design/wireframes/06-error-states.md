# Wireframe: Error States & Empty States

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

This document covers all error handling, empty states, edge cases, and recovery flows across the entire application.

---

## 1. Empty States

### 1.1 No Entries Yet (First Launch)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│                         📝                                           │
│                  Welcome to ChronoXP™ Journal                        │
│                                                                       │
│          Your private, offline journaling companion.                 │
│                                                                       │
│            Ready to start your first entry?                          │
│                                                                       │
│                      [+ New Entry]                                   │
│                                                                       │
│          Or explore: [Take Tour] [Import Entries] [Settings]         │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Actions:**
- **[+ New Entry]:** Opens blank Canvas
- **[Take Tour]:** Interactive feature walkthrough (future)
- **[Import Entries]:** Opens import dialog for .chrxp files
- **[Settings]:** Opens settings (for theme, font adjustments before writing)

---

### 1.2 Search No Results

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
│     │  • Use semantic search (Pro) to find by meaning       │     │
│     │  • Browse all entries in Library                      │     │
│     │                                                         │     │
│     │       [Try Semantic Search]  [Go to Library]           │     │
│     │                                                         │     │
│     └─────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

---

### 1.3 Library Empty (All Entries Deleted)

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│                         📚                                           │
│                    Your Library is empty                             │
│                                                                       │
│         All entries have been deleted or moved.                      │
│                                                                       │
│                      [+ New Entry]                                   │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

### 1.4 No Tags Yet

```
┌─────────────────────────────────────────────────────────────────────┐
│  Library                       [Calendar] [Timeline] [Tags]          │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ┌───────────────────────┐  ┌────────────────────────────────────┐ │
│  │ All Tags (0)          │  │                                    │ │
│  │ ──────────────────    │  │         🏷️                         │ │
│  │                       │  │    No tags yet                     │ │
│  │ No tags created yet.  │  │                                    │ │
│  │                       │  │  Tags help organize your entries.  │ │
│  │ [+ Create New Tag]    │  │  Add tags while writing or from   │ │
│  │                       │  │  the Library.                      │ │
│  └───────────────────────┘  └────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

---

### 1.5 Records Cavern at Level 0 (0 XP)

```
┌─────────────────────────────────────────────────────────────────────┐
│  Records Cavern                                                      │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│     [Empty stone chamber, single torch on wall, no artifacts]        │
│                                                                       │
│                                                                       │
│              🏛️                                                       │
│         Your Cavern awaits                                           │
│                                                                       │
│    Start journaling to unlock your first artifact!                  │
│                                                                       │
│                      [Start Writing]                                 │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Progress to first artifact:  ░░░░░░░░░░ 0%  (10 XP needed)   │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Error States

### 2.1 Autosave Failed

```
┌─────────────────────────────────────────────────────────────────────┐
│  ⚠️ Failed to save entry  [Retry] [Save Manually]  |  Word count: 97│
└─────────────────────────────────────────────────────────────────────┘
```

**Modal (if retry fails repeatedly):**
```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Save Error                                                [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP couldn't save your entry. This might be due to:       │
│                                                                  │
│  • Database locked (another process using file)                 │
│  • Disk full (check available storage)                          │
│  • File permissions error                                       │
│                                                                  │
│  Your entry is temporarily saved in memory. What would you      │
│  like to do?                                                    │
│                                                                  │
│  [Retry Save] [Export to .txt] [Copy to Clipboard] [Close App] │
│                                                                  │
│  Note: If you close the app without saving, this entry will    │
│  be lost.                                                       │
└──────────────────────────────────────────────────────────────────┘
```

---

### 2.2 Database Locked (Vault Password)

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

**Failed Password (3 attempts):**
```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Incorrect Password                                         [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  You've entered an incorrect password 3 times.                  │
│                                                                  │
│  To protect your data, the app will now close. If you've       │
│  forgotten your password, you'll need to restore from a        │
│  backup or reset the vault (this will delete all entries).     │
│                                                                  │
│  [Close App] [Learn About Recovery]                            │
└──────────────────────────────────────────────────────────────────┘
```

---

### 2.3 Database Corruption Detected

```
┌──────────────────────────────────────────────────────────────────┐
│  🛠️ Database Error                                              [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP detected corruption in your journal database.         │
│                                                                  │
│  This can happen due to:                                        │
│  • Unexpected app crash                                         │
│  • Disk errors                                                  │
│  • Power failure during save                                    │
│                                                                  │
│  What would you like to do?                                     │
│                                                                  │
│  [Try Automatic Repair]  (Recommended)                          │
│  [Restore from Backup]   (If you have a .chrxp backup)          │
│  [Export Recoverable Data] (Save what can be rescued)           │
│                                                                  │
│  Warning: Repairing may result in data loss. Always keep       │
│  backups of your journal.                                       │
└──────────────────────────────────────────────────────────────────┘
```

**Repair in Progress:**
```
┌──────────────────────────────────────────────────────────────────┐
│  🛠️ Repairing Database...                                          │
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP is attempting to repair your database.                │
│                                                                  │
│  ████████████░░░░░░░░ 60%                                        │
│                                                                  │
│  This may take several minutes. Please don't close the app.     │
│                                                                  │
│  Checking entries... 142/156 recovered                          │
└──────────────────────────────────────────────────────────────────┘
```

**Repair Complete:**
```
┌──────────────────────────────────────────────────────────────────┐
│  ✓ Repair Complete                                            [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Your database has been repaired!                               │
│                                                                  │
│  Recovered: 142 entries                                         │
│  Lost: 14 entries (corrupted beyond recovery)                   │
│                                                                  │
│  Recommendation: Export your journal as a backup now.           │
│                                                                  │
│  [Export Backup] [View Recovered Entries] [Close]              │
└──────────────────────────────────────────────────────────────────┘
```

---

### 2.4 AI Model Missing (Pro Feature)

```
┌──────────────────────────────────────────────────────────────────┐
│  🤖 AI Model Not Found                                         [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP AI features require local language models.            │
│                                                                  │
│  Required models:                                               │
│  • Phi-3-mini-4k-instruct (~2.3 GB)                             │
│  • nomic-embed-text-v1.5 (~500 MB)                              │
│                                                                  │
│  Total download: ~2.8 GB                                        │
│                                                                  │
│  [Download Models] [Learn More] [Use Without AI]               │
│                                                                  │
│  Note: Models run entirely offline on your device. No          │
│  internet connection required after download.                  │
└──────────────────────────────────────────────────────────────────┘
```

**Download in Progress:**
```
┌──────────────────────────────────────────────────────────────────┐
│  🤖 Downloading AI Models...                              [Cancel]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Phi-3-mini-4k-instruct                                          │
│  ████████████████░░ 85% (1.95 GB / 2.3 GB)                       │
│  Speed: 5.2 MB/s  •  Time remaining: ~1 minute                  │
│                                                                  │
│  nomic-embed-text-v1.5                                           │
│  ░░░░░░░░░░░░░░░░░░ Waiting...                                   │
│                                                                  │
│  Models will be stored in: C:\Users\...\ChronoXP\models\        │
└──────────────────────────────────────────────────────────────────┘
```

---

### 2.5 Disk Space Low

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Low Disk Space                                             [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Your device is running low on storage space.                  │
│                                                                  │
│  Available: 250 MB                                              │
│  Recommended: 1 GB                                              │
│                                                                  │
│  To prevent data loss, free up space or export your journal   │
│  to another drive.                                              │
│                                                                  │
│  [Export Backup] [Open Storage Settings] [Dismiss]             │
└──────────────────────────────────────────────────────────────────┘
```

---

### 2.6 Import Error

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Import Failed                                              [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP couldn't import "journal-backup.chrxp"                │
│                                                                  │
│  Error: Invalid file format or corrupted archive                │
│                                                                  │
│  This might be because:                                         │
│  • File is not a valid .chrxp export                            │
│  • File was created with a newer version of ChronoXP           │
│  • File is encrypted (enter password on import screen)         │
│  • File is corrupted or incomplete                              │
│                                                                  │
│  [Try Again] [Import as Markdown Folder] [Get Help]            │
└──────────────────────────────────────────────────────────────────┘
```

---

### 2.7 Export Error

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Export Failed                                              [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP couldn't export your entries.                         │
│                                                                  │
│  Error: Permission denied (can't write to selected folder)      │
│                                                                  │
│  [Choose Different Location] [Try Again] [Cancel]              │
└──────────────────────────────────────────────────────────────────┘
```

---

### 2.8 Unsaved Changes Warning

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Unsaved Changes                                            [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  You have unsaved changes to the current entry.                 │
│                                                                  │
│  Would you like to save before closing?                         │
│                                                                  │
│  [Save] [Don't Save] [Cancel]                                   │
└──────────────────────────────────────────────────────────────────┘
```

---

### 2.9 Network Error (If LAN Sync Enabled - Future)

```
┌──────────────────────────────────────────────────────────────────┐
│  📶 Sync Error                                                 [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP couldn't sync with other devices on your network.     │
│                                                                  │
│  Error: Connection timeout                                      │
│                                                                  │
│  Check that:                                                    │
│  • Other devices are on and connected to same network          │
│  • Firewall isn't blocking ChronoXP                             │
│  • LAN sync is enabled on other devices                        │
│                                                                  │
│  [Retry Sync] [Disable LAN Sync] [Help]                        │
│                                                                  │
│  Your data is safe. Sync will resume when connection is        │
│  restored.                                                      │
└──────────────────────────────────────────────────────────────────┘
```

---

### 2.10 App Update Available

```
┌──────────────────────────────────────────────────────────────────┐
│  🎉 Update Available                                           [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP v1.2.0 is now available!                              │
│                                                                  │
│  What's new:                                                    │
│  • New artifact: Time Compass                                   │
│  • Improved semantic search accuracy                            │
│  • Bug fixes and performance improvements                       │
│                                                                  │
│  [Download Update] [View Release Notes] [Remind Me Later]      │
│                                                                  │
│  Note: Your data will not be affected by the update.           │
└──────────────────────────────────────────────────────────────────┘
```

---

## 3. Loading States

### 3.1 App Launch Loading

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│                          ░░░░░░░░░░░░░░░░                            │
│                         ChronoXP™ Journal                            │
│                                                                       │
│                          Loading...                                  │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

### 3.2 AI Model Loading (First Use)

```
┌─────────────────────────────────────────────────────────────────────┐
│  ┌────────────────────────────────────────────────────────────┐     │
│  │ 🤖 Initializing AI...                                      │     │
│  │                                                            │     │
│  │ Loading language model into memory. This may take up to   │     │
│  │ 30 seconds on first launch.                               │     │
│  │                                                            │     │
│  │ ████████████████░░ 85%                                     │     │
│  └────────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

---

### 3.3 Searching (Semantic Search)

```
┌─────────────────────────────────────────────────────────────────────┐
│     ┌─────────────────────────────────────────────────────────┐     │
│     │ 🔍  times I felt lonely...                           [×]│     │
│     └─────────────────────────────────────────────────────────┘     │
│     ┌─────────────────────────────────────────────────────────┐     │
│     │                                                         │     │
│     │                   🔍                                    │     │
│     │            Searching with AI...                         │     │
│     │                                                         │     │
│     │     This may take a moment for semantic search.        │     │
│     │                                                         │     │
│     │                   [Cancel]                              │     │
│     │                                                         │     │
│     └─────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Confirmation Dialogs

### 4.1 Delete Entry Confirmation

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Delete Entry?                                              [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Are you sure you want to delete "Reflecting on Travel"?       │
│                                                                  │
│  This action cannot be undone.                                  │
│                                                                  │
│  [Delete] [Cancel]                                              │
└──────────────────────────────────────────────────────────────────┘
```

---

### 4.2 Delete Multiple Entries

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Delete 12 Entries?                                         [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Are you sure you want to delete 12 selected entries?          │
│                                                                  │
│  This action cannot be undone. Consider exporting a backup     │
│  first.                                                         │
│                                                                  │
│  [Export Backup First] [Delete Anyway] [Cancel]                │
└──────────────────────────────────────────────────────────────────┘
```

---

### 4.3 Clear All Data

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚠️ Clear All Data?                                            [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  This will permanently delete:                                  │
│  • All entries (156 entries)                                    │
│  • All tags (24 tags)                                           │
│  • All XP and artifacts                                         │
│  • All settings (except app preferences)                        │
│                                                                  │
│  This action CANNOT be undone.                                  │
│                                                                  │
│  Type "DELETE ALL" to confirm:                                  │
│  ┌────────────────────────┐                                     │
│  │                        │                                     │
│  └────────────────────────┘                                     │
│                                                                  │
│  [Confirm Deletion] [Cancel]                                    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 5. Graceful Degradation

### 5.1 AI Disabled (User Choice or Error)

When AI features unavailable, show fallback UI:

**Muse (disabled):**
- No inline suggestions appear
- AI-related buttons grayed out with tooltip: "AI features disabled"

**Semantic Search (locked):**
- Search modal shows only keyword search
- "Semantic Search (Pro)" tab shows lock icon + upgrade prompt

**Archivist (disabled):**
- No auto-tag suggestions
- Manual tagging still works

---

### 5.2 Offline Mode (Always)

ChronoXP is always offline, but for future LAN sync:

```
┌─────────────────────────────────────────────────────────────┐
│  📶 Offline Mode  |  Word count: 97  |  XP: 250             │
└─────────────────────────────────────────────────────────────┘
```

All features work without internet. Green "Offline" badge indicates intentional privacy.

---

## 6. Recovery Flows

### 6.1 Crash Recovery

On restart after unexpected crash:

```
┌──────────────────────────────────────────────────────────────────┐
│  🛠️ Welcome Back                                               [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  ChronoXP recovered from an unexpected closure.                 │
│                                                                  │
│  Good news: Your data is intact!                                │
│                                                                  │
│  We found an unsaved draft from your last session:              │
│  "Reflecting on Travel" (97 words)                              │
│                                                                  │
│  [Restore Draft] [Discard Draft] [View Details]                │
└──────────────────────────────────────────────────────────────────┘
```

---

### 6.2 Backup Restoration

```
┌──────────────────────────────────────────────────────────────────┐
│  📦 Restore from Backup                                        [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Select a .chrxp backup file to restore:                        │
│                                                                  │
│  [Choose File]  journal-backup-2025-11-15.chrxp                 │
│                                                                  │
│  Backup contains:                                               │
│  • 156 entries                                                  │
│  • 24 tags                                                      │
│  • Created: November 15, 2025                                   │
│                                                                  │
│  ⚠️ Warning: This will replace all current data.                │
│                                                                  │
│  [Restore Backup] [Merge with Current Data] [Cancel]           │
└──────────────────────────────────────────────────────────────────┘
```

---

## Accessibility Notes

### Error Messages
- **ARIA live regions:** Error toasts announced to screen readers
- **Focus management:** Focus moves to error dialog when it appears
- **Keyboard: **Esc closes error modals, Enter confirms primary action

### Color & Contrast
- **Error red:** High contrast (#D32F2F on white, #FF6B6B on dark)
- **Warning amber:** (#F57C00)
- **Success green:** (#388E3C)
- **Icons + text:** Never rely on color alone

---

## Developer Notes

### Error Handling Strategy
1. **Catch errors early:** Validate inputs, wrap database calls in try-catch
2. **Log locally:** Write errors to audit table (never send to server)
3. **User-friendly messages:** Avoid technical jargon ("Database locked" instead of "SQLite error code 5")
4. **Recovery options:** Always offer [Retry], [Export], or [Get Help]
5. **Prevent data loss:** Auto-save drafts, offer export before destructive actions

### Testing Error States
- Simulate disk full: Fill drive, attempt save
- Simulate database corruption: Modify .db file manually
- Simulate crash: Kill process during save operation
- Simulate AI model missing: Delete model files, restart app

---

**End of Error States & Empty States Wireframe**
