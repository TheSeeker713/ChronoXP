# Wireframe: Records Cavern

**Project:** ChronoXP™ Journal  
**Date:** October 23, 2025

---

## Overview

The Records Cavern is the gamification space where users view their progress, unlock artifacts, and explore their journaling journey in a cozy fantasy setting.

---

## 1. Records Cavern (Initial State - Level 1)

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [🏛️ Cavern] [⚙️ Settings]    [🔍] [Ctrl+K] │
├─────────────────────────────────────────────────────────────────────┤
│  Records Cavern                           [⛶ Fullscreen] [− Zoom]   │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│     🏔️⛰️ [Background: Distant painted mountains, twilight sky]      │
│       🌲  🌲 [Mid-ground: Ancient stone archway, weathered trees]    │
│         🕯️  📖 [Artifacts: Single candle, worn leather journal]     │
│      ┌──────────────────────────────────────┐                       │
│      │      [Wooden desk with quill]        │ ← Player's workspace  │
│      └──────────────────────────────────────┘                       │
│     ════════════════════════════════════════  [Cobblestone floor]   │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Progress to next unlock:  ████████░░ 80%  (20 XP away)       │  │
│  │ Next artifact: "Memory Jar"                                  │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
│  XP: 30  |  Artifacts: 1/8  |  [View XP Log]                        │
└─────────────────────────────────────────────────────────────────────┘
```

**Visual Design:**
- **Hand-painted background:** Mountains, sky, trees (layered for parallax)
- **Artifacts:** Currently unlocked artifacts placed organically in scene
  - 🕯️ Candle (glowing warmth)
  - 📖 First Journal (on desk)
- **Lighting:** Warm candlelight, soft shadows painted into textures
- **Atmosphere:** Cozy, calm, inviting (not dark or ominous)

**Components:**
- **Top toolbar:** [⛶ Fullscreen] [− Zoom controls]
- **Progress bar:** Wooden plaque at bottom showing next unlock
- **Status bar:** XP count, artifacts unlocked, [View XP Log] link

---

## 2. Records Cavern (Mid-Level - Level 3)

```
┌─────────────────────────────────────────────────────────────────────┐
│  Records Cavern                           [⛶ Fullscreen] [− Zoom]   │
│  ─────────────────────────────────────────────────────────────────  │
│                                                                       │
│     🏔️⛰️ [Background: Mountains with starry night sky]              │
│       🌲🏛️🌲 [Mid-ground: Stone archway, decorative tapestries]      │
│      🪴 📚🕯️🏺📜 [Artifacts: Multiple items on stone shelves]        │
│         ┌──────────────────────────┐                                │
│         │   Ornate wooden desk     │                                │
│         │   🖋️ Brass quill, 📖 books │                                │
│         └──────────────────────────┘                                │
│      ════════════════════════════════  [Polished stone floor]       │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Progress to next unlock:  ██████░░░░ 60%  (80 XP away)       │  │
│  │ Next artifact: "Star Map"                                    │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                       │
│  XP: 320  |  Artifacts: 5/8  |  [View XP Log]                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Visual Evolution:**
- **More space:** Room appears larger, ceiling vaulted
- **More artifacts:** 5 artifacts displayed (Memory Jar, Ancient Scroll, Cozy Lantern, Bookshelf, Candle)
- **Richer details:** Tapestries on walls, plants in corners, brass fixtures
- **Lighting:** Multiple light sources (candles, lantern glow, moonlight through window)

---

## 3. Artifact Interaction (Click to Examine)

### Hover State
```
     🏺 [Artifact glows softly with painted highlight]
   (Cursor changes to magnifying glass icon)
```

### Click Opens Modal
```
┌─────────────────────────────────────────────────────────────────────┐
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                                                            [×]│  │
│  │                                                               │  │
│  │               [Hand-painted closeup of Memory Jar]            │  │
│  │                                                               │  │
│  │  ┌─────────────────────────────────────────────────────────┐ │  │
│  │  │ Memory Jar                                              │ │  │
│  │  │ ─────────────────────────────────────────────────────   │ │  │
│  │  │                                                         │ │  │
│  │  │ "A glass jar filled with paper scraps, each one       │ │  │
│  │  │  holding a fleeting memory."                          │ │  │
│  │  │                                                         │ │  │
│  │  │ Unlocked: November 3, 2025                            │ │  │
│  │  │ Required XP: 50                                        │ │  │
│  │  │                                                         │ │  │
│  │  │ Related entries: 12 entries since unlock              │ │  │
│  │  │ [View Related Entries]                                │ │  │
│  │  │                                                         │ │  │
│  │  │                      [Close]                           │ │  │
│  │  └─────────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

**Modal Features:**
- **Closeup artwork:** Detailed hand-painted image of artifact (shows texture, age, craftsmanship)
- **Flavor text:** Poetic description of artifact's meaning
- **Unlock date:** When user earned this artifact
- **XP requirement:** Shows milestone achieved
- **Related entries:** Count of entries written since unlocking (optional future feature: click to see list)
- **Parchment background:** Hand-painted texture, serif fantasy font

---

## 4. XP Log (View Progress History)

```
┌─────────────────────────────────────────────────────────────────────┐
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │ XP Log                                                     [×]│  │
│  │ ─────────────────────────────────────────────────────────────  │  │
│  │                                                               │  │
│  │ Total XP: 320  |  [Filter: All Activities ▼]                 │  │
│  │                                                               │  │
│  │ Today                                                         │  │
│  │ ───────────────────────────────────────────────────────────  │  │
│  │ +10 XP  •  Wrote entry (97 words)            2:34 PM        │  │
│  │                                                               │  │
│  │ Yesterday                                                     │  │
│  │ ───────────────────────────────────────────────────────────  │  │
│  │ +10 XP  •  Wrote entry (201 words)           8:12 AM        │  │
│  │ +5 XP   •  Tagged 5 entries                  10:45 AM       │  │
│  │                                                               │  │
│  │ 3 days ago                                                    │  │
│  │ ───────────────────────────────────────────────────────────  │  │
│  │ +5 XP   •  Welcome back bonus!               6:20 PM        │  │
│  │ +10 XP  •  Wrote entry (489 words)           6:45 PM        │  │
│  │                                                               │  │
│  │ [Load More]                                                  │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

**Features:**
- **Chronological list:** All XP-earning actions logged
- **Grouped by date:** Today, Yesterday, 3 days ago, etc.
- **Action description:** Clear explanation of why XP was earned
- **Time stamp:** When action occurred
- **Filter dropdown:** Filter by activity type (writing, tagging, quests, etc.)
- **Infinite scroll:** Load more history on demand

---

## 5. Artifact Unlock Notification (Toast)

```
┌─────────────────────────────────────────────────────────────────────┐
│                                       ┌────────────────────────┐    │
│                                       │ ✨ Artifact unlocked!  │    │
│                                       │                        │    │
│                                       │  🏺 Memory Jar         │    │
│                                       │                        │    │
│                                       │  [View in Cavern]      │    │
│                                       └────────────────────────┘    │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Behavior:**
- **Trigger:** When user reaches XP threshold
- **Animation:** Gentle slide-in from right + soft glow
- **Sound:** Optional soft chime (can be disabled in settings)
- **[View in Cavern] button:** Opens Cavern with new artifact highlighted (gentle pulse)
- **Auto-dismiss:** 5 seconds (or user clicks [×])

**Highlight in Cavern (after toast clicked):**
```
      🏺 [Newly unlocked artifact pulses with painted glow]
    ✨ ← Sparkle indicator fades after 10 seconds
```

---

## 6. Pan & Zoom Controls

### Toolbar
```
┌──────────────────────────────────────────────────────────────────┐
│  [🔍+] Zoom In  |  [🔍−] Zoom Out  |  [⟲] Reset View            │
└──────────────────────────────────────────────────────────────────┘
```

### Interaction
- **Click + drag:** Pan camera to explore scene
- **Scroll wheel:** Zoom in/out (reveals texture details)
- **Keyboard arrows:** Pan camera (accessibility)
- **Double-click artifact:** Opens detail modal
- **Reset button:** Returns to default view (centered, 100% zoom)

### Zoom Levels
- **100% (default):** Full room visible
- **150%:** Closer view, see artifact details
- **200%:** Max zoom, see brush strokes and texture work
- **50%:** Zoomed out, see entire cavern evolution

---

## 7. Quest Prompt (Reflection)

```
┌─────────────────────────────────────────────────────────────────────┐
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │ 💭 New Reflection Prompt                                   [×]│  │
│  │ ─────────────────────────────────────────────────────────────  │  │
│  │                                                               │  │
│  │  "Revisit the Past"                                          │  │
│  │                                                               │  │
│  │  Read three entries from last month. What themes do you     │  │
│  │  notice?                                                     │  │
│  │                                                               │  │
│  │  This is optional — no pressure if you're not in the mood.  │  │
│  │  You can always come back to it.                            │  │
│  │                                                               │  │
│  │  Reward: +15 XP                                              │  │
│  │                                                               │  │
│  │  [Start Quest] [Maybe Later] [Don't Show Again]             │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

**Quest Features:**
- **Non-intrusive:** Appears as modal, not forced
- **Encouraging tone:** "Optional", "no pressure"
- **Clear reward:** Shows XP bonus
- **Three options:**
  - **[Start Quest]:** Opens Canvas with quest prompt as ghost placeholder
  - **[Maybe Later]:** Dismisses modal, quest stays in quest log
  - **[Don't Show Again]:** Hides this specific quest forever

---

## 8. Simple Mode (Cavern Hidden)

When user enables Simple Mode (Settings → Appearance → Enable Simple Mode):

```
┌─────────────────────────────────────────────────────────────────────┐
│ [📝 Canvas] [📚 Library] [⚙️ Settings]            [🔍] [Ctrl+K]     │
├─────────────────────────────────────────────────────────────────────┤
│  (Cavern tab hidden from navigation)                                │
│  (XP counter hidden from status bar)                                │
│  (Artifact unlock toasts suppressed)                                │
│  (Quest prompts hidden)                                             │
│                                                                       │
│  Note: XP still accumulates in background. Disable Simple Mode     │
│        anytime to view your progress.                               │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 9. Easter Eggs & Hidden Interactions

### Click Hidden Areas

**Click window:**
```
[Window shows painted weather: rain, snow, sunset, stars]
(Cycles through scenes on each click)
```

**Click bookshelf corner:**
```
[Reveals painted note: "The journaler's journey is never finished."]
```

**Click moss patch on floor:**
```
[Reveals tiny painted mushrooms growing]
```

### Rare Ambient Animations

- **Fireflies:** Slow-moving painted glowing dots (1-2 visible at a time)
- **Cat silhouette:** Crosses window occasionally (1% chance per minute)
- **Page flutter:** Desk papers gently move in "breeze" (subtle)
- **Candle flicker:** Frame-by-frame painted animation (2-3 frames loop)

### Secret Command

Type `konami` in Command Bar (Ctrl+K):
```
┌────────────────────────────────────────────────────────┐
│ 🎮 Developer Mode Unlocked                             │
│                                                        │
│ • View debug console                                   │
│ • Instant unlock all artifacts (test mode)            │
│ • Add custom XP amount                                 │
│                                                        │
│ [Open Dev Console]  [Close]                           │
└────────────────────────────────────────────────────────┘
```

---

## 10. Fullscreen Cavern Mode

```
┌─────────────────────────────────────────────────────────────────────┐
│                      [Press Esc to exit fullscreen]                  │
│                                                                       │
│                                                                       │
│     🏔️⛰️ [Full immersive view of cavern, no UI chrome]              │
│       🌲🏛️🌲 [Larger scene, more detail visible]                      │
│      🪴 📚🕯️🏺📜 [Artifacts at full resolution]                        │
│         ┌──────────────────────────┐                                │
│         │      Wooden desk         │                                │
│         └──────────────────────────┘                                │
│                                                                       │
│                                                                       │
│  [Hover bottom edge to show progress bar]                           │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

**Features:**
- **No navigation bar:** Immersive view of artwork
- **Hover reveals controls:** Progress bar, zoom, exit button appear on hover (bottom/top edges)
- **Click artifacts:** Still opens detail modals
- **Esc key:** Exits fullscreen

---

## 11. Cavern Evolution Comparison

### Tier 1: Humble Study (0-50 XP)
- Small stone chamber
- Single candle, worn journal
- Rough-hewn walls
- Cobblestone floor
- Dusk sky through small window

### Tier 2: Growing Archive (50-200 XP)
- Stone shelves appear
- Wrought iron lanterns
- Potted fern
- Rolled maps on desk
- Warm amber glow

### Tier 3: Scholar's Library (200-800 XP)
- Vaulted ceiling
- Floor-to-ceiling bookshelves
- Decorative tapestry
- Reading nook with cushions
- Brass telescope

### Tier 4: Grand Repository (800-1600 XP)
- Spiral staircase to loft
- Ornate carved furniture
- Mystical artifacts glowing
- Stained glass window
- Fireplace with crackling logs

### Tier 5: Master's Sanctum (1600+ XP)
- Multi-level chamber
- Library ladder
- Cozy alcoves
- Enchanted items with arcane runes
- Observatory window (cosmic vista)
- Hanging plants

**Transition:** Gentle crossfade between tiers (2-second fade)

---

## Accessibility Notes

### Keyboard Navigation
- **Tab:** Cycle through clickable artifacts
- **Enter:** Open artifact detail modal
- **Arrow keys:** Pan camera
- **+/−:** Zoom in/out
- **Esc:** Exit fullscreen or close modal

### Screen Reader
- **Artifact names:** Announced as "Memory Jar artifact, unlocked November 3"
- **Progress bar:** Announced as "80% progress to next unlock, Memory Jar"
- **XP actions:** Announced as "Earned 10 XP for writing entry"

### Reduced Motion
- **No parallax scrolling:** Layers move together (disabled)
- **No artifact pulse:** Static glow instead of animation
- **Toast entrance:** Fade only (no slide)

### Color Blindness
- **Not reliant on color:** Artifacts identifiable by shape/icon
- **XP progress bar:** Uses texture fill, not just color

---

## Performance Targets

- **Cavern render:** <200ms (initial load)
- **Pan/zoom:** 60fps (smooth interaction)
- **Artifact modal open:** <100ms
- **Transition between tiers:** <3s (crossfade + new scene load)

---

## Developer Notes

### Tech Stack
- **2D rendering:** HTML5 Canvas or SVG (layered for parallax)
- **Artwork:** Pre-rendered PNG/WebP images (4K resolution)
- **Interactions:** Click detection on artifact bounding boxes
- **State:** Track unlocked artifacts, XP, current tier in SQLite

### Asset Pipeline
- **Artists:** Paint each tier as separate master image (4K+)
- **Export:** PNG with transparency for layered elements (background, mid-ground, foreground)
- **Optimization:** Compress with WebP for web delivery (60-70% quality)

---

**End of Records Cavern Wireframe**
