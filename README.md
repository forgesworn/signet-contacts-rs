# signet-contacts

The contacts-v2 wire format for Signet, in Rust: building a pairing request,
reading the ack, opening the vault envelope and parsing the projection. It is
a port of the TypeScript reference, [`forgesworn/signet-contacts`](https://github.com/forgesworn/signet-contacts).
The wire specification is `docs/WIRE.md` in that repository (v2, frozen). Where
this crate and `docs/WIRE.md` disagree, `docs/WIRE.md` wins and this crate has
the bug.

There is no relay I/O and no storage here. The crate takes strings and events
in and gives parsed values out.

## What it covers

| Module | Contents |
|---|---|
| `constants` | Event kinds, freshness and staleness limits, size caps, the `Capability` enum, `normalise_capabilities`, `clamp_staleness` |
| `pairing` | `build_pairing_uri`, `web_carrier`, `Directory`, `PairingError`, relay URL and challenge checks |
| `ack` | `parse_ack`, `decrypt_ack_event`, `ack_event_is_fresh`, the `Ack` type |
| `code` | `pairing_code`, `format_pairing_code` (the six-digit verification code) |
| `envelope` | `open_vault_envelope`, `open_vault_envelope_text`, the size buckets |
| `projection` | `parse_projection` and its types (`Projection`, `ProjectedContact`, `Tier`, `Identity`, and so on), `frontier_newer`, `is_newer` |
| `sanitise` | `sanitize_wire_text`, the one text sanitiser on this wire |
| `tags` | `projection_tag`, `proposal_tag`, `ack_tag`, `scoped_contact_id` |

The most-used items are also re-exported at the crate root.

## What it does not cover

This is the consumer half only. It has no pairing-request parser, no
projection builder, and nothing for proposals, contact exchange, contact
invites, contact cards, state, app invites or channel checks. The reference
has all of these.

## Vectors

`src/vectors/` holds five vector files, copied byte for byte from the
reference at upstream commit `fbb13cd`:

- `pairing.v2.json`
- `pairing-code.json`
- `envelope.v2.json`
- `projection.v2.json`
- `sanitise.json`

They are byte-identical to the files of the same name in
[`forgesworn/signet-contacts-conformance`](https://github.com/forgesworn/signet-contacts-conformance),
and `tests/vectors_match_conformance.rs` checks each file's SHA-256 against the
value in that repository's `vectors/manifest.json`. Do not edit them.

Not yet exercised here:

- Vector files: `proposal.v1.json`, `contact-invite-v1.json`,
  `channel-check-v1.json`, `contact-card-v1.json`, `contact-card-blur-v1.json`.
- Case files under `vectors/cases/`: all of them. There is no case runner.
  Five of them match what this crate implements and could be run:
  `ack.json`, `projection-parse.json`, `envelope-parse.json`,
  `relay-url.json` and the projection part of `upstream-malformed.json`. The
  others (`app-invite`, `contact-exchange`, `invite`, `nostr`, `pairing-uri`,
  `projection-build`, `proposal-parse`, `state`, `stringify`) need the
  producer half or the other parts of the reference.

## Tests

```
cargo test
```

The unit tests in `src/tests.rs` assert every case in the five vector files.
`tests/vectors_match_conformance.rs` checks the vector hashes.
`tests/no_game_names.rs` checks that library code names no application.

The crate uses Rust edition 2024.

## Licence

MIT. See `LICENSE`. `NOTICE` records where the code and the vectors came from.
