use lazy_static::lazy_static;

use crate::settings;

lazy_static! {
    pub static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("Config can be loaded");
}
