use AlertSource;

pub mod ManageEngine {
    /// This is used for interacting with ManageEngine. 
    /// Dev Note: All functions must be private since only implemented functions are to be used. Helper functions created must not be exposed
    struct manage_engine {

    }
    impl AlertSource for manage_engine {
        fn get_source_name(&self) -> $str {
            "manage_engine"
        }
    }
}