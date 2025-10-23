# ChronoXP™ Journal — Documentation Index

**Project Codename:** ChronoXP™ Journal  
**Tagline:** *A private, neuro-affirming, offline-first journaling companion with gentle gamification.*  
**Last Updated:** October 22, 2025

---

## 📖 Quick Start

**New to ChronoXP?** Start here:
1. **[Vision](01-vision/vision.md)** — Why ChronoXP exists and what makes it different
2. **[Jobs To Be Done](01-vision/jtbd.md)** — The core problems we solve
3. **[Personas](02-research/personas.md)** — Who we're building for

**Ready to build?**
1. **[System Architecture](05-architecture/overview.md)** — Technical overview
2. **[Data Model](05-architecture/data-model.md)** — Database schema
3. **[Acceptance Criteria](10-delivery/acceptance.md)** — MVP requirements

---

## 📂 Documentation Structure

### 01. Vision & Strategy
- **[Vision](01-vision/vision.md)** — Core mission, non-negotiables, design principles
- **[Jobs To Be Done (JTBD)](01-vision/jtbd.md)** — User needs, pain points, success criteria

### 02. Research & Discovery
- **[Personas](02-research/personas.md)** — Target users (Reflective, Neurospicy, Gamer)
- [Research Synthesis](research/research-synthesis.md) *(existing)* — User research findings

### 03. Product Strategy
- **[Product Pillars](03-product/pillars.md)** — Four core pillars + feature mapping
- **[Monetization](08-product/monetization.md)** — Pricing, revenue model, ethical guardrails
- [Metrics (Beta)](product/metrics-beta.md) *(existing)* — Success metrics

### 04. Design & UX
- **[Information Architecture](04-design/ia.md)** — Surfaces, navigation, global elements
- **UX Flows:**
  - **[F-1: Start Writing](04-design/flows/f-1-start-writing.md)** — First session experience
  - **[F-2: Save & Tag](04-design/flows/f-2-save-tag.md)** — Autosave, tagging, Archivist
  - **[F-3: Recall by Meaning](04-design/flows/f-3-recall-meaning.md)** — Semantic search
  - **[F-4: Export Packet](04-design/flows/f-4-export-packet.md)** — .chrxp export/import
  - **[F-5: Cavern Reward](04-design/flows/f-5-cavern-reward.md)** — Artifact unlocks
- [Design System](ux/design-system.md) *(existing)* — Colors, typography, components
- [Wireframes](ux/wireframes/screen-specs.md) *(existing)* — Visual mockups

### 05. Technical Architecture
- **[System Overview](05-architecture/overview.md)** — Tech stack, data flow, deployment
- **[Data Model](05-architecture/data-model.md)** — SQLCipher schema, queries, migrations

### 06. Gamification
- **[Gamification Design](06-gamification/design.md)** — XP system, artifacts, Records Cavern, ethical considerations

### 07. UX & Accessibility
- **[Accessibility](07-ux/accessibility.md)** — WCAG compliance, neurodivergent support, a11y features
- **[Keyboard Map](07-ux/keyboard-map.md)** — Complete keyboard shortcut reference
- **[Microcopy Guide](07-ux/microcopy-guide.md)** — Voice, tone, error messages, button labels
- [A11y/ND Checklist](ux/a11y-nd-checklist.md) *(existing)* — Pre-launch accessibility audit

### 08. Product (Additional)
- **[Monetization](08-product/monetization.md)** — Pricing strategy, revenue projections
- [Feature Map](product/feature-map.md) *(existing)* — Roadmap, prioritization

### 09. Security & Privacy
- **[Security & Privacy](09-security/privacy.md)** — Threat model, encryption, GDPR/CCPA compliance
- [Security & Privacy (existing)](tech/security-privacy.md) — Technical security notes

### 10. Delivery & Testing
- **[Acceptance Criteria](10-delivery/acceptance.md)** — MVP requirements, testing checklist, release criteria

---

## 🎯 By Role

### Product Manager / Designer
1. [Vision](01-vision/vision.md)
2. [Personas](02-research/personas.md)
3. [Product Pillars](03-product/pillars.md)
4. [Information Architecture](04-design/ia.md)
5. [UX Flows](04-design/flows/)
6. [Microcopy Guide](07-ux/microcopy-guide.md)

### Engineer / Developer
1. [System Architecture](05-architecture/overview.md)
2. [Data Model](05-architecture/data-model.md)
3. [Keyboard Map](07-ux/keyboard-map.md)
4. [Acceptance Criteria](10-delivery/acceptance.md)
5. [Security & Privacy](09-security/privacy.md)

### Accessibility Specialist
1. [Accessibility](07-ux/accessibility.md)
2. [Personas (Neurospicy focus)](02-research/personas.md)
3. [Gamification Design](06-gamification/design.md) (ethical considerations)
4. [A11y/ND Checklist](ux/a11y-nd-checklist.md)

### Business / Marketing
1. [Vision](01-vision/vision.md)
2. [Jobs To Be Done](01-vision/jtbd.md)
3. [Monetization](08-product/monetization.md)
4. [Metrics (Beta)](product/metrics-beta.md)

---

## 🔍 By Topic

### Privacy & Security
- [Vision (Non-negotiables)](01-vision/vision.md)
- [Security & Privacy](09-security/privacy.md)
- [Data Model (Encryption)](05-architecture/data-model.md)

### AI Features
- [Product Pillars (Offline AI Concierge)](03-product/pillars.md)
- [System Architecture (llama.cpp integration)](05-architecture/overview.md)
- [Gamification (AI ethics)](06-gamification/design.md)

### Neurodivergent-Friendly Design
- [Personas (Neurospicy Maker)](02-research/personas.md)
- [Accessibility](07-ux/accessibility.md)
- [Microcopy Guide](07-ux/microcopy-guide.md)
- [Gamification Design (No streak shaming)](06-gamification/design.md)

### Gamification
- [Gamification Design](06-gamification/design.md)
- [UX Flow: Cavern Reward](04-design/flows/f-5-cavern-reward.md)
- [Product Pillars (Safe Gamification)](03-product/pillars.md)

---

## 📋 Quick Reference

| Need | Document |
|------|----------|
| **Why does ChronoXP exist?** | [Vision](01-vision/vision.md) |
| **Who are we building for?** | [Personas](02-research/personas.md) |
| **What features ship in MVP?** | [Acceptance Criteria](10-delivery/acceptance.md) |
| **How does semantic search work?** | [System Architecture](05-architecture/overview.md), [UX Flow: Recall](04-design/flows/f-3-recall-meaning.md) |
| **How do we make money?** | [Monetization](08-product/monetization.md) |
| **Is ChronoXP accessible?** | [Accessibility](07-ux/accessibility.md) |
| **What's the database schema?** | [Data Model](05-architecture/data-model.md) |
| **How do keyboard shortcuts work?** | [Keyboard Map](07-ux/keyboard-map.md) |
| **How do we write microcopy?** | [Microcopy Guide](07-ux/microcopy-guide.md) |
| **What's in the .chrxp export?** | [UX Flow: Export](04-design/flows/f-4-export-packet.md) |

---

## 🛠️ Contributing

**Found a gap in documentation?**
1. Open an issue on GitHub
2. Submit a pull request
3. Email: docs@chronoxp.app

**Documentation standards:**
- Use Markdown (.md)
- Keep line length < 120 characters (readability)
- Include date + version at top of each doc
- Link to related docs (avoid duplication)

---

## 📜 License

Documentation is licensed under **CC BY 4.0** (Creative Commons Attribution).  
Code (when repo is public) will be **MIT** or **GPL** (TBD).

---

## 📧 Contact

- **General:** hello@chronoxp.app
- **Security:** security@chronoxp.app
- **Privacy:** privacy@chronoxp.app
- **Accessibility:** accessibility@chronoxp.app
- **Support:** support@chronoxp.app

---

**Last updated:** October 22, 2025  
**Docs version:** 0.1.0 (Pre-MVP)
