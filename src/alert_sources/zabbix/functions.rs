pub mod zabbix {
    /// This is used for interacting with Zabbix.
    /// Dev Note: All functions must be private since only implemented functions are to be used. Helper functions created must not be exposed
    use crate::alert_sources::AlertSourceInfo;
    use std::collections::HashMap;
    use std::error::Error;

    use crate::errors::{UnsupportedError, ZabbixError};
    use reqwest;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value as jsonValue};

    use crate::alert_sources::base::AlertSource;
    use crate::alert_sources::response::{Alert, AlertList};

    use crate::alert_sources::zabbix::payload::{ActiveTrigger, Event};

    fn default_result() -> jsonValue {
        return jsonValue::Null;
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ZabbixAPIResponse {
        pub jsonrpc: String,
        pub error: Option<ZabbixAPIError>,
        #[serde(default = "default_result")]
        pub result: jsonValue,
        pub id: jsonValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ZabbixAPIError {
        pub code: i64,
        pub message: String,
        pub data: String,
    }

    pub struct ZabbixHandler {
        pub connect_url: String,
        pub identifier: String,
        connection_params: serde_json::Value,
        auth_key: String,
        auth_mechanism: String,
    }

    type AuthKey = String;


    impl AlertSource for ZabbixHandler {
        fn new_from_object(obj: &AlertSourceInfo) -> Result<Self, Box<dyn Error>> {
            Ok(ZabbixHandler {
                auth_key: "".to_string(),
                auth_mechanism: obj.auth_type.to_string(),
                connect_url: obj.connect_url.to_string(),
                connection_params: obj.connection_params.clone(),
                identifier: obj.identifier.to_string(),
            })
        }

        fn get_source_name(&self) -> &str {
            "zabbix"
        }

        fn process_webhook(&self) -> Result<AlertList, Box<dyn Error>> {
            return Ok(Vec::new());
        }

        fn get_active_alerts(&mut self) -> Result<AlertList, Box<dyn Error>> {
            log::debug!("Getting Active Alerts from Zabbix - {}", &self.identifier);

            let auth_key: AuthKey = get_zbx_authkey(self, true).unwrap();
            self.auth_key = auth_key;

            log::debug!("Received auth Key from Zabbix - {}", &self.auth_key);

            let trigger_payload: jsonValue = get_active_trigger_payload(&self.auth_key);

            let zbx_response: jsonValue = query_zabbix(self, trigger_payload).unwrap();
            if !zbx_response.is_array() {
                return Err(Box::new(ZabbixError(
                    "Expected active triggers response to be array.".to_string(),
                )));
            };

            let active_triggers: Vec<ActiveTrigger> = serde_json::from_value(zbx_response).unwrap();

            log::info!("Active Triggers - {:?}", active_triggers);

            // This will store final alerts to be returned
            let mut alert_list: AlertList = Vec::new();

            // this is used for storing alert corresponding to a trigger. This will be updated when we have event details
            let mut alert_map: HashMap<String, HashMap<&str, jsonValue>> =
                HashMap::with_capacity(active_triggers.len());

            let mut event_ids: Vec<String> = Vec::new();

            for i in active_triggers.iter() {
                let mut groups = Vec::new();
                for group in i.groups.iter() {
                    groups.push(jsonValue::String(group.name.clone()));
                }

                let trigger_id = i.triggerid.clone();
                event_ids.push(i.lastEvent.eventid.clone());

                let alert: HashMap<&str, jsonValue> = HashMap::from([
                    (
                        "source",
                        jsonValue::String(self.get_source_name().to_string()),
                    ),
                    ("alert_start_time", jsonValue::String("12345".to_string())),
                    ("subject", jsonValue::String(i.description.clone())),
                    ("entity", jsonValue::String(i.hosts[0].name.clone())),
                    ("groups", jsonValue::Array(groups)),
                    ("trigger_id", jsonValue::String(trigger_id.clone())),
                    (
                        "alert_status",
                        jsonValue::String(
                            match i.lastEvent.acknowledged.as_str() {
                                "0" => "New",
                                "1" => "Acknowledged",
                                _ => "Unknown",
                            }
                            .to_string(),
                        ),
                    ),
                    ("event_id", jsonValue::String(i.lastEvent.eventid.clone())),
                    (
                        "priority",
                        jsonValue::String(
                            match i.priority.as_str() {
                                "1" => "P1",
                                "2" => "P2",
                                "3" => "P3",
                                "4" => "P4",
                                _ => i.priority.as_str(),
                            }
                            .to_string(),
                        ),
                    ),
                ]);

                alert_map.insert(trigger_id.clone(), alert.clone());
            }

            // get event details for active trigger
            let event_payload = get_event_query_payload(event_ids, &self.auth_key);
            let event_response: jsonValue = query_zabbix(self, event_payload).unwrap();
            if !event_response.is_array() {
                return Err(Box::new(ZabbixError(
                    "Expected active triggers response to be array.".to_string(),
                )));
            };

            let events: Vec<Event> = serde_json::from_value(event_response).unwrap();

            // parse events to get data
            for event in events.iter() {
                let objectid = event.objectid.clone();
                // map event data to trigger data

                let mut alert_info = alert_map.get(&objectid.clone()).unwrap();

                // let alert = Alert {
                //     source: alert_info.get("source").unwrap().to_string(),
                //     event_id: String,
                //     trigger_id: String,
                //     subject: String,
                //     entity: String,
                //     groups: Vec<String>,
                //     alert_start_time: String,
                //     alert_age: String,
                //     alert_status: String,
                //     priority: String,
                //     tags: HashMap<String, String>
                // };

                // alert_list.push(alert);
            }

            Ok(alert_list)
        }
        fn acknowledge_alert(&self) -> bool {
            todo!()
        }
    }

    fn get_active_trigger_payload(auth_key: &String) -> jsonValue {
        return json!({
            "jsonrpc": "2.0",
            "method": "trigger.get",
            "params": {
                "selectTags": "extend",
                "selectHosts": [
                    "name",
                    "hostid",
                    "host",
                    "tags"
                ],
                "selectLastEvent": "extend",
                "selectGroups": [
                    "name",
                    "groupid"
                ],
                "monitored": 1,
                "lastChangeSince": "",
                "output": [
                    "triggerid",
                    "description",
                    "priority",
                    "lastchange",
                    "host",
                    "hostgroup",
                    "value",
                    "state"
                ],
                "filter": {
                    "value": 1
                },
                "sortfield": "lastchange",
                "sortorder": "DESC"
            },
            "auth": format!("{}", auth_key),
            "id": 1
        });
    }

    fn get_event_query_payload(event_ids: Vec<String>, auth_key: &String) -> jsonValue {
        return json!({
            "jsonrpc": "2.0",
            "method": "event.get",
            "params": {
                            "eventids": event_ids,
                            "output": "extend",
                            "select_acknowledges": "extend",
                            "select_alerts": "extend",
                            "selectTags": "extend",
                        },
            "auth": format!("{}", auth_key),
            "id": 1
        });
    }

    fn get_zbx_authkey(
        zbx: &ZabbixHandler,
        check_if_valid: bool,
    ) -> Result<AuthKey, Box<dyn Error>> {
        // Ideally we should be creating static API tokens to avoid creating sessions in every call
        // Still, module provides functionality to use basic auth to generate auth token
        // This can be useful when we want to rotate token on emergency basis

        // Check auth mechanism
        let auth_mechanism = &zbx.auth_mechanism;
        let auth_key: &str;

        match auth_mechanism.as_str() {
            "None" => auth_key = "",
            "APIKey" => {
                let response_opt = &zbx.connection_params.get("apiKey").unwrap();
                match response_opt.as_str() {
                    Some(key) => {
                        if check_if_valid {
                            let result = check_connection(zbx, key);
                            if !result.is_ok() {
                                return Err(Box::new(ZabbixError(
                                    "apiKey present in connection_params is not valid".to_string(),
                                )));
                            }
                        }
                        auth_key = key;
                    }
                    None => {
                        return Err(Box::new(ZabbixError(
                            "Cannot serialize apiKey as str".to_string(),
                        )));
                    }
                }
            }
            "BasicAuth" => {
                // connect using username password
                return create_new_session(zbx);
            }
            _ => {
                return Err(Box::new(UnsupportedError(
                    "This auth mechanism is not supported".to_string(),
                )))
            }
        };
        return Ok(auth_key.to_string() as AuthKey);
    }

    fn check_connection(zbx: &ZabbixHandler, key: &str) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn create_new_session(zbx: &ZabbixHandler) -> Result<AuthKey, Box<dyn Error>> {
        // TODO: explore possibility of re-using auth key for multiple sessions

        let username = zbx.connection_params.get("username");

        if username.is_none() {
            return Err(Box::new(ZabbixError(
                "Couldn't find username in connection params".to_string(),
            )));
        }

        let zbx_username: String;

        match username {
            Some(name) if name.is_string() => {
                zbx_username = name.as_str().unwrap().to_string();
            }
            _ => {
                return Err(Box::new(ZabbixError(
                    "Error parsing zabbix username to string".to_string(),
                )));
            }
        }

        let password = zbx.connection_params.get("password");

        if password.is_none() {
            return Err(Box::new(ZabbixError(
                "Couldn't find password in connection params".to_string(),
            )));
        }

        let zbx_password: String;

        match password {
            Some(pass) if pass.is_string() => {
                zbx_password = pass.as_str().unwrap().to_string();
            }
            _ => {
                return Err(Box::new(ZabbixError(
                    "Error parsing zabbix password to string".to_string(),
                )));
            }
        }

        let zbx_query = json!({
            "jsonrpc": "2.0",
            "method": "user.login",
            "params": {
                "user": format!("{}", zbx_username),
                "password": format!("{}", zbx_password)
            },
            "id" : ""
        });
        // Parse the string of data into serde_json::Value.
        log::debug!("Logging using params - {}", zbx_query);

        log::info!("Signing in to Zabbix to retrieve Auth Key");
        let data_result: jsonValue = query_zabbix(zbx, zbx_query).unwrap();

        match data_result {
            jsonValue::String(key) => Ok(key as AuthKey),
            _ => Err(Box::new(ZabbixError(
                "Unexpected response type received. Expected String".to_string(),
            ))),
        }
    }

    fn query_zabbix(zbx: &ZabbixHandler, payload: jsonValue) -> Result<jsonValue, Box<dyn Error>> {
        // Dont add scheme before url. Source can be both on http or https
        let uri = &zbx.connect_url;
        log::info!("Using URL - {}", uri);
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(uri)
            .header("Content-Type", "application/json-rpc")
            .json(&payload)
            .send();

        let parsed_response: ZabbixAPIResponse;

        match response {
            Ok(res) => {
                log::info!("Status code received from Zabbix - {}", res.status());
                // json_response tries to be of type ZabbixAPIResponse since this has to be assigned to parsed_response eventually
                let json_response = res.json();
                match json_response {
                    Ok(zbx_message) => {
                        log::debug!("{:?}", zbx_message);
                        parsed_response = zbx_message;
                    }
                    Err(err) => {
                        return Err(Box::new(ZabbixError(err.to_string())));
                    }
                }
            }
            Err(err) => {
                return Err(Box::new(ZabbixError(err.to_string())));
            }
        }

        if let Some(error) = parsed_response.error {
            return Err(Box::new(ZabbixError(format!("{}", error.data))));
        };
        match parsed_response.result {
            jsonValue::Null => Err(Box::new(ZabbixError(
                "Error Querying Zabbix - No Output received".to_string(),
            ))),
            _ => Ok(parsed_response.result),
        }
    }
}
