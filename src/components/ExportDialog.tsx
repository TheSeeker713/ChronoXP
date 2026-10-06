import { useState } from "react";
import { X, Download, Upload, Eye, EyeOff } from "lucide-react";
import { exportJournal, importJournal } from "../utils/ipc";
import toast from "react-hot-toast";
import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";

interface ExportDialogProps {
  onClose: () => void;
}

export function ExportDialog({ onClose }: ExportDialogProps) {
  const [tab, setTab] = useState<"export" | "import">("export");
  const [includeAttachments, setIncludeAttachments] = useState(true);
  const [includeSettings, setIncludeSettings] = useState(false);
  const [usePassword, setUsePassword] = useState(true);
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);

  const handleExport = async () => {
    if (usePassword && password !== confirmPassword) {
      toast.error("Passwords don't match");
      return;
    }
    if (usePassword && password.length < 6) {
      toast.error("Password must be at least 6 characters");
      return;
    }

    const savePath = await saveDialog({
      defaultPath: `chronoxp_journal_${new Date().toISOString().slice(0, 10)}.chrxp`,
      filters: [{ name: "ChronoXP Archive", extensions: ["chrxp"] }],
    });

    if (!savePath) return;
    const pathStr = savePath.toString();

    setIsProcessing(true);
    try {
      const result = await exportJournal(
        {
          include_entries: true,
          include_attachments: includeAttachments,
          include_settings: includeSettings,
          password: usePassword ? password : null,
        },
        pathStr,
      );
      toast.success(`Exported ${result.entry_count} entries`);
      onClose();
    } catch (err) {
      toast.error(`Export failed: ${err}`);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleImport = async () => {
    const selected = await openDialog({
      multiple: false,
      filters: [{ name: "ChronoXP Archive", extensions: ["chrxp"] }],
    });

    if (!selected) return;

    const filePath = Array.isArray(selected) ? selected[0] : selected;
    setIsProcessing(true);
    try {
      const result = await importJournal(filePath, usePassword ? password : undefined);
      toast.success(`Imported ${result.imported} entries, skipped ${result.skipped} duplicates`);
      onClose();
    } catch (err) {
      toast.error(`Import failed: ${err}`);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50 p-4"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div className="bg-[#16213e] border border-[#0f3460] rounded-2xl w-full max-w-md shadow-2xl">
        <div className="flex items-center justify-between px-5 py-4 border-b border-[#0f3460]">
          <h2 className="text-base font-semibold text-white">Export & Import</h2>
          <button onClick={onClose} className="text-gray-500 hover:text-gray-300">
            <X size={18} />
          </button>
        </div>

        {/* Tabs */}
        <div className="flex border-b border-[#0f3460]">
          {(["export", "import"] as const).map((t) => (
            <button
              key={t}
              onClick={() => setTab(t)}
              className={`flex-1 py-3 text-sm font-medium capitalize transition-colors ${
                tab === t
                  ? "border-b-2 border-blue-500 text-blue-400"
                  : "text-gray-400 hover:text-gray-200"
              }`}
            >
              {t === "export" ? (
                <span className="flex items-center justify-center gap-2">
                  <Download size={14} /> Export
                </span>
              ) : (
                <span className="flex items-center justify-center gap-2">
                  <Upload size={14} /> Import
                </span>
              )}
            </button>
          ))}
        </div>

        <div className="p-5">
          {tab === "export" && (
            <div className="space-y-4">
              <div className="space-y-2">
                <p className="text-sm text-gray-400 font-medium">Include:</p>
                <CheckboxOption
                  checked={true}
                  disabled
                  label="Entries (text + metadata)"
                />
                <CheckboxOption
                  checked={includeAttachments}
                  onChange={setIncludeAttachments}
                  label="Attachments (images, files)"
                />
                <CheckboxOption
                  checked={includeSettings}
                  onChange={setIncludeSettings}
                  label="Settings & preferences"
                />
              </div>

              <div className="border-t border-[#0f3460] pt-4">
                <CheckboxOption
                  checked={usePassword}
                  onChange={setUsePassword}
                  label="🔐 Encrypt with password (recommended)"
                />
                {usePassword && (
                  <div className="mt-3 space-y-2">
                    <PasswordField
                      value={password}
                      onChange={setPassword}
                      show={showPassword}
                      onToggleShow={() => setShowPassword(!showPassword)}
                      placeholder="Export password"
                    />
                    <PasswordField
                      value={confirmPassword}
                      onChange={setConfirmPassword}
                      show={showPassword}
                      onToggleShow={() => setShowPassword(!showPassword)}
                      placeholder="Confirm password"
                    />
                    <p className="text-xs text-amber-400/80">
                      ⚠ Remember this password! It cannot be recovered.
                    </p>
                  </div>
                )}
              </div>

              <button
                onClick={handleExport}
                disabled={isProcessing}
                className="w-full bg-blue-600 hover:bg-blue-500 disabled:opacity-40 text-white font-medium py-2.5 rounded-xl transition-colors flex items-center justify-center gap-2"
              >
                <Download size={15} />
                {isProcessing ? "Exporting…" : "Export as .chrxp"}
              </button>
            </div>
          )}

          {tab === "import" && (
            <div className="space-y-4">
              <p className="text-sm text-gray-400">
                Import a <code className="text-blue-400">.chrxp</code> archive. Existing entries
                with matching IDs will be skipped.
              </p>

              <CheckboxOption
                checked={usePassword}
                onChange={setUsePassword}
                label="Archive is password-protected"
              />
              {usePassword && (
                <PasswordField
                  value={password}
                  onChange={setPassword}
                  show={showPassword}
                  onToggleShow={() => setShowPassword(!showPassword)}
                  placeholder="Archive password"
                />
              )}

              <button
                onClick={handleImport}
                disabled={isProcessing}
                className="w-full bg-blue-600 hover:bg-blue-500 disabled:opacity-40 text-white font-medium py-2.5 rounded-xl transition-colors flex items-center justify-center gap-2"
              >
                <Upload size={15} />
                {isProcessing ? "Importing…" : "Choose .chrxp File…"}
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function CheckboxOption({
  checked,
  onChange,
  disabled,
  label,
}: {
  checked: boolean;
  onChange?: (v: boolean) => void;
  disabled?: boolean;
  label: string;
}) {
  return (
    <label className={`flex items-center gap-2.5 ${disabled ? "opacity-50" : "cursor-pointer"}`}>
      <input
        type="checkbox"
        checked={checked}
        disabled={disabled}
        onChange={(e) => onChange?.(e.target.checked)}
        className="w-4 h-4 rounded accent-blue-500"
      />
      <span className="text-sm text-gray-300">{label}</span>
    </label>
  );
}

function PasswordField({
  value,
  onChange,
  show,
  onToggleShow,
  placeholder,
}: {
  value: string;
  onChange: (v: string) => void;
  show: boolean;
  onToggleShow: () => void;
  placeholder: string;
}) {
  return (
    <div className="relative">
      <input
        type={show ? "text" : "password"}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="w-full bg-[#0d0d1a] border border-[#0f3460] rounded-xl px-3 py-2 pr-10 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-blue-500"
      />
      <button
        type="button"
        onClick={onToggleShow}
        className="absolute right-2.5 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-200"
      >
        {show ? <EyeOff size={15} /> : <Eye size={15} />}
      </button>
    </div>
  );
}
