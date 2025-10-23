# System Architecture Overview

**Project:** ChronoXP™ Journal  
**Date:** October 22, 2025

---

## Architecture Principles

1. **Offline-first:** All features work without network connection
2. **Privacy by default:** No data leaves device without explicit user action
3. **CPU-friendly:** Runs on modest hardware (16 GB RAM, no GPU required)
4. **Fast startup:** < 400ms cold launch on mid-spec Windows 11
5. **Portable:** Cross-platform (Windows, macOS, Linux)

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                     PRESENTATION                    │
│  ┌───────────────────────────────────────────────┐  │
│  │   React + TypeScript + Vite + Tailwind CSS   │  │
│  │   (WebView front-end)                        │  │
│  └───────────────────────────────────────────────┘  │
│         │ ▲                                          │
│         │ │ IPC (invoke/emit)                        │
│         ▼ │                                          │
│  ┌───────────────────────────────────────────────┐  │
│  │         Tauri Core (Rust Backend)            │  │
│  │  • Entry CRUD                                │  │
│  │  • Search (keyword + semantic)               │  │
│  │  • Export/import (.chrxp)                    │  │
│  │  • Settings management                       │  │
│  │  • File system access                        │  │
│  └───────────────────────────────────────────────┘  │
│         │ ▲                                          │
│         │ │                                          │
│         ▼ │                                          │
│  ┌───────────────────────────────────────────────┐  │
│  │         AI Runtime (llama.cpp)               │  │
│  │  • Muse (inline suggestions)                 │  │
│  │  • Archivist (embeddings, tagging)           │  │
│  │  • Concierge (command routing)               │  │
│  └───────────────────────────────────────────────┘  │
│         │ ▲                                          │
│         │ │                                          │
│         ▼ │                                          │
│  ┌───────────────────────────────────────────────┐  │
│  │         Data Layer                           │  │
│  │  • SQLCipher (encrypted SQLite)              │  │
│  │  • Vector store (sqlite-vec or similar)      │  │
│  │  • File attachments (encrypted at rest)      │  │
│  └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

---

## Technology Stack

### Shell: Tauri
**Why Tauri:**
- Rust backend (performance, security, memory safety)
- Native WebView (small bundle size vs. Electron)
- Cross-platform (Windows, macOS, Linux)
- Secure IPC (invoke/emit pattern)
- Native file system access

**Version:** Tauri 2.x (latest stable)

---

### Front-End: React + TypeScript + Vite + Tailwind CSS

#### React
- **Version:** React 18+
- **State management:** Zustand (lightweight, simple)
- **Routing:** React Router (for surfaces: Canvas, Library, Cavern, Settings)

#### TypeScript
- **Version:** 5.x
- **Strict mode:** Enabled
- **Type safety:** Full coverage (no `any` types in production code)

#### Vite
- **Build tool:** Fast HMR, optimized production builds
- **Dev server:** < 100ms reload

#### Tailwind CSS
- **Why:** Utility-first, low-motion theme easy to implement
- **Customization:**
  - Custom color palette (low-stimulation defaults)
  - Reduced motion utilities
  - Dyslexia-friendly font integration

---

### Storage: SQLCipher (Encrypted SQLite)

#### Database
- **Engine:** SQLite 3.x with SQLCipher extension
- **Encryption:** AES-256
- **Location:** OS-specific app data directory
  - Windows: `%APPDATA%\ChronoXP\data.db`
  - macOS: `~/Library/Application Support/ChronoXP/data.db`
  - Linux: `~/.config/ChronoXP/data.db`

#### Attachments
- **Storage:** File system (not in DB)
- **Location:** `%APPDATA%\ChronoXP\attachments\`
- **Encryption:** At-rest encryption (OS-level or AES-256-GCM per file)
- **Naming:** UUIDs (e.g., `a3f2b9c4-5d6e-7f8g.jpg`)

#### Vector Store
- **Option A (MVP):** `sqlite-vec` extension (SQLite plugin for vector search)
- **Option B (future):** FAISS-lite or Hnswlib (if performance needs improve)
- **Storage:** BLOB column in `embeddings` table

---

### AI Runtime: llama.cpp

#### Why llama.cpp
- **CPU-friendly:** Optimized for x86/ARM CPUs (no GPU required)
- **GGUF support:** Quantized models (4-bit, 8-bit)
- **Small footprint:** 500MB-2GB RAM for inference (depending on model)
- **Offline:** No network calls

#### Integration
- **Method 1 (Preferred):** Rust FFI bindings to `llama.cpp` C++ library
- **Method 2 (Fallback):** Invoke `llama-cli` binary via subprocess

#### Models (Optional Downloads)
- **Muse (suggestions):** Phi-3-mini-4k-instruct (3.8B params, ~2.3GB quantized)
- **Archivist (embeddings):** nomic-embed-text-v1.5 (~500MB)
- **Concierge (NLU):** Llama-3.2-1B-Instruct (~1GB)

**Download location:** `%APPDATA%\ChronoXP\models\`

**First-run wizard:**
1. "ChronoXP can use AI for smart suggestions and search."
2. "Download recommended models? (2.5 GB total)"
3. `[Download Now]` | `[Skip — I'll add models later]`

---

## Data Flow Examples

### Example 1: Write Entry with Muse Suggestion

```
┌─────────────┐
│   User      │ Types text
└─────┬───────┘
      │
      ▼
┌─────────────┐
│  React UI   │ Debounce 300ms → Send to backend
└─────┬───────┘
      │ IPC invoke("get_suggestion", { text: "..." })
      ▼
┌─────────────┐
│ Tauri Core  │ Calls llama.cpp via FFI
└─────┬───────┘
      │
      ▼
┌─────────────┐
│ llama.cpp   │ Generates completion (local inference)
└─────┬───────┘
      │
      ▼
┌─────────────┐
│ Tauri Core  │ Returns suggestion to UI
└─────┬───────┘
      │ IPC emit("suggestion_ready", { text: "..." })
      ▼
┌─────────────┐
│  React UI   │ Displays greyed-out inline text
└─────────────┘
```

**Latency:** ~200-500ms (depending on CPU, context length)

---

### Example 2: Semantic Search

```
┌─────────────┐
│   User      │ Enters query: "family stress"
└─────┬───────┘
      │
      ▼
┌─────────────┐
│  React UI   │ Debounce 500ms → Send to backend
└─────┬───────┘
      │ IPC invoke("search", { query: "family stress" })
      ▼
┌─────────────┐
│ Tauri Core  │ 1. Generate query embedding via llama.cpp
│             │ 2. Cosine similarity with stored embeddings
│             │ 3. Fetch top 10 results from SQLite
└─────┬───────┘
      │
      ▼
┌─────────────┐
│  SQLCipher  │ SELECT entries WHERE ...
└─────┬───────┘
      │
      ▼
┌─────────────┐
│ Tauri Core  │ Returns ranked results
└─────┬───────┘
      │ IPC emit("search_results", { results: [...] })
      ▼
┌─────────────┐
│  React UI   │ Displays results in search modal
└─────────────┘
```

**Latency:** < 500ms (target)

---

### Example 3: Export .chrxp

```
┌─────────────┐
│   User      │ Clicks "Export as .chrxp"
└─────┬───────┘
      │
      ▼
┌─────────────┐
│  React UI   │ Shows export options dialog
└─────┬───────┘
      │ IPC invoke("export", { options: {...}, password: "..." })
      ▼
┌─────────────┐
│ Tauri Core  │ 1. Create temp directory
│             │ 2. Export entries to Markdown
│             │ 3. Copy attachments
│             │ 4. Backup SQLite DB
│             │ 5. ZIP all files
│             │ 6. Encrypt ZIP (if password provided)
│             │ 7. Write to user-selected path
└─────┬───────┘
      │
      ▼
┌─────────────┐
│ File System │ chronoxp_journal_2025-10-22.chrxp
└─────┬───────┘
      │
      ▼
┌─────────────┐
│ Tauri Core  │ Returns success
└─────┬───────┘
      │ IPC emit("export_complete")
      ▼
┌─────────────┐
│  React UI   │ Shows success toast
└─────────────┘
```

**Time:** < 10s (for 1000 entries + attachments)

---

## Security Architecture

### Encryption Layers

1. **Database (SQLCipher):**
   - AES-256 encryption at rest
   - Key stored in OS keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service)

2. **Optional Vault Password:**
   - User-provided password → PBKDF2 key derivation
   - Second encryption layer (for paranoid users)

3. **Export Encryption:**
   - AES-256-GCM for `.chrxp` files
   - Password-based (not tied to OS keychain)

### Threat Model (See `/docs/09-security/privacy.md`)
- **Protects against:**
  - Unauthorized local file access (encryption at rest)
  - Data exfiltration by malicious apps (no network calls)
  - Cloud provider breaches (no cloud storage)
- **Does NOT protect against:**
  - Physical access to unlocked device (OS-level concern)
  - Keyloggers, screen recorders (OS/antivirus concern)
  - State-level adversaries (out of scope)

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Cold launch | < 400ms | Time to cursor-ready in Canvas |
| Autosave | < 3s | From last keystroke to DB write |
| Search (semantic) | < 500ms | Query to results displayed |
| Muse suggestion | < 300ms | From pause to inline text |
| Export (.chrxp) | < 10s | For 1000 entries + attachments |
| Memory usage (idle) | < 200 MB | With AI models loaded |
| Memory usage (active) | < 500 MB | During inference |

---

## Deployment & Distribution

### Packaging
- **Windows:** `.exe` installer (NSIS or WiX)
- **macOS:** `.dmg` with app bundle
- **Linux:** `.AppImage`, `.deb`, `.rpm`

### Auto-Updates
- **Update check:** GitHub Releases API (privacy-safe)
- **Download:** User-initiated (not automatic)
- **Notification:** "Version X.Y.Z available. [Download] [Skip]"

### Model Distribution
- **Models NOT bundled** (too large for app package)
- **First-run wizard:** Optional download
- **Manual download:** Settings → AI → Download Models

---

## Folder Structure (Tauri Project)

```
chronoxp/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # Tauri entry point
│   │   ├── db.rs        # SQLCipher operations
│   │   ├── ai.rs        # llama.cpp integration
│   │   ├── export.rs    # .chrxp export/import
│   │   └── search.rs    # Semantic search logic
│   ├── Cargo.toml       # Rust dependencies
│   └── tauri.conf.json  # Tauri configuration
│
├── src/                 # React front-end
│   ├── components/      # UI components
│   ├── pages/           # Canvas, Library, Cavern, Settings
│   ├── stores/          # Zustand state management
│   ├── utils/           # Helper functions
│   └── main.tsx         # React entry point
│
├── public/              # Static assets
├── models/              # AI models (optional downloads)
├── docs/                # Documentation (this folder)
└── package.json         # Node dependencies
```

---

## Development Workflow

### Local Development
1. Install dependencies:
   ```bash
   npm install
   cd src-tauri && cargo build
   ```

2. Run dev server:
   ```bash
   npm run tauri dev
   ```
   - Vite dev server (React): `http://localhost:5173`
   - Tauri window opens automatically

3. Hot reload: Vite HMR for React; Rust recompiles on save

### Production Build
```bash
npm run tauri build
```
- Outputs: `src-tauri/target/release/bundle/`
- Includes installers for current OS

---

## Testing Strategy

### Unit Tests
- **Rust (backend):** `cargo test`
- **React (front-end):** Jest + React Testing Library

### Integration Tests
- **Tauri IPC:** Mock invoke/emit calls
- **Database:** In-memory SQLite for tests

### End-to-End Tests
- **Tool:** Playwright or Tauri's WebDriver
- **Scenarios:**
  - Write entry → autosave → search → find entry
  - Export → import → verify integrity
  - Enable Muse → type → accept suggestion

### Accessibility Tests
- **Tool:** axe-core (automated WCAG checks)
- **Manual:** Keyboard-only navigation, screen reader (NVDA, JAWS, VoiceOver)

---

## CI/CD Pipeline (Future)

### GitHub Actions
1. **On push to main:**
   - Run Rust tests
   - Run React tests
   - Build for all platforms (Windows, macOS, Linux)
   - Upload artifacts to GitHub Releases

2. **On tag (e.g., `v1.0.0`):**
   - Create production builds
   - Sign installers (Windows: Authenticode, macOS: Notarization)
   - Publish to GitHub Releases
   - Update website download links

---

## Open Questions (To Resolve Before MVP)

1. **Vector store:** `sqlite-vec` vs. FAISS-lite? (Benchmark performance)
2. **Model hosting:** GitHub Releases vs. CDN? (Bandwidth concerns)
3. **Vault password:** Required or optional? (UX vs. security trade-off)
4. **Concierge NLU:** Rule-based or LLM for MVP? (Complexity vs. intelligence)

---

## References
- [Tauri Docs](https://tauri.app/v2/guides/)
- [llama.cpp GitHub](https://github.com/ggerganov/llama.cpp)
- [SQLCipher](https://www.zetetic.net/sqlcipher/)
- [sqlite-vec](https://github.com/asg017/sqlite-vec)
