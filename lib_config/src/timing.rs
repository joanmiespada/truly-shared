
use chrono::prelude::{DateTime, Utc};

pub fn iso8601(st: &DateTime<Utc>) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
}

pub fn from_iso8601(st: &String) -> DateTime<Utc> {
    let aux = st.parse::<DateTime<Utc>>().unwrap();
    aux
}