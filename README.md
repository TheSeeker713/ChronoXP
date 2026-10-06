# ChronoXP™ Journal

> **A private, neuro-affirming, offline-first journaling companion.**  
> Encrypted at rest. Local-only. Yours forever.

---

## Status

**MVP (Phases 0–3) — active development**

| Phase | Description | Status |
|-------|-------------|--------|
| 0 | Product freeze + `DECISIONS.md` | ✅ Done |
| 1 | Tauri 2 skeleton + data model + migrations | ✅ Done |
| 2 | Security core (AES-256-GCM, keychain, app lock, Argon2id) | ✅ Done |
| 3 | Desktop MVP (timeline, TipTap editor, tags, FTS, export/import) | ✅ Done |
| 4 | Encrypted Google Drive backup, full importers, biometric | ⏳ Next |
| 5 | Local AI (Muse, Archivist, Concierge via llama.cpp) | ⏳ Planned |
| 6 | Mobile (iOS/Android via Tauri 2 mobile) | ⏳ Planned |

---

## How to Run

### Prerequisites

**Windows & macOS:**

```bash
# Install Node.js 22+ from https://nodejs.org
# Install Rust from https://rustup.rs

# Windows: Install Visual Studio Build Tools (C++ workload)
# macOS: Install Xcode Command Line Tools
xcode-select --install
```

**Linux (Ubuntu/Debian):**

```bash
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  patchelf
```

### Development

```bash
git clone https://github.com/TheSeeker713/chronoxp
cd chronoxp

npm install
npm run tauri dev
```

The Vite dev server starts on `http://localhost:1420` and the Tauri window opens automatically.
Rust recompiles on save; React/TS reloads via Vite HMR.

### Production Build

```bash
npm run tauri build
```

Output: `src-tauri/target/release/bundle/`

### Tests

```bash
# Frontend (Vitest)
npm test

# Rust unit tests
cd src-tauri && cargo test
```

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Shell | Tauri 2.x (Rust + WebView) |
| Frontend | React 19 + TypeScript 5 |
| Build | Vite 6 + `@tailwindcss/vite` |
| Styles | Tailwind CSS 4 |
| Rich text | TipTap 2 (ProseMirror) |
| State | Zustand 5 |
| Backend | Rust 1.83 |
| Database | SQLite via `rusqlite` (bundled) |
| Encryption | AES-256-GCM (`aes-gcm` crate) |
| KDF | Argon2id (`argon2` crate) |
| Keychain | OS-native (`keyring` crate) |
| Tests | Vitest + cargo test |

See [`docs/DECISIONS.md`](docs/DECISIONS.md) for the full architecture decision record.

---

## Security

- **Field-level AES-256-GCM** on all `markdown` content. SQLite file is readable metadata only.
- **Master key** stored in OS keychain (Windows Credential Manager / macOS Keychain / libsecret).
- **Vault password** (optional): Argon2id(m=64MiB, t=3, p=1) derives a wrap key; master key is
  wrapped and stored in the `settings` table. Even with DB access, no plaintext without password.
- **Recovery key**: 52-char base32 of the raw master key, shown **once** on first setup.
- **Export encryption**: AES-256-GCM + PBKDF2-SHA256 (100,000 iterations) per-archive password.
- **App lock**: configurable idle timeout (default 15 min); key zeroed from memory on lock.

---

## Architecture

```
chronoxp/
├── src/                    # React 19 frontend
│   ├── components/         # LockScreen, Canvas, Library, Search, etc.
│   ├── stores/             # Zustand (auth, entries, settings)
│   ├── utils/              # IPC wrappers, time helpers
│   └── types/              # Shared TypeScript types
├── src-tauri/              # Rust backend (Tauri 2)
│   └── src/
│       ├── lib.rs          # Tauri builder + command registration
│       ├── crypto/         # AES-GCM, Argon2id, keychain
│       ├── db/             # SQLite schema + migrations
│       ├── commands/       # entries, tags, search, export, security, settings
│       ├── backup/         # BackupProvider trait (stub)
│       └── importers/      # Importer trait + stubs (Day One, Apple Journal, Markdown)
├── docs/
│   ├── DECISIONS.md        # Architecture decision record
│   └── ...                 # Product docs (vision, wireframes, flows, etc.)
└── .github/workflows/
    └── ci.yml              # Windows + macOS + Linux builds + tests
```

---

## Data

- **Location**: OS app-data directory
  - Windows: `%APPDATA%\app.chronoxp.journal\journal.db`
  - macOS: `~/Library/Application Support/app.chronoxp.journal/journal.db`
  - Linux: `~/.local/share/app.chronoxp.journal/journal.db`
- **No cloud**: zero network calls from the app binary
- **Export**: `.chrxp` (ZIP, optionally AES-256-GCM encrypted)

---

## License

TBD — options under consideration: GPL-3.0, AGPL-3.0, Fair Source.  
The code is Jeremy's own work. See [`docs/DECISIONS.md`](docs/DECISIONS.md#7-license).

---

## Philosophy

> *"Your thoughts. Your device. Your pace. No judgment. No tracking. Just a safe space to write, remember, and grow."*
