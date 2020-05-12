use ureq;

use super::{Error, ErrorKind};

macro_rules! duckdns_url {
    ($domains: expr, $token: expr) => {
        format!(
            "https://www.duckdns.org/update?domains={}&token={}",
            $domains, $token
        )
    };
}

pub fn update_ips<I, T>(domains: I, token: &str) -> Result<(), Error>
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    let domains_vec: Vec<String> = domains.into_iter().map(|d| d.into()).collect();
    let domains_query = domains_vec.join(",");

    let url = duckdns_url!(domains_query, token);
    let response = ureq::get(&url).call();

    match response.into_string() {
        Ok(content) if content == "OK" => Ok(()),
        _ => Err(Error {
            kind: ErrorKind::InvalidRequest,
            message: format!("{} IP domains could not be updated", domains_query),
        }),
    }
}
