# ChronoXP™ Wireframe Implementation Todo List

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025  
**Purpose:** Track implementation of all wireframed features

---

## Overview

This todo list breaks down the comprehensive wireframe documentation into actionable development tasks. Each task references specific wireframe documents for implementation details.

**Total Wireframes:** 10 documents (~67,000 words)  
**Total Tasks:** 156 implementation tasks  
**Estimated Timeline:** 6-12 months (solo developer)

---

## Phase 0: Project Setup (Week 1-2)

### 0.1 Development Environment
- [ ] Initialize Tauri 2.x + React + TypeScript project
- [ ] Configure Rust backend (Cargo.toml dependencies)
- [ ] Set up SQLite + SQLCipher (encrypted database)
- [ ] Configure Vite build system
- [ ] Set up ESLint + Prettier (code formatting)
- [ ] Create Git repository and .gitignore
- [ ] Set up GitHub Actions (CI/CD for builds)
- [ ] Configure cross-platform builds (Windows, macOS, Linux)

### 0.2 Core Dependencies
- [ ] Install SQLCipher (encrypted SQLite)
- [ ] Install llama.cpp (local AI inference)
- [ ] Install sqlite-vec (vector embeddings for semantic search)
- [ ] Install Tauri HTTP client (AI model downloads)
- [ ] Install React Router (navigation)
- [ ] Install Zustand (state management)
- [ ] Install TipTap or ProseMirror (rich text editor)

### 0.3 Database Schema
- [ ] Create `entries` table (id, date, title, content, mood, word_count, created_at, updated_at)
- [ ] Create `tags` table (id, name, color, created_at)
- [ ] Create `entry_tags` junction table (entry_id, tag_id)
- [ ] Create `embeddings` table (entry_id, embedding_vector) — for semantic search
- [ ] Create `artifacts` table (id, name, description, unlocked_at, tier)
- [ ] Create `xp_log` table (id, action, xp_earned, timestamp, entry_id)
- [ ] Create `settings` table (key, value, updated_at)
- [ ] Create `easter_eggs` table (id, discovered_at, times_triggered)
- [ ] Set up database migrations system (rust-sqlite-migration)

**Reference:** [Data Model](../../tech/data-model.md)

---

## Phase 1: App Shell & Foundation (Week 3-4)

### 1.1 Window Chrome
- [ ] Implement platform-specific window chrome (Windows, macOS, Linux)
- [ ] Custom title bar with app icon and title
- [ ] Window controls (minimize, maximize/restore, close)
- [ ] Window drag region
- [ ] Remember window size and position (persist in settings)
- [ ] Fullscreen mode (F11) toggle

**Reference:** [01-app-shell.md](./01-app-shell.md) Section 1

### 1.2 Global Navigation Bar
- [ ] Navigation bar component (Canvas, Library, Cavern, Settings tabs)
- [ ] Tab highlighting (active state)
- [ ] Keyboard shortcuts (Ctrl+1/2/3/4 for tab switching)
- [ ] Tab icons (📝 📚 🏛️ ⚙️)
- [ ] Responsive layout (collapse to icons on small windows)

**Reference:** [01-app-shell.md](./01-app-shell.md) Section 2

### 1.3 Global Status Bar
- [ ] Status bar component (bottom of window)
- [ ] Autosave indicator (⟳ Saving... / ✓ Saved / ⚠️ Failed)
- [ ] Word count display (current entry)
- [ ] XP counter (total XP, Pro feature)
- [ ] Offline badge (📶 Offline, always green)
- [ ] Network status indicator (future LAN sync)

**Reference:** [01-app-shell.md](./01-app-shell.md) Section 3

### 1.4 Command Bar (Ctrl+K)
- [ ] Command Bar modal overlay (full-screen darkened backdrop)
- [ ] Fuzzy search input (filter commands as you type)
- [ ] Command categories (Navigation, Actions, Search, Settings)
- [ ] Keyboard navigation (arrow keys, Enter to execute)
- [ ] Recent commands history
- [ ] Command shortcuts display (Ctrl+N, Ctrl+F, etc.)
- [ ] Close on Escape or click outside

**Reference:** [01-app-shell.md](./01-app-shell.md) Section 4

### 1.5 Keyboard Shortcuts
- [ ] Global keyboard event listener
- [ ] Keyboard shortcuts overlay (Ctrl+?)
- [ ] Print cheat sheet functionality (PDF export)
- [ ] Customizable shortcuts (Settings → Keyboard)
- [ ] Vim-style navigation option (hjkl keys)

**Reference:** [01-app-shell.md](./01-app-shell.md) Section 5

### 1.6 Toast Notifications
- [ ] Toast notification system (slide-in from bottom-right)
- [ ] 5 toast types: artifacts, quests, welcome, errors, export
- [ ] Auto-dismiss after 5 seconds (or manual close)
- [ ] Queue system (max 3 toasts visible at once)
- [ ] Accessibility: ARIA live regions for screen readers

**Reference:** [01-app-shell.md](./01-app-shell.md) Section 6

---

## Phase 2: Journal Canvas (Writing Surface) (Week 5-7)

### 2.1 Basic Canvas
- [ ] Rich text editor integration (TipTap or ProseMirror)
- [ ] Empty canvas with ghost placeholder ("Want a soft start?...")
- [ ] Current date display (top-left)
- [ ] Cursor focus on load
- [ ] Plain text mode (no formatting, just writing)

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 1

### 2.2 Autosave System
- [ ] Autosave on blur (when user clicks away)
- [ ] Autosave every 30 seconds (background timer)
- [ ] Autosave indicator (⟳ Saving... / ✓ Saved / ⚠️ Failed)
- [ ] Manual save (Ctrl+S)
- [ ] Unsaved changes warning (on close)
- [ ] Conflict resolution (if multiple devices, future LAN sync)

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 2

### 2.3 Formatting Toolbar
- [ ] Bold (Ctrl+B), Italic (Ctrl+I)
- [ ] Headings (H1, H2, H3)
- [ ] Bullet list, numbered list
- [ ] Blockquote
- [ ] Horizontal rule
- [ ] Clear formatting button
- [ ] Fullscreen/Zen mode toggle (F11)

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 3

### 2.4 Tagging System
- [ ] Tag pill display (below content)
- [ ] Add tag button (Ctrl+T opens tag picker modal)
- [ ] Tag colors (randomly assigned or user-customizable)
- [ ] Remove tag (click × on pill)
- [ ] Click tag to filter Library view
- [ ] Tag autocomplete (as you type in tag picker)

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 4

### 2.5 Mood Selector
- [ ] Mood dropdown button in toolbar
- [ ] 9 pre-defined moods (😊 Happy, 😔 Sad, 😰 Anxious, etc.)
- [ ] [No mood] option
- [ ] Mood icon display (next to date)
- [ ] Mood stored in database (mood column in entries table)

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 5

### 2.6 Word Count & Milestones
- [ ] Real-time word count (display in status bar)
- [ ] Milestone toasts (100, 500, 1000 words)
- [ ] Character count option (Settings toggle)
- [ ] Reading time estimate (optional, Settings toggle)

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 6

### 2.7 Entry Navigation
- [ ] Previous entry (Ctrl+[)
- [ ] Next entry (Ctrl+])
- [ ] Entry ID display (bottom-right corner)
- [ ] Smooth transition between entries

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 7

---

## Phase 3: AI Features (Muse, Archivist, Concierge) (Week 8-11)

### 3.1 AI Model Management
- [ ] Download AI models on first run (Phi-3-mini, nomic-embed)
- [ ] Progress indicator (download speed, ETA)
- [ ] Model storage location (Settings → AI Features)
- [ ] Load models into llama.cpp on app launch
- [ ] Unload models (free memory)
- [ ] Model update checker (Settings → AI Features)

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 8, [07-settings-panel.md](./07-settings-panel.md) Section 3

### 3.2 Muse (Writing Suggestions)
- [ ] Detect writing pauses (no typing for 60 seconds)
- [ ] Generate suggestion via llama.cpp (Phi-3-mini)
- [ ] Display suggestion in floating card (above cursor)
- [ ] Accept suggestion (Tab key)
- [ ] Dismiss suggestion (Esc key, keep typing)
- [ ] Suggestion frequency settings (Rarely, Occasionally, Frequently)
- [ ] Writing style settings (Encouraging, Analytical, Poetic, Minimal)
- [ ] Disable Muse (Settings toggle)

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 8

### 3.3 Archivist (Auto-Tagging)
- [ ] Generate embeddings on save (nomic-embed-text-v1.5)
- [ ] Store embeddings in sqlite-vec table
- [ ] Suggest tags based on entry content (llama.cpp classification)
- [ ] Display tag suggestions (toast notification after save)
- [ ] One-click accept/reject suggestions
- [ ] Processing timing settings (Immediate, On save, Manual only)

**Reference:** [02-journal-canvas.md](./02-journal-canvas.md) Section 9

### 3.4 Concierge (Semantic Search)
- [ ] Natural language search queries ("Show me entries about hope")
- [ ] Convert query to embedding (nomic-embed-text-v1.5)
- [ ] Cosine similarity search (sqlite-vec)
- [ ] Relevance % display (how close to query)
- [ ] Combine with keyword search (hybrid search)

**Reference:** [04-search-modal.md](./04-search-modal.md) Section 3

---

## Phase 4: Library (Entry Browsing) (Week 12-14)

### 4.1 Calendar View
- [ ] Month grid (7 columns, 5-6 rows)
- [ ] Entry indicators (● filled, ○ empty)
- [ ] Click date to filter entries
- [ ] Previous/next month navigation
- [ ] Jump to today button
- [ ] Multiple entries on same day (stacked indicators)

**Reference:** [03-library-surface.md](./03-library-surface.md) Section 1

### 4.2 Timeline View
- [ ] Chronological list (newest first)
- [ ] Date grouping headers (Today, Yesterday, This Week, etc.)
- [ ] Infinite scroll (load 50 entries at a time)
- [ ] Entry preview (title, tags, excerpt, word count, mood)
- [ ] Click to open in Canvas
- [ ] Scroll to today button

**Reference:** [03-library-surface.md](./03-library-surface.md) Section 2

### 4.3 Tags View
- [ ] Left sidebar: All tags with entry counts
- [ ] Right panel: Entries filtered by selected tag
- [ ] Tag color indicators
- [ ] Sort tags (alphabetical, by count, by recent)
- [ ] Rename tag (inline edit)
- [ ] Delete tag (with confirmation)

**Reference:** [03-library-surface.md](./03-library-surface.md) Section 3

### 4.4 Filters & Sorting
- [ ] Date range picker (from/to dates)
- [ ] Mood filter (checkboxes for all moods)
- [ ] Word count slider (min/max)
- [ ] Has tags filter (only entries with tags)
- [ ] Sort options (date, word count, mood)
- [ ] Clear all filters button

**Reference:** [03-library-surface.md](./03-library-surface.md) Section 4

### 4.5 Entry Preview Panel
- [ ] Preview pane (right side of Library)
- [ ] Title, tags, excerpt (first 200 chars)
- [ ] Word count, mood icon, date
- [ ] [Edit] [Pin] [Export] [Delete] buttons
- [ ] Click preview to open full entry in Canvas

**Reference:** [03-library-surface.md](./03-library-surface.md) Section 5

### 4.6 Bulk Actions
- [ ] Bulk selection mode (checkboxes on entries)
- [ ] Select all / deselect all
- [ ] Bulk tag (add tag to multiple entries)
- [ ] Bulk export (export selected entries)
- [ ] Bulk delete (with confirmation)

**Reference:** [03-library-surface.md](./03-library-surface.md) Section 6

### 4.7 Pinned Entries
- [ ] Pin entry (star icon, Ctrl+P)
- [ ] Pinned section at top of Library
- [ ] Drag to reorder pinned entries
- [ ] Unpin entry (click star again)

**Reference:** [03-library-surface.md](./03-library-surface.md) Section 7

---

## Phase 5: Search (Week 15-16)

### 5.1 Search Modal
- [ ] Search overlay (Ctrl+F)
- [ ] Full-screen darkened backdrop
- [ ] Search input with focus on open
- [ ] Close on Escape or click outside

**Reference:** [04-search-modal.md](./04-search-modal.md) Section 1

### 5.2 Keyword Search
- [ ] SQLite FTS (Full-Text Search) index on entries
- [ ] Highlight matching terms in results
- [ ] Paginated results (12 per page)
- [ ] Result preview (title, excerpt, tags, date)
- [ ] Click result to open in Canvas

**Reference:** [04-search-modal.md](./04-search-modal.md) Section 2

### 5.3 Semantic Search (Pro Feature)
- [ ] Natural language query input
- [ ] Generate embedding for query
- [ ] Cosine similarity search (sqlite-vec)
- [ ] Relevance % display
- [ ] Locked state for Free tier (upgrade prompt)

**Reference:** [04-search-modal.md](./04-search-modal.md) Section 3

### 5.4 Advanced Filters
- [ ] Filters sidebar (toggle with [Filters] button)
- [ ] Date range picker
- [ ] Tag checkboxes (multi-select)
- [ ] Mood checkboxes (multi-select)
- [ ] Word count slider
- [ ] [Clear Filters] button

**Reference:** [04-search-modal.md](./04-search-modal.md) Section 4

### 5.5 Search History
- [ ] Store last 5 searches in localStorage
- [ ] Display recent searches (clickable)
- [ ] Clear search history button

**Reference:** [04-search-modal.md](./04-search-modal.md) Section 5

### 5.6 Search Command Syntax
- [ ] `tag:work` (filter by tag)
- [ ] `mood:happy` (filter by mood)
- [ ] `words:>500` (word count filter)
- [ ] `today`, `yesterday`, `this week`
- [ ] Help tooltip (explain syntax)

**Reference:** [04-search-modal.md](./04-search-modal.md) Section 6

---

## Phase 6: Records Cavern (Gamification) (Week 17-20)

### 6.1 XP System
- [ ] XP tracking in database (xp_log table)
- [ ] XP counter in status bar
- [ ] XP gain events (write entry, milestone, streak, etc.)
- [ ] XP toast notifications (+5 XP, +10 XP, etc.)
- [ ] Total XP display in Cavern

**Reference:** [05-records-cavern.md](./05-records-cavern.md) Section 1

### 6.2 Artifact System
- [ ] 16 artifacts defined (see gamification/design.md)
- [ ] Unlock conditions (XP thresholds, special events)
- [ ] Artifact unlock detection (check on XP gain)
- [ ] Artifact unlock toast (gentle slide-in)
- [ ] Artifact storage in database (artifacts table)

**Reference:** [05-records-cavern.md](./05-records-cavern.md) Section 2

### 6.3 Cavern Visual Evolution (5 Tiers)
- [ ] **Tier 1 (0-50 XP):** Small stone chamber, single candle, worn journal
- [ ] **Tier 2 (50-200 XP):** Lantern, small bookshelf, tapestry
- [ ] **Tier 3 (200-800 XP):** Multiple lanterns, artifact shelves, wall hangings
- [ ] **Tier 4 (800-1600 XP):** Grand study, ornate desk, stained glass window
- [ ] **Tier 5 (1600+ XP):** Master's Sanctum, cosmic window, multi-level chamber
- [ ] Hand-painted assets for each tier (commission or paint in Photoshop/Procreate)

**Reference:** [05-records-cavern.md](./05-records-cavern.md) Section 3

### 6.4 Artifact Interaction
- [ ] Hover over artifact (painted glow effect)
- [ ] Click artifact (open detail modal)
- [ ] Artifact detail modal (closeup, flavor text, unlock date, related entries)
- [ ] [< Previous] / [Next >] buttons (navigate artifacts)
- [ ] [View Related Entries] button (filter Library)

**Reference:** [05-records-cavern.md](./05-records-cavern.md) Section 4

### 6.5 XP Log
- [ ] Chronological list of all XP-earning actions
- [ ] Filter by type (entries, milestones, streaks, quests)
- [ ] Date, action description, XP earned
- [ ] Scroll to see full history

**Reference:** [05-records-cavern.md](./05-records-cavern.md) Section 5

### 6.6 Quest Prompts (Optional)
- [ ] Reflection prompt modal (random, gentle frequency)
- [ ] [Start Writing] button (opens Canvas with prompt)
- [ ] [Maybe Later] button (dismiss)
- [ ] [Don't Show Again] checkbox (disable quests)
- [ ] Quest completion detection (+10 XP bonus)

**Reference:** [05-records-cavern.md](./05-records-cavern.md) Section 6

### 6.7 Pan & Zoom Controls
- [ ] Click + drag to pan around cavern scene
- [ ] Scroll to zoom in/out (reveals brush stroke details)
- [ ] Reset view button (return to default zoom/position)
- [ ] Smooth animations (no jarring jumps)

**Reference:** [05-records-cavern.md](./05-records-cavern.md) Section 7

### 6.8 Simple Mode (Hide Gamification)
- [ ] Simple Mode toggle (Settings → Appearance)
- [ ] Hide Cavern tab from navigation
- [ ] Hide XP counter in status bar
- [ ] Suppress artifact unlock toasts
- [ ] Suppress quest prompts
- [ ] XP still accumulates in background (can re-enable later)

**Reference:** [05-records-cavern.md](./05-records-cavern.md) Section 8

---

## Phase 7: Error Handling & Empty States (Week 21-22)

### 7.1 Empty States
- [ ] No entries yet (welcome message with [New Entry] button)
- [ ] No search results (suggestions, [Clear Filters] button)
- [ ] No tags (encourage tagging, tutorial link)
- [ ] Empty Cavern (Tier 1, no artifacts yet)
- [ ] Empty Library (filtered view, no matches)

**Reference:** [06-error-states.md](./06-error-states.md) Sections 1-5

### 7.2 Autosave Errors
- [ ] Autosave failed toast (⚠️ Failed to save)
- [ ] Retry button (attempt save again)
- [ ] Manual save button (force save)
- [ ] Export to .txt button (emergency backup)
- [ ] Copy to clipboard button (last resort)

**Reference:** [06-error-states.md](./06-error-states.md) Section 6

### 7.3 Database Errors
- [ ] Database locked (vault password entry modal)
- [ ] 3 failed password attempts → close app
- [ ] Database corruption detection (automatic repair flow)
- [ ] Repair progress indicator (recovered X entries)
- [ ] Repair failed → restore from backup prompt

**Reference:** [06-error-states.md](./06-error-states.md) Sections 7-8

### 7.4 AI Model Errors
- [ ] AI model missing (download prompt)
- [ ] Download progress indicator (~2.8 GB)
- [ ] Download failed (retry, skip for now)
- [ ] Disk space low warning (free up X GB)

**Reference:** [06-error-states.md](./06-error-states.md) Section 9

### 7.5 Import/Export Errors
- [ ] Invalid file format (explain supported formats)
- [ ] Corrupted file (cannot parse)
- [ ] Permission denied (check file access)
- [ ] Duplicate handling (skip, import all, ask me)

**Reference:** [06-error-states.md](./06-error-states.md) Section 10

### 7.6 Unsaved Changes Warning
- [ ] Detect unsaved changes (dirty state)
- [ ] Modal on close ([Save and Close] [Don't Save] [Cancel])
- [ ] Keyboard shortcuts (Enter, D, Esc)

**Reference:** [06-error-states.md](./06-error-states.md) Section 11

### 7.7 Confirmation Dialogs
- [ ] Delete entry confirmation (single entry)
- [ ] Delete multiple entries (with backup recommendation)
- [ ] Clear all data (type "DELETE ALL" to confirm)

**Reference:** [06-error-states.md](./06-error-states.md) Sections 12-13

---

## Phase 8: Settings Panel (Week 23-24)

### 8.1 Settings Navigation
- [ ] Settings modal (full-screen overlay)
- [ ] Left sidebar (category navigation)
- [ ] Right panel (settings for selected category)
- [ ] [×] Close button (top-right)

**Reference:** [07-settings-panel.md](./07-settings-panel.md) Section 1

### 8.2 Appearance Settings
- [ ] Theme selector (Light, Dark, Low-Stimulation, High-Contrast)
- [ ] Font family dropdown (System, OpenDyslexic, Atkinson, Comic Sans, Monospace)
- [ ] Font size slider (12px - 24px)
- [ ] Line height slider (1.2 - 2.0)
- [ ] Enable animations checkbox
- [ ] Show word count checkbox
- [ ] Show XP counter checkbox (Pro feature)
- [ ] Enable Simple Mode checkbox
- [ ] Window behavior options (remember size/position, start fullscreen, minimize to tray)

**Reference:** [07-settings-panel.md](./07-settings-panel.md) Section 2

### 8.3 AI Features Settings (Pro)
- [ ] AI status indicator (Active / Models not loaded)
- [ ] Muse settings (enable/disable, frequency, writing style)
- [ ] Archivist settings (enable/disable, processing timing)
- [ ] Concierge settings (enable/disable, suggest related entries)
- [ ] Model management (installed models, storage location, update)
- [ ] Locked state for Free tier (upgrade prompt)

**Reference:** [07-settings-panel.md](./07-settings-panel.md) Section 3

### 8.4 Privacy & Data Settings
- [ ] Encryption status (AES-256 enabled)
- [ ] Vault password (No password / Password protected)
- [ ] Change vault password modal (with strength meter)
- [ ] Database location (path, [Change Location] [Open Folder])
- [ ] Database size display (MB, entry count)
- [ ] Last backup timestamp
- [ ] [Create Backup Now] button
- [ ] Privacy controls (offline-only, LAN sync, crash reports)
- [ ] [View Privacy Policy] link
- [ ] [Export All Entries] [Import Entries] [Clear All Data] buttons

**Reference:** [07-settings-panel.md](./07-settings-panel.md) Section 4

### 8.5 Keyboard Shortcuts Settings
- [ ] Searchable shortcuts list
- [ ] Edit shortcut (click [Change] button, press new key combo)
- [ ] Conflict detection (warn if shortcut already used)
- [ ] [Reset to Defaults] button
- [ ] [Export Shortcuts] [Import Shortcuts] buttons (JSON)
- [ ] Show keyboard hints checkbox (tooltips on hover)
- [ ] Enable Vim-style navigation checkbox (hjkl keys)

**Reference:** [07-settings-panel.md](./07-settings-panel.md) Section 5

### 8.6 Accessibility Settings
- [ ] High contrast mode checkbox
- [ ] Large focus indicators checkbox (3px outline)
- [ ] Underline links checkbox
- [ ] Text zoom slider (50% - 200%)
- [ ] Cursor size dropdown (System, Large 2x, Extra-large 3x, High-contrast)
- [ ] Reduce motion checkbox (disable parallax, smooth scrolling)
- [ ] Disable all animations checkbox
- [ ] Pause animated artifacts checkbox
- [ ] Screen reader optimization checkbox
- [ ] Announce autosave checkbox
- [ ] Announce XP gains checkbox
- [ ] Screen reader detection (NVDA, JAWS, VoiceOver)
- [ ] Dyslexia-friendly font checkbox (OpenDyslexic)
- [ ] Increase line spacing checkbox (1.8 instead of 1.5)
- [ ] Highlight current line checkbox
- [ ] Reading mode checkbox (dim non-focused paragraphs)
- [ ] Simple Mode checkbox (hide gamification)
- [ ] Disable notification toasts checkbox
- [ ] Minimize UI chrome checkbox
- [ ] Color blindness simulation dropdown (Protanopia, Deuteranopia, Tritanopia, Monochromacy)
- [ ] [Run Accessibility Audit] button
- [ ] [View WCAG Compliance Report] button

**Reference:** [07-settings-panel.md](./07-settings-panel.md) Section 6

### 8.7 Export & Import Settings
- [ ] Export format selector (.chrxp, .md, .json, .txt)
- [ ] Include checkboxes (all entries, tags, attachments, embeddings, XP/artifacts)
- [ ] Date range selector (all time, past year, custom range)
- [ ] Password-protect export checkbox (with password input)
- [ ] [Export Now] button (opens file picker)
- [ ] Import file picker (supported formats listed)
- [ ] Import mode (merge, replace all)
- [ ] Duplicate handling dropdown (skip, import all, ask me)
- [ ] Automatic backups toggle (Pro feature)
- [ ] Backup frequency dropdown (daily, weekly, monthly)
- [ ] Backup location (path, [Change])
- [ ] Keep X backups dropdown (7, 14, 30, all)

**Reference:** [07-settings-panel.md](./07-settings-panel.md) Section 7

### 8.8 About Settings
- [ ] App logo display
- [ ] Version number (clickable for easter egg)
- [ ] License tier display (Free, Pro, Level 1, Level 2, Annual, Lifetime)
- [ ] [View Pricing] [Manage Subscription] buttons
- [ ] System information (OS, architecture, database version, AI engine version)
- [ ] Storage breakdown (database size, AI models size, total)
- [ ] [View Detailed Diagnostics] button
- [ ] Links ([Website] [Documentation] [GitHub] [Privacy Policy] [Support])
- [ ] Update checker (auto-check toggle, last checked, status)
- [ ] [Check for Updates Now] button
- [ ] Credits section (built by TheSeeker713, contributors)
- [ ] Open source libraries list (Tauri, React, SQLCipher, llama.cpp)
- [ ] [View Full Credits] [View Open Source Licenses] buttons

**Reference:** [07-settings-panel.md](./07-settings-panel.md) Section 8

---

## Phase 9: Onboarding & First Run (Week 25-26)

### 9.1 Welcome Screen
- [ ] Hand-painted journal illustration (quill, candle)
- [ ] "Welcome to ChronoXP™" heading
- [ ] Tagline ("A private, offline journaling companion...")
- [ ] [Get Started] button (proceed to vault setup)
- [ ] [Import from Another App] button (jump to import wizard)

**Reference:** [08-onboarding-first-run.md](./08-onboarding-first-run.md) Section 1

### 9.2 Vault Setup Wizard
- [ ] Step 1: Choose security level (Device-Only / Password Protection)
- [ ] Security level explanations (pros/cons for each)
- [ ] Step 2: Create password (if Password Protection selected)
- [ ] Password input with strength meter
- [ ] Confirm password input
- [ ] "I understand my password cannot be recovered" checkbox
- [ ] Password hint input (optional, stored unencrypted)

**Reference:** [08-onboarding-first-run.md](./08-onboarding-first-run.md) Sections 2-3

### 9.3 AI Model Download
- [ ] Step 3: AI features introduction
- [ ] [Download AI models now] / [Skip for now] buttons
- [ ] Download progress (Phi-3-mini 2.3 GB, nomic-embed 500 MB)
- [ ] Estimated time remaining
- [ ] [Cancel Download] button
- [ ] Download complete screen ([Take a Quick Tour] [Start Writing])

**Reference:** [08-onboarding-first-run.md](./08-onboarding-first-run.md) Sections 4-5

### 9.4 Feature Tour (5 Steps)
- [ ] Tour overlay system (darkened backdrop, focus highlight)
- [ ] Step 1: Canvas (writing surface intro)
- [ ] Step 2: Muse (AI suggestions demo)
- [ ] Step 3: Library (browsing entries)
- [ ] Step 4: Cavern (gamification intro)
- [ ] Step 5: Command Bar (keyboard superpower)
- [ ] [Back] [Next] [Skip Tour] buttons on each step
- [ ] Progress indicator (1 of 5, 2 of 5, etc.)
- [ ] Tour complete screen (confetti, [Start Writing] [Replay Tour] [Close])

**Reference:** [08-onboarding-first-run.md](./08-onboarding-first-run.md) Section 6

### 9.5 Import Wizard (Alternative Path)
- [ ] Import file picker (Day One, Journey, Markdown, .txt, .chrxp)
- [ ] Import progress indicator (converting, encrypting, writing)
- [ ] Import complete screen (X entries imported, X tags preserved)
- [ ] [Take a Quick Tour] [Start Writing] buttons

**Reference:** [08-onboarding-first-run.md](./08-onboarding-first-run.md) Section 8

### 9.6 First Entry Celebration
- [ ] Detect first entry autosave
- [ ] Celebration toast (🎉 First Entry Complete!)
- [ ] +5 XP award
- [ ] [View in Library] [Write Another] [Close] buttons

**Reference:** [08-onboarding-first-run.md](./08-onboarding-first-run.md) Section 9

### 9.7 Vault Unlock Screen (Subsequent Launches)
- [ ] ChronoXP logo
- [ ] "Welcome back" message
- [ ] Password input
- [ ] Password hint display (if set)
- [ ] [Unlock] button
- [ ] 3 failed attempts warning (delete vault option)

**Reference:** [08-onboarding-first-run.md](./08-onboarding-first-run.md) Section 10

---

## Phase 10: Modals & Overlays (Week 27-28)

### 10.1 Tag Picker Modal
- [ ] Search/create tags input
- [ ] Current tags display (pills with × to remove)
- [ ] Suggested tags (AI-powered, Pro feature)
- [ ] Recent tags section
- [ ] All tags section (alphabetical)
- [ ] [+ tag] buttons to add tags
- [ ] [Done] [Cancel] buttons
- [ ] Create new tag flow (no matching tags found)

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 1

### 10.2 Mood Selector Dropdown
- [ ] 9 pre-defined moods (😊 Happy, 😔 Sad, etc.)
- [ ] [No mood] option
- [ ] Click to select mood
- [ ] Keyboard navigation (arrow keys, Enter)

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 2

### 10.3 Export Dialog
- [ ] Format selector (.chrxp, .md, .json, .txt)
- [ ] Include checkboxes (all entries, tags, attachments, embeddings, XP)
- [ ] Date range selector (all time, past year, past month, custom)
- [ ] Password-protect export checkbox (with password inputs)
- [ ] Estimated file size display
- [ ] [Choose Save Location] [Export] [Cancel] buttons
- [ ] Export progress indicator (X% complete, X of Y entries)
- [ ] Export complete modal (file path, size, [Open Folder] [Export Another] [Close])

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 3

### 10.4 Import Dialog
- [ ] File picker ([Choose Files...])
- [ ] Supported formats list (.chrxp, .json, .md, .txt)
- [ ] Import mode (merge, replace all)
- [ ] Duplicate handling dropdown (skip, import all, ask me)
- [ ] Password input (if .chrxp is password-protected)
- [ ] Import progress indicator (X% complete, X of Y entries)
- [ ] Import complete modal (X entries imported, X tags preserved, [View Library] [Close])

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 4

### 10.5 Artifact Detail Modal
- [ ] Hand-painted artifact closeup (full-screen image)
- [ ] Artifact name and flavor text
- [ ] Unlocked date and unlock condition
- [ ] Related entries count ([View Related Entries] button)
- [ ] [< Previous] [Next >] buttons (navigate artifacts)
- [ ] Close on [×] or Esc

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 5

### 10.6 Quest Prompt Modal
- [ ] Reflection prompt text (e.g., "What made you smile today?")
- [ ] Bonus XP display (+10 XP)
- [ ] [Start Writing] [Maybe Later] [Don't Show Again] buttons
- [ ] Quest completion detection (toast notification)

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 6

### 10.7 Artifact Unlock Toast
- [ ] Non-blocking slide-in notification (bottom-right)
- [ ] Artifact thumbnail and name
- [ ] Flavor text preview
- [ ] [View in Cavern] [Close] buttons
- [ ] Auto-dismiss after 5 seconds

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 7

### 10.8 Confirmation Dialogs
- [ ] Unsaved changes warning ([Save and Close] [Don't Save] [Cancel])
- [ ] Delete entry confirmation (single entry, shows entry preview)
- [ ] Delete multiple entries ([Export Backup First] [Delete Anyway] [Cancel])
- [ ] Clear all data (type "DELETE ALL" to confirm)

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Sections 8-10

### 10.9 Pro Upgrade Modal
- [ ] Feature breakdown (Muse, Archivist, Concierge)
- [ ] "All AI runs locally" message
- [ ] One-time $1.99 purchase
- [ ] [Purchase for $1.99] [Learn More] [Maybe Later] buttons
- [ ] 30-day money-back guarantee message
- [ ] After purchase: AI model download progress

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 11

### 10.10 Subscription Upgrade Modal
- [ ] Level 1, Level 2, Annual, Lifetime comparison table
- [ ] Feature lists for each tier
- [ ] Pricing and savings display
- [ ] [Choose Level 1] [Choose Level 2] [Choose Annual] [Choose Lifetime] buttons
- [ ] [Compare All Tiers] [Maybe Later] buttons

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 12

### 10.11 Keyboard Shortcuts Overlay
- [ ] Full shortcuts reference (Global, Writing, Library sections)
- [ ] [Print Cheat Sheet] [Close] buttons
- [ ] Triggered by Ctrl+?

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 13

### 10.12 Update Available Notification
- [ ] Version number display
- [ ] Release notes summary (What's new)
- [ ] [View Release Notes] link
- [ ] [Download Update] [Remind Me Later] buttons

**Reference:** [09-modals-overlays.md](./09-modals-overlays.md) Section 14

---

## Phase 11: Easter Eggs & Hidden Features (Week 29-30)

### 11.1 Konami Code (Developer Mode)
- [ ] Detect Konami Code (↑ ↑ ↓ ↓ ← → ← → B A Enter)
- [ ] Activation modal (🎮 Developer Mode Activated!)
- [ ] Developer Console (Ctrl+Shift+D)
- [ ] Database inspector (SQL query execution)
- [ ] Performance monitor (FPS, memory usage)
- [ ] "The Developer's Notes" secret artifact unlock

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 1

### 11.2 Records Cavern Secrets
- [ ] Cosmic window (click to change weather: night, dawn, midday, sunset)
- [ ] Moss patch (click to sprout mushrooms)
- [ ] Cat silhouette (1% random chance, click for +10 XP)
- [ ] Bookshelf secret (click specific book for hidden note)

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 2

### 11.3 Canvas Secrets
- [ ] Triple-click empty canvas (random encouraging quote)
- [ ] Type "xyzzy" (text adventure magic word, Muse responds)
- [ ] Type "hello ChronoXP" or "hello Muse" (personal greeting)

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 3

### 11.4 Library Achievements
- [ ] Perfect month detection (entry every day for full month)
- [ ] Golden glow around completed month in Calendar
- [ ] Confetti animation (brief, 2 seconds)
- [ ] +50 XP and "The Chronicle's Seal" artifact unlock
- [ ] Click today's date 7x (time-travel mode modal)

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 4

### 11.5 Search Easter Eggs
- [ ] Search "meaning of life" (philosophical secret message, +10 XP)
- [ ] Search "42" (Hitchhiker's Guide reference, +5 XP)

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 5

### 11.6 Command Bar Secrets
- [ ] "surprise me" command (random action: jump to entry, artifact, prompt, quote)
- [ ] "make it rain" command (confetti animation, +5 XP)
- [ ] "unicorn mode" command (replace artifacts with unicorns, toggleable)

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 6

### 11.7 Settings Secrets
- [ ] Click version number 10x (developer credits modal)

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 7

### 11.8 XP Milestones
- [ ] Write entry at 3:33 AM (+15 XP "Night Owl")
- [ ] Write 1000+ word entry (+25 XP "Epic Entry", unlock "The Endless Scroll")
- [ ] Write entry on birthday (+30 XP "Birthday Reflection", unlock "The Candle of Years")

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 8

### 11.9 First Entry Anniversary
- [ ] Detect one-year anniversary of first entry
- [ ] Anniversary modal (stats: total entries, words, days journaled)
- [ ] Year in Review visualization (timeline, tags, moods, streak)
- [ ] +100 XP and "The Eternal Flame" artifact unlock

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 9

### 11.10 Easter Egg Accessibility
- [ ] Settings → Accessibility → "Disable Easter eggs and surprises" checkbox
- [ ] When enabled: no confetti, no cat, no random prompts, no toasts (but hidden interactions remain)

**Reference:** [10-easter-eggs.md](./10-easter-eggs.md) Section 10

---

## Phase 12: Polish & Testing (Week 31-34)

### 12.1 Visual Design (Hand-Painted Assets)
- [ ] Commission or paint all Cavern tiers (5 variations)
- [ ] Paint all 16 artifacts (closeup + thumbnail versions)
- [ ] Paint UI elements (progress bars, buttons, borders)
- [ ] Ensure consistent painterly style (visible brush strokes)
- [ ] Export assets as high-res PNGs (2x retina displays)

**Reference:** [Gamification Design](../../06-gamification/design.md), [Design System](../../ux/design-system.md)

### 12.2 Accessibility Audit
- [ ] Keyboard navigation test (Tab order, focus indicators)
- [ ] Screen reader test (NVDA, JAWS, VoiceOver)
- [ ] Color contrast check (WCAG AA compliance)
- [ ] Text scaling test (up to 200%)
- [ ] Motion reduction test (prefers-reduced-motion CSS)
- [ ] High contrast mode test
- [ ] Focus indicators visible (3px blue outline)

**Reference:** [Accessibility Checklist](../../ux/a11y-nd-checklist.md)

### 12.3 Performance Optimization
- [ ] App launch time (<400ms target)
- [ ] Canvas render time (<16ms for 60 FPS)
- [ ] Search latency (<500ms for 1000 entries)
- [ ] Autosave latency (<200ms)
- [ ] Database query optimization (indexes, prepared statements)
- [ ] AI inference optimization (model quantization, batching)
- [ ] Asset lazy loading (Cavern images load on tab switch)

**Reference:** Performance targets in each wireframe document

### 12.4 Cross-Platform Testing
- [ ] Windows 10/11 (test window chrome, file paths)
- [ ] macOS (test menu bar, keyboard shortcuts)
- [ ] Linux (Ubuntu, Fedora, Arch) (test window manager compatibility)
- [ ] High-DPI displays (2x, 3x scaling)
- [ ] Small screens (1280x720 minimum)
- [ ] Large screens (4K, ultrawide)

### 12.5 Security Audit
- [ ] SQLCipher encryption verification (AES-256)
- [ ] Vault password hashing (Argon2, bcrypt)
- [ ] Memory wiping on app close (clear passwords, keys)
- [ ] File permission checks (ensure vault.db is private)
- [ ] No network requests (except AI model downloads, which are optional)
- [ ] Crash logs don't leak entry content

**Reference:** [Security & Privacy](../../tech/security-privacy.md)

### 12.6 User Testing
- [ ] Recruit 10 beta testers (diverse backgrounds)
- [ ] Onboarding flow testing (clear? confusing?)
- [ ] Usability testing (can users find features?)
- [ ] Accessibility testing (users with disabilities)
- [ ] Gamification perception testing (motivating? stressful?)
- [ ] Bug reporting system (GitHub Issues)

### 12.7 Documentation
- [ ] User guide (getting started, features, FAQ)
- [ ] Developer documentation (architecture, build instructions)
- [ ] API documentation (internal Rust/TypeScript APIs)
- [ ] Changelog (track all versions)
- [ ] Privacy policy (legal requirements)
- [ ] Terms of service (legal requirements)

---

## Phase 13: Monetization & Distribution (Week 35-38)

### 13.1 Payment Integration
- [ ] Stripe integration (credit card processing)
- [ ] One-time purchase flow (AI Concierge $1.99)
- [ ] Subscription management (Level 1, Level 2, Annual, Lifetime)
- [ ] License key generation and validation
- [ ] Refund handling (30-day money-back guarantee)
- [ ] Student discount verification (SheerID integration)

**Reference:** [Monetization](../../08-product/monetization.md)

### 13.2 App Packaging
- [ ] Tauri bundler configuration (Windows .exe, macOS .dmg, Linux .AppImage/.deb)
- [ ] Code signing certificates (Windows Authenticode, macOS Developer ID)
- [ ] Auto-update system (Tauri updater)
- [ ] Version numbering (semantic versioning: 1.0.0)
- [ ] Release notes generation (auto-generate from Git commits)

### 13.3 Distribution Channels
- [ ] Official website (chronoxp.app) with download links
- [ ] GitHub Releases (open-source distribution)
- [ ] Microsoft Store (Windows)
- [ ] Mac App Store (macOS)
- [ ] Flathub (Linux)
- [ ] itch.io (indie game platform, optional)

### 13.4 Marketing & Launch
- [ ] Landing page (features, pricing, screenshots)
- [ ] Demo video (2-minute walkthrough)
- [ ] Social media accounts (Twitter, Mastodon, Reddit)
- [ ] Press kit (logo, screenshots, press release)
- [ ] Beta tester testimonials (social proof)
- [ ] Launch on Product Hunt, Hacker News, Reddit r/selfhosted

---

## Phase 14: Post-Launch (Ongoing)

### 14.1 Monitoring & Analytics
- [ ] Error tracking (Sentry or self-hosted equivalent, opt-in only)
- [ ] Crash reports (user can choose to send or not)
- [ ] Feature usage stats (anonymized, opt-in)
- [ ] Performance metrics (app launch time, search latency)

### 14.2 Bug Fixes & Updates
- [ ] GitHub Issues triage (respond within 48 hours)
- [ ] Bug fix releases (patch versions: 1.0.1, 1.0.2)
- [ ] Security patches (critical updates within 24 hours)
- [ ] User-reported bugs prioritized

### 14.3 Feature Roadmap
- [ ] Voting system for Level 2 subscribers (feature requests)
- [ ] Quarterly feature releases (minor versions: 1.1.0, 1.2.0)
- [ ] Long-term features (LAN sync, mobile apps, voice journaling)

**Reference:** [Tier Roadmap](../../08-product/tier-roadmap.md)

---

## Summary

**Total Tasks:** 156  
**Estimated Timeline:** 38 weeks (~9 months)  
**Solo Developer:** Realistic with aggressive focus  
**Team of 2-3:** Could complete in 6 months

**Next Steps:**
1. Complete Phase 0 (Project Setup)
2. Build vertical slice (Phases 1-2: App Shell + Canvas)
3. Test with early users
4. Iterate based on feedback
5. Continue phased rollout

**Key Success Metrics:**
- App launches in <400ms
- Autosave completes in <200ms
- Zero data loss (comprehensive error handling)
- WCAG AA accessibility compliance
- Positive beta tester feedback on gamification (motivating, not stressful)

---

**End of Wireframe Implementation Todo List**
