use clap::{App, Arg};
use std::error::Error;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config{
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

fn parse_positive_int(val: &str)-> MyResult<usize>{
    match val.parse(){
        Ok(n) if n > 0 => Ok(n),
        _ =>Err(From::from(val)),
    }
}
pub fn get_args()->MyResult<Config>{
    let matches = App::new("headr")
            .version("0.1.1")
            .author("daihui.zhang")
            .about("rust head")
            .arg(
                Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
            )
            .arg(
                Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
            )
            .arg(
                Arg::with_name("files")
                    .value_name("FILE")
                    .help("Input file(s)")
                    .multiple(true)
                    .default_value("-"),
            )
            .get_matches();

    let lines = matches
                .value_of("lines")
                .map(parse_positive_int)
                .transpose()
                .map_err(|e| format!("illegal line count -- {}", e))?;

    // Option::map 从Some中解包一个&str然后把结果丢到parse_positive_int中，
    // Option::transpose 将Option<Result>结果转换为Result<Option>
    // files, lines至少有一个值，所以unwrap是安全的
        
    let bytes = matches
                .value_of("bytes")
                .map(parse_positive_int)
                .transpose()
                .map_err(|e| format!("illegal byte count --{}", e))?;
        
    Ok(Config { 
        files: matches.values_of_lossy("files").unwrap(), 
        lines: lines.unwrap(), 
        bytes
    })
}

pub fn run(config: Config)->MyResult<()>{
    println!("{:#?}", config);
    Ok(())
}