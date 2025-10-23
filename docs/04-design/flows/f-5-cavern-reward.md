# UX Flow: F-5 Cavern Reward

**Flow ID:** F-5  
**User Goal:** Experience progress through artifact unlocks without pressure  
**Primary Persona:** Productivity Gamer, Neurospicy Maker (in moderation)

---

## Entry Point
User writes journal entries and accumulates XP over time.

---

## Flow Steps

### Step 1: XP Accumulation (Background)
**Triggers:**
- Write entry (50+ words): +10 XP
- Return after 3+ day gap: +5 XP
- Complete reflection prompt: +15 XP
- Tag 5 entries: +5 XP (one-time per batch)

**System:**
- XP tracked in `settings` table or user profile
- No visible counter during writing (distraction-free)
- Optional: XP badge in toolbar (if user enables in Settings)

**Database:**
```sql
UPDATE settings SET value = value + 10
WHERE key = 'user_xp' AND user_id = 1;
```

---

### Step 2: Threshold Reached
**Trigger:** XP crosses artifact unlock threshold  
**System:**
- Checks unlock conditions (e.g., 50 XP, 100 XP, 200 XP, etc.)
- If threshold met: queues artifact unlock

**Example Thresholds:**
| XP | Artifact |
|----|----------|
| 10 | First Journal (starter artifact) |
| 50 | Memory Jar |
| 100 | Ancient Scroll |
| 200 | Cozy Lantern |
| 400 | Bookshelf |
| 800 | Star Map |

---

### Step 3: Unlock Notification (Gentle)
**Trigger:** XP threshold crossed  
**System:**
- Shows subtle toast notification (bottom-right)
- Does NOT interrupt writing flow
- Can be dismissed or clicked

**Toast:**
```
┌───────────────────────────────────────┐
│ ✨ Artifact unlocked!                 │
│ "Memory Jar"                          │
│ [View in Cavern] [Later]              │
└───────────────────────────────────────┘
```

**Audio (optional, if user enabled):**
- Soft chime (2-3 seconds, low volume)
- Can be disabled: Settings → Sound → ☐ Artifact unlock sound

**User Actions:**
- Click `[View in Cavern]` → Navigates to Records Cavern
- Click `[Later]` or ignore → Toast fades after 10 seconds
- Toast reappears once more (max 2 times) if ignored

---

### Step 4: Navigate to Cavern
**Trigger:** User clicks `[View in Cavern]` OR opens Cavern manually  
**System:**
- Switches to Records Cavern surface
- Highlights new artifact with subtle glow

**Screen (Cavern with new artifact):**
```
┌─────────────────────────────────────────────────────┐
│  Records Cavern                     [Hide] [XP: 250]│
├─────────────────────────────────────────────────────┤
│                                                     │
│         🏔️                                          │
│       🪴  📚  🕯️  ✨🏺✨                             │ ← New: Memory Jar
│      ┌─────────┐                                    │
│      │  Desk   │                                    │
│      └─────────┘                                    │
│                                                     │
│  Progress to next unlock: ████░░░░░░ 40%            │
│  Next: "Ancient Scroll" (50 XP away)                │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Animation (respects reduced motion):**
- **If reduced motion OFF:**
  - Artifact fades in with gentle bounce (500ms)
  - Glow effect pulses 3 times (1s each)
- **If reduced motion ON:**
  - Artifact fades in (no bounce)
  - Static outline highlight (no pulse)

---

### Step 5: Inspect Artifact (Optional)
**Trigger:** User clicks new artifact  
**System:**
- Opens artifact detail modal

**Modal:**
```
┌─────────────────────────────────────────────────────┐
│  Memory Jar                                 [×]     │
├─────────────────────────────────────────────────────┤
│                                                     │
│          🏺                                         │
│                                                     │
│  "A glass jar filled with paper scraps, each one   │
│  holding a fleeting memory. Some are joyful, some  │
│  bittersweet, but all are yours to keep."          │
│                                                     │
│  Unlocked: October 22, 2025                         │
│                                                     │
│  [Close]                                            │
└─────────────────────────────────────────────────────┘
```

**User Actions:**
- Read flavor text (cozy, evocative, non-judgmental)
- Click `[Close]` or press `Esc`

---

### Step 6: Return to Writing (No Pressure)
**Trigger:** User closes modal or Cavern  
**System:**
- Returns to last active surface (usually Journal Canvas)
- No further prompts or interruptions

**Key Principle:** Artifact unlock is a **pleasant discovery**, not a forced celebration. User can ignore entirely if not interested.

---

## Success Criteria
- ✅ Artifact unlocks feel rewarding, not stressful
- ✅ Notifications are non-intrusive (dismissable, non-blocking)
- ✅ User can disable gamification entirely with "Simple Mode"
- ✅ Unlock cadence is tuned to short sessions (not hours of grinding)
- ✅ Animation respects reduced motion preferences

---

## XP Tuning (Post-MVP)

### Goals
- Unlock 1st artifact within **first session** (feels good immediately)
- Unlock 2nd artifact within **3-5 sessions** (builds anticipation)
- Long-term cadence: **1 artifact per 5-10 entries** (not too fast, not too slow)

### Anti-Goals
- ❌ No daily XP caps (punishes enthusiastic users)
- ❌ No XP decay (no pressure to "keep up")
- ❌ No competitive leaderboards (privacy violation + stress)

---

## Edge Cases

### E1: User writes 10 entries in one day (XP burst)
- **System:** Awards XP normally (no cap)
- **Result:** May unlock 2-3 artifacts in one session
- **Notification:** Bundles into single toast: "3 artifacts unlocked!"

### E2: User returns after months-long gap
- **System:** Awards +5 "Welcome back" XP (one-time per gap)
- **No penalty** for being away
- **Microcopy:** "Welcome back! No pressure — pick up where you left off."

### E3: User has Simple Mode enabled
- **System:** XP still accumulates (in background)
- **No toasts or Cavern access**
- **If user later disables Simple Mode:** All artifacts instantly available (no re-earning required)

### E4: User closes toast without viewing
- **System:** Re-shows toast once more (max 2 times total)
- **If still ignored:** Artifact remains unlocked, user can discover in Cavern later

### E5: Reduced motion is enabled
- **Animation:** Simplified (fade-in only, no bounce or glow pulse)
- **Still visually distinct** (outline or border)

---

## Accessibility Notes
- **Keyboard navigation:**
  - `Tab` to focus toast buttons
  - `Enter` to click button
  - `Esc` to dismiss
- **Screen reader:**
  - "Artifact unlocked: Memory Jar. View in Cavern, or dismiss."
  - "Records Cavern. Memory Jar unlocked. A glass jar filled with paper scraps..."
- **Reduced motion:** All animations disabled if OS preference set

---

## Microcopy (See `/docs/07-ux/microcopy-guide.md`)

### Toast
- ✅ "✨ Artifact unlocked!" (celebratory, not gamey)
- ❌ "Level up!" (too gamey)
- ❌ "Achievement unlocked!" (Xbox vibes, not calm)

### Flavor Text (Examples)
- **First Journal:** "A well-worn journal, its pages filled with the echoes of your thoughts."
- **Memory Jar:** "A glass jar filled with paper scraps, each one holding a fleeting memory."
- **Ancient Scroll:** "A scroll from another time, reminding you that reflection is a timeless practice."
- **Cozy Lantern:** "A warm glow to light your way through the cavern of memory."
- **Bookshelf:** "A collection of stories — some finished, some still being written."
- **Star Map:** "A map of constellations you've charted through your own words."

**Tone:** Cozy, poetic, non-judgmental. Never "You did it!" or "Congrats!" (too performative).

---

## Gamification Philosophy

### Safe Gamification = Discovery, Not Achievement
- **Traditional gamification (bad):**
  - "You earned 50 points! Only 950 more to level up!"
  - "Don't lose your streak!"
  - "You're in the top 10%!"
  
- **ChronoXP gamification (good):**
  - "You've unlocked a new artifact. Take a look when you're ready."
  - "No pressure. Your progress is yours alone."
  - "The Cavern evolves with you, at your own pace."

### What We Avoid
- ❌ Streaks (create guilt)
- ❌ Leaderboards (create comparison)
- ❌ Time pressure (create stress)
- ❌ Forced celebrations (create annoyance)

### What We Embrace
- ✅ Gentle milestones (create satisfaction)
- ✅ Personal progress (create motivation)
- ✅ Aesthetic rewards (create delight)
- ✅ Optional engagement (create freedom)

---

## Analytics (Optional, Privacy-Respecting)
If user opts in:
- % of users who view artifacts vs. dismiss toasts
- Average XP per session
- Most/least popular artifacts (for future design)
- % of users who disable gamification (Simple Mode)

**Never logged:** Entry content, specific unlock dates, user identity.

---

## Future Enhancements (Post-MVP)
- **Artifact placement:** User can arrange artifacts in Cavern (drag-and-drop)
- **Seasonal artifacts:** Special unlocks during holidays (opt-in)
- **Custom artifacts:** User uploads own icons/images
- **Cavern themes:** Different visual styles (forest, library, spaceship)
- **Artifact interactions:** Click artifact → triggers related search (e.g., "Memory Jar" → searches entries tagged "memory")
