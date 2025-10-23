# Acceptance Criteria for MVP

**Project:** ChronoXP™ Journal  
**MVP Target Date:** TBD  
**Last Updated:** October 22, 2025

---

## Definition of MVP

**Minimum Viable Product** = The simplest version of ChronoXP that delivers core value to users.

**Core value:** Write, save, search, and export journal entries privately, with optional AI assistance and gentle gamification.

---

## MVP Feature Scope

### ✅ In Scope (Must Ship)
1. **Journal Canvas** (write, autosave, rich text editing)
2. **Library** (calendar view, tag browsing, basic search)
3. **Search** (keyword + basic semantic search via local AI)
4. **Export/Import** (`.chrxp` format with Markdown + attachments)
5. **Settings** (themes, fonts, AI toggle, Simple Mode)
6. **Encryption** (SQLCipher database, OS keychain integration)
7. **Keyboard navigation** (full keyboard-only support)
8. **Accessibility** (WCAG AA, dyslexia-friendly fonts, reduced motion)
9. **AI (optional):** Muse (inline suggestions), Archivist (embeddings + auto-tags)
10. **Gamification (optional):** Records Cavern (basic), XP, 5-8 artifacts

### ❌ Out of Scope (Post-MVP)
- Concierge (advanced NLU commands) — rule-based only for MVP
- Quests / reflection prompts — too complex
- Advanced analytics (mood trends, word clouds) — Pro feature, later
- LAN sync — future feature
- Mobile apps (iOS, Android) — desktop-first
- Artifact placement / customization — post-MVP polish
- Seasonal events / special artifacts — post-MVP

---

## Acceptance Criteria (Functional)

### 1. Journal Canvas

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **Write entry** | User can type text in Canvas; cursor auto-focused on launch | [ ] |
| **Autosave** | Entry saved within 3 seconds of last keystroke | [ ] |
| **Rich text formatting** | Bold, italic, headings (H1-H3), lists work correctly | [ ] |
| **Inline images** | User can paste or drag-and-drop image; displays in entry | [ ] |
| **Muse suggestions (if enabled)** | Greyed-out next-line suggestion appears after 300ms pause | [ ] |
| **Accept Muse** | Pressing `Tab` accepts suggestion; text becomes black | [ ] |
| **Dismiss Muse** | Pressing `Esc` removes suggestion; Muse pauses 30s | [ ] |
| **Tag input** | User can add tags via `Ctrl/Cmd+T`; autocomplete works | [ ] |
| **Word count** | Status bar shows accurate word count in real-time | [ ] |
| **Navigate entries** | `[<]` / `[>]` arrows or `Ctrl/Cmd+[` / `]` navigate by date | [ ] |

---

### 2. Library

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **Calendar view** | Calendar shows dots on dates with entries | [ ] |
| **Click date** | Clicking calendar date opens entry in Canvas | [ ] |
| **Tag browsing** | Clicking tag filters entries by that tag | [ ] |
| **All Entries list** | Shows chronological list (newest first) with previews | [ ] |
| **Search bar** | Typing query shows results (keyword + semantic) | [ ] |

---

### 3. Search

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **Semantic search (5 test prompts)** | See Test Cases below; top 3 results relevant | [ ] |
| **Keyword fallback** | If semantic disabled, keyword search still works | [ ] |
| **Performance** | Results appear in < 500ms for 1000-entry journal | [ ] |
| **Keyboard navigation** | Arrow keys navigate results; `Enter` opens entry | [ ] |
| **No results** | Shows helpful message ("No entries found. Try...") | [ ] |

**Test Cases (Semantic Search):**
1. Query: "entries about job stress" → Should find "work burnout", "career pressure"
2. Query: "family and health" → Should find entries mentioning both topics
3. Query: "when did I write about moving?" → Should return relocation-related entries
4. Query: "times I felt anxious" → Should return anxious/worried entries
5. Query: "important thoughts" → Should return substantial reflections (high word count or pinned)

---

### 4. Export / Import

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **Export .chrxp** | Clicking "Export" creates valid ZIP file | [ ] |
| **Markdown files** | Entries exported as readable `.md` files with YAML front matter | [ ] |
| **Attachments included** | Images/files copied to `/attachments/` folder | [ ] |
| **Manifest valid** | `manifest.json` contains correct metadata + checksum | [ ] |
| **Import .chrxp** | Importing file restores all entries, tags, attachments | [ ] |
| **Integrity check** | Corrupted file is rejected with clear error message | [ ] |
| **Export encryption (optional)** | Password-protected export decrypts correctly on import | [ ] |
| **Duplicate handling** | Importing same file twice skips duplicates | [ ] |

---

### 5. Settings

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **Theme toggle** | Light/Dark mode switches correctly | [ ] |
| **Font selection** | OpenDyslexic, Atkinson, Comic Sans, System fonts apply | [ ] |
| **Font size** | Adjustable from 12px to 24px | [ ] |
| **Line spacing** | Adjustable from 1.0x to 2.0x | [ ] |
| **Reduced motion** | Animations disabled when toggled on | [ ] |
| **Simple Mode** | Hides XP, Cavern, gamification when enabled | [ ] |
| **AI toggles** | Muse, Archivist can be individually disabled | [ ] |
| **Settings persist** | Changes saved and restored on app restart | [ ] |

---

### 6. Encryption & Security

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **SQLCipher enabled** | Database file is encrypted (cannot open in plain SQLite) | [ ] |
| **Key stored in OS keychain** | Encryption key retrieved from Windows Credential Manager / macOS Keychain | [ ] |
| **Optional vault password** | If set, app prompts for password on launch | [ ] |
| **Wrong vault password** | Incorrect password shows error; does not corrupt data | [ ] |
| **No telemetry** | Network monitor shows zero outbound connections (except update check) | [ ] |

---

### 7. Keyboard Navigation

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **All features accessible** | Tester completes 10 tasks using keyboard only (no mouse) | [ ] |
| **Focus visible** | All interactive elements show thick blue outline when focused | [ ] |
| **Tab order logical** | Tab moves top→bottom, left→right | [ ] |
| **Esc closes modals** | Pressing `Esc` closes search, settings, artifact dialogs | [ ] |
| **Shortcuts work** | See keyboard map; test 20 common shortcuts | [ ] |

---

### 8. Accessibility (WCAG AA)

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **Contrast ratios** | All text meets 4.5:1 (normal) or 3:1 (large) | [ ] |
| **Screen reader (NVDA)** | Tester navigates app with screen reader; no confusing labels | [ ] |
| **Keyboard-only** | All tasks completable without mouse | [ ] |
| **Resizable text** | Zoom to 200%; no horizontal scrolling, all text readable | [ ] |
| **Reduced motion** | No flashing, no jarring animations when setting enabled | [ ] |
| **Dyslexia-friendly fonts** | OpenDyslexic renders correctly; line spacing adjustable | [ ] |

---

### 9. AI Features (Optional)

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **Muse suggestion** | After typing + 300ms pause, inline grey text appears | [ ] |
| **Suggestion quality** | 7/10 suggestions are contextually relevant (manual review) | [ ] |
| **Archivist embeddings** | After entry saved, embedding generated within 5 seconds | [ ] |
| **Auto-tags** | Archivist suggests 2-3 relevant tags; user can accept/dismiss | [ ] |
| **Performance (CPU-only)** | Inference runs on CPU (no GPU); < 500ms for Muse, < 5s for Archivist | [ ] |
| **Model download** | User can download GGUF models from Settings; progress shown | [ ] |
| **Works offline** | AI features work with zero network connection | [ ] |

---

### 10. Gamification (Optional)

| Requirement | Acceptance Test | Pass/Fail |
|-------------|----------------|-----------|
| **XP awarded** | Writing 50+ word entry awards +10 XP | [ ] |
| **Artifact unlock** | Reaching XP threshold (e.g., 50 XP) unlocks artifact | [ ] |
| **Unlock notification** | Toast appears (bottom-right); can dismiss or view in Cavern | [ ] |
| **Records Cavern** | 2.5D isometric room displays unlocked artifacts | [ ] |
| **Artifact details** | Clicking artifact shows flavor text + unlock date | [ ] |
| **Simple Mode hides** | Enabling Simple Mode removes XP counter, Cavern, toasts | [ ] |
| **No streak shaming** | Calendar never shows red X's or "you missed a day" | [ ] |

---

## Acceptance Criteria (Non-Functional)

### Performance

| Metric | Target | Test | Pass/Fail |
|--------|--------|------|-----------|
| **Cold launch** | < 400ms to cursor-ready | Time from click to typing | [ ] |
| **Autosave latency** | < 3s from last keystroke | Monitor DB write | [ ] |
| **Search (semantic)** | < 500ms for 1000 entries | Measure query→results | [ ] |
| **Export (.chrxp)** | < 10s for 1000 entries + 50 MB attachments | Measure export time | [ ] |
| **Memory (idle)** | < 200 MB | Task Manager / Activity Monitor | [ ] |
| **Memory (active)** | < 500 MB during AI inference | Monitor during Muse suggestion | [ ] |
| **Disk usage** | < 50 MB (app bundle, excluding user data) | Measure install size | [ ] |

---

### Compatibility

| Platform | Target | Test | Pass/Fail |
|----------|--------|------|-----------|
| **Windows 10/11** | Works on x64 | Install + run on Win 10, Win 11 | [ ] |
| **macOS 12+** | Works on Intel + Apple Silicon | Install + run on macOS Monterey, Ventura, Sonoma | [ ] |
| **Linux (Ubuntu 22.04)** | Works on x64 | Install + run on Ubuntu 22.04 LTS | [ ] |
| **CPU-only (no GPU)** | AI works without GPU | Test on laptop with integrated graphics | [ ] |
| **16 GB RAM** | App runs smoothly | Test on machine with 16 GB RAM | [ ] |

---

### Reliability

| Requirement | Test | Pass/Fail |
|-------------|------|-----------|
| **No data loss** | Force-quit app mid-typing → entry recovered on restart | [ ] |
| **No crashes** | 100 entries written, 50 searches, 10 exports → zero crashes | [ ] |
| **Autosave reliable** | Write 1000 words, wait 5s → all text in DB | [ ] |
| **Export integrity** | Export → import → all entries match (checksum verified) | [ ] |

---

### Security

| Requirement | Test | Pass/Fail |
|-------------|------|-----------|
| **Database encrypted** | Attempt to open `data.db` in plain SQLite → fails | [ ] |
| **Key not hardcoded** | Search codebase for encryption key → not found | [ ] |
| **No telemetry** | Network monitor during 1-hour session → zero outbound calls (except update check) | [ ] |
| **Export encryption works** | Password-protected export → cannot open without correct password | [ ] |

---

### Usability

| Requirement | Test | Pass/Fail |
|-------------|------|-----------|
| **First-time user** | Non-technical user can write and save entry within 2 minutes | [ ] |
| **Neurodivergent-friendly** | ADHD tester: "Low friction, no stress" (qualitative feedback) | [ ] |
| **Error messages clear** | All errors use plain language (no "Error 500" or stack traces) | [ ] |
| **Help accessible** | User can find keyboard shortcuts (F1 or Help menu) | [ ] |

---

## Release Checklist (Before MVP Launch)

### Code Quality
- [ ] All automated tests pass (Rust `cargo test`, React Jest)
- [ ] No compiler warnings (Rust, TypeScript)
- [ ] Code reviewed by at least one other person
- [ ] Security-sensitive code (crypto, DB) double-checked

### Documentation
- [ ] README.md: Installation, basic usage
- [ ] CHANGELOG.md: Version history
- [ ] Privacy policy published (website + in-app)
- [ ] LICENSE file (MIT or GPL)

### Testing
- [ ] All acceptance criteria above: PASS
- [ ] Keyboard-only navigation tested
- [ ] Screen reader tested (NVDA on Windows)
- [ ] Tested on Windows 10, Windows 11, macOS, Ubuntu
- [ ] Tested with 1000+ entries (performance)

### Packaging
- [ ] Windows installer (.exe) builds correctly
- [ ] macOS disk image (.dmg) builds correctly
- [ ] Linux AppImage builds correctly
- [ ] Installer size < 100 MB (excluding AI models)

### Marketing / Launch
- [ ] Website live with download links
- [ ] GitHub repo public with source code
- [ ] Social media announcement prepared
- [ ] Support email setup (support@chronoxp.app)
- [ ] Monitoring (no telemetry, but manual check for GitHub issues)

---

## Post-Launch (Within 7 Days)

| Task | Target | Status |
|------|--------|--------|
| **Monitor GitHub Issues** | Respond to bugs within 24 hours | [ ] |
| **User feedback survey** | Email 50 early users for feedback | [ ] |
| **Crash reports** | If any crashes reported, hotfix within 48 hours | [ ] |
| **Performance issues** | If startup > 400ms reported, investigate | [ ] |
| **WCAG compliance** | If accessibility issues reported, prioritize fix | [ ] |

---

## Success Metrics (3 Months Post-Launch)

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Downloads** | 1,000+ | GitHub Releases downloads |
| **Active users** | 500+ (estimated 50% retention) | N/A (no telemetry) — based on support emails, GitHub stars |
| **Crash rate** | < 1% | GitHub Issues (user reports) |
| **NPS (Net Promoter Score)** | 40+ | User survey: "Would you recommend ChronoXP?" |
| **GitHub stars** | 100+ | Social proof / community interest |
| **Support tickets** | < 10/week | Manageable support load |

---

## Known Limitations (MVP)

### Acceptable for MVP
- **Single device only:** No sync (LAN sync is post-MVP)
- **Desktop only:** No mobile apps
- **English only:** Localization is post-MVP
- **Basic Concierge:** Rule-based commands only (no advanced NLU)
- **Limited artifacts:** 5-8 only (more post-MVP)
- **No quests:** Reflection prompts post-MVP

### Not Acceptable (Must Fix Before Launch)
- ❌ Data loss on crash
- ❌ Corrupted exports
- ❌ Unencrypted database (if SQLCipher fails)
- ❌ Inaccessible to keyboard-only users
- ❌ WCAG violations (contrast, screen reader)

---

## Regression Testing (Before Each Release)

**Smoke test (15 minutes):**
1. Install app
2. Write entry, autosave
3. Search for entry
4. Export `.chrxp`
5. Import `.chrxp`
6. Verify entry restored

**Full test (2 hours):**
- Run through all acceptance criteria
- Test on all supported platforms (Windows, macOS, Linux)
- Keyboard-only navigation
- Screen reader test (NVDA)

---

## Definition of Done (MVP)

✅ **All acceptance criteria pass**  
✅ **Zero known data-loss bugs**  
✅ **WCAG AA compliant**  
✅ **Performance targets met** (< 400ms launch, < 500ms search)  
✅ **Installers build successfully** (Windows, macOS, Linux)  
✅ **Documentation complete** (README, privacy policy, keyboard map)  
✅ **Code reviewed & security-checked**  
✅ **Tested on real hardware** (not just VMs)

**When all ✅ above → Ship MVP 🚀**

---

## References
- [WCAG 2.1 Quick Reference](https://www.w3.org/WAI/WCAG21/quickref/)
- [Tauri Testing Guide](https://tauri.app/v2/develop/tests/)
- [Semantic Versioning](https://semver.org/) (for versioning releases)
