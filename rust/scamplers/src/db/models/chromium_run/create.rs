use diesel::prelude::*;
use scamplers_schema::{chip_loading, chromium_run, gems};

use crate::db::{
    DbOperation,
    models::chromium_run::{
        ChromiumRun, ChromiumRunId, NewChromiumRun, NewChromiumRunCommon, NewOcmChromiumRun,
        NewOcmGems, NewPoolMultiplexChipLoading, NewPoolMultiplexChromiumRun, NewPoolMultiplexGems,
        NewSingleplexChipLoading, NewSingleplexChromiumRun, NewSingleplexGems,
    },
    util::{ChildrenWithSelfId, SetParentId},
};

impl<'a> Insertable<chromium_run::table> for &'a NewChromiumRun {
    type Values = <&'a NewChromiumRunCommon as Insertable<chromium_run::table>>::Values;

    fn values(self) -> Self::Values {
        use NewChromiumRun::{Ocm, PoolMultiplex, Singleplex};

        // Cloning is cheap
        let inner = match self {
            Singleplex(NewSingleplexChromiumRun { inner, .. })
            | Ocm(NewOcmChromiumRun { inner, .. })
            | PoolMultiplex(NewPoolMultiplexChromiumRun { inner, .. }) => inner,
        };

        inner.values()
    }
}

impl SetParentId for NewSingleplexGems {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.inner.chromium_run_id
    }
}

impl ChildrenWithSelfId<NewSingleplexGems> for NewSingleplexChromiumRun {
    fn children(&mut self) -> &mut [NewSingleplexGems] {
        &mut self.gems
    }
}

impl SetParentId for NewOcmGems {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.inner.chromium_run_id
    }
}

impl ChildrenWithSelfId<NewOcmGems> for NewOcmChromiumRun {
    fn children(&mut self) -> &mut [NewOcmGems] {
        &mut self.gems
    }
}

impl SetParentId for NewPoolMultiplexGems {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.inner.chromium_run_id
    }
}

impl ChildrenWithSelfId<NewPoolMultiplexGems> for NewPoolMultiplexChromiumRun {
    fn children(&mut self) -> &mut [NewPoolMultiplexGems] {
        &mut self.gems
    }
}

impl SetParentId for NewSingleplexChipLoading {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.inner.gems_id
    }
}

impl ChildrenWithSelfId<NewSingleplexChipLoading> for NewSingleplexGems {
    fn children(&mut self) -> &mut [NewSingleplexChipLoading] {
        &mut self.loading
    }
}

impl SetParentId for NewPoolMultiplexChipLoading {
    fn parent_id_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.inner.gems_id
    }
}

impl ChildrenWithSelfId<NewSingleplexChipLoading> for NewOcmGems {
    fn children(&mut self) -> &mut [NewSingleplexChipLoading] {
        &mut self.loading
    }
}

impl ChildrenWithSelfId<NewPoolMultiplexChipLoading> for NewPoolMultiplexGems {
    fn children(&mut self) -> &mut [NewPoolMultiplexChipLoading] {
        &mut self.loading
    }
}

impl DbOperation<ChromiumRun> for NewChromiumRun {
    fn execute(self, db_conn: &mut PgConnection) -> crate::result::ScamplersResult<ChromiumRun> {
        use NewChromiumRun::{Ocm, PoolMultiplex, Singleplex};

        macro_rules! insert_gems_and_loadings {
            ($cr:expr, $cr_id:expr) => {{
                let new_gems = $cr.children_with_self_id($cr_id);

                let gems_ids = diesel::insert_into(gems::table)
                    .values(&*new_gems)
                    .returning(gems::id)
                    .get_results(db_conn)?;

                for (ng, created) in new_gems.into_iter().zip(gems_ids) {
                    let loadings = ng.children_with_self_id(created);
                    diesel::insert_into(chip_loading::table)
                        .values(&*loadings)
                        .execute(db_conn)?;
                }
            }};
        }

        let chromium_run_id = diesel::insert_into(chromium_run::table)
            .values(&self)
            .returning(chromium_run::id)
            .get_result(db_conn)?;

        match self {
            Singleplex(mut cr) => insert_gems_and_loadings!(cr, chromium_run_id),
            Ocm(mut cr) => insert_gems_and_loadings!(cr, chromium_run_id),
            PoolMultiplex(mut cr) => insert_gems_and_loadings!(cr, chromium_run_id),
        }

        ChromiumRunId(chromium_run_id).execute(db_conn)
    }
}
