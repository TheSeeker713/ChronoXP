import { invoke } from "@tauri-apps/api/core";
import type {
  Entry,
  EntryFull,
  Tag,
  SearchResult,
  ExportOptions,
  ExportResult,
  ImportResult,
  VaultStatus,
  RecoveryKey,
  AppSettings,
  CreateEntryPayload,
  UpdateEntryPayload,
} from "../types";

// ── Security / Auth ──────────────────────────────────────────────

export async function getVaultStatus(): Promise<VaultStatus> {
  return invoke("get_vault_status");
}

export async function initializeVault(
  vaultPassword?: string,
): Promise<RecoveryKey> {
  return invoke("initialize_vault", { vaultPassword: vaultPassword ?? null });
}

export async function unlockVault(password?: string): Promise<void> {
  return invoke("unlock_vault", { password: password ?? null });
}

export async function lockVault(): Promise<void> {
  return invoke("lock_vault");
}

export async function setVaultPassword(
  currentPassword: string | null,
  newPassword: string,
): Promise<void> {
  return invoke("set_vault_password", { currentPassword, newPassword });
}

// ── Entries ──────────────────────────────────────────────────────

export async function listEntries(
  limit?: number,
  offset?: number,
): Promise<Entry[]> {
  return invoke("list_entries", { limit: limit ?? 50, offset: offset ?? 0 });
}

export async function getEntry(id: number): Promise<EntryFull> {
  return invoke("get_entry", { id });
}

export async function createEntry(
  payload: CreateEntryPayload,
): Promise<EntryFull> {
  return invoke("create_entry", { payload });
}

export async function updateEntry(
  payload: UpdateEntryPayload,
): Promise<EntryFull> {
  return invoke("update_entry", { payload });
}

export async function deleteEntry(id: number): Promise<void> {
  return invoke("delete_entry", { id });
}

export async function pinEntry(id: number, pinned: boolean): Promise<void> {
  return invoke("pin_entry", { id, pinned });
}

// ── Tags ─────────────────────────────────────────────────────────

export async function listTags(): Promise<Tag[]> {
  return invoke("list_tags");
}

export async function createTag(name: string, color?: string): Promise<Tag> {
  return invoke("create_tag", { name, color: color ?? "#6B7280" });
}

export async function deleteTag(id: number): Promise<void> {
  return invoke("delete_tag", { id });
}

// ── Search ────────────────────────────────────────────────────────

export async function searchEntries(query: string): Promise<SearchResult[]> {
  return invoke("search_entries", { query });
}

// ── Export / Import ───────────────────────────────────────────────

export async function exportJournal(
  options: ExportOptions,
  savePath: string,
): Promise<ExportResult> {
  return invoke("export_journal", { options, savePath });
}

export async function importJournal(
  filePath: string,
  password?: string,
): Promise<ImportResult> {
  return invoke("import_journal", {
    filePath,
    password: password ?? null,
  });
}

// ── Settings ──────────────────────────────────────────────────────

export async function getSettings(): Promise<AppSettings> {
  return invoke("get_settings");
}

export async function updateSettings(
  settings: Partial<AppSettings>,
): Promise<void> {
  return invoke("update_settings", { settings });
}
