import { create } from "zustand";
import { getSettings, updateSettings } from "../utils/ipc";
import type { AppSettings } from "../types";

interface SettingsState {
  settings: AppSettings | null;
  isLoading: boolean;

  fetchSettings: () => Promise<void>;
  updateSetting: <K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K],
  ) => Promise<void>;
}

const DEFAULT_SETTINGS: AppSettings = {
  theme: "dark",
  font_family: "Inter",
  auto_lock_minutes: 15,
  vault_password_enabled: false,
  simple_mode: false,
  schema_version: 1,
};

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: null,
  isLoading: false,

  fetchSettings: async () => {
    set({ isLoading: true });
    try {
      const settings = await getSettings();
      set({ settings, isLoading: false });
    } catch {
      // Use defaults if settings not yet available
      set({ settings: DEFAULT_SETTINGS, isLoading: false });
    }
  },

  updateSetting: async <K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K],
  ) => {
    const current = get().settings ?? DEFAULT_SETTINGS;
    const updated = { ...current, [key]: value };
    set({ settings: updated });
    try {
      await updateSettings({ [key]: value });
    } catch (err) {
      console.error("Failed to update setting:", err);
      set({ settings: current }); // revert on failure
    }
  },
}));
