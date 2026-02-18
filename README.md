# 🦁 تقویم شاهنشاهی ایران (Imperial Calendar) ☀️
**جامع‌ترین موتور محاسبات تقویم اصیل ایرانی؛ طراحی شده برای پایداری، سرعت و پاسداشت هویت ملی در تمامی پلتفرم‌ها.**

[![Crates.io](https://img.shields.io/crates/v/imperial-cal.svg)](https://crates.io/crates/imperial-cal)
[![PyPI](https://img.shields.io/pypi/v/imperial-cal.svg)](https://pypi.org/project/imperial-cal/)

## 🏛 آرمان و فلسفه
این پروژه با هدف بازگرداندن دقت و اصالت به گاه‌شماری ایرانی طراحی شده است. تقویم شاهنشاهی نه تنها یک ابزار زمان‌سنجی، بلکه نمادی از پیوستگی فرهنگی ما از کوروش بزرگ تا عصر مدرن است.

### تمایزات کلیدی:
* **واژگان اصیل:** استفاده از **«امرداد»** (بی‌مرگی) و **«آدینه»** (نام باستانی پایان هفته).
* **مناسبت‌های ملی:** زنده نگه داشتن رویدادهایی مانند **جشن سده**، **مهرگان**، **روز نجات آذربایجان** و **زادروز رضاشاه کبیر**.
* **پشتیبانی همه‌جانبه:** یک هسته واحد برای استفاده در موبایل، وب، دسکتاپ و سرور.

---

## ✨ قابلیت‌های برجسته
1. **تبدیل سه‌گانه:** تبدیل بی‌دردسر بین میلادی، هجری شمسی و شاهنشاهی.
2. **محاسبات زمانی:** قابلیت افزودن یا کاستن روزها با دقت ۱۰۰٪ در محاسبه کبیسه‌ها.
3. **تشخیص روز هفته:** نمایش نام روزهای هفته به زبان پارسی اصیل.
4. **رویدادنگار:** دیتابیس داخلی از مهم‌ترین جشن‌ها و وقایع تاریخی ایران.

---

## 📅 مناسبت‌های موجود در دیتابیس
* ۱ فروردین: جشن نوروز (آغاز سال نو)
* ۶ فروردین: زادروز اشو زرتشت
* ۱۴ مرداد: عید مشروطیت
* ۱۶ مهر: جشن مهرگان
* ۴ آبان: زادروز محمدرضا شاه پهلوی (آریامهر)
* ۲۱ آذر: روز نجات آذربایجان
* ۲۵ آذر: روز مادر
* ۱۷ دی: روز آزادی بانوان (کشف حجاب)
* ۱ بهمن: جشن سده
* ۲۴ اسفند: زادروز رضاشاه کبیر (روز پدر)

---

## 🛠 راهنمای نصب و استفاده

### پایتون (Python)
```bash
pip install imperial-cal
```

```python
import imperial_cal

today = imperial_cal.ShahanshahiDate.today()
print(f"امروز: {today}")
```

### راست (Rust)
```bash
cargo add imperial-cal
```

```rust
use imperial_cal::ShahanshahiDate;

fn main() {
    let date = ShahanshahiDate::from_jalali(1403, 1, 1);
    println!("تاریخ: {}", date);
}
```

### اندروید (Kotlin)
```kotlin
import org.imperial_cal.ShahanshahiDate

val today = ShahanshahiDate.today()
val weekday = today.dayOfWeek()
```

### آی‌او‌اس (Swift)
```swift
import ImperialCal

let date = ShahanshahiDate.today()
print("امروز: \(date)")
```

### وب (JavaScript/WASM)
```javascript
import init, { ShahanshahiDate } from './imperial_cal_wasm.js';

await init();
const today = ShahanshahiDate.today();
```

### خط فرمان (CLI)
```bash
cargo install imperial-cal --bin shc
shc today
```

---

## 🤝 مشارکت
این پروژه متعلق به تمامی ایرانیانی است که دغدغه حفظ هویت ملی خود را دارند. ما از Pull Request‌های شما استقبال می‌کنیم:

* افزودن مناسبت‌های تاریخی مستند
* بهبود Bindingها برای زبان‌های مختلف
* بهینه‌سازی کدها

---

## 📜 لایسنس
این پروژه تحت لایسنس MIT منتشر شده است.

**پاینده ایران 🦁☀️**
