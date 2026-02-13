use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Event {
    pub month: u8,
    pub day: u8,
    pub name: String,
}

 
const EVENTS_JSON: &str = include_str!("../data/events.json");

pub fn load_events() -> Vec<Event> {
    serde_json::from_str(EVENTS_JSON).expect("Error parsing embedded events.json")
}

pub fn events_on(m: u8, d: u8) -> Vec<String> {
    load_events()
        .into_iter()
          .filter(|e| e.month == m && e.day == d)
        .map(|e| e.name)
        
         .collect()
}