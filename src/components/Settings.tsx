import { useEffect } from "react";
import { useSettingsStore } from "../stores/settingsStore";
import { useAuthStore } from "../stores/authStore";
import { ExportDialog } from "./ExportDialog";
import { useState } from "react";

export function Settings() {
  const { settings, fetchSettings, updateSetting } = useSettingsStore();
  const { vaultPasswordEnabled } = useAuthStore();
  const [showExport, setShowExport] = useState(false);

  useEffect(() => {
    fetchSettings();
  }, []);

  if (!settings) return <div className="p-8 text-gray-500">Loading settings…</div>;

  return (
    <div className="h-full overflow-y-auto">
      <div className="px-6 py-4 border-b border-[#0f3460] sticky top-0 bg-[#1a1a2e] z-10">
        <h1 className="text-lg font-semibold text-white">Settings</h1>
      </div>

      <div className="max-w-xl mx-auto py-6 px-6 space-y-8">
        {/* Appearance */}
        <Section title="Appearance">
          <SettingRow label="Theme">
            <select
              value={settings.theme}
              onChange={(e) =>
                updateSetting("theme", e.target.value as "dark" | "light" | "low-stim")
              }
              className="bg-[#0d0d1a] border border-[#0f3460] rounded-lg px-3 py-1.5 text-sm text-white focus:outline-none focus:border-blue-500"
            >
              <option value="dark">Dark</option>
              <option value="light">Light</option>
              <option value="low-stim">Low-Stimulation</option>
            </select>
          </SettingRow>

          <SettingRow label="Simple mode" description="Hides gamification elements">
            <Toggle
              checked={settings.simple_mode}
              onChange={(v) => updateSetting("simple_mode", v)}
            />
          </SettingRow>
        </Section>

        {/* Security */}
        <Section title="Security">
          <SettingRow
            label="Auto-lock after"
            description="Lock the journal after this many minutes of inactivity"
          >
            <select
              value={settings.auto_lock_minutes}
              onChange={(e) => updateSetting("auto_lock_minutes", Number(e.target.value))}
              className="bg-[#0d0d1a] border border-[#0f3460] rounded-lg px-3 py-1.5 text-sm text-white focus:outline-none focus:border-blue-500"
            >
              <option value={5}>5 minutes</option>
              <option value={10}>10 minutes</option>
              <option value={15}>15 minutes</option>
              <option value={30}>30 minutes</option>
              <option value={60}>1 hour</option>
              <option value={0}>Never</option>
            </select>
          </SettingRow>

          <SettingRow
            label="Vault password"
            description={
              vaultPasswordEnabled
                ? "Your journal is protected with a vault password"
                : "Add an extra layer of protection with a vault password"
            }
          >
            <span
              className={`text-xs px-2 py-1 rounded-full ${
                vaultPasswordEnabled
                  ? "bg-green-600/20 text-green-400"
                  : "bg-gray-600/20 text-gray-400"
              }`}
            >
              {vaultPasswordEnabled ? "Enabled" : "Disabled"}
            </span>
          </SettingRow>
        </Section>

        {/* Export & Import */}
        <Section title="Data & Backups">
          <p className="text-sm text-gray-400 mb-4">
            Export your entire journal as an encrypted <code className="text-blue-400">.chrxp</code>{" "}
            archive, or import from a previous backup.
          </p>
          <button
            onClick={() => setShowExport(true)}
            className="bg-[#0f3460] hover:bg-[#1a4a80] text-blue-300 px-4 py-2 rounded-xl text-sm font-medium transition-colors"
          >
            Export / Import Journal…
          </button>
        </Section>

        {/* About */}
        <Section title="About">
          <div className="text-sm text-gray-400 space-y-1">
            <p>
              <span className="text-gray-300">Version:</span> 0.1.0 (MVP)
            </p>
            <p>
              <span className="text-gray-300">Storage:</span> Encrypted AES-256-GCM (field-level)
            </p>
            <p>
              <span className="text-gray-300">Database:</span> SQLite (local-only)
            </p>
            <p className="mt-3 text-gray-500 text-xs">
              Your journal never leaves your device. No cloud. No tracking.
            </p>
          </div>
        </Section>
      </div>

      {showExport && <ExportDialog onClose={() => setShowExport(false)} />}
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div>
      <h2 className="text-xs font-semibold text-blue-400/70 uppercase tracking-wider mb-3">
        {title}
      </h2>
      <div className="space-y-4">{children}</div>
    </div>
  );
}

function SettingRow({
  label,
  description,
  children,
}: {
  label: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex items-start justify-between gap-4">
      <div>
        <p className="text-sm text-gray-200">{label}</p>
        {description && <p className="text-xs text-gray-500 mt-0.5">{description}</p>}
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}

function Toggle({ checked, onChange }: { checked: boolean; onChange: (v: boolean) => void }) {
  return (
    <button
      role="switch"
      aria-checked={checked}
      onClick={() => onChange(!checked)}
      className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${
        checked ? "bg-blue-600" : "bg-gray-600"
      }`}
    >
      <span
        className={`inline-block h-3.5 w-3.5 transform rounded-full bg-white transition-transform ${
          checked ? "translate-x-4" : "translate-x-0.5"
        }`}
      />
    </button>
  );
}
