use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub mod events;

pub const SH_OFFSET: i32 = 1180;

pub const MONTH_NAMES: [&str; 12] = [
    "Farvardin", "Ordibehesht", "Khordad", "Tir", "Amordad", "Shahrivar", 
    "Mehr", "Aban", "Azar", "Dey", "Bahman", "Esfand"
];

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShahanshahiDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl ShahanshahiDate {
    
    pub fn new(jy: i32, jm: u8, jd: u8) -> Option<Self> {
        if jm < 1 || jm > 12 { return None; }
        let max = days_in_month(jy, jm);
        if jd < 1 || jd > max { return None; }
        Some(Self {
            year: jy + SH_OFFSET,
            month: jm,
            day: jd,
        })
    }

    /// Create a ShahanshahiDate from Jalali date components
    pub fn from_jalali(jy: i32, jm: u8, jd: u8) -> Self {
        Self {
            year: jy + SH_OFFSET,
            month: jm,
            day: jd,
        }
    }

     
    pub fn from_gregorian(gy: i32, gm: u32, gd: u32) -> Option<Self> {
     
        NaiveDate::from_ymd_opt(gy, gm, gd)?;
        let (jy, jm, jd) = gregorian_to_jalali(gy, gm as i32, gd as i32);
        Some(Self {
            year: jy + SH_OFFSET,
            month: jm as u8,
            day: jd as u8,
        })
    }

    
    pub fn today() -> Self {
        let t = chrono::Local::now().date_naive();
    
        Self::from_gregorian(t.year(), t.month(), t.day()).unwrap()
    }

     
    pub fn events(&self) -> Vec<String> {
        events::events_on(self.month, self.day)
    }

     
    pub fn get_month_name(&self) -> String {
        MONTH_NAMES[(self.month - 1) as usize].to_string()
    }
}

 

pub fn month_name(m: u8) -> &'static str {
    MONTH_NAMES[(m - 1) as usize]
}

pub fn is_jalali_leap(jy: i32) -> bool {
    let mut a = jy - 474;
    if a < 0 { a -= 1; }
    let b = 474 + (a % 2820);
    ((b + 38) * 682) % 2816 < 682
}

pub fn days_in_month(y: i32, m: u8) -> u8 {
    match m {
        1..=6 => 31,
        7..=11 => 30,
        12 => if is_jalali_leap(y) { 30 } else { 29 },
        _ => 0,
    }
}

fn gregorian_to_jalali(gy: i32, gm: i32, gd: i32) -> (i32, i32, i32) {
    let gdm = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let jdm = [31, 31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 29];

    let gy_adj = gy - 1600;
    let gm_adj = gm - 1;
    let gd_adj = gd - 1;

    let mut gdn = 365 * gy_adj + (gy_adj + 3) / 4 - (gy_adj + 99) / 100 + (gy_adj + 399) / 400;
    for i in 0..gm_adj { gdn += gdm[i as usize]; }

    if gm_adj > 1 && ((gy % 4 == 0 && gy % 100 != 0) || (gy % 400 == 0)) {
        gdn += 1;
    }
    gdn += gd_adj;

    let mut jdn = gdn - 79;
    let jnp = jdn / 12053;
    jdn %= 12053;

    let mut jy = 979 + 33 * jnp + 4 * (jdn / 1461);
    jdn %= 1461;

    if jdn >= 366 {
        jy += (jdn - 1) / 365;
        jdn = (jdn - 1) % 365;
    }

    let mut jm = 0;
    let mut jd = 0;
    for i in 0..11 {
        if jdn < jdm[i] {
            jm = i + 1;
            jd = jdn + 1;
            break;
        }
        jdn -= jdm[i];
    }
    if jm == 0 { jm = 12; jd = jdn + 1; }

    (jy, jm as i32, jd as i32)
}

impl std::fmt::Display for ShahanshahiDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:04}/{:02}/{:02}", self.year, self.month, self.day)
    }
}

 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nowruz_2025() {
        let d = ShahanshahiDate::from_gregorian(2025, 3, 21).unwrap();
        assert_eq!(d.year, 2584);
        assert_eq!(d.month, 1);
        assert_eq!(d.day, 1);
    }

    #[test]
    fn test_invalid_date() {
        
        assert!(ShahanshahiDate::new(1400, 12, 30).is_none());
    }
}