import { describe, it, expect, beforeEach, vi } from "vitest";
import { mockInvoke } from "../test-setup";
import {
  getVaultStatus,
  listEntries,
  searchEntries,
  createEntry,
} from "../utils/ipc";

beforeEach(() => {
  vi.clearAllMocks();
});

describe("getVaultStatus", () => {
  it("calls the correct command", async () => {
    mockInvoke.mockResolvedValueOnce({
      initialized: true,
      vault_password_enabled: false,
      locked: false,
    });
    const status = await getVaultStatus();
    expect(mockInvoke).toHaveBeenCalledWith("get_vault_status");
    expect(status.initialized).toBe(true);
    expect(status.locked).toBe(false);
  });
});

describe("listEntries", () => {
  it("calls with default limit and offset", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    await listEntries();
    expect(mockInvoke).toHaveBeenCalledWith("list_entries", {
      limit: 50,
      offset: 0,
    });
  });

  it("calls with custom limit and offset", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    await listEntries(20, 10);
    expect(mockInvoke).toHaveBeenCalledWith("list_entries", {
      limit: 20,
      offset: 10,
    });
  });
});

describe("searchEntries", () => {
  it("calls search with query", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    await searchEntries("travel");
    expect(mockInvoke).toHaveBeenCalledWith("search_entries", {
      query: "travel",
    });
  });
});

describe("createEntry", () => {
  it("creates entry with markdown content", async () => {
    const fakeEntry = {
      id: 1,
      created_at: "2026-01-01T00:00:00Z",
      updated_at: "2026-01-01T00:00:00Z",
      title: null,
      markdown: "<p>Hello world</p>",
      content_preview: "Hello world",
      mood: null,
      word_count: 2,
      pinned: false,
      deleted_at: null,
      tags: [],
    };
    mockInvoke.mockResolvedValueOnce(fakeEntry);
    const entry = await createEntry({ markdown: "<p>Hello world</p>" });
    expect(mockInvoke).toHaveBeenCalledWith("create_entry", {
      payload: { markdown: "<p>Hello world</p>" },
    });
    expect(entry.id).toBe(1);
    expect(entry.word_count).toBe(2);
  });
});
