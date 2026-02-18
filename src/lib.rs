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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShahanshahiDate {
    #[cfg_attr(feature = "python", pyo3(get))]
    pub year: i32,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub month: u8,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub day: u8,
}

impl ShahanshahiDate {
    pub fn new(y: i32, m: u8, d: u8) -> Option<Self> {
        if !(1..=12).contains(&m) {
            return None;
        }

        let final_year = if y < 1600 { y + SH_OFFSET } else { y };

        let jy = final_year - SH_OFFSET;
        if d < 1 || d > days_in_month(jy, m) {
            return None;
        }

        Some(Self {
            year: final_year,
            month: m,
            day: d,
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

    pub fn day_of_week(&self) -> &'static str {
        let (gy, gm, gd) = self.to_gregorian().unwrap();
        let nd = chrono::NaiveDate::from_ymd_opt(gy, gm, gd).unwrap();

        let fa_weekdays = [
            "شنبه",
            "یک‌شنبه",
            "دوشنبه",
            "سه‌شنبه",
            "چهارشنبه",
            "پنج‌شنبه",
            "آدینه",
        ];

        // چند روز از یکشنبه گذشته؟
        let d = nd.weekday().num_days_from_sunday();

        // تبدیل به شروع از شنبه
        let idx = ((d + 1) % 7) as usize;

        fa_weekdays[idx]
    }

    pub fn add_days(&self, days: i32) -> Option<Self> {
        let (gy, gm, gd) = self.to_gregorian()?;
        let nd = NaiveDate::from_ymd_opt(gy, gm, gd)?;
        let new_nd =
            nd.checked_add_signed(chrono::Duration::try_days(days as i64).unwrap_or_default())?;
        Self::from_gregorian(new_nd.year(), new_nd.month(), new_nd.day())
    }

    pub fn events(&self) -> Vec<String> {
        events::events_on(self.month, self.day)
    }

    pub fn get_month_name(&self) -> String {
        MONTH_NAMES[(self.month - 1) as usize].to_string()
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl ShahanshahiDate {
    #[new]
    fn py_new(y: i32, m: u8, d: u8) -> PyResult<Self> {
        Self::new(y, m, d).ok_or_else(|| PyValueError::new_err("Invalid date"))
    }

    #[staticmethod]
    #[pyo3(name = "from_jalali")]
    fn py_from_jalali(jy: i32, jm: u8, jd: u8) -> Self {
        Self::from_jalali(jy, jm, jd)
    }

    #[staticmethod]
    #[pyo3(name = "from_gregorian")]
    fn py_from_gregorian(gy: i32, gm: u32, gd: u32) -> PyResult<Self> {
        Self::from_gregorian(gy, gm, gd).ok_or_else(|| PyValueError::new_err("Invalid date"))
    }

    #[staticmethod]
    #[pyo3(name = "today")]
    fn py_today() -> Self {
        Self::today()
    }

    fn events(&self) -> Vec<String> {
        self.events()
    }
    fn get_month_name(&self) -> String {
        self.get_month_name()
    }
    fn to_gregorian(&self) -> PyResult<(i32, u32, u32)> {
        self.to_gregorian()
            .ok_or_else(|| PyValueError::new_err("Conversion failed"))
    }
    fn to_jalali(&self) -> (i32, u8, u8) {
        self.to_jalali()
    }
    fn day_of_week(&self) -> String {
        self.day_of_week()
    }
    fn add_days(&self, days: i32) -> PyResult<Self> {
        self.add_days(days)
            .ok_or_else(|| PyValueError::new_err("Math error"))
    }

    fn __str__(&self) -> String {
        format!("{:04}/{:02}/{:02}", self.year, self.month, self.day)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

// کمک‌کننده‌ها (Helpers) - بدون تغییر
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
    let jy = jy - 979;
    let jm = jm - 1;
    let jd = jd - 1;

    let mut j_day_no = 365 * jy + jy / 33 * 8 + ((jy % 33) + 3) / 4;

    for i in 0..jm {
        j_day_no += if i < 6 { 31 } else { 30 };
    }

    j_day_no += jd;

    let mut g_day_no = j_day_no + 79;

    let mut gy = 1600 + 400 * (g_day_no / 146097);
    g_day_no %= 146097;

    let mut leap = true;

    if g_day_no >= 36525 {
        g_day_no -= 1;
        gy += 100 * (g_day_no / 36524);
        g_day_no %= 36524;

        if g_day_no >= 365 {
            g_day_no += 1;
        } else {
            leap = false;
        }
    }

    gy += 4 * (g_day_no / 1461);
    g_day_no %= 1461;

    if g_day_no >= 366 {
        leap = false;
        g_day_no -= 1;
        gy += g_day_no / 365;
        g_day_no %= 365;
    }

    let gd_month = [
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
    let mut gd = g_day_no + 1;

    for (i, &v) in gd_month.iter().enumerate() {
        if gd <= v {
            gm = i + 1;
            break;
        }
        gd -= v;
    }

    (gy, gm as i32, gd)
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
    }
}

#[test]
fn test_nowruz_weekday() {
    // 2025-03-21 = جمعه
    let d = ShahanshahiDate::from_gregorian(2025, 3, 21).unwrap();
    assert_eq!(d.day_of_week(), "آدینه");
}
