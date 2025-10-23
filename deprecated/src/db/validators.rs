use std::{fmt::Display, str::FromStr};

use url::Url;
use uuid::Uuid;

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

pub(crate) fn children_parent_ids_matches_parent_id<T>(
    parent_id: Uuid,
    children: &[T],
    extract_parent_id_from_child: impl Fn(&T) -> &Uuid,
) -> garde::Result {
    if children
        .iter()
        .any(|child| ![parent_id, Uuid::nil()].contains(extract_parent_id_from_child(child)))
    {
        return Err(garde::Error::new(
            "the parent ID specified in the child does not match the parent being updated",
        ));
    }

    Ok(())
}
