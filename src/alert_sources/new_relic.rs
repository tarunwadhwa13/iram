pub mod NewRelic {
    use crate::alert_sources::base::AlertSource;
    use crate::alert_sources::response::AlertList;
    /// This is used for interacting with NewRelic.
    /// Dev Note: All functions must be private since only implemented functions are to be used. Helper functions created must not be exposed
    use crate::alert_sources::AlertSourceInfo;
    use std::error::Error;

    pub struct NewRelicHandler {
        pub connect_url: String,
        pub identifier: String,
        connection_params: serde_json::Value,
        auth_key: String,
        auth_mechanism: String,
    }

    impl AlertSource for NewRelicHandler {
        fn new_from_object(obj: &AlertSourceInfo) -> Self {
            NewRelicHandler {
                auth_key: "".to_string(),
                auth_mechanism: obj.auth_type.to_string(),
                connect_url: obj.connect_url.to_string(),
                connection_params: obj.connection_params.clone(),
                identifier: obj.identifier.to_string(),
            }
        }

        fn get_source_name(&self) -> &str {
            "newrelic"
        }

        fn test_connection(&mut self) -> bool {
            true
        }

        fn process_webhook(&self) -> Result<AlertList, Box<dyn Error>> {
            return Ok(Vec::new());
        }

        fn acknowledge_alert(&self) -> bool {
            todo!()
        }

        fn get_active_alerts(&mut self) -> Result<AlertList, Box<dyn Error>> {
            return Ok(Vec::new());
        }
    }
}
