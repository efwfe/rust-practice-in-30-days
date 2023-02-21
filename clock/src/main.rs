use std::mem::zeroed;
use std::os::unix::raw::time_t;
use chrono::{Date, Local, TimeZone};
use chrono::DateTime;
use clap::{App, Arg};


struct Clock;

impl Clock{
    fn get()->DateTime<Local>{
        Local::now()
    }

    #[cfg(not(windows))]
    fn set<Tz:TimeZone>(t: DateTime<Tz>)->(){
        use libc::{timeval, time_t, suseconds_t};
        use libc::{settimeofday, timezone};
        let t= t.with_timezone(&Local);
        let mut u: timeval = unsafe{zeroed()};
        u.tv_sec= t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as * const timeval, mock_tz);
        }
    }
}
// Clock起到一个命名空间的作用，现在添加这个结构体为了以后的代码修改提供一些可拓展性
fn main() {
    let now = Clock::get();

    let app = App::new("clock")
        .version("0.1.1")
        .author("David Zhang")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339")
        )
        .arg(
            Arg::with_name("datetime")
                .help("When <action> is 'set'. apply <datetime> \
                otherwise ignore.")
        );

    let args = app.get_matches();
    let std = args.value_of("std").unwrap();
    let action = args.value_of("action").unwrap();
    if action == "set"{
        let t_ = args.value_of("datetime").unwrap();
        let parser = match std  {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!("Unable to parse{} according to {}",t_, std);
        let t = parser(t_).expect(&err_msg);
        Clock::set(t);
    }

     match std{
        "timestamp"=> println!("{}", now.timestamp()),
        "rfc2822"=>println!("{}",now.to_rfc2822()),
        "rfc3339"=>println!("{}", now.to_rfc3339()),
        _=> unimplemented!(),
    }

}
