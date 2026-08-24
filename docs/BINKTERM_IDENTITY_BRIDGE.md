# BinkTerm Identity Bridge

## Purpose

Allows BinkTerm BBS sessions to enter late.sh doors while preserving
the originating BBS identity.

## Flow

BinkTerm
  |
  | BINKTERM_USER_ID
  | BINKTERM_USERNAME
  | BINKTERM_DISPLAY_NAME
  v

late-ssh session bootstrap

  |
  v

bbs_identity_links

  |
  v

late.sh user account

## Design Rules

- BinkTerm identity is resolved through an explicit identity link.
- No automatic fallback occurs when a BinkTerm identity is supplied.
- Late user_id remains the ownership identity.
- External BBS identity remains session context.

## Lateania

Lateania receives the complete BbsIdentity context.

BBS mode only changes terminal presentation:
- ASCII-safe portraits
- ASCII rating indicators
- safer hints

The game state remains shared.

