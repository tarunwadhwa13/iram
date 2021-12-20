extern crate iram;

#[cfg(test)]
mod test{
    static ALERT_SOURCE: &str  = "Zabbix";
    static TEST_ZABBIX_SOURCE: &str = "SLA-Zabbix-Test";

    use iram::alert_sources::base::AlertSource;

    #[test]
    fn create_zabbix_source(){
        let zbx_obj = iram::alert_sources::get_alert_source_handler(ALERT_SOURCE, TEST_ZABBIX_SOURCE).unwrap();
        assert_eq!(zbx_obj.get_source_name().to_lowercase(), "zabbix")
    }

    #[test]
    fn get_active_alerts_from_zabbix() {
        let mut zbx_obj = iram::alert_sources::get_alert_source_handler(ALERT_SOURCE, TEST_ZABBIX_SOURCE).unwrap();
        let active_alerts = zbx_obj.get_active_alerts();
        assert_eq!(active_alerts.is_ok(), true)
    }
}