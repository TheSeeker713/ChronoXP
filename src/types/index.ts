export interface Entry {
  id: number;
  created_at: string;
  updated_at: string;
  title: string | null;
  content_preview: string;
  mood: string | null;
  word_count: number;
  pinned: boolean;
  deleted_at: string | null;
  tags: Tag[];
}

export interface EntryFull extends Entry {
  markdown: string;
}

export interface Tag {
  id: number;
  name: string;
  color: string;
  created_at: string;
}

export interface SearchResult {
  id: number;
  created_at: string;
  title: string | null;
  snippet: string;
  tags: Tag[];
}

export interface ExportOptions {
  include_entries: boolean;
  include_attachments: boolean;
  include_settings: boolean;
  password: string | null;
}

export interface AppSettings {
  theme: "dark" | "light" | "low-stim";
  font_family: string;
  auto_lock_minutes: number;
  vault_password_enabled: boolean;
  simple_mode: boolean;
  schema_version: number;
}

export interface UnlockPayload {
  password?: string;
}

export interface CreateEntryPayload {
  title?: string;
  markdown: string;
  mood?: string;
  tags?: string[];
}

export interface UpdateEntryPayload {
  id: number;
  title?: string;
  markdown: string;
  mood?: string;
  tags?: string[];
}

export interface VaultStatus {
  initialized: boolean;
  vault_password_enabled: boolean;
  locked: boolean;
}

export interface RecoveryKey {
  key_b32: string;
}

export interface ExportResult {
  path: string;
  entry_count: number;
}

export interface ImportResult {
  imported: number;
  skipped: number;
}
