use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub directory: String,
    pub repo: Repo,
    pub update: Update,
    pub server: Server,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            directory: default_dir(),
            repo: Repo::default(),
            update: Update::default(),
            server: Server::default(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Repo {
    pub remote: String,
    pub branch: String,
    pub tag: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Update {
    pub frequency: String,
}

impl Default for Update {
    fn default() -> Self {
        Update {
            frequency: "@daily".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Server {
    pub port: i64,
    pub bind_address: String,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            port: 4750,
            bind_address: "0.0.0.0".to_string(),
        }
    }
}

fn default_dir() -> String {
    match env::var("XDG_CACHE_HOME") {
        Ok(val) => format!("{}/raincloud/", val),
        Err(_) => ".".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn test_repo_deserialize() {
        let expected = Repo {
            remote: "https://github.com/KGB33/raincloud.git".to_string(),
            branch: "main".to_string(),
            tag: "v1.2.3".to_string(),
        };
        let actual: Repo = toml::from_str(
            r#"
            remote = "https://github.com/KGB33/raincloud.git"
            branch = "main"
            tag = "v1.2.3""#,
        )
        .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_repo_default() {
        let expected = Repo {
            remote: String::default(),
            branch: String::default(),
            tag: String::default(),
        };
        let actual = Repo::default();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_update_deserialize() {
        let expected = Update {
            frequency: "0 22 * * 1-5".to_string(),
        };
        let actual: Update =
            toml::from_str(r#"frequency = "0 22 * * 1-5" # https://crontab.guru/#0_22)*_*_1-5"#)
                .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_update_default() {
        let expected = Update {
            frequency: "@daily".to_string(),
        };
        let actual = Update::default();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_server_deserialize() {
        let expected = Server {
            port: 4750,
            bind_address: "0.0.0.0".to_string(),
        };
        let actual: Server = toml::from_str(
            r#"
            port = 4750
            bind_address = "0.0.0.0"
            "#,
        )
        .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_server_default() {
        let expected = Server {
            port: 4750,
            bind_address: "0.0.0.0".to_string(),
        };
        let actual = Server::default();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_config_deserialize() {
        let actual: Config = toml::from_str(
            r#"
            directory = "/foo/bar/baz"
            [repo]
            remote = "https://github.com/KGB33/raincloud.git"
            branch = "main"
            tag = "v1.2.3"
            
            [update]
            frequency = "0 22 * * 1-5" # https://crontab.guru/#0_22_*_*_1-5
            
            [server]
            port = 4750
            bind_address = "0.0.0.0"
        "#,
        )
        .unwrap();
        let expected = Config {
            directory: "/foo/bar/baz".to_string(),
            repo: Repo {
                remote: "https://github.com/KGB33/raincloud.git".to_string(),
                branch: "main".to_string(),
                tag: "v1.2.3".to_string(),
            },
            update: Update {
                frequency: "0 22 * * 1-5".to_string(),
            },
            server: Server {
                port: 4750,
                bind_address: "0.0.0.0".to_string(),
            },
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_config_default() {
        let expected = Config {
            directory: default_dir(),
            repo: Repo::default(),
            update: Update::default(),
            server: Server::default(),
        };
        let actual = Config::default();
        assert_eq!(actual, expected);
    }
}
