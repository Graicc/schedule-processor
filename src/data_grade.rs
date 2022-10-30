use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradeRoot {
    pub individual: Vec<Individual>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Individual {
    pub course_name: String,
    pub course_id: String,
    pub instructor_name: String,
    pub course_sem: String,
    pub grades: Grades,
    pub google_chart: GoogleChart,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grades {
    #[serde(rename = "A")]
    pub a: Nums,
    #[serde(rename = "B")]
    pub b: Nums,
    #[serde(rename = "C")]
    pub c: Nums,
    #[serde(rename = "D")]
    pub d: Nums,
    #[serde(rename = "F")]
    pub f: Nums,
    #[serde(rename = "S")]
    pub s: Nums,
    #[serde(rename = "U")]
    pub u: Nums,
    #[serde(rename = "IN")]
    pub in_field: Nums,
    #[serde(rename = "LA")]
    pub la: Nums,
    #[serde(rename = "AU")]
    pub au: Nums,
    #[serde(rename = "NR")]
    pub nr: Nums,
    #[serde(rename = "W")]
    pub w: Nums,
    #[serde(rename = "TOTAL")]
    pub total: Nums,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nums {
    pub raw: i64,
    pub percentage: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleChart {
    pub cols: Vec<Col>,
    pub rows: Vec<Row>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Col {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub c: Vec<C2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C2 {
    pub v: Value,
}
