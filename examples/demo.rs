use shahanshahi_core::{ShahanshahiDate,month_name};

fn main(){

   
    let today = ShahanshahiDate::today();

    println!("Today (Shahanshahi): {}", today);
    println!("Year: {}", today.year);
    println!("Month: {}", today.month);
    println!("Day: {}", today.day);
    println!("Month name: {}", month_name(today.month));
    let ev = today.events();
    if !ev.is_empty(){
        println!("Events today: {:?}", ev);
    } else {
        println!("No events today");
    }

     
    let nowruz = ShahanshahiDate::from_gregorian(2025,3,21);

    println!("\nNowruz 2025 → Shahanshahi:");
    println!("{}", nowruz);

    println!("Events: {:?}", nowruz.events());

    
    let from_jalali = ShahanshahiDate::from_jalali(1403,1,1);

    println!("\nJalali 1403/1/1 → Shahanshahi:");
    println!("{}", from_jalali);

}
