# Security Policy

## Reporting a vulnerability

If you discover a security issue in OILSHIP, please do **not** open a public GitHub issue.

Instead, reach the team privately:

- DM on X: [@Oilship2026](https://x.com/Oilship2026)
- We will respond within 48 hours and coordinate a private disclosure channel.

## Scope

The following are in scope for responsible disclosure:

- The Anchor program in `programs/oilship/`
- The risk scoring engine in `watch/`
- The SDK in `sdk/` (signer construction, PDA derivation, instruction encoding)
- The Wreck Fund accounting model

Out of scope:

- Issues in third-party dependencies that are already publicly known
- DoS via flood traffic against public RPCs
- Self-XSS in tooling

## Disclosure timeline

1. Report received, acknowledged within 48 hours.
2. Triage within 5 business days.
3. Fix developed and tested.
4. Coordinated disclosure with the reporter.
5. Public advisory after a fix has shipped.

Thank you for helping keep the convoy safe.
