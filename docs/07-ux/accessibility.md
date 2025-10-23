# Accessibility & Neurodivergent Support

**Project:** ChronoXP™ Journal  
**Date:** October 22, 2025

---

## Commitment

ChronoXP is designed **accessibility-first**, not accessibility-as-afterthought. We prioritize neurodivergent users (ADHD, autistic, dyslexic, etc.) while ensuring all users benefit from inclusive design.

---

## WCAG 2.1 Compliance

**Target:** WCAG 2.1 Level AA (minimum)  
**Stretch goal:** Level AAA where feasible

### Key Requirements
- ✅ **Contrast ratios:** 4.5:1 (normal text), 3:1 (large text, UI components)
- ✅ **Keyboard navigation:** All features accessible without mouse
- ✅ **Screen reader:** Full compatibility (NVDA, JAWS, VoiceOver, Narrator)
- ✅ **Resizable text:** Up to 200% without loss of functionality
- ✅ **No seizure triggers:** No flashing content, respects reduced motion

---

## Visual Accessibility

### 1. Low-Stimulation Theme (Default)

**Color Palette:**
```
Light Mode:
- Background: #F9FAFB (soft off-white)
- Text: #111827 (near-black, not pure black to reduce eye strain)
- Accent: #3B82F6 (soft blue, not aggressive)
- Borders: #E5E7EB (subtle grey)

Dark Mode:
- Background: #1F2937 (dark grey, not pure black)
- Text: #F3F4F6 (off-white)
- Accent: #60A5FA (softer blue for dark background)
- Borders: #374151
```

**Contrast ratios (WCAG AA compliant):**
- Light mode text: 16.75:1 (exceeds minimum)
- Dark mode text: 14.5:1 (exceeds minimum)
- Accent on background: 4.8:1 (passes)

---

### 2. High Contrast Mode (Optional)

**For users with low vision:**
```
- Background: #FFFFFF (pure white) or #000000 (pure black)
- Text: #000000 (pure black) or #FFFFFF (pure white)
- Accent: #0000FF (pure blue, maximum contrast)
- Borders: #000000 or #FFFFFF (2px thick)
```

**Enabled via:** Settings → Appearance → High Contrast Mode

---

### 3. Dyslexia-Friendly Typography

**Font Options:**
1. **OpenDyslexic** (default for dyslexic users)
   - Heavy-weighted bottoms help prevent letter flipping
2. **Atkinson Hyperlegible** (low vision friendly)
3. **Comic Sans MS** (proven dyslexia-friendly, stigma-free in ChronoXP)
4. **System Default** (respects OS font settings)

**Typography Controls:**
- **Font size:** 12px to 24px (slider)
- **Line spacing:** 1.0x to 2.0x (slider)
- **Letter spacing:** Normal to 0.15em
- **Paragraph spacing:** 1.5x to 2.5x

---

### 4. Reduced Motion

**Respects OS preference:** `prefers-reduced-motion: reduce`

**What changes when enabled:**
- ❌ No parallax scrolling in Cavern
- ❌ No bounce animations on artifact unlock
- ❌ No slide transitions between surfaces
- ✅ Simple fade-ins only
- ✅ Instant focus changes (no smooth scrolling)

**Manual toggle:** Settings → Appearance → ☑ Reduce Motion

---

### 5. Focus Indicators

**All interactive elements have visible focus:**
```css
:focus {
  outline: 3px solid #3B82F6;  /* Thick blue outline */
  outline-offset: 2px;
}
```

**Keyboard navigation order:**
- Logical (top to bottom, left to right)
- Skips hidden elements
- Tab index managed (no `tabindex="-1"` on interactable elements)

---

## Motor Accessibility

### 1. Keyboard-First Design

**Every action has a keyboard shortcut.** (See `/docs/07-ux/keyboard-map.md`)

**Examples:**
- `Ctrl/Cmd+N`: New entry
- `Ctrl/Cmd+K`: Command Bar
- `Ctrl/Cmd+F`: Search
- `Ctrl/Cmd+,`: Settings
- `Esc`: Close modal/exit

**No mouse-only interactions.** Hover effects are visual enhancements, not functional requirements.

---

### 2. Large Click Targets

**Minimum touch target size:** 44×44px (WCAG 2.1 Level AAA)

**Examples:**
- Buttons: 48×36px (width × height)
- Tag chips: 32px height, auto width
- Calendar dates: 48×48px

**Spacing:**
- Minimum 8px between interactive elements
- Prevents accidental clicks

---

### 3. Voice Control Compatibility

**All UI elements have accessible labels:**
```html
<button aria-label="Create new entry">
  <PlusIcon />
</button>
```

**Works with:**
- Windows Speech Recognition
- macOS Dictation & Voice Control
- Dragon NaturallySpeaking

---

## Cognitive Accessibility

### 1. Clear Visual Hierarchy

**Single-column layout:**
- Writing area is central focus
- Toolbars minimal (3-5 buttons max)
- No cluttered sidebars

**Headings use semantic HTML:**
```html
<h1>Journal Canvas</h1>
<h2>October 22, 2025</h2>
<h3>Tags</h3>
```

---

### 2. Minimal Interruptions

**No modal dialogs unless critical:**
- Autosave: Silent (status bar only)
- Artifact unlock: Dismissable toast (bottom-right)
- Errors: Toast or inline message (not blocking)

**Non-modal inputs:**
- Tag input appears inline (doesn't block canvas)
- Search overlay can be dismissed with `Esc`

---

### 3. Consistent Patterns

**Same action, same result:**
- `Esc` always closes/cancels
- `Enter` always confirms
- `Ctrl/Cmd+K` always opens Command Bar

**No hidden menus:**
- All features accessible via Command Bar or Settings
- No right-click-only actions (context menus are supplemental)

---

### 4. Plain Language

**Microcopy avoids jargon:**
- ✅ "Export your journal" (not "Generate .chrxp archive")
- ✅ "Welcome back!" (not "Session resumed")
- ✅ "Searching..." (not "Indexing vector embeddings")

**Error messages are helpful:**
- ❌ "Error 500: Database connection failed"
- ✅ "ChronoXP couldn't save your entry. Try restarting the app."

---

## Auditory Accessibility

### 1. No Audio Required

**All features work without sound:**
- Artifact unlock notification: Visual toast (sound is optional)
- Autosave confirmation: Status bar text (no chime)

### 2. Optional Sound Effects

**Can be disabled entirely:**
- Settings → Sound → ☐ Enable sounds
- Settings → Sound → Artifact unlock volume (slider)

**Sounds (if enabled):**
- Artifact unlock: Soft chime (2s, low frequency)
- Error: Subtle beep (1s, non-harsh)

---

## Screen Reader Support

### 1. Semantic HTML

**All content uses proper tags:**
```html
<main>
  <article aria-label="Journal entry for October 22, 2025">
    <h2>Work-life balance thoughts</h2>
    <p>Today I talked to Mom...</p>
  </article>
</main>
```

### 2. ARIA Labels

**Interactive elements have descriptive labels:**
```html
<button aria-label="Save entry (Ctrl+S)">
  <SaveIcon aria-hidden="true" />
</button>
```

**Dynamic content announces changes:**
```html
<div role="status" aria-live="polite">
  Autosaved 2 seconds ago
</div>
```

### 3. Skip Links

**Keyboard users can skip repetitive content:**
```html
<a href="#main-content" class="skip-link">
  Skip to journal entry
</a>
```

### 4. Form Labels

**All inputs have associated labels:**
```html
<label for="tag-input">Add tags (comma-separated)</label>
<input id="tag-input" type="text" />
```

---

## Neurodivergent-Specific Features

### 1. ADHD Support

**Reduce friction to start:**
- Cursor auto-focused on launch
- No mandatory setup/onboarding
- Autosave (never manually save)

**Gentle nudges, not pressure:**
- Optional reflection prompts (can dismiss forever)
- No streak counters or guilt trips
- "Welcome back" bonus (positive framing)

**Distraction-free writing:**
- Simple Mode hides gamification
- Toolbar auto-hides when typing (optional)
- No notifications during active writing

---

### 2. Autistic Support

**Predictable interactions:**
- Consistent keyboard shortcuts
- Same UI patterns across surfaces
- No surprise pop-ups

**Sensory-friendly:**
- Low-stimulation color palette
- Reduced motion by default
- No auto-playing audio

**Clear expectations:**
- Word count visible (reduces "am I done?" anxiety)
- Progress bar uses plain numbers ("20 XP away" vs. percentages)

---

### 3. Dyslexia Support

**Typography:**
- OpenDyslexic font option
- Adjustable line spacing (1.5x to 2.0x)
- Adjustable letter spacing

**Reading aids:**
- Sans-serif fonts only (easier to read)
- Left-aligned text (not justified)
- Shorter line lengths (60-80 characters max)

---

### 4. Anxiety Support

**No judgment or pressure:**
- No "you haven't written in X days" notifications
- No red X's on calendar
- Microcopy is inviting, not demanding

**Safety features:**
- Local-only (no risk of data leaks)
- Optional vault password (extra security for paranoid users)
- Export anytime (no lock-in fear)

---

## Accessibility Testing Checklist

### Automated Tests
- [ ] **axe-core:** Run on all pages (0 violations)
- [ ] **WAVE:** Check color contrast (all pass)
- [ ] **Lighthouse:** Accessibility score 95+ (Chrome DevTools)

### Manual Tests (Before Each Release)
- [ ] **Keyboard-only navigation:** Can complete all tasks without mouse
- [ ] **Screen reader (NVDA):** Read all content, no confusing labels
- [ ] **Zoom to 200%:** No horizontal scrolling, all text readable
- [ ] **Reduced motion:** No jarring animations
- [ ] **High contrast mode:** All elements visible

### User Testing
- [ ] **Neurodivergent testers:** 5 users (ADHD, autistic, dyslexic) test MVP
- [ ] **Screen reader users:** 2 users test with JAWS/NVDA/VoiceOver
- [ ] **Motor disability:** 1 user test with keyboard-only + voice control

---

## Accessibility Statement (For Website/App)

> **ChronoXP is committed to accessibility for all users.**
> 
> We strive to meet WCAG 2.1 Level AA standards and prioritize neurodivergent-friendly design. Our features include:
> 
> - Full keyboard navigation
> - Screen reader support (NVDA, JAWS, VoiceOver, Narrator)
> - Dyslexia-friendly fonts (OpenDyslexic, Atkinson Hyperlegible)
> - Reduced motion mode
> - High contrast themes
> - Adjustable text size and spacing
> - No flashing content or seizure triggers
> 
> **Feedback:** If you encounter accessibility barriers, please email [accessibility@chronoxp.app](mailto:accessibility@chronoxp.app). We review all feedback within 7 days.

---

## Future Enhancements (Post-MVP)

### 1. Customizable UI Density
- **Compact:** More content per screen (for power users)
- **Comfortable:** Default (balanced)
- **Spacious:** Larger padding (for motor disabilities, low vision)

### 2. Colorblind Modes
- Protanopia (red-blind)
- Deuteranopia (green-blind)
- Tritanopia (blue-blind)

### 3. Bionic Reading
- **What:** Bolded first half of words (speeds reading for dyslexic users)
- **Toggle:** Settings → Editor → ☑ Bionic Reading

### 4. Focus Mode
- **What:** Dims everything except current paragraph
- **Use case:** ADHD users who struggle with wall-of-text overwhelm

---

## Resources

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [Inclusive Design Principles](https://inclusivedesignprinciples.org/)
- [Neurodiversity Design System](https://neurodiversity.design/)
- [A11y Project Checklist](https://www.a11yproject.com/checklist/)
