use bouncycastle_core::errors::SerializedStateError;
use bouncycastle_core::serializable_state::{LIB_VERSION, SemVer};
use bouncycastle_core::traits::SerializableState;

pub struct TestFrameworkSerializableState {}

impl TestFrameworkSerializableState {
    pub fn new() -> Self {
        Self {}
    }

    /// Test all the members of trait SerializableState.
    ///
    /// Expects ta be handed an instance of the object that has some in-progress state to be serialized.
    pub fn test<
        const SERIALIZED_STATE_LEN: usize,
        S: SerializableState<SERIALIZED_STATE_LEN> + Clone,
    >(
        &self,
        instance: &S,
    ) {
        // There's not a lot we can test here in the abstract, but we can test a few things to
        // ensure that the SerializableState trait has been impl'd correctly.

        // we need to work on a clone because .serialize_state() moves self, which you can't do on a
        // borrowed instance.
        let instance_clone = instance.clone();

        // You can serialize and then deserialize the state.
        let serialized_state = instance_clone.serialize_state();
        assert_eq!(serialized_state.len(), SERIALIZED_STATE_LEN);

        let _deserialized_state = S::from_serialized_state(serialized_state).unwrap();

        // The serialized state MUST include a prefix indicating the current version of the library.
        let state_sized: [u8; 3] = serialized_state[..3].try_into().unwrap();
        assert_eq!(SemVer::from(state_sized), LIB_VERSION);

        // All implementations MUST reject a serialized state from lib ver 0.0.0
        // This doesn't really serve any purpose except testing that all impl's have properly
        // used the helper functions.
        let mut ver0_serialized_state = serialized_state.clone();
        ver0_serialized_state[..3].copy_from_slice(&[0, 0, 0]);
        match S::from_serialized_state(ver0_serialized_state) {
            Err(SerializedStateError::IncompatibleVersion) => { /* good */ }
            _ => {
                panic!("Expected IncompatibleVersion error")
            }
        }

        // All implementations MUST reject a serialized state from a future version.
        let mut future_ver = LIB_VERSION;
        future_ver.patch += 1;
        let mut futurever_serialized_state = serialized_state.clone();
        futurever_serialized_state[..3]
            .copy_from_slice(&[future_ver.major, future_ver.minor, future_ver.patch]);
        match S::from_serialized_state(futurever_serialized_state) {
            Err(SerializedStateError::IncompatibleVersion) => { /* good */ }
            _ => {
                panic!("Expected IncompatibleVersion error")
            }
        }
    }
}
