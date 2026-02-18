# 🦁 تقویم شاهنشاهی ایران (Imperial Calendar) ☀️
**جامع‌ترین موتور محاسبات تقویم اصیل ایرانی؛ طراحی شده برای پایداری، سرعت و پاسداشت هویت ملی در تمامی پلتفرم‌ها.**

[![Crates.io](https://img.shields.io/crates/v/imperial-cal.svg)](https://crates.io/crates/imperial-cal)
[![PyPI](https://img.shields.io/pypi/v/imperial-cal.svg)](https://pypi.org/project/imperial-cal/)

## 🏛 آرمان و فلسفه
این پروژه با هدف بازگرداندن دقت و اصالت به گاه‌شماری ایرانی طراحی شده است. تقویم شاهنشاهی نه تنها یک ابزار زمان‌سنجی، بلکه نمادی از پیوستگی فرهنگی ما از کوروش بزرگ تا عصر مدرن است. در این کتابخانه، تمامی مناسبت‌های غیرملی حذف شده و تمرکز بر جشن‌های باستانی و رویدادهای تاریخی ایران‌زمین است.

### تمایزات کلیدی:
* **واژگان اصیل:** استفاده از **«امرداد»** (بی‌مرگی) و **«آدینه»** (نام باستانی پایان هفته).
* **مناسبت‌های ملی:** زنده نگه داشتن رویدادهایی مانند **جشن سده**، **مهرگان**، **روز نجات آذربایجان** و **زادروز رضاشاه کبیر**.
* **پشتیبانی همه‌جانبه:** یک هسته واحد برای استفاده در موبایل، وب، دسکتاپ و سرور.

---

## ✨ قابلیت‌های برجسته
1. **تبدیل سه‌گانه:** تبدیل بی‌دردسر بین میلادی، هجری شمسی و شاهنشاهی.
2. **محاسبات زمانی (Date Math):** قابلیت افزودن یا کاستن روزها با دقت ۱۰۰٪ در محاسبه کبیسه‌ها.
3. **تشخیص روز هفته:** نمایش نام روزهای هفته به زبان پارسی اصیل.
4. **رویدادنگار:** دیتابیس داخلی از مهم‌ترین جشن‌ها و وقایع تاریخی ایران.

---

## 🛠 راهنمای نصب و استفاده (پلتفرم‌ها)

### ۱. پایتون (Python) 🐍
نصب از طریق PyPI:
```bash
pip install imperial-cal
نمونه کد:

Python
import imperial_cal

# دریافت تاریخ امروز و رویدادها
today = imperial_cal.ShahanshahiDate.today()
print(f"امروز: {today} ({today.day_of_week()})")
print(f"مناسبت‌ها: {today.events()}")

# محاسبات: ۱۰۰ روز بعد از نوروز ۲۵۸۵
nowruz = imperial_cal.ShahanshahiDate.from_gregorian(2026, 3, 21)
future = nowruz.add_days(100)
۲. راست (Rust) 🦀
افزودن به پروژه:

Bash
cargo add imperial-cal
نمونه کد:

Rust
use imperial_cal::ShahanshahiDate;

fn main() {
    let date = ShahanshahiDate::from_jalali(1403, 1, 1); // تبدیل از شمسی
    println!("تاریخ شاهنشاهی: {}", date);
    println!("روز هفته: {}", date.day_of_week()); // آدینه، شنبه...
}
۳. اندروید (Android / Kotlin) 🤖
این کتابخانه از طریق لایه JNI یا UniFFI در اندروید قابل استفاده است.
نمونه کد در Kotlin:

Kotlin
import org.imperial_cal.ShahanshahiDate

val today = ShahanshahiDate.today()
val (year, month, day) = today.toJalali() // تبدیل به شمسی برای نمایش
val weekday = today.dayOfWeek()
۴. آی‌او‌اس (iOS / Swift) 🍏
هسته Rust به صورت یک XCFramework کامپایل شده و در Xcode قابل فراخوانی است.
نمونه کد در Swift:

Swift
import ImperialCal

let date = ShahanshahiDate.today()
print("امروز در تقویم شاهنشاهی: \(date.description)")
if let nextWeek = date.addDays(days: 7) {
    print("هفته آینده: \(nextWeek.description)")
}
۵. وب (Web / JS / WASM) 🌐
با استفاده از WebAssembly، می‌توانید این کتابخانه را در مرورگر (React, Vue, Node.js) اجرا کنید.
نمونه کد در JavaScript:

JavaScript
import init, { ShahanshahiDate } from './imperial_cal_wasm.js';

async fn run() {
    await init();
    const today = ShahanshahiDate.today();
    console.log(today.get_month_name()); // امرداد، شهریور...
}
📅 بخشی از مناسبت‌های موجود در دیتابیس
۱ فروردین: جشن نوروز (آغاز سال نو)

۶ فروردین: زادروز اشو زرتشت

۱۴ مرداد: عید مشروطیت

۱۶ مهر: جشن مهرگان

۴ آبان: زادروز محمدرضا شاه پهلوی (آریامهر)

۲۱ آذر: روز نجات آذربایجان

۲۵ آذر: روز مادر

۱۷ دی: روز آزادی بانوان (کشف حجاب)

۱ بهمن: جشن سده

۲۴ اسفند: زادروز رضاشاه کبیر (روز پدر)

💻 خط فرمان (CLI)
برای استفاده مستقیم در ترمینال:

Bash
cargo install imperial-cal --bin shc

shc today        # نمایش تاریخ امروز و مناسبت‌ها
shc convert 2026 3 21  # تبدیل میلادی به شاهنشاهی
🤝 مشارکت
این پروژه متعلق به تمامی ایرانیانی است که دغدغه حفظ هویت ملی خود را دارند. ما از Pull Request‌های شما برای موارد زیر استقبال می‌کنیم:

افزودن مناسبت‌های تاریخی مستند.

بهبود Bindingها برای زبان‌های Swift و Kotlin.

بهینه‌سازی کدهای WebAssembly.

📜 لایسنس
این پروژه تحت لایسنس MIT منتشر شده است و استفاده از آن در پروژه‌های تجاری و شخصی کاملاً آزاد است.

پاینده ایران 🦁☀️
