
#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

#![allow(unused_imports)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate tera;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate xml;


mod utils;
mod biz;
mod views;

use std::collections::HashMap;

use std::io;
use std::thread;

use std::sync::RwLock;
use std::sync::atomic::{ AtomicPtr, Ordering };
use std::sync::Arc;

#[derive(Clone)]
pub struct Alarm {
    pub _id : u32,
    pub _source : Option<String>,
    pub _target : Option<String>,
    pub _time : Option<chrono::DateTime<chrono::Local>>,

    pub _description : Option<String>,
}

impl Alarm {

    pub fn new()->Alarm {
        Alarm {
            _id : 0,
            _source : None,
            _target : None,
            _time : None,
            _description : None,
        }
    }

    pub fn from(id:u32, src:Option<String>,
                tat:Option<String>,
                t:Option<chrono::DateTime<chrono::Local>>,
                dsp:Option<String>) ->Alarm {
        Alarm {
            _id : id,
            _source : src,
            _target : tat,
            _time : t,
            _description : dsp,
        }
    }
}

pub struct AlarmManager {

    _alarms : RwLock<HashMap<u32, Arc<RwLock<Vec<String>>>>>,

    _current_date : RwLock<u32>,

    _active_alarms : RwLock<HashMap<u32, Alarm>>,
}

impl AlarmManager {

    pub fn new()->AlarmManager {

        let alarms = HashMap::new();

        AlarmManager {
            _alarms : RwLock::new(alarms),
            _current_date : RwLock::new(0),
            _active_alarms : Default::default(),
        }
    }

    pub fn get_today(&self)->u32 {

        if let Ok(x) = self._current_date.read(){
            return *x;
        }

        0
    }

    pub fn set_today(&self, t : u32) {

        if let Ok(mut x) = self._current_date.write() {
            *x = t;
        }

    }

    pub fn active_alarm(&self, alarm : Alarm) {

    }

    pub fn disable_alarm(&self, no : u32) {

    }

    pub fn add_log(&self, log : String) -> bool {

        let (date, _) = utils::get_today_date_time();
        let today = self.get_today();

        if today != date {
            self.set_today(date);
        }
        let today2 = self.get_today();

        let mut rr = self._alarms.write();

        if let Ok(ref mut r) = rr {
            let value = r
                .entry(date)
                .or_insert(Arc::new(RwLock::new(vec![])));

            if let Ok(ref mut v) = value.write() {
                v.push(log);

                return true;
            }
        }

        false
    }

    pub fn get_by_date(&self, date : u32)->Option<Arc<RwLock<Vec<String>>>> {

        if let Ok(hm) = self._alarms.read() {
            let ov = hm.get(&date);

            if let Some(v) = ov {

                return Some(v.clone());
            }
        }

        None
    }

    pub fn get_all_dates(&self)->Vec<u32> {

        let mut v = vec![];

        let rr = self._alarms.read();

        if let Ok(ref r) = rr {

            for key in r.keys() {
                v.push(*key);
            }
        }

        v
    }
}

lazy_static! {
    pub static ref ALARM_MANAGER:AlarmManager = {
        AlarmManager::new()
    };
}

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
