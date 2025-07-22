use diesel_async::{AsyncPgConnection, RunQueryDsl};
use garde::Validate;
use scamplers_core::model::person::{NewMsLogin, UserRole};
use serde::Deserialize;

use super::WriteToDb;
use crate::db::model::person::grant_roles_to_user;

#[derive(Deserialize, Validate, Clone)]
#[garde(allow_unvalidated)]
#[serde(transparent)]
pub(super) struct NewAdmin(#[garde(dive)] NewMsLogin);

impl NewAdmin {
    pub(super) async fn write(
        self,
        db_conn: &mut AsyncPgConnection,
    ) -> super::super::error::Result<()> {
        let created_user = self.0.write_to_db(db_conn).await?;

        // For convenience, grant the admin roles here, though this should be factored
        // out eventually into a `PersonUpdate` struct that we can just populate and
        // call from in here, rather than copying code
        diesel::select(grant_roles_to_user(
            created_user.id().to_string(),
            vec![UserRole::AppAdmin],
        ))
        .execute(db_conn)
        .await?;

        Ok(())
    }
}
