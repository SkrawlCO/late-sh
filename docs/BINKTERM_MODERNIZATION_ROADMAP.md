# BinkTerm Modernization Roadmap

## Current Status: Identity Foundation Complete

BinkTerm can now act as a gateway into modern late.sh door experiences while
preserving a clean boundary between external BBS identity, late.sh account
ownership, session transport, and game code.

The identity foundation is complete and validated.

## Completed: BinkTerm Identity Bridge

Implemented:

- BinkTerm identity received during late-ssh session bootstrap
- `BINKTERM_USER_ID`, `BINKTERM_USERNAME`, and `BINKTERM_DISPLAY_NAME`
  identity flow
- explicit identity linking through `bbs_identity_links`
- authoritative ownership remains tied to the late.sh user UUID
- no automatic native fallback when BinkTerm identity is supplied

## Completed: SessionOrigin

`SessionOrigin` represents session provenance without exposing SSH details to
door code.

Current values:

- `Native`
- `Bbs(BbsIdentity)`

This keeps transport/bootstrap concerns outside door implementations.

## Completed: DoorContext Boundary

`DoorContext` is the boundary between late-ssh session infrastructure and door
implementations.

Doors receive their session context through this abstraction rather than
directly consuming SSH or BinkTerm-specific state.

Session-mode behavior remains available through:

- `bbs_mode()`
- `native_mode()`

## Completed: Unified DoorIdentity API

Door identity is exposed through:

`context.identity()`

The returned `DoorIdentity` provides:

- `user_id`
- `display_name`
- `provider`
- `external_identity`

All migrated door consumers use this unified API. The older individual
DoorContext identity helpers were removed after migration.

Architecture:

BinkTermPHP
    |
    v
late-ssh session bootstrap
    |
    v
BbsIdentity
    |
    v
bbs_identity_links
    |
    v
late.sh account
    |
    v
SessionOrigin
    |
    v
DoorContext
    |
    v
DoorIdentity
    |
    v
door experiences

## Identity Architecture Principles

1. BBS identity provides provenance and session context, not ownership.
2. The late.sh UUID remains authoritative for authorization, ownership, and
   persistence.
3. Doors do not depend on SSH implementation.
4. Doors do not depend on BinkTermPHP implementation.
5. Identity transport remains inside late-ssh.
6. `DoorContext` remains the door boundary.
7. `DoorIdentity` is the unified door-facing identity API.
8. Session mode remains separate from identity so terminal presentation can
   adapt without creating separate game identities or persistence.

## Validation

Identity foundation restore point:

- Branch: `door-identity-api`
- Commit: `5cdec895` — `Complete unified DoorIdentity API`
- Tag: `door-identity-v1`
- Full validation: `make check`
- Result: 3298 tests passed, 0 skipped

This is the current authoritative restore point for identity architecture.

## Next Planning Phase

With identity transport and the door-facing identity boundary complete, the
next modernization milestone should be selected deliberately from the broader
BinkTerm/L33Test platform roadmap rather than by extending transport-specific
logic into doors.

Future work should preserve the completed identity invariants above.

## Future Direction

BinkTerm Modern aims to support an integrated BBS ecosystem including:

- classic BBS access
- modern terminal experiences
- persistent multiplayer worlds
- integrated door games
- social and community features
- shared identity across compatible experiences
- infrastructure that can eventually be packaged for use by other BBS sysops

## Project Direction

L33Test is the primary development and validation environment.

The immediate priority is to build the ecosystem fully and correctly for
L33Test before optimizing it for distribution. Broader packaging, installation,
and no-programming-required administration should follow a mature, proven
implementation rather than drive premature abstraction.

While building for L33Test first, architectural choices should avoid
unnecessary barriers to eventual reuse by other BBS systems and sysops.

A primary long-term goal is to promote BBS culture and share tools that enrich
the wider BBS community.
