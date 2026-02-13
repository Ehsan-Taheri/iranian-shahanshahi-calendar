use clap::{Parser,Subcommand};
use shahanshahi_core::{ShahanshahiDate,month_name};

#[derive(Parser)]
struct Cli{
    #[command(subcommand)]
    command:Commands,
}

#[derive(Subcommand)]
enum Commands{
    Today,
    Convert{
        year:i32,
        month:u32,
        day:u32
    }
}

fn main(){
    let cli=Cli::parse();

    match cli.command{
        Commands::Today=>{
            let d=ShahanshahiDate::today();
            println!("امروز: {}",d);
            println!("نام ماه: {}",month_name(d.month));

            let ev=d.events();
            if ev.is_empty(){
                println!("مناسبتی ندارد");
            }else{
                println!("مناسبت‌ها:");
                for e in ev{
                    println!("- {}",e);
                }
            }
        }

        Commands::Convert{year,month,day}=>{
            let d=ShahanshahiDate::from_gregorian(year,month,day);
            println!("شاهنشاهی: {}",d);
            println!("نام ماه: {}",month_name(d.month));

            for e in d.events(){
                println!("مناسبت: {}",e);
            }
        }
    }
}
