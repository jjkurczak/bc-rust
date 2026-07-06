//! Helper functions for standardizing serialization and deserialization of stateful objects.

use crate::errors::SerializedStateError;

/// A semantic library version, ordered by `major`, then `minor`, then `patch`.
///
/// The field declaration order matters: the derived [`Ord`]/[`PartialOrd`] compare fields
/// lexicographically in declaration order, which is exactly semantic-version precedence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    // A semantic version can often also take a suffix, e.g. "alpha", "beta", "rc1", etc.
    // We're not going to model that here because it's not useful for versioning serialized states.
}

impl From<[u8; 3]> for SemVer {
    fn from(v: [u8; 3]) -> Self {
        SemVer { major: v[0], minor: v[1], patch: v[2] }
    }
}

impl From<SemVer> for [u8; 3] {
    fn from(v: SemVer) -> Self {
        [v.major, v.minor, v.patch]
    }
}

/// Parse a decimal ASCII string (a Cargo version component) into a u8 at compile time.
const fn parse_version_component(s: &str) -> u8 {
    let bytes = s.as_bytes();
    let mut result: u8 = 0;
    let mut i = 0;
    while i < bytes.len() {
        let d = bytes[i];
        assert!(d >= b'0' && d <= b'9', "version component must be numeric");
        // A component > 255 overflows u8 and fails the build (SemVer fields are u8 by design).
        result = result * 10 + (d - b'0');
        i += 1;
    }
    result
}

/// The current library version, taken from this crate's `Cargo.toml` at compile time (via Cargo's
/// `CARGO_PKG_VERSION_*` env vars) so it can never drift from the published version.
pub const LIB_VERSION: SemVer = SemVer {
    major: parse_version_component(env!("CARGO_PKG_VERSION_MAJOR")),
    minor: parse_version_component(env!("CARGO_PKG_VERSION_MINOR")),
    patch: parse_version_component(env!("CARGO_PKG_VERSION_PATCH")),
};

#[test]
fn test_cmp_lib_ver() {
    use core::cmp::Ordering;

    assert!([0, 0, 0] < [0, 0, 1]);

    let cmp = |a: [u8; 3], b: [u8; 3]| SemVer::from(a).cmp(&SemVer::from(b));
    assert_eq!(cmp([0, 2, 1], [1, 1, 1]), Ordering::Less);
    assert_eq!(cmp([2, 1, 1], [1, 1, 1]), Ordering::Greater);
    assert_eq!(cmp([1, 0, 2], [1, 1, 1]), Ordering::Less);
    assert_eq!(cmp([1, 2, 0], [1, 1, 1]), Ordering::Greater);
    assert_eq!(cmp([1, 1, 0], [1, 1, 1]), Ordering::Less);
    assert_eq!(cmp([1, 1, 2], [1, 1, 1]), Ordering::Greater);
    assert_eq!(cmp([1, 1, 1], [1, 1, 1]), Ordering::Equal);
}

/// Puts the library version into the first three bytes of the state array.
///
/// Hands back a slice to the same array, starting after the version tag.
pub fn add_lib_ver<const SERIALIZED_LEN: usize>(state: &mut [u8; SERIALIZED_LEN]) -> &mut [u8] {
    state[..3].copy_from_slice(&<[u8; 3]>::from(LIB_VERSION));
    &mut state[3..]
}

/// A helper for deserializing an object's state
///
/// The state_out array must have length at least SERIALIZED_LEN - 3.
///
/// Returns the number of bytes written to state_out, or a [SerializedStateError::IncompatibleVersion] if the
/// serialized state contains a version header earlier than the specified `not_before` version.
///
/// Note that for testability, this will always reject if the serialized state contains a version tag
/// of `[0,0,0]`.
///
/// Hands back a slice to the same array, starting after the version tag.
pub fn check_lib_ver<const SERIALIZED_LEN: usize>(
    state: &[u8; SERIALIZED_LEN],
    not_before: Option<[u8; 3]>,
) -> Result<&[u8], SerializedStateError> {
    let ver_bytes: [u8; 3] = state[..3].try_into().unwrap();
    let ver = SemVer::from(ver_bytes);

    let not_before = SemVer::from(not_before.unwrap_or([0, 0, 0]));

    if ver < not_before {
        return Err(SerializedStateError::IncompatibleVersion);
    };
    // Nothing is ever compatible with [0,0,0]
    if ver == SemVer::from([0, 0, 0]) {
        return Err(SerializedStateError::IncompatibleVersion);
    };

    // Also not compatible with future versions.
    if ver > LIB_VERSION {
        return Err(SerializedStateError::IncompatibleVersion);
    }

    Ok(&state[3..])
}
