use serde::Deserialize;
use std::fs;

#[derive(Deserialize,Clone)]
pub struct Event{
    pub month:u8,
    pub day:u8,
    pub name:String,
}

pub fn load_events()->Vec<Event>{
    let data=fs::read_to_string("data/events.json")
        .expect("cannot read events.json");
    serde_json::from_str(&data)
        .expect("bad json")
}

pub fn events_on(m:u8,d:u8)->Vec<String>{
    load_events()
        .into_iter()
        .filter(|e|e.month==m && e.day==d)
        .map(|e|e.name)
        .collect()
}
