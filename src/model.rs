use serde::Deserialize;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Port {
    pub(crate) name: String,
    pub(crate) code: Option<String>,
    pub(crate) city: String,
    pub(crate) country: String,
    pub(crate) alias: Vec<String>,
    pub(crate) regions: Vec<String>,
    pub(crate) coordinates: Option<Vec<f64>>,
    pub(crate) province: Option<String>,
    pub(crate) timezone: Option<String>,
    pub(crate) unlocs: Vec<String>,
}
