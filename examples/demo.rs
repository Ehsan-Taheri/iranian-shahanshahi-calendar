use imperial_cal::{ShahanshahiDate, month_name};

fn main() {
    println!("=== 📅 Taghvim-e Shahanshahi (Demo) ===\n");

    let today = ShahanshahiDate::today();
    println!("📆 Tarikh-e Emrooz (Shahanshahi): {}", today);
    println!("   Esm-e Mah: {}", month_name(today.month));
    println!("   Rooz-e Hafte: {}", today.day_of_week());

    let ev = today.events();
    if !ev.is_empty() {
        println!("   🎉 Monasebat: {:?}", ev);
    } else {
        println!("   📭 Monasebati baraye emrooz sabt nashode.");
    }
    println!("--------------------------------------------------");

    let gy = 2026;
    let gm = 3;
    let gd = 21;
    if let Some(nowruz) = ShahanshahiDate::from_gregorian(gy, gm, gd) {
        println!(
            "🌍 Miladi: {}/{}/{} ➡️  Shahanshahi: {}",
            gy, gm, gd, nowruz
        );
        println!("   Monasebat: {:?}", nowruz.events());
    }
    println!("--------------------------------------------------");

    // 3. Tabdil az Shamsi (Jalali) be Shahanshahi
    let shamsi_date = ShahanshahiDate::from_jalali(1405, 1, 1);
    println!("🇮🇷 Shamsi: 1405/01/01 ➡️  Shahanshahi: {}", shamsi_date);
    println!("--------------------------------------------------");

    // 4. Tabdil az Shahanshahi be Baghiye Taghvim-ha (Khorooji)
    let sample_date = ShahanshahiDate::new(2585, 1, 13).unwrap();
    println!("👑 Shahanshahi: {}", sample_date);

    let (j_year, j_month, j_day) = sample_date.to_jalali();
    println!("   🔄 Tabdil be Shamsi: {}/{}/{}", j_year, j_month, j_day);

    if let Some((g_year, g_month, g_day)) = sample_date.to_gregorian() {
        println!("   🔄 Tabdil be Miladi: {}/{}/{}", g_year, g_month, g_day);
    }
    println!("--------------------------------------------------");

    // 5. Mohasebat-e Tarikh (Riaziyat-e Taghvim)
    println!("🧮 Mohasebat-e Tarikh:");
    println!("   Mabda: {}", today);

    if let Some(future_date) = today.add_days(100) {
        println!(
            "   ⏳ 100 Rooz Ba'd: {} ({})",
            future_date,
            future_date.day_of_week()
        );
    }

    if let Some(past_date) = today.add_days(-30) {
        println!(
            "   ⏪ 30 Rooz Ghabl: {} ({})",
            past_date,
            past_date.day_of_week()
        );
    }
    println!("==================================================");
}
