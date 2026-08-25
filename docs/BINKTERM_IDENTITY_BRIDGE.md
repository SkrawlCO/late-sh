# BinkTerm Identity Bridge

## Purpose

Allows BinkTerm BBS sessions to enter late.sh door experiences while preserving
the originating BBS identity without making doors aware of BinkTermPHP, SSH, or
the transport that created the session.

The bridge separates:

- the authoritative late.sh account identity used for ownership and persistence
- the external BBS identity that describes where the session originated

## Architecture

BinkTermPHP
    |
    | BINKTERM_USER_ID
    | BINKTERM_USERNAME
    | BINKTERM_DISPLAY_NAME
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
authoritative late.sh user UUID
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

## Identity Resolution

When BinkTerm identity is supplied, late-ssh constructs the external BBS
identity during session bootstrap and resolves it through `bbs_identity_links`.

The resulting late.sh user UUID remains the authoritative identity used by
doors for ownership and persistence.

There is no automatic native-identity fallback when a BinkTerm identity is
supplied. A BBS identity must resolve through the explicit link.

## SessionOrigin

`SessionOrigin` records session provenance.

Current origins:

- `Native`
- `Bbs(BbsIdentity)`

Transport-specific identity remains behind this layer. Doors do not inspect SSH
state or BinkTerm environment variables.

## DoorContext

`DoorContext` is the boundary between session/bootstrap infrastructure and door
implementations.

It owns the authoritative late.sh user identity, player-facing name, and
session origin. Door code receives identity through this context rather than
through transport-specific session fields.

Session-mode queries remain separate from identity:

- `bbs_mode()`
- `native_mode()`

This is intentional. Session mode can affect presentation or terminal behavior
without changing account ownership.

## DoorIdentity

Doors consume identity through:

`context.identity()`

The transport-neutral `DoorIdentity` view contains:

- `user_id`
- `display_name`
- `provider`
- `external_identity`

Semantics:

- `user_id` is the authoritative late.sh UUID used for ownership and persistence.
- `display_name` is the player-facing name for the current door session.
- `provider` identifies identity provenance without exposing transport details.
- `external_identity` is the stable upstream identity when one exists.

Native late.sh sessions have no external identity.

## Design Invariants

1. Doors do not know about SSH.
2. Doors do not know about BinkTermPHP.
3. Identity transport stays inside late-ssh session/bootstrap infrastructure.
4. `DoorContext` remains the boundary presented to doors.
5. `DoorIdentity` is the unified door-facing identity API.
6. The late.sh UUID is authoritative for ownership and persistence.
7. External identity describes provenance; it is not ownership identity.
8. Session mode and identity are separate concepts.
9. Native and BBS sessions linked to the same late.sh account share the same
   underlying persistent game state.

## Lateania

Lateania consumes the same `DoorIdentity` API as the other migrated doors.

BBS mode may change terminal presentation, including ASCII-safe rendering and
hints, but it does not create a separate Lateania identity or game state.

The player remains the same authoritative late.sh account whether the session
entered natively or through BinkTerm.

## Validation Checkpoint

Validated restore point:

- Branch: `door-identity-api`
- Commit: `5cdec895` — `Complete unified DoorIdentity API`
- Tag: `door-identity-v1`
- Validation command: `make check`
- Result: 3298 tests passed, 0 skipped

Earlier restore points:

- `identity-bridge-complete-v1`
- `door-context-v2`
- `lateania-identity-v1`

`door-identity-v1` is the authoritative restore point for the completed unified
door identity architecture.
