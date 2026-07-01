//! Helper functions for standardizing serialization and deserialization of stateful objects.

use crate::errors::CoreError;

/// The current library version.
// There is almost certainly a more elegant way to do this.
pub const LIB_VERSION: [u8; 3] = [0, 1, 2];

/// Compare two library semantic versions in the standard C format:
/// * if a < b => -1
///* if a == b => 0
///* if a > b => 1
pub fn cmp_lib_ver(a: &[u8; 3], b: &[u8; 3]) -> i8 {
    if a[0] < b[0] {
        -1
    } else if a[0] > b[0] {
        1
    }
    // first component is equal
    else if a[1] < b[1] {
        -1
    } else if a[1] > b[1] {
        1
    }
    // first two components are equal
    else if a[2] < b[2] {
        -1
    } else if a[2] > b[2] {
        1
    }
    // all three components are equal
    else {
        0
    }
}

#[test]
fn test_cmp_lib_ver() {
    assert_eq!(cmp_lib_ver(&[0, 2, 1], &[1, 1, 1]), -1);
    assert_eq!(cmp_lib_ver(&[2, 1, 1], &[1, 1, 1]), 1);
    assert_eq!(cmp_lib_ver(&[1, 0, 2], &[1, 1, 1]), -1);
    assert_eq!(cmp_lib_ver(&[1, 2, 0], &[1, 1, 1]), 1);
    assert_eq!(cmp_lib_ver(&[1, 1, 0], &[1, 1, 1]), -1);
    assert_eq!(cmp_lib_ver(&[1, 1, 2], &[1, 1, 1]), 1);
    assert_eq!(cmp_lib_ver(&[1, 1, 1], &[1, 1, 1]), 0);
}

/// Puts the library version into the first three bytes of the state array.
///
/// Hands back a slice to the same array, starting after the version tag.
pub fn add_lib_ver<const SERIALIZED_LEN: usize>(state: &mut [u8; SERIALIZED_LEN]) -> &mut [u8] {
    state[..3].copy_from_slice(&LIB_VERSION);
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
    let ver: [u8; 3] = state[..3].try_into().unwrap();

    let not_before = not_before.unwrap_or([0, 0, 0]);

    if cmp_lib_ver(&ver, &not_before) < 0 {
        return Err(CoreError::IncompatibleVersion);
    };
    if ver == [0, 0, 0] {
        return Err(CoreError::IncompatibleVersion);
    };

    Ok(&state[3..])
}
