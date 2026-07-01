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

/// A helper for serializing an object's state
///
/// The state array must have length SERIALIZED_LEN - 3 to account for adding the 3-byte symver tag.
pub fn add_lib_ver<const SERIALIZED_LEN: usize>(state: &[u8]) -> [u8; SERIALIZED_LEN] {
    assert_eq!(state.len(), SERIALIZED_LEN - 3);

    let mut out = [0u8; SERIALIZED_LEN];
    out[..3].copy_from_slice(&LIB_VERSION);
    out[3..].copy_from_slice(state);

    out
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
pub fn remove_lib_ver<const SERIALIZED_LEN: usize>(
    state_in: &[u8; SERIALIZED_LEN],
    state_out: &mut [u8],
    not_before: Option<[u8; 3]>,
) -> Result<usize, CoreError> {
    assert!(state_out.len() >= SERIALIZED_LEN - 3);

    let ver: [u8; 3] = state_in[..3].try_into().unwrap();

    let not_before = not_before.unwrap_or([0, 0, 0]);

    if cmp_lib_ver(&ver, &not_before) < 0 {
        return Err(CoreError::IncompatibleVersion);
    };
    if ver == [0, 0, 0] {
        return Err(CoreError::IncompatibleVersion);
    };

    state_out.copy_from_slice(&state_in[3..]);
    Ok(SERIALIZED_LEN - 3)
}
