use clap::{App, Arg, ArgMatches, Error as ClapError, ErrorKind as ClapErrorKind};
use dirs;
use serde::Deserialize;
use std::ffi::OsString;
use std::fs;
use std::path::Path;

use super::{Error, ErrorKind};

const DEFAULT_CONFIG_FOLDER_NAME: &str = "duckdns-ip-updater";
const DEFAULT_CONFIG_FILE_NAME: &str = "default.conf";

#[derive(Debug, Deserialize)]
pub struct Config {
    domains: Vec<String>,
    token: String,
}

impl Config {
    pub fn new<I, T>(args: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches_result = build_command_line_parser().get_matches_from_safe(args);

        match matches_result {
            Ok(matches) => Self::from_matches(matches),
            Err(error) => Err(convert_command_line_error(error)),
        }
    }

    pub fn get_domains(&self) -> &Vec<String> {
        &self.domains
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    fn from_matches(matches: ArgMatches) -> Result<Self, Error> {
        if let Some(domain) = matches.value_of("domain") {
            let mut domains = Vec::new();

            domains.push(String::from(domain));

            let token = String::from(matches.value_of("token").unwrap());

            return Ok(Self { domains, token });
        }

        if let Some(config) = matches.value_of("config") {
            Self::from_file_path(&config)
        } else if let Some(config_dir) = dirs::config_dir() {
            Self::from_file_path(
                config_dir
                    .join(DEFAULT_CONFIG_FOLDER_NAME)
                    .join(DEFAULT_CONFIG_FILE_NAME),
            )
        } else {
            Err(Error {
                kind: ErrorKind::ConfigFolderAccess,
                message: String::from("error: Cannot determine the config folder"),
            })
        }
    }

    fn from_file_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        match fs::read_to_string(path) {
            Ok(config_content) => Config::from_toml_str(&config_content),
            _ => Err(Error {
                kind: ErrorKind::InvalidConfigFile,
                message: format!("Cannot open configuration file"),
            }),
        }
    }

    fn from_toml_str(content: &str) -> Result<Self, Error> {
        toml::from_str(content).or(Err(Error {
            kind: ErrorKind::InvalidConfigFile,
            message: format!("Configuration file content is invalid"),
        }))
    }
}

fn build_command_line_parser<'a, 'b>() -> App<'a, 'b> {
    let mut app = App::new("DuckDNS IP Updater")
        .version("1.0")
        .author("Balmath <Balmath@users.noreply.github.com>")
        .about("Update the DNS entry with the ip of the running machine");

    app = add_config_arg(app);
    app = add_domain_arg(app);
    add_token_arg(app)
}

fn add_config_arg<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true)
            .conflicts_with_all(&["domain", "token"]),
    )
}

fn add_domain_arg<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("domain")
            .short("d")
            .long("domain")
            .value_name("DOMAIN")
            .help("Sets the domain you want to update")
            .takes_value(true)
            .requires("token")
            .conflicts_with("config"),
    )
}

fn add_token_arg<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("token")
            .short("t")
            .long("token")
            .value_name("TOKEN")
            .help("Sets your personal token")
            .takes_value(true)
            .requires("domain")
            .conflicts_with("config"),
    )
}

fn convert_command_line_error(error: ClapError) -> Error {
    match error {
        ClapError {
            kind: ClapErrorKind::HelpDisplayed,
            message,
            ..
        } => Error {
            kind: ErrorKind::DisplayHelp,
            message,
        },
        ClapError {
            kind: ClapErrorKind::VersionDisplayed,
            message,
            ..
        } => Error {
            kind: ErrorKind::DisplayVersion,
            message,
        },
        ClapError { message, .. } => Error {
            kind: ErrorKind::InvalidArgument,
            message,
        },
    }
}

#[cfg(test)]
mod tests {
    use rand::RngCore;
    use std::env;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_with_valid_arguments() {
        let config = Config::new(&[
            "duckdns-ip-updater",
            "--domain",
            "mydomain",
            "--token",
            "mytoken",
        ])
        .unwrap();
        assert_eq!(config.get_domains(), &["mydomain"]);
        assert_eq!(config.get_token(), "mytoken");
    }

    #[test]
    fn test_with_valid_short_arguments() {
        let config =
            Config::new(&["duckdns-ip-updater", "-d", "mydomain", "-t", "mytoken"]).unwrap();
        assert_eq!(config.get_domains(), &["mydomain"]);
        assert_eq!(config.get_token(), "mytoken");
    }

    #[test]
    fn test_with_invalid_short_arguments() {
        let config =
            Config::new(&["duckdns-ip-updater", "-n", "mydomain", "-t", "mytoken"]).unwrap_err();

        let message = String::from(
            "error: Found argument \'-n\' which wasn\'t expected, or isn\'t valid in this context\n\nUSAGE:\n    duckdns-ip-updater [OPTIONS]\n\nFor more information try --help");
        assert_eq!(
            config,
            Error {
                kind: ErrorKind::InvalidArgument,
                message: String::from(message)
            }
        );
    }

    #[test]
    fn test_with_help_argument() {
        let config = Config::new(&["duckdns-ip-updater", "--help"]).unwrap_err();

        let message = String::from(
            "DuckDNS IP Updater 0.1\nBalmath <Balmath@users.noreply.github.com>\nUpdate the DNS entry with the ip of the running machine\n\nUSAGE:\n    duckdns-ip-updater [OPTIONS]\n\nFLAGS:\n    -h, --help       Prints help information\n    -V, --version    Prints version information\n\nOPTIONS:\n    -c, --config <FILE>      Sets a custom config file\n    -d, --domain <DOMAIN>    Sets the domain you want to update\n    -t, --token <TOKEN>      Sets your personal token");
        assert_eq!(
            config,
            Error {
                kind: ErrorKind::DisplayHelp,
                message: String::from(message)
            }
        );
    }

    #[test]
    fn test_with_version_argument() {
        let config = Config::new(&["duckdns-ip-updater", "--version"]).unwrap_err();

        let message = String::from("");
        assert_eq!(
            config,
            Error {
                kind: ErrorKind::DisplayVersion,
                message: String::from(message)
            }
        );
    }

    struct TempDir(PathBuf);

    impl TempDir {
        fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
            let TempDir(ref p) = *self;
            p.join(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let TempDir(ref p) = *self;
            fs::remove_dir_all(p).unwrap();
        }
    }

    fn create_temp_dir() -> TempDir {
        let mut temp_dir = env::temp_dir();
        let mut random = rand::thread_rng();

        temp_dir.push(&format!("test{}", random.next_u64()));

        fs::create_dir(&temp_dir).unwrap();

        TempDir(temp_dir)
    }

    #[test]
    fn test_with_config_argument() {
        let temp_dir = create_temp_dir();

        let config_file_path = temp_dir.join("config.conf");

        fs::write(
            &config_file_path,
            r#"
            domains = ["my_domain1", "my_domain2"]
            token = "my_token"
        "#,
        )
        .unwrap();

        let config = Config::new(&[
            OsString::from("duckdns-ip-updater"),
            OsString::from("--config"),
            OsString::from(config_file_path),
        ])
        .unwrap();

        assert_eq!(config.get_domains(), &["my_domain1", "my_domain2"]);
        assert_eq!(config.get_token(), "my_token");
    }
}
