use anyhow::anyhow;
#[cfg(feature = "app")]
use diesel::{PgConnection, prelude::*};
use regex::Regex;
#[cfg(feature = "app")]
use scamplers_schema::index_kit;
use std::{fmt::Display, sync::LazyLock};

use crate::result::ServerError;

pub(super) static INDEX_SET_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^SI-([NA]{2}|[TN]{2}|[GA]{2}|[TS]{2}|[TT]{2})-[A-H]\d{1,2}$").unwrap()
});
pub(super) static DNA_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[ACGT]{8}|[ACGT]{10}$").unwrap());

pub(super) const INDEX_SET_NAME_ERROR_MESSAGE: &str = "malformed index set name";

pub(super) trait IndexSetName: AsRef<str> {
    fn kit_name(&self) -> anyhow::Result<&str> {
        self.as_ref()
            .get(3..5)
            .ok_or(anyhow!(INDEX_SET_NAME_ERROR_MESSAGE))
    }

    fn well_name(&self) -> anyhow::Result<&str> {
        self.as_ref()
            .get(6..8)
            .ok_or(anyhow!(INDEX_SET_NAME_ERROR_MESSAGE,))
    }
}

impl<T> IndexSetName for T where T: AsRef<str> {}

pub(super) fn map_err(e: impl Display) -> ServerError {
    ServerError {
        message: e.to_string(),
        ..Default::default()
    }
}

#[cfg(feature = "app")]
pub(super) fn insert_kit_name(kit_name: &str, db_conn: &mut PgConnection) -> anyhow::Result<()> {
    diesel::insert_into(index_kit::table)
        .values(index_kit::name.eq(kit_name))
        .on_conflict_do_nothing()
        .execute(db_conn)?;

    Ok(())
}
