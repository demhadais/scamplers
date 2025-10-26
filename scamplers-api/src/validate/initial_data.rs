use crate::{initial_data::InitialData, validate::Validate};

impl Validate for InitialData {
    fn validate(&self, db_conn: &mut diesel::PgConnection) -> Result<(), super::Error> {
        let Self {
            institution,
            app_admin,
        } = &self;

        institution.validate(db_conn)?;
        app_admin.validate(db_conn)?;
        // self.index_set_urls.validate(db_conn)?;

        Ok(())
    }
}
