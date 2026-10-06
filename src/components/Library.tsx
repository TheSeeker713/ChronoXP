import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { Pin, Plus } from "lucide-react";
import { useEntryStore } from "../stores/entryStore";
import { formatDate, formatTime } from "../utils/time";

export function Library() {
  const { entries, isLoading, fetchEntries, openEntry, newEntry } = useEntryStore();
  const navigate = useNavigate();

  useEffect(() => {
    fetchEntries();
  }, []);

  const handleOpen = async (id: number) => {
    await openEntry(id);
    navigate("/canvas");
  };

  const handleNew = () => {
    newEntry();
    navigate("/canvas");
  };

  const pinned = entries.filter((e) => e.pinned && !e.deleted_at);
  const rest = entries.filter((e) => !e.pinned && !e.deleted_at);

  return (
    <div className="h-full overflow-y-auto">
      <div className="px-6 py-4 flex items-center justify-between border-b border-[#0f3460] sticky top-0 bg-[#1a1a2e] z-10">
        <h1 className="text-lg font-semibold text-white">Library</h1>
        <button
          onClick={handleNew}
          className="flex items-center gap-2 bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded-xl text-sm font-medium transition-colors"
        >
          <Plus size={15} />
          New Entry
        </button>
      </div>

      {isLoading && (
        <div className="flex items-center justify-center py-16 text-gray-500">Loading…</div>
      )}

      {!isLoading && entries.length === 0 && (
        <div className="flex flex-col items-center justify-center py-20 text-gray-500">
          <div className="text-4xl mb-4">📖</div>
          <p className="text-base text-gray-400 mb-2">No entries yet</p>
          <p className="text-sm">Start writing to see your journal here.</p>
        </div>
      )}

      <div className="max-w-2xl mx-auto py-4 px-4">
        {pinned.length > 0 && (
          <>
            <div className="text-xs font-medium text-amber-400/80 uppercase tracking-wider mb-2 px-2">
              Pinned
            </div>
            {pinned.map((entry) => (
              <EntryCard key={entry.id} entry={entry} onClick={() => handleOpen(entry.id)} />
            ))}
            <div className="border-b border-[#0f3460]/50 my-4" />
          </>
        )}

        {rest.length > 0 && (
          <>
            {pinned.length > 0 && (
              <div className="text-xs font-medium text-gray-500 uppercase tracking-wider mb-2 px-2">
                All Entries
              </div>
            )}
            {rest.map((entry) => (
              <EntryCard key={entry.id} entry={entry} onClick={() => handleOpen(entry.id)} />
            ))}
          </>
        )}
      </div>
    </div>
  );
}

function EntryCard({
  entry,
  onClick,
}: {
  entry: import("../types").Entry;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className="w-full text-left p-4 rounded-xl mb-2 hover:bg-[#16213e] border border-transparent hover:border-[#0f3460] transition-all group"
    >
      <div className="flex items-start justify-between gap-3">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            {entry.pinned && <Pin size={11} className="text-amber-400 shrink-0" />}
            <h3 className="font-medium text-white text-sm truncate">
              {entry.title ?? (entry.content_preview.slice(0, 60) || "Untitled")}
            </h3>
          </div>
          <p className="text-xs text-gray-500 line-clamp-2 leading-relaxed">
            {entry.content_preview}
          </p>
          {entry.tags.length > 0 && (
            <div className="flex flex-wrap gap-1 mt-2">
              {entry.tags.map((tag) => (
                <span
                  key={tag.id}
                  className="text-xs bg-blue-600/15 text-blue-400/80 px-2 py-0.5 rounded-full"
                >
                  {tag.name}
                </span>
              ))}
            </div>
          )}
        </div>
        <div className="text-right shrink-0">
          <div className="text-xs text-gray-500">{formatDate(entry.created_at)}</div>
          <div className="text-xs text-gray-600">{formatTime(entry.created_at)}</div>
          {entry.word_count > 0 && (
            <div className="text-xs text-gray-600 mt-1">{entry.word_count}w</div>
          )}
        </div>
      </div>
    </button>
  );
}
