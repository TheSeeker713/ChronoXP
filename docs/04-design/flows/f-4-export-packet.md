# UX Flow: F-4 Export Packet

**Flow ID:** F-4  
**User Goal:** Export complete journal data for backup, portability, or migration  
**Primary Persona:** Reflective Journaler (values data ownership)

---

## Entry Point
User wants to back up their journal or move to another device.

---

## Flow Steps

### Step 1: Navigate to Export
**Trigger:** User opens Settings  
**Actions:**
1. Press `Ctrl/Cmd+,` OR click `[Settings]` in toolbar
2. Click `Export` in sidebar

**Screen:**
```
┌────────────┬────────────────────────────────────────┐
│ Settings   │  Export & Backup                       │
│            │                                        │
│ Appearance │  📦 Export Your Journal                │
│ Editor     │                                        │
│ AI         │  Create a portable backup of all your  │
│ Privacy    │  entries, tags, and attachments.       │
│ Keyboard   │                                        │
│ →Export    │  [Export as .chrxp]                    │
│ About      │                                        │
│            │  ───────────────────────────────────   │
│            │                                        │
│            │  📥 Import from .chrxp                 │
│            │                                        │
│            │  [Choose File...]                      │
└────────────┴────────────────────────────────────────┘
```

---

### Step 2: Click "Export as .chrxp"
**Trigger:** User clicks `[Export as .chrxp]` button  
**System:**
- Opens export options dialog (modal)

**Dialog:**
```
┌─────────────────────────────────────────────────────┐
│  Export Journal                             [×]     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Choose what to include:                           │
│  ☑ Entries (text + metadata)                       │
│  ☑ Tags                                             │
│  ☑ Attachments (images, files)                     │
│  ☑ Vector embeddings (for semantic search)         │
│  ☑ XP & artifacts (gamification data)              │
│  ☐ Settings (themes, preferences)                  │
│                                                     │
│  ───────────────────────────────────────────────    │
│                                                     │
│  🔐 Encryption (optional):                          │
│  ○ No encryption (faster, portable)                │
│  ● AES-256-GCM with password                       │
│                                                     │
│  Password: [********************]                   │
│  Confirm:  [********************]                   │
│                                                     │
│  ⚠️ Remember this password! It cannot be recovered. │
│                                                     │
│  [Cancel]                          [Export]         │
└─────────────────────────────────────────────────────┘
```

**User Actions:**
- Check/uncheck items to include
- Choose encryption (optional)
- Enter password if encrypting
- Click `[Export]`

---

### Step 3: Choose Save Location
**Trigger:** User clicks `[Export]`  
**System:**
- Opens native file save dialog (OS-specific)
- Default filename: `chronoxp_journal_2025-10-22.chrxp`

**Dialog (Windows):**
```
┌─────────────────────────────────────────────────────┐
│  Save As                                            │
├─────────────────────────────────────────────────────┤
│  Save in: [Documents ▼]                             │
│                                                     │
│  📁 My Documents                                    │
│    📁 ChronoXP Backups                              │
│    📄 chronoxp_journal_2025-09-15.chrxp             │
│                                                     │
│  File name: [chronoxp_journal_2025-10-22.chrxp___] │
│  Save as type: [ChronoXP Archive (*.chrxp) ▼]      │
│                                                     │
│  [Save] [Cancel]                                    │
└─────────────────────────────────────────────────────┘
```

**User Actions:**
- Choose directory
- Optionally rename file
- Click `[Save]`

---

### Step 4: Export Processing
**Trigger:** User confirms save location  
**System (background):**

1. **Create temp directory** in OS temp folder
2. **Export entries to Markdown:**
   ```
   /entries/
     2025-10-22_1542.md
     2025-10-21_0830.md
     ...
   ```
   Each file contains:
   ```markdown
   ---
   id: 123
   created_at: 2025-10-22T15:42:00Z
   updated_at: 2025-10-22T15:45:00Z
   tags: [work, stress, reflection]
   mood: reflective
   word_count: 156
   xp_awarded: 10
   ---
   
   # Work-life balance thoughts
   
   Today I talked to Mom about family expectations...
   ```

3. **Copy attachments:**
   ```
   /attachments/
     img_20251022_001.jpg
     document.pdf
   ```

4. **Export metadata:**
   ```
   /meta/
     tags.json         (all tags)
     artifacts.json    (unlocked items)
     settings.json     (if user opted in)
   ```

5. **Backup database:**
   ```
   /database/
     chronoxp.db       (full SQLite copy)
   ```

6. **Export vectors (optional):**
   ```
   /vectors/
     embeddings.bin    (FAISS index or similar)
   ```

7. **Generate manifest:**
   ```json
   {
     "version": "1.0.0",
     "export_date": "2025-10-22T15:45:00Z",
     "entry_count": 47,
     "attachment_count": 12,
     "encrypted": true,
     "encryption_method": "AES-256-GCM",
     "includes": ["entries", "tags", "attachments", "vectors", "xp"],
     "checksum": "sha256:a3f2b9c..."
   }
   ```

8. **Compress to ZIP:**
   - All files → `chronoxp_journal_2025-10-22.chrxp` (ZIP format)

9. **Encrypt (if password provided):**
   - AES-256-GCM encryption of entire ZIP
   - Password-based key derivation (PBKDF2, 100k iterations)

10. **Write to chosen location**

11. **Clean up temp directory**

**Progress Indicator:**
```
┌─────────────────────────────────────────────────────┐
│  Exporting...                                       │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ████████████████████░░░ 75%                        │
│                                                     │
│  Packaging attachments...                           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

### Step 5: Export Complete
**Trigger:** Export finishes successfully  
**System:**
- Closes progress dialog
- Shows success toast notification

**Toast (bottom-right):**
```
┌───────────────────────────────────────┐
│ ✓ Export complete!                    │
│ chronoxp_journal_2025-10-22.chrxp     │
│ [Open Folder] [Dismiss]               │
└───────────────────────────────────────┘
```

**User Actions:**
- Click `[Open Folder]` → Opens file explorer to saved location
- Click `[Dismiss]` or wait 10s → Toast fades

---

## Success Criteria
- ✅ Export completes without data loss (integrity hash verified)
- ✅ `.chrxp` file can be re-imported on another device
- ✅ Encrypted export requires correct password to open
- ✅ Export time < 10 seconds for typical journal (1000 entries)
- ✅ Markdown files are human-readable without ChronoXP

---

## Import Flow (F-4b)

### Step 1: Choose Import
**Trigger:** User clicks `[Choose File...]` in Export settings  
**System:**
- Opens native file picker
- Filters: `*.chrxp` files only

---

### Step 2: Select File
**User:** Selects `.chrxp` file  
**System:**
- Reads `manifest.json` to check version and encryption

**If encrypted:**
```
┌─────────────────────────────────────────────────────┐
│  Import Encrypted Journal                   [×]     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  This backup is password-protected.                │
│                                                     │
│  Password: [____________________]                   │
│                                                     │
│  [Cancel]                          [Import]         │
└─────────────────────────────────────────────────────┘
```

---

### Step 3: Import Processing
**Trigger:** User enters password (if needed) and clicks `[Import]`  
**System:**

1. **Decrypt** (if encrypted)
2. **Verify checksum** (ensure integrity)
3. **Extract ZIP** to temp directory
4. **Merge data:**
   - Entries: Insert into `entries` table (skip duplicates by ID)
   - Tags: Merge into `tags` table
   - Attachments: Copy to `appdata/attachments/`
   - Vectors: Rebuild index
5. **Restore XP/artifacts** (if included)
6. **Clean up temp directory**

**Progress:**
```
┌─────────────────────────────────────────────────────┐
│  Importing...                                       │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ████████████░░░░░░░░ 60%                           │
│                                                     │
│  Restoring entries...                               │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

### Step 4: Import Complete
**Toast:**
```
┌───────────────────────────────────────┐
│ ✓ Import successful!                  │
│ 47 entries restored.                  │
│ [View Library] [Dismiss]              │
└───────────────────────────────────────┘
```

---

## Edge Cases

### E1: Export fails mid-process
- **System:** Rolls back, deletes incomplete file
- **User:** Sees error toast: "Export failed. Please try again."

### E2: Import file is corrupted
- **System:** Checksum mismatch detected
- **User:** Error dialog: "This file appears corrupted. Import aborted."

### E3: Wrong password on import
- **System:** Decryption fails
- **User:** Error: "Incorrect password. Please try again."

### E4: Duplicate entries on import
- **System:** Skips duplicates (by entry ID)
- **Toast:** "47 entries imported, 3 duplicates skipped."

### E5: Disk full during export
- **System:** Catches disk space error
- **User:** Error: "Not enough disk space. Free up space and try again."

---

## Accessibility Notes
- **Keyboard navigation:**
  - `Tab` through options and buttons
  - `Space` to toggle checkboxes
  - `Enter` to confirm export/import
- **Screen reader:**
  - "Export options dialog. Choose what to include. Entries checkbox, checked."
  - "Password field. Required if encryption is enabled."

---

## Microcopy
- **Export button:** "Export as .chrxp" (not "Backup" — less techni

cal)
- **Password warning:** "Remember this password! It cannot be recovered." (strong, clear)
- **Success toast:** "Export complete!" (celebratory, not clinical)

---

## Security Notes
- **Encryption:** AES-256-GCM (authenticated encryption)
- **Key derivation:** PBKDF2 with 100,000 iterations
- **No password recovery:** If user forgets password, backup is unrecoverable (by design)
- **Integrity:** SHA-256 checksum in manifest ensures tampering is detected

---

## File Format Spec (`.chrxp`)

### Structure
```
chronoxp_journal_2025-10-22.chrxp (ZIP)
├── manifest.json
├── entries/
│   ├── 2025-10-22_1542.md
│   ├── 2025-10-21_0830.md
│   └── ...
├── attachments/
│   ├── img_20251022_001.jpg
│   └── ...
├── meta/
│   ├── tags.json
│   ├── artifacts.json
│   └── settings.json (optional)
├── database/
│   └── chronoxp.db
└── vectors/
    └── embeddings.bin (optional)
```

### Compatibility
- Forward-compatible: Future versions can import older `.chrxp` files
- Backward-compatible: Older versions ignore new fields in manifest

---

## Analytics (Optional, Privacy-Respecting)
If user opts in:
- % of users who export their data
- Average export file size
- % of exports with encryption enabled
- Import success rate

**Never logged:** File contents, passwords, export locations.
