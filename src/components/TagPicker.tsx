import { useEffect, useState } from "react";
import { X, Plus } from "lucide-react";
import { listTags, createTag } from "../utils/ipc";
import type { Tag } from "../types";

interface TagPickerProps {
  selectedTags: string[];
  onClose: () => void;
  onConfirm: (tags: string[]) => void;
}

export function TagPicker({ selectedTags, onClose, onConfirm }: TagPickerProps) {
  const [allTags, setAllTags] = useState<Tag[]>([]);
  const [selected, setSelected] = useState<Set<string>>(new Set(selectedTags));
  const [newTagInput, setNewTagInput] = useState("");
  const [isCreating, setIsCreating] = useState(false);

  useEffect(() => {
    listTags().then(setAllTags).catch(console.error);
  }, []);

  const toggle = (name: string) => {
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(name)) next.delete(name);
      else next.add(name);
      return next;
    });
  };

  const handleCreate = async () => {
    const name = newTagInput.trim();
    if (!name) return;
    setIsCreating(true);
    try {
      const tag = await createTag(name);
      setAllTags((prev) => [...prev, tag]);
      setSelected((prev) => new Set([...prev, tag.name]));
      setNewTagInput("");
    } catch (err) {
      console.error("Failed to create tag:", err);
    } finally {
      setIsCreating(false);
    }
  };

  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50 p-4"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div className="bg-[#16213e] border border-[#0f3460] rounded-2xl w-full max-w-sm shadow-2xl">
        <div className="flex items-center justify-between px-4 py-3 border-b border-[#0f3460]">
          <h3 className="text-sm font-semibold text-white">Tags</h3>
          <button onClick={onClose} className="text-gray-500 hover:text-gray-300">
            <X size={16} />
          </button>
        </div>

        <div className="p-4">
          {/* Existing tags */}
          <div className="flex flex-wrap gap-2 mb-4 max-h-48 overflow-y-auto">
            {allTags.map((tag) => (
              <button
                key={tag.id}
                onClick={() => toggle(tag.name)}
                className={`text-sm px-3 py-1 rounded-full transition-colors ${
                  selected.has(tag.name)
                    ? "bg-blue-600 text-white"
                    : "bg-[#0f3460] text-gray-300 hover:bg-[#1a3a6e]"
                }`}
              >
                {tag.name}
              </button>
            ))}
            {allTags.length === 0 && (
              <p className="text-xs text-gray-500">No tags yet. Create one below.</p>
            )}
          </div>

          {/* Create new tag */}
          <div className="flex items-center gap-2">
            <input
              type="text"
              value={newTagInput}
              onChange={(e) => setNewTagInput(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") handleCreate();
              }}
              placeholder="New tag name…"
              className="flex-1 bg-[#0d0d1a] border border-[#0f3460] rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-blue-500"
            />
            <button
              onClick={handleCreate}
              disabled={!newTagInput.trim() || isCreating}
              className="p-2 bg-blue-600 hover:bg-blue-500 disabled:opacity-40 text-white rounded-lg transition-colors"
            >
              <Plus size={16} />
            </button>
          </div>
        </div>

        <div className="flex justify-end gap-2 px-4 py-3 border-t border-[#0f3460]">
          <button
            onClick={onClose}
            className="px-4 py-2 text-sm text-gray-400 hover:text-gray-200 transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={() => onConfirm(Array.from(selected))}
            className="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 text-white rounded-xl font-medium transition-colors"
          >
            Apply
          </button>
        </div>
      </div>
    </div>
  );
}
