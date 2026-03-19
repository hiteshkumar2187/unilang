# UniLang — Project Governance

**Version:** 1.0.0-draft
**Last Updated:** 2026-03-19

---

## Overview

UniLang follows the Apache Software Foundation's governance model, adapted for our project's current stage. This document defines roles, responsibilities, and decision-making processes.

---

## Roles

### 1. User
Anyone who uses UniLang. Users contribute by filing bug reports, requesting features, and providing feedback.

### 2. Contributor
Anyone who has contributed code, documentation, tests, or other artifacts to the project via accepted pull requests. Contributors have no formal decision-making power but are valued members of the community.

### 3. Committer
Contributors who have earned write access to the repository through sustained, high-quality contributions. Committers can:
- Merge pull requests (with required approvals)
- Triage issues
- Participate in release votes

**Path to Committer:** Nomination by an existing committer after 6+ months of active contribution, followed by a lazy consensus vote (72-hour window, no vetoes).

### 4. PMC Member (Project Management Committee)
Committers who take on governance responsibilities. PMC members can:
- Vote on releases
- Vote on new committers and PMC members
- Make binding decisions on project direction
- Represent the project to the Apache Foundation

**Path to PMC:** Nomination by an existing PMC member, followed by a majority vote of the PMC.

### 5. PMC Chair
A PMC member elected to serve as the project's liaison to the Apache Foundation Board. Rotates annually.

---

## Decision-Making

### Lazy Consensus
Most decisions (bug fixes, minor features, documentation) use **lazy consensus**: a proposal is made, and if no one objects within 72 hours, it is accepted.

### Voting
Major decisions require explicit votes:

| Decision Type | Voting Body | Threshold |
|---------------|-------------|-----------|
| Release | PMC | Majority (3+ binding +1s, no vetoes) |
| New Committer | PMC | Lazy consensus (72h, no vetoes) |
| New PMC Member | PMC | Majority vote |
| Architecture change (RFC) | Committers + PMC | 3+ approvals from committers |
| License/governance change | PMC | Unanimous |

### Vote Types

| Vote | Meaning |
|------|---------|
| **+1** | Approve (binding if from PMC member) |
| **+0** | No strong opinion |
| **-0** | Minor concerns but not blocking |
| **-1** | Veto (must include technical justification) |

---

## Release Process

1. Release manager (a committer) prepares the release candidate
2. All tests must pass on all supported platforms
3. Release candidate is posted for community review (72 hours minimum)
4. PMC votes (minimum 3 binding +1 votes, no vetoes)
5. Release is published and announced

### Versioning

UniLang uses [Semantic Versioning](https://semver.org/):
- **Major (X.0.0):** Breaking language changes
- **Minor (0.X.0):** New features, backward-compatible
- **Patch (0.0.X):** Bug fixes, backward-compatible

---

## Communication Channels

| Channel | Purpose | Visibility |
|---------|---------|------------|
| GitHub Issues | Bug reports, feature requests | Public |
| GitHub Discussions | Questions, RFCs, proposals | Public |
| dev@ mailing list (planned) | Technical discussion, votes | Public (archived) |
| private@ mailing list (planned) | Security issues, conduct reports | PMC only |

**All technical decisions must happen on public channels.** Private channels are reserved for security vulnerabilities and conduct issues only.

---

## Apache Foundation Alignment

UniLang is designed from the ground up to meet Apache incubation requirements:

- [x] Apache License 2.0
- [x] NOTICE file
- [x] Contributor License Agreement (CLA) requirement
- [x] Public decision-making
- [x] Vendor-neutral governance
- [x] Release process documentation
- [ ] Incubation proposal (planned)
- [ ] Mentor assignment (planned)

---

*This governance model will evolve as the project grows. Changes to this document require PMC unanimous approval.*
