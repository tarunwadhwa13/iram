use super::base::AlertSource;

pub mod NewRelic {
    /// This is used for interacting with NewRelic. 
    /// Dev Note: All functions must be private since only implemented functions are to be used. Helper functions created must not be exposed
    struct NewRelic {

    }
    impl AlertSource for NewRelic {
        fn get_source_name(&self) -> &str {
            "new_relic"
        }

    }

}