use std::{collections::HashMap};
use chrono::{DateTime, Local};
use std::error::Error as Err;
use crate::models::issues::Issues;

#[derive(Debug)]
pub struct Transitions{
    transitions: HashMap<String, String>
}                                                                                                              

impl Transitions{
    pub fn new() -> Transitions {

        let mut data = HashMap::new();

        data.insert("To-do".to_string(), 1.to_string());
        data.insert("In Analysis".to_string(), 2.to_string());
        data.insert("Retest fail".to_string(), 3.to_string());

        
        Transitions { transitions:data }

    }

    pub fn check_status_to_send_message(issue: &Issues) -> Result<i64, Box<dyn Err>> {
        let current_date: DateTime<Local> = Local::now();
        let today = &current_date.naive_local().date();
    
        let day_issue = issue.date;
       
        let calc_diff = today.signed_duration_since(day_issue);
       
        let dias = calc_diff.num_days();
    
        Ok(dias)
    }
}





