#![allow(unsafe_op_in_unsafe_fn)]

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;

pub mod events;

pub const SH_OFFSET: i32 = 1180;

pub const MONTH_NAMES: [&str; 12] = [
    "فروردین",
    "اردیبهشت",
    "خرداد",
    "تیر",
    "امرداد",
    "شهریور",
    "مهر",
    "آبان",
    "آذر",
    "دی",
    "بهمن",
    "اسفند",
];

#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ShahanshahiDate {
    #[cfg_attr(feature = "python", pyo3(get))]
    pub year: i32,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub month: u8,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub day: u8,
}

impl ShahanshahiDate {
    pub fn new(jy: i32, jm: u8, jd: u8) -> Option<Self> {
        if !(1..=12).contains(&jm) {
            return None;
        }

        let max = days_in_month(jy, jm);
        if jd < 1 || jd > max {
            return None;
        }
        Some(Self {
            year: jy + SH_OFFSET,
            month: jm,
            day: jd,
        })
    }
    pub fn today() -> Self {
        let t = chrono::Local::now().date_naive();
        let (jy, jm, jd) = gregorian_to_jalali(t.year(), t.month() as i32, t.day() as i32);
        Self {
            year: jy + SH_OFFSET,
            month: jm as u8,
            day: jd as u8,
        }
    }

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

    pub fn to_gregorian(&self) -> Option<(i32, u32, u32)> {
        let (gy, gm, gd) =
            jalali_to_gregorian(self.year - SH_OFFSET, self.month as i32, self.day as i32);
        Some((gy, gm as u32, gd as u32))
    }
    pub fn to_jalali(&self) -> (i32, u8, u8) {
        (self.year - SH_OFFSET, self.month, self.day)
    }
    pub fn events(&self) -> Vec<String> {
        events::events_on(self.month, self.day)
    }

    pub fn get_month_name(&self) -> String {
        MONTH_NAMES[(self.month - 1) as usize].to_string()
    }

    pub fn day_of_week(&self) -> String {
         
        let (gy, gm, gd) = jalali_to_gregorian(self.year - SH_OFFSET, self.month as i32, self.day as i32);
        
        if let Some(nd) = NaiveDate::from_ymd_opt(gy, gm as u32, gd as u32) {
            match nd.weekday() {
                chrono::Weekday::Sat => "شنبه",
                chrono::Weekday::Sun => "یک‌شنبه",
                chrono::Weekday::Mon => "دوشنبه",
                chrono::Weekday::Tue => "سه‌شنبه",
                chrono::Weekday::Wed => "چهارشنبه",
                chrono::Weekday::Thu => "پنج‌شنبه",
                chrono::Weekday::Fri => "آدینه",
            }.to_string()
        } else {
            "نامشخص".to_string()
        }
    }

    pub fn add_days(&self, days: i32) -> Option<Self> {
        let (gy, gm, gd) =
            jalali_to_gregorian(self.year - SH_OFFSET, self.month as i32, self.day as i32);
        let nd = NaiveDate::from_ymd_opt(gy, gm as u32, gd as u32)?;
        let new_nd =
            nd.checked_add_signed(chrono::Duration::try_days(days as i64).unwrap_or_default())?;
        Self::from_gregorian(new_nd.year(), new_nd.month(), new_nd.day())
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl ShahanshahiDate {
    #[new]
    fn py_new(jy: i32, jm: u8, jd: u8) -> PyResult<Self> {
        Self::new(jy, jm, jd).ok_or_else(|| PyValueError::new_err("Invalid date"))
    }

    #[staticmethod]
    #[pyo3(name = "from_jalali")]
    fn py_from_jalali(jy: i32, jm: u8, jd: u8) -> Self {
        Self::from_jalali(jy, jm, jd)
    }

    #[staticmethod]
    #[pyo3(name = "from_gregorian")]
    fn py_from_gregorian(gy: i32, gm: u32, gd: u32) -> PyResult<Self> {
        Self::from_gregorian(gy, gm, gd)
            .ok_or_else(|| PyValueError::new_err("Invalid Gregorian date"))
    }

    #[staticmethod]
    #[pyo3(name = "today")]
    fn py_today() -> Self {
        Self::today()
    }

    #[pyo3(name = "events")]
    fn py_events(&self) -> Vec<String> {
        self.events()
    }

    #[pyo3(name = "get_month_name")]
    fn py_get_month_name(&self) -> String {
        self.get_month_name()
    }

    #[pyo3(name = "to_gregorian")]
    fn py_to_gregorian(&self) -> PyResult<(i32, u32, u32)> {
        self.to_gregorian()
            .ok_or_else(|| PyValueError::new_err("Conversion failed"))
    }

    #[pyo3(name = "day_of_week")]
    fn py_day_of_week(&self) -> String {
        self.day_of_week()
    }

    #[pyo3(name = "add_days")]
    fn py_add_days(&self, days: i32) -> PyResult<Self> {
        self.add_days(days)
            .ok_or_else(|| PyValueError::new_err("Invalid date calculation"))
    }

    fn __str__(&self) -> String {
        format!("{:04}/{:02}/{:02}", self.year, self.month, self.day)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[cfg_attr(feature = "python", pyfunction)]
pub fn month_name(m: u8) -> String {
    if (1..=12).contains(&m) {
        MONTH_NAMES[(m - 1) as usize].to_string()
    } else {
        "".to_string()
    }
}

pub fn is_jalali_leap(jy: i32) -> bool {
    let mut a = jy - 474;
    if a < 0 {
        a -= 1;
    }
    let b = 474 + (a % 2820);
    ((b + 38) * 682) % 2816 < 682
}

pub fn days_in_month(y: i32, m: u8) -> u8 {
    match m {
        1..=6 => 31,
        7..=11 => 30,
        12 => {
            if is_jalali_leap(y) {
                30
            } else {
                29
            }
        }
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
    for i in 0..gm_adj {
        gdn += gdm[i as usize];
    }

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
    for (i, &days) in jdm.iter().enumerate().take(11) {
        if jdn < days {
            jm = i + 1;
            jd = jdn + 1;
            break;
        }
        jdn -= days;
    }
    if jm == 0 {
        jm = 12;
        jd = jdn + 1;
    }

    (jy, jm as i32, jd)
}

fn jalali_to_gregorian(jy: i32, jm: i32, jd: i32) -> (i32, i32, i32) {
    let mut gy = if jy <= 979 { 621 } else { 1600 };
    let jy2 = jy - if jy <= 979 { 0 } else { 979 };

    let mut days = 365 * jy2 + (jy2 / 33) * 8 + (jy2 % 33 + 3) / 4;
    for i in 0..(jm - 1) {
        days += if i < 6 { 31 } else { 30 };
    }
    days += jd - 1;

    let mut gy2 = 400 * (days / 146097);
    days %= 146097;
    if days > 36524 {
        days -= 1;
        gy2 += 100 * (days / 36524);
        days %= 36524;
        if days >= 365 {
            days += 1;
        }
    }
    gy2 += 4 * (days / 1461);
    days %= 1461;
    if days > 365 {
        gy2 += (days - 1) / 365;
        days = (days - 1) % 365;
    }
    gy += gy2;

    let leap = (gy % 4 == 0 && gy % 100 != 0) || (gy % 400 == 0);
    let gdm = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut gm = 0;
    let mut gd = days + 1;
    for (i, &m_days) in gdm.iter().enumerate() {
        if gd <= m_days {
            gm = i as i32 + 1;
            break;
        }
        gd -= m_days;
    }

    (gy, gm, gd)
}

impl std::fmt::Display for ShahanshahiDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:04}/{:02}/{:02}", self.year, self.month, self.day)
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn imperial_cal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShahanshahiDate>()?;
    m.add_function(wrap_pyfunction!(month_name, m)?)?;
    Ok(())
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
