# Product Pillars → Features Mapping

**Project:** ChronoXP™ Journal  
**Date:** October 22, 2025

---

## Overview

ChronoXP is built on four foundational pillars. Each pillar represents a core promise to users, with specific features that deliver on that promise.

---

## Pillar 1: Offline AI Concierge

### Core Promise
**Intelligent journaling assistance that runs entirely on your device — no internet required, no data leaves your machine.**

### Features

#### 1.1 Muse (Inline Writing Assistant)
- **What it does:** Suggests next-line completions or alternative phrasings as you write
- **How it works:**
  - Powered by local GGUF model (e.g., Phi-3-mini, Llama-3.2-1B)
  - Appears as greyed-out inline text
  - Accept with `Tab` or `Alt+Enter`; ignore by continuing to type
- **Opt-in:** Disabled by default; enable in Settings → AI
- **Privacy:** All processing happens on-device via `llama.cpp`

#### 1.2 Archivist (Background Knowledge Extraction)
- **What it does:** Automatically extracts mood, topics, and keywords from entries; builds local vector store for semantic search
- **How it works:**
  - Runs as a background task (debounced after 3 seconds of no typing)
  - Generates embeddings using lightweight local model
  - Stores vectors in local DB (SQLite with `sqlite-vec` extension)
  - Does NOT send data to external APIs
- **User control:** Can disable auto-tagging in Settings; manual tags always override

#### 1.3 Concierge (Natural Language Command Router)
- **What it does:** Interprets natural language commands via the Command Bar (⌘/Ctrl+K)
- **Examples:**
  - "Summarize this week" → generates local summary
  - "Find entries about family" → semantic search
  - "Set a daily nudge at 8 PM" → schedules local notification
  - "Show me my most-used words" → local text analysis
- **How it works:**
  - Intent classifier (lightweight NLU model or rule-based for MVP)
  - Routes to appropriate function (search, analytics, settings, etc.)
  - All processing local; no cloud API calls

### Technical Stack
- **Runtime:** `llama.cpp` (bundled binary, invoked via Rust)
- **Models:** GGUF format, optional download (3-7B parameter models quantized to 4-bit)
- **Storage:** Embeddings in SQLite BLOB; index with `sqlite-vec` or FAISS-lite
- **Performance target:** Suggestions within 300ms; search within 500ms

---

## Pillar 2: Neuro-Affirming UX

### Core Promise
**A calm, accessible, keyboard-first interface designed for neurodivergent users — but welcoming to everyone.**

### Features

#### 2.1 Low-Stimulation Theme
- **Visual design:**
  - Muted color palette (soft blues, grays, off-white)
  - Minimal animations (reduced motion by default)
  - Single-column focus (no cluttered sidebars)
  - High contrast mode option (WCAG AA compliance)
- **Customization:**
  - Dark mode / light mode / custom themes
  - Adjustable UI density (compact / comfortable / spacious)

#### 2.2 Dyslexia-Friendly Typography
- **Font options:**
  - OpenDyslexic
  - Comic Sans MS (proven dyslexia-friendly)
  - Atkinson Hyperlegible
  - System default
- **Typography controls:**
  - Adjustable line spacing (1.0x to 2.0x)
  - Letter spacing adjustment
  - Font size (12px to 24px)

#### 2.3 Full Keyboard Navigation
- **Every action has a shortcut** (see `/docs/07-ux/keyboard-map.md`)
- **Examples:**
  - `Ctrl/Cmd+K`: Open Command Bar
  - `Ctrl/Cmd+N`: New entry
  - `Ctrl/Cmd+F`: Search
  - `Ctrl/Cmd+,`: Settings
  - `Esc`: Close modal / exit search
  - `Tab` / `Shift+Tab`: Navigate UI elements
- **Visual focus indicators:** Thick, high-contrast outlines

#### 2.4 Microcopy Principles: "Invite, Don't Judge"
- **Examples:**
  - ❌ "You haven't written in 7 days!"
  - ✅ "Welcome back. Want to pick up where you left off?"
  - ❌ "Streak broken!"
  - ✅ "Take your time. No pressure."
- **Documented in:** `/docs/07-ux/microcopy-guide.md`

#### 2.5 Simple Mode (Gamification Toggle)
- **What it does:** Hides all XP, artifacts, and Records Cavern
- **When to use:** For users who find gamification stressful or distracting
- **Accessible via:** Settings → Appearance → Enable Simple Mode

### Design Principles
1. **Reduce cognitive load at every step**
2. **No forced decisions upfront** (defaults should "just work")
3. **Visual hierarchy: writing first, everything else second**
4. **Respect sensory sensitivities** (motion, color, sound)

---

## Pillar 3: Safe Gamification

### Core Promise
**Progress feels like discovery, not obligation. XP and unlocks celebrate what you've done, never shame what you haven't.**

### Features

#### 3.1 Records Cavern (2.5D Progress Visualization)
- **Visual style:**
  - Cozy, low-poly 2.5D isometric room
  - Parallax scrolling (subtle, respects reduced motion)
  - Warm lighting (lanterns, fireflies)
- **How it evolves:**
  - Starts as a small, empty cave with a single desk
  - Unlocks artifacts (books, plants, paintings, trinkets) as XP accrues
  - No deadlines or "decay" — artifacts stay forever
- **Artifacts:**
  - Cosmetic only (no gameplay advantage)
  - Thematic (e.g., "Ancient Scroll" for 10 entries, "Memory Jar" for 50)
  - Can be placed/arranged in the Cavern (future feature)

#### 3.2 XP System (Rewarding Starting & Returning)
- **XP sources:**
  - **Write an entry:** 10 XP (minimum 50 words)
  - **Return after 3+ days:** 5 bonus XP ("Welcome back!")
  - **Complete a reflection prompt:** 15 XP
  - **Tag 5 entries:** 5 XP (one-time per batch)
  - **Use semantic search:** 2 XP (encourages engagement)
- **What XP does NOT reward:**
  - ❌ Daily streaks (too much pressure)
  - ❌ Word count grinding (quality over quantity)
  - ❌ Forced actions (e.g., "share with friends")
- **Leveling:**
  - XP unlocks new artifacts at set thresholds
  - No competitive leaderboards
  - No visible "level" number — just progress bar

#### 3.3 Optional Quests (Gentle Reflection Loops)
- **Examples:**
  - "Revisit: Read three entries from last month"
  - "Reflect: Write about a recurring theme in your journal"
  - "Explore: Try the 'Gratitude' template"
- **Design:**
  - Suggested, never required
  - Can be dismissed forever
  - No penalties for ignoring
  - Rewards are small XP boosts + artifact unlocks

#### 3.4 No Streak Shaming
- **What we don't do:**
  - ❌ "You're on a 14-day streak! Don't break it!"
  - ❌ Red X's on calendar for missed days
  - ❌ Notifications like "Where have you been?"
- **What we do instead:**
  - ✅ Celebrate when you *do* write
  - ✅ Show gentle "return bonus" XP
  - ✅ Calendar shows activity, but gaps are just empty (not red)

### Gamification Philosophy
- **Intrinsic > Extrinsic:** XP celebrates internal growth, not external comparison
- **Discovery > Achievement:** Artifacts are found, not earned
- **Optional > Mandatory:** Everything can be turned off

---

## Pillar 4: Privacy by Default

### Core Promise
**Your data never leaves your device without your explicit action. No tracking. No telemetry. No cloud lock-in.**

### Features

#### 4.1 Encrypted Local Storage
- **Database:** SQLCipher (AES-256 encryption)
- **Vault password:** Optional second layer of encryption (for paranoid users)
- **Attachments:** Stored in `appdata/attachments/`, encrypted at rest
- **Keys:** Derived from OS keychain (Windows Credential Manager, macOS Keychain)

#### 4.2 On-Device Vector Store
- **Embeddings:** Stored locally in SQLite BLOB
- **Index:** `sqlite-vec` extension or lightweight FAISS alternative
- **No external API:** All embedding generation happens via local GGUF model

#### 4.3 No Telemetry
- **What we don't collect:**
  - ❌ Entry content
  - ❌ Search queries
  - ❌ Usage patterns
  - ❌ Device info (beyond OS for compatibility)
  - ❌ Crash reports (without opt-in)
- **What we do collect (optional):**
  - ✅ Anonymous feature usage (e.g., "X% of users enable Simple Mode") — only if user opts in during setup
  - ✅ Crash logs — only if user clicks "Send Report"

#### 4.4 Explicit Export Only
- **`.chrxp` format:**
  - ZIP archive containing:
    - `entries/` (Markdown files, one per entry)
    - `attachments/` (images, files)
    - `meta/` (tags, XP, settings as JSON)
    - `database.db` (full SQLite backup)
    - `vectors.bin` (optional, for semantic search portability)
    - `manifest.json` (version, export date, integrity hash)
- **Export options:**
  - Unencrypted (for portability)
  - AES-256-GCM encrypted (requires password to re-import)
- **Import:** One-click restore from `.chrxp` file

#### 4.5 No Auto-Updates Without Consent
- **Update model:**
  - App checks for updates (GitHub Releases API)
  - User sees notification: "Version X.Y.Z available"
  - User chooses: "Download now" | "Remind me later" | "Skip this version"
- **Model downloads:**
  - AI models are NOT bundled (too large)
  - First-run wizard offers download (optional)
  - Can use app without AI features if models not downloaded

### Threat Model
- **What we protect against:**
  - ✅ Unauthorized access to local data (encryption at rest)
  - ✅ Data exfiltration by malicious apps (no network calls)
  - ✅ Cloud provider breaches (no cloud storage)
- **What we don't protect against:**
  - ❌ Physical access to unlocked device (OS-level concern)
  - ❌ Keyloggers, screen recorders (OS/antivirus concern)
  - ❌ State-level adversaries (not in scope)

---

## Pillar Summary Table

| Pillar | Key Features | User Benefit | Technical Approach |
|--------|-------------|--------------|-------------------|
| **Offline AI Concierge** | Muse, Archivist, Concierge | Intelligent help without privacy trade-offs | `llama.cpp` + GGUF models |
| **Neuro-Affirming UX** | Low-stim theme, keyboard nav, dyslexia fonts | Accessible to neurodivergent users | Design system + a11y standards |
| **Safe Gamification** | Records Cavern, XP, no streak shaming | Progress feels rewarding, not stressful | Visual design + ethical game mechanics |
| **Privacy by Default** | Encrypted DB, no telemetry, explicit export | Complete data ownership and trust | SQLCipher + local-only architecture |

---

## Feature Prioritization for MVP

### P0 (Must ship in MVP)
- ✅ Offline-first architecture
- ✅ Encrypted SQLite storage
- ✅ Basic writing canvas (rich text)
- ✅ Autosave
- ✅ Keyword + semantic search (basic)
- ✅ Full keyboard navigation
- ✅ Low-stimulation theme
- ✅ `.chrxp` export/import
- ✅ Simple Mode toggle

### P1 (Ship within 1-2 months of MVP)
- ⚠️ Muse (inline AI suggestions)
- ⚠️ Archivist (auto-tagging, embeddings)
- ⚠️ Records Cavern (basic version)
- ⚠️ XP system
- ⚠️ Dyslexia-friendly fonts

### P2 (Future enhancements)
- 🔮 Concierge (advanced NLU)
- 🔮 Quests / reflection prompts
- 🔮 Artifact placement in Cavern
- 🔮 Advanced analytics (word clouds, mood trends)
- 🔮 LAN sync (optional, explicit)

