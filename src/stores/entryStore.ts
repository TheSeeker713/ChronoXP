import { create } from "zustand";
import {
  listEntries,
  getEntry,
  createEntry,
  updateEntry,
  deleteEntry,
  pinEntry,
  searchEntries,
} from "../utils/ipc";
import type { Entry, EntryFull, SearchResult, CreateEntryPayload, UpdateEntryPayload } from "../types";

interface EntryState {
  entries: Entry[];
  currentEntry: EntryFull | null;
  searchResults: SearchResult[] | null;
  searchQuery: string;
  isLoading: boolean;
  isSaving: boolean;
  saveError: string | null;
  lastSavedAt: Date | null;

  fetchEntries: (limit?: number, offset?: number) => Promise<void>;
  openEntry: (id: number) => Promise<void>;
  newEntry: () => void;
  saveEntry: (payload: CreateEntryPayload | UpdateEntryPayload) => Promise<void>;
  removeEntry: (id: number) => Promise<void>;
  togglePin: (id: number) => Promise<void>;
  search: (query: string) => Promise<void>;
  clearSearch: () => void;
  clearCurrentEntry: () => void;
}

export const useEntryStore = create<EntryState>((set, get) => ({
  entries: [],
  currentEntry: null,
  searchResults: null,
  searchQuery: "",
  isLoading: false,
  isSaving: false,
  saveError: null,
  lastSavedAt: null,

  fetchEntries: async (limit = 50, offset = 0) => {
    set({ isLoading: true });
    try {
      const entries = await listEntries(limit, offset);
      set({ entries, isLoading: false });
    } catch (err) {
      console.error("Failed to fetch entries:", err);
      set({ isLoading: false });
    }
  },

  openEntry: async (id: number) => {
    set({ isLoading: true });
    try {
      const entry = await getEntry(id);
      set({ currentEntry: entry, isLoading: false });
    } catch (err) {
      console.error("Failed to open entry:", err);
      set({ isLoading: false });
    }
  },

  newEntry: () => {
    set({
      currentEntry: {
        id: 0,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        title: null,
        markdown: "",
        content_preview: "",
        mood: null,
        word_count: 0,
        pinned: false,
        deleted_at: null,
        tags: [],
      },
    });
  },

  saveEntry: async (payload: CreateEntryPayload | UpdateEntryPayload) => {
    set({ isSaving: true, saveError: null });
    try {
      let saved: EntryFull;
      if ("id" in payload && payload.id > 0) {
        saved = await updateEntry(payload as UpdateEntryPayload);
      } else {
        saved = await createEntry(payload as CreateEntryPayload);
      }
      set({
        currentEntry: saved,
        isSaving: false,
        lastSavedAt: new Date(),
      });
      // Refresh list in background
      get().fetchEntries();
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      set({ isSaving: false, saveError: msg });
      throw new Error(msg);
    }
  },

  removeEntry: async (id: number) => {
    await deleteEntry(id);
    set((state) => ({
      entries: state.entries.filter((e) => e.id !== id),
      currentEntry: state.currentEntry?.id === id ? null : state.currentEntry,
    }));
  },

  togglePin: async (id: number) => {
    const entry = get().entries.find((e) => e.id === id);
    if (!entry) return;
    const newPinned = !entry.pinned;
    await pinEntry(id, newPinned);
    set((state) => ({
      entries: state.entries.map((e) =>
        e.id === id ? { ...e, pinned: newPinned } : e,
      ),
    }));
  },

  search: async (query: string) => {
    set({ searchQuery: query });
    if (!query.trim()) {
      set({ searchResults: null });
      return;
    }
    try {
      const results = await searchEntries(query);
      set({ searchResults: results });
    } catch (err) {
      console.error("Search failed:", err);
      set({ searchResults: [] });
    }
  },

  clearSearch: () => set({ searchResults: null, searchQuery: "" }),
  clearCurrentEntry: () => set({ currentEntry: null }),
}));
