
#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

#![allow(unused_imports)]

extern crate rocket;


#[macro_use]
extern crate tera;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate chrono;
extern crate xml;


mod utils;
mod biz;
mod views;

use std::collections::HashMap;

use std::io;
use std::thread;

fn run() ->io::Result<()> {

    let config = utils::Configuration::load()?;
    let mut ctx = biz::BizContext::new(config.clone());

    println!("Config {:?}", config);

    let mut workdays = HashMap::new();
    utils::parse_workdays(&mut workdays)?;
    println!("workdays {:?}", workdays);
    println!("Ready, go");


    loop {
        let (date, _) = utils::get_today_date_time();
        if let Some(v) = workdays.get(&date.to_string()) {
            if *v {
                thread::sleep_ms(1000);
                ctx.check_sh_market()?;

                for p in &config._monitor_processes {
                    utils::check_process(&p)?;
                }

            } else {
                thread::sleep_ms(1000 * 3600);
            }
        }

    }

    //Ok(())
}

fn main() {

    thread::spawn(move ||{
        loop {
            views::server();
        }
    });

    loop {
        if let Err(e) = run() {
            println!("Run failed: {:?}", e);

            use std::thread;
            thread::sleep_ms(1000 * 300);
        }
    }
}
