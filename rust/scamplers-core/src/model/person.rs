use crate::{
    model::{IsUpdate, Pagination, SortByGroup, institution::Institution},
    string::NonEmptyString,
};
use scamplers_macros::{
    base_api_model, base_api_model_with_default, db_enum, db_insertion_with_wasm, db_query,
    db_selection, db_update,
};
#[cfg(feature = "backend")]
use scamplers_schema::person;
use uuid::Uuid;

#[db_enum]
#[derive(PartialEq)]
pub enum UserRole {
    AppAdmin,
    ComputationalStaff,
    BiologyStaff,
}

#[db_insertion_with_wasm]
#[derive(Clone)]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct NewPerson {
    #[garde(dive)]
    name: NonEmptyString,
    #[garde(email)]
    email: String,
    #[garde(dive)]
    #[builder(default)]
    orcid: Option<NonEmptyString>,
    institution_id: Uuid,
    #[builder(default)]
    ms_user_id: Option<Uuid>,
    #[serde(default)]
    #[builder(default)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    roles: Vec<UserRole>,
}
impl NewPerson {
    #[must_use]
    pub fn new_user_route() -> String {
        "/users".to_string()
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonHandle {
    id: Uuid,
    link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonSummary {
    #[serde(flatten)]
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    handle: PersonHandle,
    name: String,
    email: Option<String>,
    orcid: Option<String>,
}
impl PersonSummary {
    pub fn id(&self) -> &Uuid {
        self.handle.id()
    }
    pub fn link(&self) -> &str {
        self.handle.link()
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonCore {
    #[serde(flatten)]
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    summary: PersonSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    institution: Institution,
}
impl PersonCore {
    pub fn id(&self) -> &Uuid {
        self.summary.id()
    }
    pub fn link(&self) -> &str {
        self.summary.link()
    }
    pub fn name(&self) -> &str {
        self.summary.name()
    }
    pub fn email(&self) -> &Option<String> {
        self.summary.email()
    }
    pub fn orcid(&self) -> &Option<String> {
        self.summary.orcid()
    }
}

#[base_api_model]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(::derive_builder::Builder)]
#[builder(pattern = "owned", build_fn(error = crate::model::BuilderError), setter(into, strip_option))]
pub struct Person {
    #[serde(flatten)]
    #[getset(skip)]
    core: PersonCore,
    #[builder(default)]
    roles: Vec<UserRole>,
}
impl Person {
    pub fn id(&self) -> &Uuid {
        self.core.id()
    }
    pub fn link(&self) -> &str {
        self.core.link()
    }
    pub fn name(&self) -> &str {
        self.core.name()
    }
    pub fn email(&self) -> &Option<String> {
        self.core.email()
    }
    pub fn orcid(&self) -> &Option<String> {
        self.core.orcid()
    }
    pub fn institution(&self) -> &Institution {
        self.core.institution()
    }
}

#[base_api_model]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(::derive_builder::Builder)]
#[builder(pattern = "owned", build_fn(error = crate::model::BuilderError), setter(into, strip_option))]
pub struct CreatedUser {
    #[serde(flatten)]
    #[getset(skip)]
    person: Person,
    api_key: String,
}
impl CreatedUser {
    pub fn id(&self) -> &Uuid {
        self.person.id()
    }
    pub fn link(&self) -> &str {
        self.person.link()
    }
    pub fn name(&self) -> &str {
        self.person.name()
    }
    pub fn email(&self) -> &Option<String> {
        self.person.email()
    }
    pub fn orcid(&self) -> &Option<String> {
        self.person.orcid()
    }
    pub fn roles(&self) -> &[UserRole] {
        &self.person.roles()
    }
    pub fn institution(&self) -> &Institution {
        &self.person.institution()
    }
}

#[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen)]
impl CreatedUser {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter, js_name = id))]
    pub fn id_owned(&self) -> Uuid {
        self.id().to_owned()
    }
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter, js_name = apiKey))]
    pub fn api_key_owned(&self) -> String {
        self.api_key().to_owned()
    }
}

#[db_update]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonUpdateCore {
    id: Uuid,
    #[garde(dive)]
    name: Option<NonEmptyString>,
    #[garde(email)]
    email: Option<String>,
    ms_user_id: Option<Uuid>,
    #[garde(dive)]
    orcid: Option<NonEmptyString>,
    institution_id: Option<Uuid>,
}

impl IsUpdate for PersonUpdateCore {
    fn is_update(&self) -> bool {
        matches!(
            self,
            Self {
                name: Some(_),
                email: Some(_),
                orcid: Some(_),
                institution_id: Some(_),
                ms_user_id: Some(_),
                ..
            }
        )
    }
}

#[base_api_model_with_default]
pub struct PersonUpdate {
    grant_roles: Vec<UserRole>,
    revoke_roles: Vec<UserRole>,
    #[serde(flatten)]
    #[garde(dive)]
    core: PersonUpdateCore,
}

#[base_api_model_with_default]
pub enum PersonOrdinalColumn {
    #[default]
    Name,
    Email,
}

#[db_query]
pub struct PersonQuery {
    ids: Vec<Uuid>,
    name: Option<String>,
    email: Option<String>,
    orcid: Option<String>,
    ms_user_id: Option<Uuid>,
    #[builder(setter(custom))]
    order_by: SortByGroup<PersonOrdinalColumn>,
    pagination: Pagination,
}
