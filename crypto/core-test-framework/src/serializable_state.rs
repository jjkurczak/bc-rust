use bouncycastle_core::errors::CoreError;
use bouncycastle_core::serializable_state::{LIB_VERSION};
use bouncycastle_core::traits::{SerializableState};

pub struct TestFrameworkSerializableState { }

impl TestFrameworkSerializableState {
    pub fn new() -> Self {
        Self { }
    }

    /// Test all the members of trait SerializableState.
    /// 
    /// Expects ta be handed an instance of the object that has some in-progress state to be serialized.
    pub fn test<const SERIALIZED_STATE_LEN: usize, S: SerializableState<SERIALIZED_STATE_LEN>>(
        &self,
        instance: &S,
    ) {
        // There's not a lot we can test here in the abstract, but we can test a few things to
        // ensure that the SerializableState trait has been impl'd correctly.
        
        // You can serialize and then deserialize the state.
        let serialized_state = instance.serialize_state();
        assert_eq!(serialized_state.len(), SERIALIZED_STATE_LEN);
        
        let _deserialized_state = S::from_serialized_state(serialized_state).unwrap();
        
        
        // The serialized state MUST include a prefix indicating the current version of the library.
        assert_eq!(serialized_state[..3], LIB_VERSION);
        
        
        // All implementations MUST reject a serialized state from lib ver 0.0.0
        // This doesn't really serve any purpose except testing that all impl's have properly
        // used the helper functions.
        let mut busted_serialized_state = serialized_state.clone();
        busted_serialized_state[..3].copy_from_slice(&[0, 0, 0]);
        match S::from_serialized_state(busted_serialized_state) {
            Err(CoreError::IncompatibleVersion) => { /* good */ },
            _ => { panic!("Expected IncompatibleVersion error") }
        }
    }
}
