# Iranian Shahanshahi Calendar (Rust)

A Rust library and CLI tool for working with the **Shahanshahi (Imperial) Iranian calendar**, built on top of accurate Gregorian ↔ Jalali conversion.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Events Database](#events-database)
- [Contributing](#contributing)
- [Testing](#testing)
- [Roadmap](#roadmap)
- [License](#license)
- [Vision](#vision)

## Introduction

This project aims to provide a clean, open, and extensible implementation of the Shahanshahi calendar for apps, tools, and research.

## Features

- **Core calendar engine**
    - Gregorian → Jalali → Shahanshahi conversion  
    - Correct Jalali leap year algorithm  
    - Days-in-month calculation  
    - No year zero  
    - Format: `YYYY/MM/DD` (e.g., `2584/01/01`)

- **Month names**
    - Farvardin
    - Ordibehesht
    - Khordad
    - Tir
    - Amordad
    - Shahrivar
    - Mehr
    - Aban
    - Azar
    - Dey
    - Bahman
    - Esfand

- **Events system**
    - Events stored in `data/events.json`
    - Supports multiple events per day
    - Easy to extend via pull requests
    - Persian text supported

- **CLI tool**
    - Show today in Shahanshahi
    - Convert dates
    - Show events for a date


# 📦 Installation

## Clone
## Installation
To install the Iranian Shahanshahi Calendar, follow these steps:

1. **Clone the repository:**
    ```bash
    git clone https://github.com/Ehsan-Taheri/iranian-shahanshahi-calendar.git
    cd iranian-shahanshahi-calendar
    ```

2. **Build the project:**
    ```bash
    cargo build --release
    ```

3. **Run the CLI tool:**
    To show today's date in Shahanshahi:
    ```bash
    cargo run -- today
    ```
    ## Usage
    To use the Iranian Shahanshahi Calendar library in your project, follow these steps:

    1. **Add the dependency** to your `Cargo.toml`:
        ```toml
        [dependencies]
        shahanshahi-core = { path = "..." }
        ```

    2. **Example usage** in your Rust code:
        ```rust
        use shahanshahi_core::{ShahanshahiDate, month_name};

        fn main() {
            let d = ShahanshahiDate::today();
            println!("{}", d);
            println!("Month name: {}", month_name(d.month));
            for e in d.events() {
                println!("Event: {}", e);
            }
        }
        ```
    To convert a Gregorian date to Shahanshahi:
    ```bash
    cargo run -- convert <year> <month> <day>
    ```
```bash
git clone https://github.com/Ehsan-Taheri/iranian-shahanshahi-calendar.git
cd iranian-shahanshahi-calendar
Build
cargo build --release
🚀 CLI Usage
Show today
cargo run -- today
Example output:

Today: 2584/01/01
Month: Farvardin
Events:
- جشن نوروز
Convert Gregorian → Shahanshahi
cargo run -- convert 2025 3 21
Output:

2584/01/01
🧠 Library Usage
Add to your Cargo.toml:

shahanshahi-core = { path = "..." }
Example:

use shahanshahi_core::{ShahanshahiDate, month_name};

fn main() {
    let d = ShahanshahiDate::today();

    println!("{}", d);
    println!("Month name: {}", month_name(d.month));

    for e in d.events() {
        println!("Event: {}", e);
    }
}
📅 Events Database
Events live in:

data/events.json
Format:

[
  { "month": 1, "day": 1, "name": "جشن نوروز" },
  { "month": 7, "day": 16, "name": "جشن مهرگان" }
]
Rules:

Month: 1–12

Day: valid day of month

UTF-8 Persian text allowed

🤝 Contributing
Contributions are welcome!

Adding events
Edit data/events.json

Add your event

Open a Pull Request

Please ensure:

Correct date

Historically accurate naming

Proper Persian spelling

Code contributions
Follow Rust style guidelines

Keep functions small and testable

Add tests for new logic

🧪 Testing
Run:

cargo test
All calendar math is covered by unit tests.

🗺️ Roadmap
WASM build for web usage

Android/iOS bindings

Localization system

iCalendar (.ics) export

Optional GUI app

📜 License
MIT License

❤️ Vision
This project is an open effort to preserve and make accessible the Shahanshahi calendar in modern software systems.


