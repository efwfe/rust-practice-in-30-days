
use chrono::Local;
use chrono::DateTime;
use clap::{App, Arg};


struct Clock;

impl Clock{
    fn get()->DateTime<Local>{
        Local::now()
    }

    fn set()->!{
        unimplemented!()
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

     match std{
        "timestamp"=> println!("{}", now.timestamp()),
        "rfc2822"=>println!("{}",now.to_rfc2822()),
        "rfc3339"=>println!("{}", now.to_rfc3339()),
        _=> {  },
    }

}
