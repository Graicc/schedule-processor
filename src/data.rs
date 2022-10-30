use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionRoot {
    pub data: Vec<Datum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datum {
    #[serde(rename = "DT_RowId")]
    pub dt_row_id: String,
    pub eventid: String,
    pub career: String,
    #[serde(rename = "crse_id")]
    pub crse_id: String,
    #[serde(rename = "crse_offer_nbr")]
    pub crse_offer_nbr: String,
    pub classs: String,
    #[serde(rename = "crse_title")]
    pub crse_title: String,
    #[serde(rename = "grading_basis")]
    pub grading_basis: String,
    pub units: String,
    #[serde(rename = "enrl_status")]
    pub enrl_status: String,
    #[serde(rename = "requisite_met")]
    pub requisite_met: String,
    #[serde(rename = "requisite_descr")]
    pub requisite_descr: String,
    #[serde(rename = "add_to_cart_data")]
    pub add_to_cart_data: Vec<AddToCartDatum>,
    pub description: String,
    #[serde(rename = "xlist_descr")]
    pub xlist_descr: String,
    #[serde(rename = "section_details")]
    pub section_details: Vec<SectionDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToCartDatum {
    #[serde(rename = "course_career")]
    pub course_career: String,
    #[serde(rename = "session_code")]
    pub session_code: String,
    #[serde(rename = "crse_id")]
    pub crse_id: String,
    #[serde(rename = "class_nbr")]
    pub class_nbr: String,
    #[serde(rename = "catalog_nbr")]
    pub catalog_nbr: String,
    #[serde(rename = "unt_taken")]
    pub unt_taken: String,
    #[serde(rename = "grading_basis")]
    pub grading_basis: String,
    #[serde(rename = "rqmnt_designtn")]
    pub rqmnt_designtn: String,
    #[serde(rename = "wait_list_okay")]
    pub wait_list_okay: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionDetail {
    #[serde(rename = "class_nbr")]
    pub class_nbr: String,
    pub section: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "meet_days")]
    pub meet_days: String,
    pub time: String,
    pub facility: String,
    pub instructors: Vec<String>,
    #[serde(rename = "seat_availability")]
    pub seat_availability: String,
    #[serde(rename = "reserved_seats")]
    pub reserved_seats: Vec<Value>,
    #[serde(rename = "instr_mode")]
    pub instr_mode: String,
    #[serde(rename = "instructor_edit")]
    pub instructor_edit: String,
    pub location: String,
    #[serde(rename = "building_address")]
    pub building_address: String,
    pub dates: String,
    #[serde(rename = "class_notes")]
    pub class_notes: Vec<String>,
    #[serde(rename = "course_topic")]
    pub course_topic: String,
    #[serde(rename = "calendar_info")]
    pub calendar_info: Vec<CalendarInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarInfo {
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "end_time")]
    pub end_time: String,
    pub all_day: Option<bool>,
}
