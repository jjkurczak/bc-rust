//! Helper functions for standardizing serialization and deserialization of stateful objects.

use crate::errors::CoreError;

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

/// The current library version.
// There is almost certainly a more elegant way to do this.
pub const LIB_VERSION: SemVer = SemVer { major: 0, minor: 1, patch: 2 };

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
/// Returns the number of bytes written to state_out, or a [CoreError::IncompatibleVersion] if the
/// serialized state contains a version header earlier than the specified `not_before` version.
///
/// Note that for testability, this will always reject if the serialized state contains a version tag
/// of `[0,0,0]`.
///
/// Hands back a slice to the same array, starting after the version tag.
pub fn check_lib_ver<const SERIALIZED_LEN: usize>(
    state: &[u8; SERIALIZED_LEN],
    not_before: Option<[u8; 3]>,
) -> Result<&[u8], CoreError> {
    let ver_bytes: [u8; 3] = state[..3].try_into().unwrap();
    let ver = SemVer::from(ver_bytes);

    let not_before = SemVer::from(not_before.unwrap_or([0, 0, 0]));

    if ver < not_before {
        return Err(CoreError::IncompatibleVersion);
    };
    // Nothing is ever compatible with [0,0,0]
    if ver == SemVer::from([0, 0, 0]) {
        return Err(CoreError::IncompatibleVersion);
    };

    // Also not compatible with future versions.
    if ver > LIB_VERSION {
        return Err(CoreError::IncompatibleVersion);
    }

    Ok(&state[3..])
}
