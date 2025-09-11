use std::{fmt::Display, str::FromStr};

use url::Url;

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_10x_genomics_url<S: AsRef<str> + Display>(s: &S, _: &()) -> garde::Result {
    let url = Url::from_str(s.as_ref())
        .map_err(|e| garde::Error::new(format!("failed to parse {s} as URL: {e}")))?;

    let Some(domain) = url.domain() else {
        return Err(garde::Error::new("malformed URL"));
    };

    if domain != "cdn.10xgenomics.com" {
        return Err(garde::Error::new(format!(
            "URL domain must be 'cdn.10xgenomics.com', found {domain}"
        )));
    }

    Ok(())
}
