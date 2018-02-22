
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

extern crate hyper;

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

use serde::Serialize;
#[derive(Clone, Serialize)]
pub struct Alarm {
    pub _id : u32,
    pub _source : String,
    pub _target : String,
    pub _time : String,
    pub _env :String,

    pub _description : String,
}

impl Alarm {

    pub fn new()->Alarm {
        Alarm {
            _id : 0,
            _source : Default::default(),
            _target : Default::default(),
            _time : Default::default(),
            _description : Default::default(),
            _env : Default::default(),
        }
    }

    pub fn get_source(&self)->&String {
        &self._source
    }

    pub fn get_target(&self)->&String {
        &self._target
    }

    pub fn get_description(&self)->&String {
        &self._description
    }

    pub fn get_time(&self)->&String {
        &self._time
    }

    pub fn get_env(&self)->&String {
        &self._env
    }

    pub fn as_str(&self)->String {
        let s = self.get_source();
        let t = self.get_target();
        let d = self.get_description();
        let time = self.get_time();
        let env = self.get_env();

        format!("[{:?}]: ID:{}, SOURCE: {}, ENV:{}, TARGET: {}, DETAILS: {}", time, self._id, s, env, t, d)
    }

    /*
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
    } */
}

struct Alarms {
    _alarms : RwLock<Vec<Alarm>>,

    _config : RwLock<Option<utils::Configuration>>,
}

impl Alarms {

    fn new()->Alarms {
        Alarms {
            _alarms : Default::default(),
            _config : Default::default(),
        }
    }

    fn set_config(&self, config : utils::Configuration) {

        if let Ok(mut w) = self._config.write() {
            *w = Some(config);
        }
    }

    fn clear(&self) {
        if let Ok(mut s) = self._alarms.write() {
            s.clear();
        } else {
            println!("Write lock failed");
        }
    }

    fn contains(&self, alarm:&Alarm) ->bool {
        use std::ops::Deref;
        let guard = self._alarms.read().unwrap();
        let alarms = guard.deref();
        for a in alarms {
            if a._id == alarm._id && a._target == alarm._target {
                return true;
            }
        }

        false
    }

    fn active_alarm(&self, alarm : Alarm) {

        if alarm._id <= 0 {
            println!("id???{}", alarm._id);
            return;
        }

        if self.contains(&alarm) {
            return;
        }

        if let Ok(mut s) = self._alarms.write() {
            let s2 = alarm.as_str();

            s.push(alarm);

            use std::ops::Deref;
            if let Ok(c) = self._config.read() {

                if let &Some(ref c2) = c.deref() {
                    utils::send_msg(c2, &s2);
                }
            }

            ::ALARM_MANAGER.add_log(s2);

        } else {
            println!("Write lock failed");
        }
    }

    fn find(&self, id:u32, target:&String)->Option<usize> {

        use std::ops::Deref;
        let guard = self._alarms.read().unwrap();
        let alarms = guard.deref();
        for i in 0.. alarms.len() {

            let x = alarms.get(i);

            let y = x.unwrap();
            let z = y.get_target();

            if target.eq(z) && y._id == id {
                return Some(i);
            }
        }

        None
    }

    fn disable_alarm(&self, id:u32, target:&String) {

        if let Ok(mut s) = self._alarms.write() {

            let idx = self.find(id, target);

            match idx {
                Some(i)=>{
                    s.remove(i);

                    use chrono;
                    let now = chrono::Local::now();
                    let log = format!("[{:?}]: alarm disabled!, ID: {}, TARGET: {}",
                                      now.to_rfc2822(), id, target);
                    ::ALARM_MANAGER.add_log(log);
                },
                None=>{}
            }

        }
    }
}

pub struct AlarmManager {

    _alarms : RwLock<HashMap<u32, Arc<RwLock<Vec<String>>>>>,

    _current_date : RwLock<u32>,

    _active_alarms : Alarms,
    _config : RwLock<Option<utils::Configuration>>,
}

impl AlarmManager {

    pub fn new()->AlarmManager {

        let alarms = HashMap::new();

        AlarmManager {

            _alarms : RwLock::new(alarms),
            _current_date : RwLock::new(0),
            _active_alarms : Alarms::new(),
            _config : Default::default(),
        }
    }

    pub fn set_config(&self, config : utils::Configuration) {

        self._active_alarms.set_config(config.clone());

        if let Ok(mut w) = self._config.write() {
            *w = Some(config);
        }
    }

    pub fn get_today(&self)->u32 {

        if let Ok(x) = self._current_date.read() {
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

        self._active_alarms.active_alarm(alarm);
    }

    pub fn disable_alarm(&self, no : u32, target:&String) {

        self._active_alarms.disable_alarm(no, target);
    }

    pub fn add_log(&self, log : String) -> bool {

        println!("Add log: {}", log);
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

    ::ALARM_MANAGER.set_config(config.clone());
    println!("Config {:?}", config);

    let mut workdays = HashMap::new();
    utils::parse_workdays(&mut workdays)?;
    println!("workdays {:?}", workdays);
    println!("Ready, go");

    let mut before = 0u32;

    loop {
        let (date, time) = utils::get_today_date_time();
        if let Some(v) = workdays.get(&date.to_string()) {
            if *v {
                //clear active alarms in everyday's init time
                if before < 90000 && time >= 90000 {
                    ::ALARM_MANAGER._active_alarms.clear();
                }
                before = time;

                thread::sleep_ms(1000);
                ctx.check_sh_market()?;

                for p in &config._monitor_processes {
                    utils::check_process(&config._env,&p)?;
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
