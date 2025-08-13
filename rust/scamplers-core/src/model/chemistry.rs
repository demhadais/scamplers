#[cfg(feature = "backend")]
use scamplers_schema::chemistry;
use serde_json::Value;
use valid_string::ValidString;

#[derive(
    Debug, ::serde::Deserialize, ::serde::Serialize, Clone, ::garde::Validate, ::valuable::Valuable,
)]
#[garde(allow_unvalidated)]
#[cfg_attr(feature = "app", derive(::diesel::Insertable))]
#[cfg_attr(feature = "app", diesel(check_for_backend(diesel::pg::Pg), table_name = chemistry))]
pub struct Chemistry {
    #[garde(dive)]
    pub name: ValidString,
    #[garde(dive)]
    pub description: ValidString,
    #[serde(flatten)]
    #[valuable(skip)]
    pub definition: Value,
    #[garde(dive)]
    pub cmdlines: Vec<Option<ValidString>>,
}
