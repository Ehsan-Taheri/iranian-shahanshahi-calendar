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
    "Farvardin",
    "Ordibehesht",
    "Khordad",
    "Tir",
    "Amordad",
    "Shahrivar",
    "Mehr",
    "Aban",
    "Azar",
    "Dey",
    "Bahman",
    "Esfand",
];

// In tag mige: age feature python fa'al bood, #[pyclass] ro ezafe kon
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

// ==========================================
// 1. API Asli-e Rust (Hamishe Compile Mishe baraye CLI va Rust)
// ==========================================
impl ShahanshahiDate {
    pub fn new(jy: i32, jm: u8, jd: u8) -> Option<Self> {
        if !(1..=12).contains(&jm) { return None; }

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
        let (jy, jm, jd) = gregorian_to_jalali(t.year(), t.month() as i32, t.day() as i32);
        Self {
            year: jy + SH_OFFSET,
            month: jm as u8,
            day: jd as u8,
        }
    }

    pub fn events(&self) -> Vec<String> {
        events::events_on(self.month, self.day)
    }

    pub fn get_month_name(&self) -> String {
        MONTH_NAMES[(self.month - 1) as usize].to_string()
    }
}

// ==========================================
// 2. Python API
// ==========================================
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

// ==========================================
// Tabe'-haye Komaki
// ==========================================
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
