//! The five vector files under `src/vectors/` must be byte-identical to the
//! copies in `forgesworn/signet-contacts-conformance`. Each expected value is
//! the SHA-256 of the whole file as recorded in that repository's
//! `vectors/manifest.json` (plain SHA-256 of the file bytes), read at its
//! commit 95418bb498f21d6b6c40c5217a9f604a6d53d002. That manifest says the
//! vectors come from `@forgesworn/signet-contacts` 0.3.0 at commit
//! 61ad343c5d621cf1bcd14ee3d4d67bb28bf17cd9.

use sha2::{Digest, Sha256};

const EXPECTED: [(&str, &str); 5] = [
    (
        "envelope.v2.json",
        "06bd0ceccabcc00e7e4f7e7e3aecfd8466bbd124667e4b1b72b333b4bcd91dc7",
    ),
    (
        "pairing-code.json",
        "b9ae009f5a931f9d7b1ae1e34cc960c7700e2dc64cef05aa45d46eb779cd3be6",
    ),
    (
        "pairing.v2.json",
        "6ca041213deb6ebef0766e656b92fd3a2cf91bfb6825ca7c6a6373c4553d4460",
    ),
    (
        "projection.v2.json",
        "07007dbd181877287a065268b0d08a6e6d0859c168569eecf4f4a3db58bc6f78",
    ),
    (
        "sanitise.json",
        "c18a913a7cd16820a990ac7eef200aa998071bf67f6e51a3138d628d404f0289",
    ),
];

#[test]
fn vector_files_match_the_conformance_manifest() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/vectors");
    for (name, expected) in EXPECTED {
        let bytes = std::fs::read(dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"));
        let actual = hex::encode(Sha256::digest(&bytes));
        assert_eq!(actual, expected, "{name} differs from the conformance copy");
    }
}
