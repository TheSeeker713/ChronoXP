import { useEntryStore } from "../stores/entryStore";
import { formatDistanceToNow } from "../utils/time";

export function StatusBar() {
  const { currentEntry, isSaving, saveError, lastSavedAt } = useEntryStore();

  return (
    <div className="flex items-center gap-4 px-4 h-7 bg-[#16213e] border-t border-[#0f3460] shrink-0 text-xs text-gray-500">
      <span className="flex items-center gap-1">
        {isSaving ? (
          <>
            <span className="inline-block animate-spin">⟳</span>
            <span>Saving…</span>
          </>
        ) : saveError ? (
          <span className="text-red-400">⚠ Save failed</span>
        ) : lastSavedAt ? (
          <span className="text-gray-600">✓ Saved {formatDistanceToNow(lastSavedAt)}</span>
        ) : null}
      </span>

      {currentEntry && (
        <>
          <span className="text-gray-700">|</span>
          <span>{currentEntry.word_count} words</span>
        </>
      )}

      <span className="ml-auto flex items-center gap-1">
        <span className="w-1.5 h-1.5 rounded-full bg-green-500 inline-block"></span>
        <span>Offline</span>
      </span>
    </div>
  );
}
