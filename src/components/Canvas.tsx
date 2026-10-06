import { useEffect, useRef, useCallback } from "react";
import { useEditor, EditorContent } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import Placeholder from "@tiptap/extension-placeholder";
import Underline from "@tiptap/extension-underline";
import {
  Bold,
  Italic,
  UnderlineIcon,
  Heading1,
  Heading2,
  List,
  ListOrdered,
  Pin,
  Plus,
  Trash2,
} from "lucide-react";
import { useEntryStore } from "../stores/entryStore";
import { wordCount } from "../utils/time";
import { TagPicker } from "./TagPicker";
import { useState } from "react";

const AUTOSAVE_DELAY = 3000;

export function Canvas() {
  const {
    currentEntry,
    newEntry,
    saveEntry,
    removeEntry,
    togglePin,
    fetchEntries,
  } = useEntryStore();

  const [showTagPicker, setShowTagPicker] = useState(false);
  const [currentTags, setCurrentTags] = useState<string[]>([]);
  const autosaveTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const isDirty = useRef(false);

  const editor = useEditor({
    extensions: [
      StarterKit,
      Underline,
      Placeholder.configure({
        placeholder: "Want a soft start? Try a one-line check-in…",
      }),
    ],
    editorProps: {
      attributes: {
        class: "tiptap-editor outline-none",
      },
    },
    onUpdate: ({ editor }) => {
      isDirty.current = true;
      scheduleAutosave(editor.getHTML());
    },
  });

  useEffect(() => {
    if (!currentEntry) return;
    if (editor && currentEntry.markdown !== editor.getHTML()) {
      editor.commands.setContent(currentEntry.markdown || "", false);
    }
    setCurrentTags(currentEntry.tags.map((t) => t.name));
    isDirty.current = false;
  }, [currentEntry?.id]); // only re-sync when entry ID changes

  const scheduleAutosave = useCallback(
    (html: string) => {
      if (autosaveTimer.current) clearTimeout(autosaveTimer.current);
      autosaveTimer.current = setTimeout(async () => {
        if (!isDirty.current) return;
        const wc = wordCount(html.replace(/<[^>]+>/g, " "));
        if (currentEntry && currentEntry.id > 0) {
          await saveEntry({
            id: currentEntry.id,
            markdown: html,
            tags: currentTags,
          });
        } else {
          await saveEntry({ markdown: html, tags: currentTags });
        }
        isDirty.current = false;
      }, AUTOSAVE_DELAY);
    },
    [currentEntry, currentTags, saveEntry],
  );

  const handleManualSave = async () => {
    if (!editor) return;
    const html = editor.getHTML();
    const wc = wordCount(html.replace(/<[^>]+>/g, " "));
    if (currentEntry && currentEntry.id > 0) {
      await saveEntry({ id: currentEntry.id, markdown: html, tags: currentTags });
    } else {
      await saveEntry({ markdown: html, tags: currentTags });
    }
    isDirty.current = false;
  };

  const handleDelete = async () => {
    if (!currentEntry || currentEntry.id === 0) return;
    if (!confirm("Delete this entry?")) return;
    await removeEntry(currentEntry.id);
    fetchEntries();
  };

  if (!currentEntry) {
    return (
      <div className="flex flex-col items-center justify-center h-full text-gray-500">
        <div className="text-5xl mb-4">📝</div>
        <p className="text-lg font-medium mb-2 text-gray-400">No entry selected</p>
        <p className="text-sm mb-6">Press Ctrl+N or open an entry from the Library.</p>
        <button
          onClick={() => newEntry()}
          className="flex items-center gap-2 bg-blue-600 hover:bg-blue-500 text-white px-5 py-2.5 rounded-xl font-medium transition-colors"
        >
          <Plus size={16} />
          New Entry
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* Toolbar */}
      <div className="flex items-center justify-between px-4 py-2 border-b border-[#0f3460] shrink-0 bg-[#16213e]/50">
        <div className="flex items-center gap-1">
          <ToolbarButton
            onClick={() => editor?.chain().focus().toggleBold().run()}
            active={editor?.isActive("bold")}
            title="Bold (Ctrl+B)"
          >
            <Bold size={14} />
          </ToolbarButton>
          <ToolbarButton
            onClick={() => editor?.chain().focus().toggleItalic().run()}
            active={editor?.isActive("italic")}
            title="Italic (Ctrl+I)"
          >
            <Italic size={14} />
          </ToolbarButton>
          <ToolbarButton
            onClick={() => editor?.chain().focus().toggleUnderline().run()}
            active={editor?.isActive("underline")}
            title="Underline (Ctrl+U)"
          >
            <UnderlineIcon size={14} />
          </ToolbarButton>
          <div className="w-px h-4 bg-[#0f3460] mx-1" />
          <ToolbarButton
            onClick={() => editor?.chain().focus().toggleHeading({ level: 1 }).run()}
            active={editor?.isActive("heading", { level: 1 })}
            title="Heading 1"
          >
            <Heading1 size={14} />
          </ToolbarButton>
          <ToolbarButton
            onClick={() => editor?.chain().focus().toggleHeading({ level: 2 }).run()}
            active={editor?.isActive("heading", { level: 2 })}
            title="Heading 2"
          >
            <Heading2 size={14} />
          </ToolbarButton>
          <div className="w-px h-4 bg-[#0f3460] mx-1" />
          <ToolbarButton
            onClick={() => editor?.chain().focus().toggleBulletList().run()}
            active={editor?.isActive("bulletList")}
            title="Bullet list"
          >
            <List size={14} />
          </ToolbarButton>
          <ToolbarButton
            onClick={() => editor?.chain().focus().toggleOrderedList().run()}
            active={editor?.isActive("orderedList")}
            title="Numbered list"
          >
            <ListOrdered size={14} />
          </ToolbarButton>
        </div>

        <div className="flex items-center gap-2">
          {currentEntry.id > 0 && (
            <button
              onClick={() => togglePin(currentEntry.id)}
              className={`p-1.5 rounded-lg transition-colors ${
                currentEntry.pinned
                  ? "text-amber-400 bg-amber-400/10"
                  : "text-gray-500 hover:text-gray-300 hover:bg-white/5"
              }`}
              title="Pin entry"
            >
              <Pin size={14} />
            </button>
          )}
          {currentEntry.id > 0 && (
            <button
              onClick={handleDelete}
              className="p-1.5 rounded-lg text-gray-500 hover:text-red-400 hover:bg-red-400/10 transition-colors"
              title="Delete entry"
            >
              <Trash2 size={14} />
            </button>
          )}
          <button
            onClick={handleManualSave}
            className="px-3 py-1 text-xs bg-blue-600/20 hover:bg-blue-600/30 text-blue-400 rounded-lg transition-colors"
          >
            Save
          </button>
        </div>
      </div>

      {/* Tags row */}
      <div className="flex items-center flex-wrap gap-2 px-6 py-2 border-b border-[#0f3460]/50 shrink-0">
        {currentTags.map((tag) => (
          <span
            key={tag}
            className="flex items-center gap-1 text-xs bg-blue-600/20 text-blue-300 px-2 py-0.5 rounded-full"
          >
            {tag}
            <button
              onClick={() => setCurrentTags((prev) => prev.filter((t) => t !== tag))}
              className="text-blue-400/60 hover:text-blue-300 ml-0.5"
            >
              ×
            </button>
          </span>
        ))}
        <button
          onClick={() => setShowTagPicker(true)}
          className="text-xs text-gray-500 hover:text-gray-300 flex items-center gap-1 transition-colors"
        >
          <Plus size={11} /> Add tag
        </button>
      </div>

      {/* Editor */}
      <div className="flex-1 overflow-y-auto px-8 py-6">
        <EditorContent editor={editor} />
      </div>

      {showTagPicker && (
        <TagPicker
          selectedTags={currentTags}
          onClose={() => setShowTagPicker(false)}
          onConfirm={(tags) => {
            setCurrentTags(tags);
            setShowTagPicker(false);
          }}
        />
      )}
    </div>
  );
}

function ToolbarButton({
  children,
  onClick,
  active,
  title,
}: {
  children: React.ReactNode;
  onClick: () => void;
  active?: boolean;
  title?: string;
}) {
  return (
    <button
      onClick={onClick}
      title={title}
      className={`p-1.5 rounded-md transition-colors ${
        active
          ? "bg-blue-600/30 text-blue-400"
          : "text-gray-400 hover:text-gray-200 hover:bg-white/5"
      }`}
    >
      {children}
    </button>
  );
}
