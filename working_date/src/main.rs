mod models;
use crate::models::issues::Issues;
use crate::models::transitions::Transitions;

fn main() -> Result<(), String> {
    let issue = Issues::new();
    
    let check_date = Transitions::check_status_to_send_message(&issue).unwrap();

    println!("{}", check_date);
    
    match check_date{
        n if n == 7 => println!("Send Message for To-do"),
        n if n == 12 => println!("Send Message for In Analysis"),
        n if n >= 3 => println!("Send Message for Retesting"),
        _ => return Err("Valor inaceitável".to_string()),
    };

    Ok(())
}
