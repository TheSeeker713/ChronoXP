import { NavLink } from "react-router-dom";
import { BookOpen, Library, Settings, Search, Lock } from "lucide-react";
import { useAuthStore } from "../stores/authStore";

interface NavBarProps {
  onSearchOpen: () => void;
}

export function NavBar({ onSearchOpen }: NavBarProps) {
  const { lock } = useAuthStore();

  const navLinkClass = ({ isActive }: { isActive: boolean }) =>
    `flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm font-medium transition-colors ${
      isActive
        ? "bg-blue-600/20 text-blue-400"
        : "text-gray-400 hover:text-gray-200 hover:bg-white/5"
    }`;

  return (
    <nav
      data-tauri-drag-region
      className="flex items-center justify-between px-4 h-11 bg-[#16213e] border-b border-[#0f3460] shrink-0"
    >
      <div className="flex items-center gap-1">
        <span className="text-sm font-bold text-blue-400 mr-3 select-none">ChronoXP</span>
        <NavLink to="/canvas" className={navLinkClass}>
          <BookOpen size={15} />
          <span>Canvas</span>
        </NavLink>
        <NavLink to="/library" className={navLinkClass}>
          <Library size={15} />
          <span>Library</span>
        </NavLink>
        <NavLink to="/settings" className={navLinkClass}>
          <Settings size={15} />
          <span>Settings</span>
        </NavLink>
      </div>

      <div className="flex items-center gap-2">
        <button
          onClick={onSearchOpen}
          className="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm text-gray-400 hover:text-gray-200 hover:bg-white/5 transition-colors"
          title="Search (Ctrl+F)"
        >
          <Search size={15} />
          <span className="text-xs text-gray-600">Ctrl+F</span>
        </button>
        <button
          onClick={() => lock()}
          className="p-1.5 rounded-lg text-gray-500 hover:text-gray-300 hover:bg-white/5 transition-colors"
          title="Lock journal (Ctrl+L)"
        >
          <Lock size={15} />
        </button>
      </div>
    </nav>
  );
}
