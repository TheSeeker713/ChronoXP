import { useEffect, useState, useCallback } from "react";
import { Routes, Route, Navigate, useNavigate } from "react-router-dom";
import { Toaster } from "react-hot-toast";
import { useAuthStore } from "./stores/authStore";
import { useEntryStore } from "./stores/entryStore";
import { LockScreen } from "./components/LockScreen";
import { NavBar } from "./components/NavBar";
import { StatusBar } from "./components/StatusBar";
import { Canvas } from "./components/Canvas";
import { Library } from "./components/Library";
import { Settings } from "./components/Settings";
import { SearchModal } from "./components/SearchModal";

const IDLE_CHECK_INTERVAL = 60_000; // 1 minute

export function App() {
  const { isLocked, isInitialized, fetchStatus, lock } = useAuthStore();
  const { newEntry } = useEntryStore();
  const navigate = useNavigate();
  const [searchOpen, setSearchOpen] = useState(false);
  const [lastActivity, setLastActivity] = useState(Date.now());

  useEffect(() => {
    fetchStatus();
  }, [fetchStatus]);

  // Refresh activity timestamp on user interaction
  const onActivity = useCallback(() => setLastActivity(Date.now()), []);

  useEffect(() => {
    window.addEventListener("keydown", onActivity, { passive: true });
    window.addEventListener("mousedown", onActivity, { passive: true });
    window.addEventListener("mousemove", onActivity, { passive: true });
    return () => {
      window.removeEventListener("keydown", onActivity);
      window.removeEventListener("mousedown", onActivity);
      window.removeEventListener("mousemove", onActivity);
    };
  }, [onActivity]);

  // Idle auto-lock
  useEffect(() => {
    if (isLocked) return;
    const autoLockMinutes =
      Number(localStorage.getItem("auto_lock_minutes") ?? 15);
    if (autoLockMinutes === 0) return;

    const interval = setInterval(() => {
      if (Date.now() - lastActivity > autoLockMinutes * 60_000) {
        lock();
      }
    }, IDLE_CHECK_INTERVAL);

    return () => clearInterval(interval);
  }, [isLocked, lastActivity, lock]);

  // Global keyboard shortcuts
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (isLocked) return;

      const ctrl = e.ctrlKey || e.metaKey;
      if (ctrl && e.key === "n") {
        e.preventDefault();
        newEntry();
        navigate("/canvas");
      }
      if (ctrl && e.key === "f") {
        e.preventDefault();
        setSearchOpen(true);
      }
      if (ctrl && e.key === "l") {
        e.preventDefault();
        lock();
      }
      if (e.key === "Escape" && searchOpen) {
        setSearchOpen(false);
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [isLocked, searchOpen, newEntry, navigate, lock]);

  if (!isInitialized || isLocked) {
    return (
      <>
        <LockScreen />
        <Toaster position="bottom-right" />
      </>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <NavBar onSearchOpen={() => setSearchOpen(true)} />

      <main className="flex-1 overflow-hidden">
        <Routes>
          <Route path="/" element={<Navigate to="/canvas" replace />} />
          <Route path="/canvas" element={<Canvas />} />
          <Route path="/library" element={<Library />} />
          <Route path="/settings" element={<Settings />} />
        </Routes>
      </main>

      <StatusBar />
      <Toaster position="bottom-right" toastOptions={{ style: { background: "#16213e", color: "#e0e0e0", border: "1px solid #0f3460" } }} />
      {searchOpen && <SearchModal onClose={() => setSearchOpen(false)} />}
    </div>
  );
}
