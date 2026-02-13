use chrono::{NaiveDate,Datelike};
pub mod events;



pub const SH_OFFSET:i32=1180;

#[derive(Debug,PartialEq,Clone)]
pub struct ShahanshahiDate{
    pub year:i32,
    pub month:u8,
    pub day:u8,
}

#[derive(Clone)]
pub struct Event{
    pub month:u8,
    pub day:u8,
    pub name:&'static str,
}

pub const MONTH_NAMES:[&str;12]=[
"Farvardin","Ordibehesht","Khordad","Tir","Amordad",
"Shahrivar","Mehr","Aban","Azar","Dey","Bahman","Esfand"
];

 
pub fn month_name(m:u8)->&'static str{
    MONTH_NAMES[(m-1)as usize]
}

 

pub fn is_jalali_leap(jy:i32)->bool{
    let mut a=jy-474;
    if a<0{a-=1;}
    let b=474+(a%2820);
    ((b+38)*682)%2816<682
}

pub fn days_in_month(y:i32,m:u8)->u8{
    match m{
        1..=6=>31,
        7..=11=>30,
        12=>if is_jalali_leap(y){30}else{29},
        _=>panic!("bad month")
    }
}

fn gregorian_to_jalali(gy:i32,gm:i32,gd:i32)->(i32,i32,i32){
    let gdm=[31,28,31,30,31,30,31,31,30,31,30,31];
    let jdm=[31,31,31,31,31,31,30,30,30,30,30,29];

    let gy=gy-1600;
    let gm=gm-1;
    let gd=gd-1;

    let mut gdn=365*gy+(gy+3)/4-(gy+99)/100+(gy+399)/400;
    for i in 0..gm{gdn+=gdm[i as usize];}

    if gm>1&&((gy+1600)%4==0&&((gy+1600)%100!=0||(gy+1600)%400==0)){
        gdn+=1;
    }

    gdn+=gd;

    let mut jdn=gdn-79;
    let jnp=jdn/12053;
    jdn%=12053;

    let mut jy=979+33*jnp+4*(jdn/1461);
    jdn%=1461;

    if jdn>=366{
        jy+=(jdn-1)/365;
        jdn=(jdn-1)%365;
    }

    let mut jm=0;
    let mut jd=0;

    for i in 0..11{
        if jdn<jdm[i]{
            jm=i+1;
            jd=jdn+1;
            break;
        }
        jdn-=jdm[i];
    }

    (jy,jm as i32,jd as i32)
}

impl ShahanshahiDate{

    pub fn from_jalali(jy:i32,jm:u8,jd:u8)->Self{
        Self{year:jy+SH_OFFSET,month:jm,day:jd}
    }

    pub fn from_gregorian(gy:i32,gm:u32,gd:u32)->Self{
        NaiveDate::from_ymd_opt(gy,gm,gd).expect("bad date");
        let (jy,jm,jd)=gregorian_to_jalali(gy,gm as i32,gd as i32);
        Self{year:jy+SH_OFFSET,month:jm as u8,day:jd as u8}
    }

    pub fn new(jy:i32,jm:u8,jd:u8)->Option<Self>{
        if jm<1||jm>12{return None;}
        let max=days_in_month(jy,jm);
        if jd<1||jd>max{return None;}
        Some(Self{year:jy+SH_OFFSET,month:jm,day:jd})
    }

    pub fn today()->Self{
        let t=chrono::Local::now().date_naive();
        Self::from_gregorian(t.year(),t.month(),t.day())
    }
    

    pub fn events(&self)->Vec<String>{
        crate::events::events_on(self.month,self.day)
    }
}

impl std::fmt::Display for ShahanshahiDate{
    fn fmt(&self,f:&mut std::fmt::Formatter)->std::fmt::Result{
        write!(f,"{:04}/{:02}/{:02}",self.year,self.month,self.day)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nowruz_2025() {
        let d = ShahanshahiDate::from_gregorian(2025,3,21);
        assert_eq!(d.year,2584);
        assert_eq!(d.month,1);
        assert_eq!(d.day,1);
    }

    #[test]
    fn test_from_jalali() {
        let d = ShahanshahiDate::from_jalali(1403,1,1);
        assert_eq!(d.year,2583);
    }
    #[test]
fn test_leap_year() {
    assert!(is_jalali_leap(1399)); // leap
    assert!(!is_jalali_leap(1400));
}

#[test]
fn test_days_esfand() {
    assert_eq!(days_in_month(1399,12),30);
    assert_eq!(days_in_month(1400,12),29);
}

#[test]
fn test_month_name() {
    assert_eq!(month_name(5),"Amordad");
}
#[test]
fn test_invalid_date() {
    assert!(ShahanshahiDate::new(1400,12,30).is_none());
}


}
