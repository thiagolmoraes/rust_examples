use chrono::NaiveDate;

#[derive(Debug)]
pub struct Issues{
    name: String,
    status_id: String,
    pub date: NaiveDate
}

impl Issues{
    pub fn new() -> Self{
        Issues{ 
            name: "KEY-1".to_string(),
            status_id: "2".to_string(),
            date: NaiveDate::parse_from_str("15/04/2024", "%d/%m/%Y").unwrap()
        }
    }
}