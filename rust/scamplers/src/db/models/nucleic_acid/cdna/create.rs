use any_value::{AnyValue, WithSnakeCaseKeys};
use diesel::prelude::*;
use scamplers_schema::{cdna, cdna_measurement, cdna_preparers, gems, tenx_assay};
use uuid::Uuid;

use crate::{
    db::{
        DbOperation,
        models::{
            nucleic_acid::{
                cdna::{Cdna, CdnaPreparer, CdnaQuery, NewCdna, NewCdnaGroup, NewCdnaMeasurement},
                common::gems_to_assay,
            },
            tenx_assay::chromium::{LibraryType, LibraryTypeSpecification},
        },
        util::{ChildrenWithSelfId, ManyToMany, ManyToManyChildrenWithSelfId, SetParentId},
    },
    result::{CdnaLibraryTypeError, ScamplersResult},
};

impl SetParentId for NewCdnaMeasurement {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.cdna_id
    }
}

impl ChildrenWithSelfId<NewCdnaMeasurement> for NewCdna {
    fn children(&mut self) -> &mut [NewCdnaMeasurement] {
        &mut self.measurements
    }
}

impl ManyToMany for CdnaPreparer {
    fn new(parent_id: Uuid, child_id: Uuid) -> Self {
        Self {
            cdna_id: parent_id,
            prepared_by: child_id,
        }
    }
}

impl ManyToManyChildrenWithSelfId<CdnaPreparer> for NewCdna {
    fn mtm_children(&self) -> &[Uuid] {
        &self.preparer_ids
    }
}

trait VecExt {
    fn library_types_and_volumes(&self) -> Vec<(LibraryType, f32)>;
}

impl VecExt for Vec<&NewCdna> {
    fn library_types_and_volumes(&self) -> Vec<(LibraryType, f32)> {
        self.iter().map(|c| (c.library_type, c.volume_µl)).collect()
    }
}

impl NewCdnaGroup {
    fn as_groups(&self) -> Vec<Vec<&NewCdna>> {
        let cdnas = match self {
            Self::Multiple { cdna } | Self::OnChipMultiplexing { cdna } => cdna.iter().collect(),
            Self::Single { cdna } => vec![cdna],
        };

        let mut grouped_cdnas = Vec::with_capacity(cdnas.len());
        let mut seen_cdnas = Vec::with_capacity(cdnas.len());

        for (i, c1) in cdnas.iter().enumerate() {
            let mut group = Vec::with_capacity(cdnas.len());
            group.push(*c1);

            for c2 in &cdnas[i..cdnas.len()] {
                if c1.library_type != c2.library_type && !seen_cdnas.contains(c2) {
                    group.push(*c2);
                    seen_cdnas.push(*c2);
                }
            }

            grouped_cdnas.push(group);
        }

        grouped_cdnas
    }

    fn assay_id(&self, db_conn: &mut PgConnection) -> ScamplersResult<Option<Uuid>> {
        let cdna = match self {
            Self::Single { cdna } => Some(cdna),
            Self::Multiple { cdna } | Self::OnChipMultiplexing { cdna } => cdna.first(),
        };

        let Some(gems_id) = cdna.map(|c| c.gems_id) else {
            return Ok(None);
        };

        let assay_id = gems_to_assay()
            .select(tenx_assay::id)
            .filter(gems::id.eq(gems_id))
            .first(db_conn)?;

        Ok(Some(assay_id))
    }

    fn validate_library_types(&self, db_conn: &mut PgConnection) -> ScamplersResult<()> {
        let assay_id = self.assay_id(db_conn)?;

        let Some(assay_id) = assay_id else {
            return Ok(());
        };

        let expected_specifications =
            LibraryTypeSpecification::list_by_assay_id(assay_id, db_conn)?;

        let expected_library_types_and_volumes: Vec<_> = expected_specifications
            .iter()
            .map(|s| (s.library_type, s.cdna_volume_µl))
            .collect();

        for cdna_group in self.as_groups() {
            if expected_library_types_and_volumes != cdna_group.library_types_and_volumes() {
                return Err(CdnaLibraryTypeError {
                    expected_specifications,
                }
                .into());
            }
        }

        Ok(())
    }

    fn into_vec(self) -> Vec<NewCdna> {
        match self {
            Self::Multiple { cdna } | Self::OnChipMultiplexing { cdna } => cdna,
            Self::Single { cdna } => vec![cdna],
        }
    }
}

impl NewCdna {
    fn snake_case_additional_data(&mut self) {
        self.additional_data = self
            .additional_data
            .take()
            .map(AnyValue::with_snake_case_keys);
    }
}

impl DbOperation<Vec<Cdna>> for NewCdnaGroup {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Vec<Cdna>> {
        self.validate_library_types(db_conn)?;

        let mut cdnas = self.into_vec();

        for cdna in &mut cdnas {
            cdna.snake_case_additional_data();
        }

        let ids = diesel::insert_into(cdna::table)
            .values(&cdnas)
            .returning(cdna::id)
            .get_results(db_conn)?;

        let mut measurements = Vec::with_capacity(cdnas.len());
        let mut preparers = Vec::with_capacity(cdnas.len());

        // We have to do two iterations for the borrow checker (one for measurements and
        // one for preparers)
        for (cdna, id) in cdnas.iter_mut().zip(&ids) {
            measurements.extend(cdna.children_with_self_id(*id).iter_mut().map(|m| &*m));
        }

        diesel::insert_into(cdna_measurement::table)
            .values(measurements)
            .execute(db_conn)?;

        for (cdna, id) in cdnas.iter_mut().zip(&ids) {
            preparers.extend(cdna.mtm_children_with_self_id(*id));
        }

        diesel::insert_into(cdna_preparers::table)
            .values(preparers)
            .execute(db_conn)?;

        CdnaQuery::builder().ids(ids).build().execute(db_conn)
    }
}
