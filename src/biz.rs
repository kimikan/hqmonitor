
use std::fs::File;

use std::io;

use std::io::{BufReader, BufRead, Read};
use std::fs::OpenOptions;

trait LineReader<R:Read> {
    fn get_line(&mut self, buf : &mut[u8])->io::Result<usize>;
}

impl <R:Read> LineReader<R> for BufReader<R> {

    //can not use default read_line,
    fn get_line(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut index = 0usize;
        loop {
            let size = self.read(&mut buf[index..index + 1])?;
            if size > 0 {
                if b'\n' == buf[index] {
                    return Ok(index + 1);
                }

                index += 1;
            } else {
                return Ok(index);
            }
        }
    }
}

fn open_file(file : &str)->io::Result<BufReader<File>> {
    let f = OpenOptions::new()
        .read(true).open(file)?;

    let r = BufReader::new(f);

    Ok(r)
}

use std::io::{Error, ErrorKind};

//parse header indicates that, if any needs to be updated
fn process_header(reader: &mut BufReader<File>) -> io::Result<String> {
    let mut str: String = String::new();
    let size = reader.read_line(&mut str)?;
    //println!("{:?}, {:?}", size, str);

    if size > 0 && str.len() > 80 {
        //let file_len = &str[16..26];
        //let file_len = &str[16..26];
        let time = &str[49..70];
        //let flags = &str[73..81];
        //let flags = str.as_bytes();
        //println!("flags: {:?}", flags);
        let new_time = time.to_owned();

        /*
        println!("txt header: time={:?}, file_len:{:?}"
                 , new_time, file_len);
        */

        return Ok(new_time);
    }

    Err(Error::from(ErrorKind::InvalidData))
}

//bool, true = successfully handled & has changed stocks
//false means,   no changes, no errors
fn process_file(file: &str) -> io::Result<String> {
    let mut r = open_file(file)?;

    let t = process_header(&mut r)?;

    Ok(t)
}

use utils;
pub struct BizContext {

    pub _txt_time : u32,
    pub _txt_adjust : i32,

    pub _dbf_time : u32,
    pub _dbf_adjust : i32,

    pub _config : utils::Configuration,
}

use std::num::ParseIntError;
use std::result::Result;

impl BizContext {

    pub fn new(config : utils::Configuration)->BizContext {

        BizContext {
            _dbf_time : 0u32,
            _txt_time : 0u32,
            _config : config,
            _txt_adjust : 0i32,
            _dbf_adjust : 0i32,
        }
    }


    fn parse_time(&self, ts:&Vec<&str>)->Result<u32, ParseIntError> {

        let h = ts[0].parse::<u32>()?;
        let m = ts[1].parse::<u32>()?;
        let s = ts[2].parse::<u32>()?;
        Ok(h * 10000 + m * 100 + s)
    }

    fn report_alarm(&mut self, t : u32) {
        let s = format!("Shanghai stopped: market_time: {}, ", t);

        let alarm = ::Alarm {
            _id : 2000,
            _source : "HQ MONITOR".to_string(),
            _target : "SH_TXT_FILE".to_string(),
            _description : s,
            _time : Default::default(),
            _env : self._config._env.clone(),
        };

        utils::report_alarm(alarm);
    }

    fn check_time(&mut self, time:u32)->io::Result<()> {

        let now = utils::get_current_time();

        let s_now = utils::get_seconds(now);
        let s_market = utils::get_seconds(time);
        let s_last = utils::get_seconds(self._txt_time);

        if self._config.is_trade_time(time) {

            if self._txt_adjust == 0 {
                //only adjust in normal time
                let diff = s_market - s_last;
                if s_last != 0 && diff <= 10 && diff >= 1 {
                    self._txt_adjust = s_now - s_market;
                    println!("Ajust shanghai market time: {}", self._txt_adjust);
                }
            }

            if self._config.is_trade_time(now) {
                if s_now - (s_market + self._txt_adjust) >= self._config._txt_interval {
                    //println!("11 {}, {}, {} {} {} {}", now, time, self._txt_adjust, self._config._txt_interval, s_now, s_market);

                    self.report_alarm(time);
                } else if (s_market + self._txt_adjust) - s_now >= self._config._txt_interval {
                    //println!("22 {}, {}, {}", now, time, self._txt_adjust);

                    self.report_alarm(time);
                }
            }
            self._txt_time = time;
        }

        Ok(())
    }

    pub fn check_txt(&mut self, file : &str)->io::Result<()> {

        let mut t = process_file(file)?;

        //20180124-15:30:05.700

        if t.len() >= 17 {
            let t_str = t.split_off(9);

            let ts:Vec<&str> = t_str.split(".").collect();
            let time_str = ts[0].to_string();
            time_str.split_at(8);
            let ts2:Vec<&str> = time_str.split(":").collect();

            if ts2.len() == 3 {
                let time_o = self.parse_time(&ts2);

                match time_o {
                    //return Ok(());
                    Ok(t)=>{
                        self.check_time(t)?;
                    },
                    Err(e)=>{
                        println!("{:?}", e);
                    }
                };
            }
        }

        Ok(())
    }

    pub fn check_sh_market(&mut self)->io::Result<()> {
        let txt = self._config._txt_file.clone();
        self.check_txt(&txt)
    }

    pub fn check_dbf(file : &str)->io::Result<()> {

        Ok(())
    }

}

