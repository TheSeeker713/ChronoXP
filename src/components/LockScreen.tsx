import { useState, useEffect } from "react";
import { Lock, KeyRound, Eye, EyeOff } from "lucide-react";
import { useAuthStore } from "../stores/authStore";

export function LockScreen() {
  const { isInitialized, vaultPasswordEnabled, initialize, unlock, error, isLoading, clearError } =
    useAuthStore();

  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [setupMode, setSetupMode] = useState(!isInitialized);
  const [wantVaultPassword, setWantVaultPassword] = useState(false);
  const [recoveryKeyShown, setRecoveryKeyShown] = useState<string | null>(null);
  const [recoveryAcknowledged, setRecoveryAcknowledged] = useState(false);

  useEffect(() => {
    setSetupMode(!isInitialized);
  }, [isInitialized]);

  useEffect(() => {
    return () => clearError();
  }, [clearError]);

  const handleSetup = async () => {
    if (wantVaultPassword) {
      if (password !== confirmPassword) return;
      if (password.length < 8) return;
    }
    try {
      const rk = await initialize(wantVaultPassword ? password : undefined);
      setRecoveryKeyShown(rk.key_b32);
    } catch {
      // error shown from store
    }
  };

  const handleUnlock = async () => {
    try {
      await unlock(vaultPasswordEnabled ? password : undefined);
    } catch {
      // error shown from store
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      if (setupMode) {
        if (recoveryKeyShown && !recoveryAcknowledged) return;
        if (!recoveryKeyShown) handleSetup();
      } else {
        handleUnlock();
      }
    }
  };

  if (recoveryKeyShown && !recoveryAcknowledged) {
    return (
      <div className="fixed inset-0 bg-[#0d0d1a] flex items-center justify-center p-6">
        <div className="bg-[#16213e] border border-[#0f3460] rounded-2xl p-8 w-full max-w-md shadow-2xl">
          <div className="text-center mb-6">
            <div className="w-14 h-14 bg-amber-500/10 rounded-full flex items-center justify-center mx-auto mb-4">
              <KeyRound className="text-amber-400" size={28} />
            </div>
            <h2 className="text-xl font-bold text-white">Save Your Recovery Key</h2>
            <p className="text-sm text-gray-400 mt-2">
              Write this down and store it somewhere safe. You will not see it again.
            </p>
          </div>

          <div className="bg-[#0d0d1a] border border-amber-500/30 rounded-xl p-4 mb-6">
            <p className="font-mono text-amber-300 text-sm break-all leading-relaxed text-center tracking-wider">
              {recoveryKeyShown}
            </p>
          </div>

          <p className="text-xs text-gray-500 text-center mb-6">
            If you forget your vault password, this key is the only way to recover your journal.
          </p>

          <button
            onClick={() => setRecoveryAcknowledged(true)}
            className="w-full bg-amber-500 hover:bg-amber-400 text-black font-semibold py-3 rounded-xl transition-colors"
          >
            I've Saved My Recovery Key
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="fixed inset-0 bg-[#0d0d1a] flex items-center justify-center p-6">
      <div className="bg-[#16213e] border border-[#0f3460] rounded-2xl p-8 w-full max-w-md shadow-2xl">
        <div className="text-center mb-8">
          <div className="w-16 h-16 bg-blue-500/10 rounded-full flex items-center justify-center mx-auto mb-4">
            <Lock className="text-blue-400" size={32} />
          </div>
          <h1 className="text-2xl font-bold text-white">
            {setupMode ? "Welcome to ChronoXP" : "Unlock Journal"}
          </h1>
          <p className="text-sm text-gray-400 mt-2">
            {setupMode
              ? "Your encrypted journal awaits. Let's get started."
              : "Enter your vault password to continue."}
          </p>
        </div>

        {error && (
          <div className="bg-red-500/10 border border-red-500/30 rounded-xl p-3 mb-4 text-sm text-red-400">
            {error}
          </div>
        )}

        {setupMode && (
          <div className="mb-4">
            <label className="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                checked={wantVaultPassword}
                onChange={(e) => setWantVaultPassword(e.target.checked)}
                className="w-4 h-4 rounded accent-blue-500"
              />
              <span className="text-sm text-gray-300">Protect with a vault password</span>
            </label>
            <p className="text-xs text-gray-500 ml-7 mt-1">
              Recommended for extra security. You must remember this password — there's no reset.
            </p>
          </div>
        )}

        {((!setupMode && vaultPasswordEnabled) || (setupMode && wantVaultPassword)) && (
          <>
            <div className="relative mb-3">
              <input
                type={showPassword ? "text" : "password"}
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Vault password"
                autoFocus
                className="w-full bg-[#0d0d1a] border border-[#0f3460] rounded-xl px-4 py-3 pr-12 text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 transition-colors"
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-200"
              >
                {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
              </button>
            </div>

            {setupMode && wantVaultPassword && (
              <div className="mb-3">
                <input
                  type={showPassword ? "text" : "password"}
                  value={confirmPassword}
                  onChange={(e) => setConfirmPassword(e.target.value)}
                  onKeyDown={handleKeyDown}
                  placeholder="Confirm password"
                  className="w-full bg-[#0d0d1a] border border-[#0f3460] rounded-xl px-4 py-3 text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 transition-colors"
                />
                {confirmPassword && password !== confirmPassword && (
                  <p className="text-xs text-red-400 mt-1 ml-1">Passwords don't match</p>
                )}
                {setupMode && wantVaultPassword && password.length > 0 && password.length < 8 && (
                  <p className="text-xs text-amber-400 mt-1 ml-1">
                    Password must be at least 8 characters
                  </p>
                )}
              </div>
            )}
          </>
        )}

        <button
          onClick={setupMode ? handleSetup : handleUnlock}
          disabled={
            isLoading ||
            (setupMode && wantVaultPassword && (password !== confirmPassword || password.length < 8))
          }
          className="w-full bg-blue-600 hover:bg-blue-500 disabled:opacity-40 disabled:cursor-not-allowed text-white font-semibold py-3 rounded-xl transition-colors mt-4"
        >
          {isLoading ? "Please wait…" : setupMode ? "Get Started" : "Unlock"}
        </button>

        {!setupMode && (
          <p className="text-xs text-gray-500 text-center mt-4">
            Forgot your password? Use your recovery key in Settings → Security.
          </p>
        )}
      </div>
    </div>
  );
}
