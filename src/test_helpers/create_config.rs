use crate::config::Config;

pub(crate) fn create_config() -> Config {
	Config::new_with_config(None).unwrap()
}
