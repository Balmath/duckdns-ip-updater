use duckdns_ip_updater::{run, Config, Error, ErrorKind};

fn main() {
    match Config::new(std::env::args_os()) {
        Ok(config) => match run(config) {
            Ok(_) => println!("Duck DNS domains successfully updated!"),
            Err(error) => handle_error(error),
        },
        Err(error) => handle_error(error),
    };
}

fn handle_error(error: Error) {
    match error {
        Error {
            kind: ErrorKind::DisplayHelp,
            message,
        }
        | Error {
            kind: ErrorKind::DisplayVersion,
            message,
        } => println!("{}", message),
        Error { message, .. } => eprintln!("{}", message),
    }
}
