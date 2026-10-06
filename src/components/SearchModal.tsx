import { useEffect, useRef, useState } from "react";
import { Search, X } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useEntryStore } from "../stores/entryStore";
import { formatDate } from "../utils/time";

interface SearchModalProps {
  onClose: () => void;
}

export function SearchModal({ onClose }: SearchModalProps) {
  const { search, searchResults, searchQuery, openEntry, clearSearch } = useEntryStore();
  const [query, setQuery] = useState(searchQuery);
  const inputRef = useRef<HTMLInputElement>(null);
  const navigate = useNavigate();

  useEffect(() => {
    inputRef.current?.focus();
    return () => clearSearch();
  }, []);

  useEffect(() => {
    const timer = setTimeout(() => {
      search(query);
    }, 200);
    return () => clearTimeout(timer);
  }, [query]);

  const handleSelect = async (id: number) => {
    await openEntry(id);
    onClose();
    navigate("/canvas");
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Escape") onClose();
  };

  return (
    <div
      className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-start justify-center pt-20 z-50 px-4"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div className="bg-[#16213e] border border-[#0f3460] rounded-2xl w-full max-w-lg shadow-2xl overflow-hidden">
        {/* Input */}
        <div className="flex items-center gap-3 px-4 py-3 border-b border-[#0f3460]">
          <Search size={18} className="text-gray-500 shrink-0" />
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Search entries…"
            className="flex-1 bg-transparent text-white placeholder-gray-500 outline-none text-base"
          />
          {query && (
            <button
              onClick={() => {
                setQuery("");
                clearSearch();
              }}
              className="text-gray-500 hover:text-gray-300"
            >
              <X size={16} />
            </button>
          )}
          <kbd className="text-xs text-gray-600 border border-gray-700 rounded px-1.5 py-0.5">
            Esc
          </kbd>
        </div>

        {/* Results */}
        <div className="max-h-96 overflow-y-auto">
          {!query && (
            <div className="px-4 py-8 text-center text-sm text-gray-500">
              Type to search your journal entries
            </div>
          )}

          {query && searchResults === null && (
            <div className="px-4 py-6 text-center text-sm text-gray-500">Searching…</div>
          )}

          {query && searchResults?.length === 0 && (
            <div className="px-4 py-8 text-center text-sm text-gray-500">
              No entries found for "{query}"
            </div>
          )}

          {searchResults && searchResults.length > 0 && (
            <ul className="py-2">
              {searchResults.map((result) => (
                <li key={result.id}>
                  <button
                    onClick={() => handleSelect(result.id)}
                    className="w-full text-left px-4 py-3 hover:bg-white/5 transition-colors"
                  >
                    <div className="flex items-start justify-between gap-3">
                      <div className="flex-1 min-w-0">
                        <p className="text-sm font-medium text-white truncate">
                          {result.title ?? "Untitled"}
                        </p>
                        <p className="text-xs text-gray-400 mt-0.5 line-clamp-2 leading-relaxed">
                          {result.snippet}
                        </p>
                        {result.tags.length > 0 && (
                          <div className="flex flex-wrap gap-1 mt-1.5">
                            {result.tags.map((t) => (
                              <span
                                key={t.id}
                                className="text-xs text-blue-400/70 bg-blue-600/10 px-1.5 py-0.5 rounded-full"
                              >
                                {t.name}
                              </span>
                            ))}
                          </div>
                        )}
                      </div>
                      <span className="text-xs text-gray-600 shrink-0">
                        {formatDate(result.created_at)}
                      </span>
                    </div>
                  </button>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </div>
  );
}
