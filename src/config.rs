use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Config {
    repo: Repo,
    update: Update,
    server: Server,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Repo {
    remote: String,
    branch: String,
    tag: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Update {
    frequency: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Server {
    port: i64,
    bind_address: String,
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
    fn test_config_deserialize() {
        let actual: Config = toml::from_str(
            r#"
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
        assert_eq!(expected, actual);
    }
}
