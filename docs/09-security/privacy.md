# Security & Privacy

**Project:** ChronoXP™ Journal  
**Date:** October 22, 2025

---

## Core Commitment

> **"Your thoughts. Your device. Nobody else's business."**

ChronoXP is built **privacy-first, not privacy-as-feature**. We never see your data, never track your usage, and never compromise your privacy for convenience or profit.

---

## Privacy Principles

### 1. Local-Only by Default
- **All data stored on your device** (not cloud)
- **No network calls** unless user explicitly exports/syncs (future feature)
- **Offline-first architecture** (works perfectly without internet)

### 2. No Telemetry, Ever
- **No analytics tracking** (not even "anonymized" usage stats)
- **No crash reports** (unless user clicks "Send Report")
- **No A/B testing** (not worth privacy trade-off)
- **No third-party SDKs** (Google Analytics, Mixpanel, etc. = banned)

### 3. Encryption at Rest
- **Database:** SQLCipher (AES-256 encryption)
- **Attachments:** Encrypted on disk (OS-level or per-file AES-256-GCM)
- **Optional vault password:** Second encryption layer (for paranoid users)

### 4. User Control
- **Export anytime:** `.chrxp` format (portable, readable Markdown)
- **Delete anytime:** Entries, tags, attachments (hard delete after 30 days in Trash)
- **No lock-in:** Can leave ChronoXP and take all data

### 5. Transparency
- **Open source:** Code is public (MIT or GPL license)
- **Auditable:** Security researchers can review for backdoors
- **Privacy policy:** Plain language, no legalese

---

## Data Collection (What We DON'T Collect)

### ❌ Never Collected
- Journal entry content
- Search queries
- Tags, attachments, metadata
- IP address
- Device identifiers (UDID, MAC address, etc.)
- Location data (GPS, Wi-Fi triangulation)
- Behavioral analytics (clicks, time spent, features used)
- Personally identifiable information (PII) beyond email (if user purchases Pro)

### ✅ Only Collected (With Explicit Consent)
- **Email address** (if user purchases Pro — for license delivery only)
- **Crash logs** (if user clicks "Send Report" after error)
  - Includes: Stack trace, OS version, app version
  - Does NOT include: Entry content, search queries, personal data

---

## Encryption Details

### Database Encryption (SQLCipher)

**Algorithm:** AES-256-CBC  
**Key derivation:** PBKDF2 (100,000 iterations)  
**Key storage:** OS keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service)

**Encryption process:**
1. On first launch, ChronoXP generates a random 256-bit key
2. Key is stored in OS keychain (encrypted by OS)
3. Database is encrypted with this key
4. Key is retrieved from keychain on each app launch

**Threat protection:**
- ✅ Protects against: Unauthorized file access (someone steals your laptop)
- ❌ Does NOT protect against: Malware on your device (keylogger, screen recorder)

---

### Optional Vault Password (Second Layer)

**For paranoid users who don't trust OS keychain:**

**How it works:**
1. User sets a vault password in Settings
2. Password is NOT stored (only its hash)
3. On app launch, user enters password
4. Password derives a second encryption key (PBKDF2, 100,000 iterations)
5. Database is re-encrypted with this second key

**Trade-off:**
- ✅ More secure (even if OS keychain is compromised, attacker needs password)
- ❌ Less convenient (user must remember password; no recovery if forgotten)

**Recommendation:** Only for high-risk users (activists, journalists, abuse survivors).

---

### Attachment Encryption

**Option A (MVP):** OS-level encryption (Windows BitLocker, macOS FileVault)  
**Option B (Future):** Per-file AES-256-GCM encryption

**Current approach (MVP):**
- Rely on OS-level full-disk encryption (most users have this enabled)
- Attachments stored in `%APPDATA%\ChronoXP\attachments\`
- No additional encryption (to avoid complexity)

**Future enhancement:**
- Encrypt each attachment with AES-256-GCM
- Key stored in database (already encrypted with SQLCipher)
- Provides defense-in-depth (even if OS encryption disabled)

---

### Export Encryption (.chrxp Files)

**Default:** Unencrypted ZIP (for portability)  
**Optional:** AES-256-GCM encrypted ZIP

**Encryption process (if user enables):**
1. User sets password in export dialog
2. Password derives key via PBKDF2 (100,000 iterations)
3. ZIP file is encrypted with AES-256-GCM (authenticated encryption)
4. Encrypted file saved as `.chrxp`

**Import process:**
1. User selects encrypted `.chrxp` file
2. Prompted for password
3. Password derives key → decrypts ZIP
4. Integrity verified (HMAC checksum)

**Security note:** If user forgets password, backup is unrecoverable (by design).

---

## Threat Model

### What We Protect Against

| Threat | Protection | Effectiveness |
|--------|-----------|---------------|
| **Unauthorized file access** (stolen device) | SQLCipher encryption | ✅ High |
| **Data exfiltration by malicious app** | No network calls | ✅ High |
| **Cloud provider breach** | No cloud storage | ✅ Perfect (N/A) |
| **Shoulder surfing** (someone watching screen) | User must close app or lock screen | ⚠️ User responsibility |
| **Forensic analysis** (attacker has device, time) | Encryption at rest | ✅ High (if vault password used) |

---

### What We DON'T Protect Against

| Threat | Why Not in Scope |
|--------|------------------|
| **Keylogger malware** | OS-level concern; antivirus/OS hardening required |
| **Screen recording malware** | OS-level concern; requires OS security updates |
| **Physical access to unlocked device** | User must lock screen; not app responsibility |
| **State-level adversaries** (NSA, Five Eyes) | Out of scope; ChronoXP is not for high-risk activists (use Tails OS, etc.) |
| **Quantum computing attacks** (future) | AES-256 is quantum-resistant for foreseeable future |

---

## Privacy Policy (Plain Language)

### What Data We Collect
**Short answer:** Almost nothing.

**Long answer:**
- If you buy Pro: We collect your email (to send license key). We don't share this with anyone.
- If you send a crash report: We collect technical info (OS version, stack trace). No personal data.
- That's it. We don't track what you write, search, or do in the app.

### What We DON'T Do
- ❌ No cookies (ChronoXP is a desktop app, not a website)
- ❌ No third-party trackers (Google Analytics, Facebook Pixel, etc.)
- ❌ No selling data (not even "anonymized" stats)
- ❌ No AI training on your entries (local models only, no data uploaded)

### Where Is Your Data Stored?
- **On your device** (Windows: `%APPDATA%\ChronoXP\`, macOS: `~/Library/Application Support/ChronoXP/`)
- **Nowhere else** (unless you export or enable future LAN sync)

### Can You Delete Your Data?
Yes. Two ways:
1. **In-app:** Delete entries → Hard delete after 30 days in Trash
2. **Nuclear option:** Uninstall ChronoXP → Manually delete `%APPDATA%\ChronoXP\` folder

### Can We Access Your Data?
**No.** We have no server, no cloud backend, no way to access your journal. Even if we wanted to, we physically can't.

### What If You Want to Leave?
- Export your data (`.chrxp` format) → Uninstall ChronoXP → Done.
- No hidden files, no registry junk (clean uninstall).

---

## GDPR & CCPA Compliance

### GDPR (EU General Data Protection Regulation)
**Status:** Compliant (by design)

**Why:**
- **Minimal data collection:** No PII beyond email (if user buys Pro)
- **User consent:** Email only collected if user purchases (explicit action)
- **Right to access:** User controls all data (export anytime)
- **Right to erasure:** User can delete all data (in-app or manual deletion)
- **No profiling:** No behavioral analytics or automated decision-making

**Edge case: Email addresses (for Pro users)**
- Stored by payment processor (Stripe, Gumroad)
- User can request deletion via support email
- Processed within 30 days (GDPR requirement)

---

### CCPA (California Consumer Privacy Act)
**Status:** Compliant

**Why:**
- **No sale of data:** We don't sell personal information (we don't even collect it)
- **Right to know:** User controls all data locally
- **Right to delete:** User can delete all data

---

## Security Best Practices (For Users)

### 1. Enable OS-Level Encryption
- **Windows:** BitLocker (included in Windows 10 Pro+)
- **macOS:** FileVault (Settings → Security & Privacy)
- **Linux:** LUKS (during install or via `cryptsetup`)

**Why:** Provides baseline protection if device is stolen.

---

### 2. Use Strong Vault Password (If Enabled)
- **Minimum:** 12 characters, mix of letters, numbers, symbols
- **Recommended:** Passphrase (e.g., "correct horse battery staple 2025")
- **Avoid:** Dictionary words, birthdays, common patterns

**Store password safely:**
- Password manager (1Password, Bitwarden, KeePass)
- NOT in ChronoXP itself (defeating purpose)

---

### 3. Lock Your Screen When Away
- **Windows:** `Win+L`
- **macOS:** `Ctrl+Cmd+Q`
- **Linux:** `Ctrl+Alt+L` (varies by distro)

**Why:** ChronoXP encrypts at rest, not in use. Unlocked device = readable journal.

---

### 4. Regular Backups (Export `.chrxp`)
- **Frequency:** Weekly or monthly (user preference)
- **Storage:** External drive, USB stick, encrypted cloud (e.g., Cryptomator + Dropbox)
- **Encryption:** Use password protection if storing in cloud

**Why:** Protection against hardware failure, ransomware.

---

### 5. Keep OS & ChronoXP Updated
- **OS updates:** Security patches fix vulnerabilities
- **ChronoXP updates:** Bug fixes, security improvements

**ChronoXP update policy:**
- User-initiated (no forced updates)
- Notification when new version available
- Changelog shows security fixes (if any)

---

## Incident Response Plan

### If Security Vulnerability Discovered

**Process:**
1. **Report received** (via email: security@chronoxp.app)
2. **Acknowledge within 24 hours**
3. **Investigate & patch** (priority based on severity)
4. **Release hotfix** (critical: within 48 hours; high: 7 days; medium: 30 days)
5. **Public disclosure** (after patch released + 90-day grace period)

**Responsible disclosure:**
- Security researchers credited (if they agree)
- No legal action against good-faith researchers (safe harbor policy)

---

### If Data Breach (Hypothetical)

**ChronoXP has no servers, so classic "breach" is impossible. However, if payment processor (Stripe) is breached:**

**Process:**
1. **Notify affected users** (email within 72 hours, per GDPR)
2. **Explain what data was exposed** (likely: email, purchase date)
3. **Mitigation steps** (e.g., monitor for phishing)
4. **Transparency report** (publish incident details on website)

**What's NOT at risk:** Journal content (we never had access to it).

---

## Security Audit (Pre-Launch)

### Automated Tools
- [ ] **OWASP Dependency Check:** Scan for vulnerable libraries (Rust crates, npm packages)
- [ ] **Bandit (Python):** If using Python scripts for build
- [ ] **SQLCipher config check:** Ensure encryption is actually enabled (not accidentally disabled)

### Manual Review
- [ ] **Code review:** Security-focused pass (especially crypto, file I/O)
- [ ] **Penetration test:** Attempt to extract data from encrypted DB (without key)
- [ ] **Threat modeling:** STRIDE analysis (Spoofing, Tampering, Repudiation, Info Disclosure, DoS, Elevation)

### Third-Party Audit (Future, if budget allows)
- Hire security firm (e.g., Trail of Bits, Cure53) for independent audit
- Focus: Crypto implementation, key management, data leakage

---

## Comparison to Competitors (Privacy)

| Feature | ChronoXP | Day One | Journey | Notion |
|---------|----------|---------|---------|--------|
| **Local-only option** | ✅ Default | ❌ Cloud-first | ❌ Cloud-only | ❌ Cloud-only |
| **End-to-end encryption** | ✅ (local) | ✅ (cloud) | ⚠️ (optional) | ❌ |
| **No telemetry** | ✅ | ❌ (opt-out) | ❌ (opt-out) | ❌ |
| **Open source** | ✅ | ❌ | ❌ | ❌ |
| **Export format** | ✅ Markdown | ✅ JSON | ✅ JSON | ⚠️ Markdown (limited) |
| **Third-party SDKs** | ❌ None | ⚠️ Some | ⚠️ Some | ✅ Many |

**ChronoXP advantage:** Most private option (true offline-first, no tracking).

---

## Future Considerations

### 1. LAN Sync (Optional, Future Feature)
**How it works:**
- Device A and Device B on same local network
- User initiates sync (manual, not automatic)
- Data encrypted in transit (TLS 1.3)
- No internet required (direct peer-to-peer)

**Privacy:**
- No cloud server involved
- No data leaves local network
- Optional (user can ignore entirely)

**Security:**
- Pairing code (prevents unauthorized sync)
- Certificate pinning (prevents MITM)

---

### 2. Cloud Sync (If We Ever Build It — NOT PLANNED)
**If users demand it, here's how we'd do it (ethically):**
- **E2EE (end-to-end encryption):** Data encrypted before upload
- **Zero-knowledge:** Server can't decrypt (user holds only key)
- **Open source server:** Users can self-host
- **Opt-in only:** Default remains local-only

**Why we're hesitant:**
- Cloud sync = attack surface (server can be breached)
- Complexity (sync conflicts, versioning)
- Cost (storage, bandwidth)
- Mission drift (privacy-first → cloud-first)

---

## Privacy Statement (For Website)

> **ChronoXP Privacy Promise**
> 
> 1. **We don't see your data.** Everything stays on your device.
> 2. **We don't track you.** No analytics, no cookies, no creepy stuff.
> 3. **We don't sell anything.** Your privacy is not a product.
> 4. **We're transparent.** Our code is open source. Audit us.
> 5. **You're in control.** Export your data anytime. Leave anytime.
> 
> Questions? Email [privacy@chronoxp.app](mailto:privacy@chronoxp.app).

---

## References
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [SQLCipher Design](https://www.zetetic.net/sqlcipher/design/)
- [GDPR Compliance Checklist](https://gdpr.eu/checklist/)
- [Threat Modeling (STRIDE)](https://learn.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats)
