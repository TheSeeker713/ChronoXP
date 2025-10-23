# Wireframe: Settings Panel

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

The Settings Panel provides access to all app configuration options, organized into logical categories. Settings persist between sessions and sync across devices (future LAN sync feature).

---

## 1. Settings Panel Layout (Main View)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Settings                                                        [×] │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│  ┌──────────────────┐  ┌────────────────────────────────────────┐  │
│  │ Appearance       │  │ Appearance                             │  │
│  │ AI Features      │  │ ────────────────────────────────────   │  │
│  │ Privacy & Data   │  │                                        │  │
│  │ Keyboard         │  │ Theme                                  │  │
│  │ Accessibility    │  │ ○ Light Mode                           │  │
│  │ Export & Import  │  │ ● Dark Mode                            │  │
│  │ About            │  │ ○ Low-Stimulation (Warm Beige)        │  │
│  │                  │  │                                        │  │
│  │                  │  │ Font Family                            │  │
│  │                  │  │ [System Default ▼]                     │  │
│  │                  │  │                                        │  │
│  │                  │  │ Font Size                              │  │
│  │                  │  │ ◄────●─────► 16px                      │  │
│  │                  │  │                                        │  │
│  │                  │  │ ☑ Enable animations                    │  │
│  │                  │  │ ☐ Enable Simple Mode (hide gamification)│  │
│  │                  │  │                                        │  │
│  └──────────────────┘  └────────────────────────────────────────┘  │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Layout:**
- **Left sidebar:** Category navigation
- **Right panel:** Settings for selected category
- **[×] Close button:** Returns to previous surface

---

## 2. Appearance Settings (Detailed)

```
┌──────────────────────────────────────────────────────────────────────┐
│  Appearance                                                           │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Theme                                                                │
│  ○ Light Mode (Default)                                              │
│  ● Dark Mode                                                          │
│  ○ Low-Stimulation (Warm Beige)                                      │
│  ○ High Contrast (Accessibility)                                     │
│                                                                       │
│  [Preview Theme]                                                      │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Font Family                                                          │
│  [System Default ▼]                                                   │
│    • System Default (Segoe UI, San Francisco, Ubuntu)                │
│    • OpenDyslexic (Dyslexia-friendly)                                │
│    • Atkinson Hyperlegible (Low-vision friendly)                     │
│    • Comic Sans (Destigmatized, friendly)                            │
│    • Monospace (For code/technical journaling)                       │
│                                                                       │
│  Font Size                                                            │
│  ◄────────●───────► 16px                                             │
│  Small (12px)                Medium (16px)              Large (24px) │
│                                                                       │
│  Line Height                                                          │
│  ◄──────●─────────► 1.5                                              │
│  Compact (1.2)              Normal (1.5)              Spacious (2.0) │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  UI Preferences                                                       │
│  ☑ Enable animations (smooth transitions, fades)                     │
│  ☑ Show word count in status bar                                     │
│  ☐ Show XP counter in status bar (Pro feature)                       │
│  ☐ Enable Simple Mode (hide all gamification)                        │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Window Behavior                                                      │
│  ☑ Remember window size and position                                 │
│  ☐ Always start in fullscreen mode                                   │
│  ☐ Minimize to system tray on close                                  │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  [Reset to Defaults]                                    [Save Changes]│
└──────────────────────────────────────────────────────────────────────┘
```

---

## 3. AI Features Settings (Pro)

```
┌──────────────────────────────────────────────────────────────────────┐
│  AI Features                                         🔒 Pro Feature   │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  AI Status: ✓ Active (Models loaded)                                 │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Muse (Writing Suggestions)                                          │
│  ☑ Enable Muse inline suggestions                                    │
│                                                                       │
│  Suggestion frequency:                                                │
│  ○ Rarely (every 10 minutes)                                         │
│  ● Occasionally (every 5 minutes)                                    │
│  ○ Frequently (every 2 minutes)                                      │
│                                                                       │
│  Writing style:                                                       │
│  [Encouraging ▼]                                                      │
│    • Encouraging (gentle, supportive prompts)                        │
│    • Analytical (thought-provoking questions)                        │
│    • Poetic (creative, metaphorical suggestions)                     │
│    • Minimal (simple, direct prompts)                                │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Archivist (Auto-Tagging)                                            │
│  ☑ Enable automatic tag suggestions                                  │
│  ☑ Auto-generate embeddings for semantic search                      │
│                                                                       │
│  Processing:                                                          │
│  ○ Immediate (process as you write)                                  │
│  ● On save (process after entry saved)                               │
│  ○ Manual only (I'll trigger it myself)                              │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Concierge (Natural Language Commands)                               │
│  ☑ Enable natural language search                                    │
│  ☑ Allow Concierge to suggest related entries                        │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Model Management                                                     │
│  Installed models:                                                    │
│  • Phi-3-mini-4k-instruct (2.3 GB) ✓ Loaded                          │
│  • nomic-embed-text-v1.5 (500 MB) ✓ Loaded                           │
│                                                                       │
│  Storage location:                                                    │
│  C:\Users\...\ChronoXP\models\                                        │
│  [Change Location]                                                    │
│                                                                       │
│  [Download Additional Models] [Update Models] [Unload Models]        │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  [Disable All AI Features]                             [Save Changes]│
└──────────────────────────────────────────────────────────────────────┘
```

**Locked State (Free Tier):**
```
┌──────────────────────────────────────────────────────────────────────┐
│  AI Features                                         🔒 Pro Feature   │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  AI features require a Pro subscription.                              │
│                                                                       │
│  With Pro, you get:                                                   │
│  • Muse: Inline writing suggestions                                  │
│  • Archivist: Auto-tagging and embeddings                            │
│  • Concierge: Natural language search                                │
│  • Semantic search: Find entries by meaning                          │
│                                                                       │
│  All AI runs locally on your device. No cloud, no tracking.          │
│                                                                       │
│  [Upgrade to Pro ($1.99)] [Learn More]                               │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 4. Privacy & Data Settings

```
┌──────────────────────────────────────────────────────────────────────┐
│  Privacy & Data                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Encryption                                                           │
│  ☑ Database encrypted with SQLCipher (AES-256)                       │
│                                                                       │
│  Vault Password                                                       │
│  ○ No password (database encrypted with device key)                  │
│  ● Password protected (require password on app launch)               │
│                                                                       │
│  [Change Vault Password]                                              │
│                                                                       │
│  Warning: If you forget your password, your entries cannot be        │
│  recovered. Keep a backup of your .chrxp exports!                    │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Data Storage                                                         │
│  Database location:                                                   │
│  C:\Users\...\ChronoXP\vault.db                                       │
│  [Change Location] [Open Folder]                                     │
│                                                                       │
│  Database size: 12.4 MB (156 entries)                                │
│  Last backup: Never                                                   │
│  [Create Backup Now]                                                  │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Privacy Controls                                                     │
│  ☑ Offline-only mode (no network access)                             │
│  ☐ Allow LAN sync (sync with devices on local network) [Beta]        │
│  ☐ Send anonymous crash reports (help improve ChronoXP)              │
│                                                                       │
│  ChronoXP never:                                                      │
│  • Sends your entries to the cloud                                   │
│  • Tracks your usage or behavior                                     │
│  • Shares your data with third parties                               │
│  • Requires an account or email address                              │
│                                                                       │
│  [View Privacy Policy]                                                │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Data Management                                                      │
│  [Export All Entries] [Import Entries] [Clear All Data]              │
│                                                                       │
│  ⚠️ Clear All Data will permanently delete:                          │
│     • All entries (156 entries)                                      │
│     • All tags (24 tags)                                             │
│     • All XP and artifacts                                           │
│     • All settings (except app preferences)                          │
│                                                                       │
│  This action CANNOT be undone. Export a backup first!                │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 5. Keyboard Settings

```
┌──────────────────────────────────────────────────────────────────────┐
│  Keyboard Shortcuts                                                   │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  [Search shortcuts...]                                                │
│                                                                       │
│  Global Shortcuts                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ Action                    │ Shortcut          │ [Edit]       │   │
│  │ ─────────────────────────────────────────────────────────    │   │
│  │ Command Bar               │ Ctrl+K            │ [Change]     │   │
│  │ New Entry                 │ Ctrl+N            │ [Change]     │   │
│  │ Search Entries            │ Ctrl+F            │ [Change]     │   │
│  │ Save Entry (manual)       │ Ctrl+S            │ [Change]     │   │
│  │ Close Modal / Cancel      │ Esc               │ [Change]     │   │
│  │ Keyboard Shortcuts Help   │ Ctrl+?            │ [Change]     │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  Canvas (Writing) Shortcuts                                          │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ Bold                      │ Ctrl+B            │ [Change]     │   │
│  │ Italic                    │ Ctrl+I            │ [Change]     │   │
│  │ Add Tag                   │ Ctrl+T            │ [Change]     │   │
│  │ Accept Muse Suggestion    │ Tab               │ [Change]     │   │
│  │ Previous Entry            │ Ctrl+[            │ [Change]     │   │
│  │ Next Entry                │ Ctrl+]            │ [Change]     │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  Library Shortcuts                                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ Navigate Calendar/List    │ Arrow Keys        │ [Change]     │   │
│  │ Open Selected Entry       │ Enter             │ [Change]     │   │
│  │ Delete Entry              │ Delete            │ [Change]     │   │
│  │ Pin Entry                 │ Ctrl+P            │ [Change]     │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  [Reset to Defaults] [Export Shortcuts] [Import Shortcuts]           │
│                                                                       │
│  ☑ Show keyboard hints in UI (tooltips on hover)                     │
│  ☐ Enable Vim-style navigation (hjkl keys)                           │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 6. Accessibility Settings

```
┌──────────────────────────────────────────────────────────────────────┐
│  Accessibility                                                        │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Visual                                                               │
│  ☑ High contrast mode                                                │
│  ☑ Large focus indicators (3px outline)                              │
│  ☐ Underline links (in addition to color)                            │
│                                                                       │
│  Text zoom level:                                                     │
│  ◄────●─────► 100%                                                    │
│  50%                    100% (Default)                    200%        │
│                                                                       │
│  Cursor:                                                              │
│  [System Default ▼]                                                   │
│    • System Default                                                   │
│    • Large cursor (2x size)                                          │
│    • Extra-large cursor (3x size)                                    │
│    • High-contrast cursor (black/white)                              │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Motion & Animation                                                   │
│  ☐ Reduce motion (disable parallax, smooth scrolling)                │
│  ☐ Disable all animations (instant transitions only)                 │
│  ☐ Pause animated artifacts in Cavern                                │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Screen Reader                                                        │
│  ☑ Optimize for screen readers (ARIA labels, live regions)           │
│  ☑ Announce autosave status                                          │
│  ☑ Announce XP gains (if gamification enabled)                       │
│                                                                       │
│  Screen reader detected: NVDA (Windows)                               │
│  [Test Screen Reader Compatibility]                                  │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Cognitive Support                                                    │
│  ☑ Dyslexia-friendly font (OpenDyslexic)                             │
│  ☑ Increase line spacing (1.8 instead of 1.5)                        │
│  ☐ Highlight current line while writing                              │
│  ☐ Reading mode (dim non-focused paragraphs)                         │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Neurodivergent Support                                              │
│  ☑ Enable Simple Mode (hide gamification, reduce cognitive load)     │
│  ☐ Disable notification toasts (except critical errors)              │
│  ☐ Minimize UI chrome (hide status bar, fewer buttons)               │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Color Blindness                                                      │
│  Color simulation:                                                    │
│  [None (Default) ▼]                                                   │
│    • None (Default)                                                   │
│    • Protanopia (Red-blind)                                          │
│    • Deuteranopia (Green-blind)                                      │
│    • Tritanopia (Blue-blind)                                         │
│    • Monochromacy (Grayscale)                                        │
│                                                                       │
│  [Preview Color Simulation]                                           │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  [Run Accessibility Audit] [View WCAG Compliance Report]             │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 7. Export & Import Settings

```
┌──────────────────────────────────────────────────────────────────────┐
│  Export & Import                                                      │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Export Entries                                                       │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ Format:                                                       │   │
│  │ ● .chrxp (ChronoXP native format - includes everything)      │   │
│  │ ○ Markdown files (.md - one file per entry)                  │   │
│  │ ○ JSON (.json - structured data)                             │   │
│  │ ○ Plain text (.txt - no formatting)                          │   │
│  │                                                               │   │
│  │ Include:                                                      │   │
│  │ ☑ All entries                                                 │   │
│  │ ☑ Tags                                                        │   │
│  │ ☑ Attachments                                                 │   │
│  │ ☑ Embeddings (for semantic search)                           │   │
│  │ ☑ XP and artifacts progress                                  │   │
│  │                                                               │   │
│  │ Date range:                                                   │   │
│  │ ○ All time                                                    │   │
│  │ ○ Past year                                                   │   │
│  │ ○ Custom range: [From: __/__/____] [To: __/__/____]          │   │
│  │                                                               │   │
│  │ Encryption:                                                   │   │
│  │ ☐ Password-protect export (AES-256-GCM)                      │   │
│  │                                                               │   │
│  │ [Export Now]                                                  │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Import Entries                                                       │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ Supported formats:                                            │   │
│  │ • .chrxp (ChronoXP native)                                    │   │
│  │ • Markdown files (.md folder)                                 │   │
│  │ • Day One JSON export                                         │   │
│  │ • Journey JSON export                                         │   │
│  │ • Plain text files (.txt)                                     │   │
│  │                                                               │   │
│  │ Import mode:                                                  │   │
│  │ ● Merge with existing entries (keep both)                    │   │
│  │ ○ Replace all entries (delete current, import new)           │   │
│  │                                                               │   │
│  │ Duplicate handling:                                           │   │
│  │ [Skip duplicates ▼]                                           │   │
│  │   • Skip duplicates (based on date + title)                  │   │
│  │   • Import all (allow duplicates)                            │   │
│  │   • Ask me for each duplicate                                │   │
│  │                                                               │   │
│  │ [Choose File to Import]                                       │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Automatic Backups (Pro Feature)                                     │
│  ☐ Enable automatic backups                                          │
│  Frequency: [Daily ▼]                                                 │
│  Location: [C:\Users\...\ChronoXP\backups\] [Change]                 │
│  Keep: [7 most recent backups ▼]                                     │
│                                                                       │
│  [Upgrade to Pro]                                                     │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 8. About Settings

```
┌──────────────────────────────────────────────────────────────────────┐
│  About ChronoXP™ Journal                                              │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│                    [ChronoXP Logo]                                    │
│                                                                       │
│                  ChronoXP™ Journal                                    │
│                     Version 1.0.0                                     │
│                                                                       │
│       A private, neuro-affirming, offline-first journaling            │
│            companion with gentle gamification.                        │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  License                                                              │
│  ● Free (Core features)                                               │
│  ○ Pro - $1.99 one-time [Upgrade]                                    │
│  ○ Extras Level 1 - $3.33/month [Subscribe]                          │
│  ○ Extras Level 2 - $4.44/month [Subscribe]                          │
│  ○ Annual Package - $11.11/year [Subscribe]                          │
│  ○ Lifetime - $33.30 one-time [Purchase]                             │
│                                                                       │
│  [View Pricing] [Manage Subscription]                                │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  System Information                                                   │
│  Operating System: Windows 11 Pro (10.0.22631)                        │
│  Architecture: x64                                                    │
│  Database: SQLCipher 4.5.5                                            │
│  AI Engine: llama.cpp (commit abc123)                                 │
│                                                                       │
│  Storage:                                                             │
│  • Database: 12.4 MB (156 entries)                                    │
│  • AI Models: 2.8 GB (2 models)                                       │
│  • Total: 2.81 GB                                                     │
│                                                                       │
│  [View Detailed Diagnostics]                                          │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Links                                                                │
│  [Website] [Documentation] [GitHub] [Privacy Policy] [Support]       │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Updates                                                              │
│  ☑ Check for updates automatically                                   │
│  Last checked: Today at 9:34 AM                                      │
│  Status: ✓ You're running the latest version                         │
│                                                                       │
│  [Check for Updates Now]                                              │
│                                                                       │
│  ──────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Credits                                                              │
│  Built with love by TheSeeker713 and contributors.                   │
│                                                                       │
│  Open source libraries:                                               │
│  • Tauri 2.x (MIT License)                                            │
│  • React 18 (MIT License)                                             │
│  • SQLCipher (BSD License)                                            │
│  • llama.cpp (MIT License)                                            │
│                                                                       │
│  [View Full Credits] [View Open Source Licenses]                     │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 9. Simple Mode Toggle (Quick Access)

**From Any Surface:**
```
Settings → Appearance → ☐ Enable Simple Mode
```

**When Enabled:**
- Hides Records Cavern tab from navigation
- Hides XP counter in status bar
- Suppresses artifact unlock toasts
- Hides quest prompts
- XP still accumulates in background (can re-enable later)

**Confirmation Dialog:**
```
┌──────────────────────────────────────────────────────────────────┐
│  Enable Simple Mode?                                          [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Simple Mode will hide:                                         │
│  • Records Cavern (gamification space)                          │
│  • XP counter and progress notifications                        │
│  • Quest prompts and artifact unlocks                           │
│                                                                  │
│  Your progress will be saved, and you can re-enable Simple     │
│  Mode anytime without losing data.                              │
│                                                                  │
│  This mode is designed for users who find gamification         │
│  distracting or stressful.                                      │
│                                                                  │
│  [Enable Simple Mode] [Cancel]                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 10. Change Vault Password Dialog

```
┌──────────────────────────────────────────────────────────────────┐
│  Change Vault Password                                        [×]│
│  ────────────────────────────────────────────────────────────   │
│                                                                  │
│  Current password:                                              │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ ••••••••••••                                            │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
│  New password:                                                  │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ ••••••••••••••••                                        │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
│  Confirm new password:                                          │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ ••••••••••••••••                                        │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
│  Password strength: ████████░░ Strong                           │
│                                                                  │
│  ⚠️ Important:                                                  │
│  • If you forget your new password, you will lose access to    │
│    all your entries.                                            │
│  • Export a backup before changing your password.              │
│  • ChronoXP cannot recover your password—keep it safe!         │
│                                                                  │
│  [Export Backup First] [Change Password] [Cancel]              │
└──────────────────────────────────────────────────────────────────┘
```

---

## Accessibility Notes

### Keyboard Navigation
- **Tab:** Navigate between settings categories (left sidebar)
- **Enter:** Select category
- **Arrow keys:** Navigate within settings (radio buttons, checkboxes)
- **Space:** Toggle checkboxes
- **Esc:** Close settings panel

### Screen Reader
- **Category names:** Announced as "Appearance settings, 1 of 7"
- **Controls:** Announced with current state ("Dark Mode, selected")
- **Sliders:** Announced with value ("Font size, 16 pixels")

### Focus Management
- When settings panel opens, focus goes to first category
- When category selected, focus goes to first control in that category
- Closing settings returns focus to previous location

---

## Performance Targets

- **Settings panel open:** <100ms
- **Category switch:** <50ms (instant feel)
- **Apply changes:** <200ms (save to SQLite, reload UI)
- **Export entries:** <5s for 1000 entries (progress indicator)

---

## Developer Notes

### Tech Stack
- **Settings storage:** SQLite `settings` table (key-value pairs)
- **Live preview:** React state updates instantly show theme changes
- **Export:** Rust backend streams data to ZIP file (Tauri API)
- **Import:** Parse files in Rust, batch insert to SQLite

### Settings Schema
```sql
CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT, -- JSON-serialized value
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**Example rows:**
```sql
INSERT INTO settings (key, value) VALUES
  ('theme', '"dark"'),
  ('font_family', '"OpenDyslexic"'),
  ('font_size', '16'),
  ('ai_enabled', 'true'),
  ('simple_mode', 'false');
```

---

**End of Settings Panel Wireframe**
