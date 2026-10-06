import { create } from "zustand";
import { getVaultStatus, initializeVault, unlockVault, lockVault } from "../utils/ipc";
import type { VaultStatus, RecoveryKey } from "../types";

interface AuthState {
  status: VaultStatus | null;
  isLocked: boolean;
  isInitialized: boolean;
  vaultPasswordEnabled: boolean;
  recoveryKey: RecoveryKey | null;
  error: string | null;
  isLoading: boolean;

  fetchStatus: () => Promise<void>;
  initialize: (password?: string) => Promise<RecoveryKey>;
  unlock: (password?: string) => Promise<void>;
  lock: () => Promise<void>;
  clearRecoveryKey: () => void;
  clearError: () => void;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  status: null,
  isLocked: true,
  isInitialized: false,
  vaultPasswordEnabled: false,
  recoveryKey: null,
  error: null,
  isLoading: false,

  fetchStatus: async () => {
    set({ isLoading: true, error: null });
    try {
      const status = await getVaultStatus();
      set({
        status,
        isLocked: status.locked,
        isInitialized: status.initialized,
        vaultPasswordEnabled: status.vault_password_enabled,
        isLoading: false,
      });
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : String(err),
        isLoading: false,
      });
    }
  },

  initialize: async (password?: string) => {
    set({ isLoading: true, error: null });
    try {
      const rk = await initializeVault(password);
      set({
        isLocked: false,
        isInitialized: true,
        vaultPasswordEnabled: !!password,
        recoveryKey: rk,
        isLoading: false,
      });
      return rk;
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      set({ error: msg, isLoading: false });
      throw new Error(msg);
    }
  },

  unlock: async (password?: string) => {
    set({ isLoading: true, error: null });
    try {
      await unlockVault(password);
      set({ isLocked: false, isLoading: false });
      // Refresh full status
      await get().fetchStatus();
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      set({ error: msg, isLoading: false });
      throw new Error(msg);
    }
  },

  lock: async () => {
    try {
      await lockVault();
    } catch {
      // Ignore lock errors — still lock UI
    }
    set({ isLocked: true });
  },

  clearRecoveryKey: () => set({ recoveryKey: null }),
  clearError: () => set({ error: null }),
}));
