# BinkTerm Modernization Roadmap

## Current Milestone: Identity Foundation

Status: In progress / Phase 1 complete

The first modernization goal is allowing BinkTerm to act as a gateway into
modern late.sh experiences while preserving clear identity boundaries.

## Completed

### BinkTerm Identity Bridge

Completed:
- BinkTerm identity received during SSH bootstrap
- Identity linking through bbs_identity_links
- Lateania receives BinkTerm player identity context
- Ownership remains tied to late.sh user identity

Architecture:

BinkTerm
    |
    v
SSH Bootstrap
    |
    v
SessionOrigin
    |
    v
Door Experiences


## Session Origin Abstraction

Completed:
- SessionOrigin abstraction added
- Doors can determine identity provenance without knowing SSH details

Current values:

- Native late.sh session
- BinkTerm session


## Next Phase

### DoorContext Abstraction

Goal:

Provide every door with a consistent session context.

Future structure:

DoorContext
    |
    +-- user identity
    +-- display identity
    +-- session origin
    +-- terminal capabilities


## Design Principles

1. BBS identity provides context, not ownership.
2. late.sh identity remains authoritative for authorization.
3. Doors should not depend on transport implementation.
4. Terminal capabilities should be detected and respected.


## Future Direction

BinkTerm Modern aims to provide a unified platform for:

- classic BBS access
- modern terminal experiences
- persistent multiplayer worlds
- integrated doors
- social/community features
