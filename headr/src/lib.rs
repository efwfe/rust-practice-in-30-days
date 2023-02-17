use clap::{App, Arg};
use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::fs::File;


#[derive(Debug)]
pub struct Config{
    files:Vec<String>,
    lines:usize,
    bytes:Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args()->MyResult<Config>{
    let matches = App::new("headr")
                .version("0.1.0")
                .author("Daihui@qq.com")
                .about("Rust head")
                .arg(Arg::with_name("files")
                    .short("f")
                    .value_name("FILE")
                    .help("Input file(s)")
                    .multiple(true)
                    .default_value("-")
                )
                .arg(Arg::with_name("lines")
                    .short("n")
                    .long("lines")
                    .help("Number of lines")
                    .default_value("10")
                )
                .arg(Arg::with_name("bytes")
                    .short("c")
                    .takes_value(true)
                    .value_name("BYTES")
                    .conflicts_with("lines")
                    .long("bytes")
                )
                .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .unwrap()?;
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("{}", e))?;

    Ok(Config { files, lines, bytes})
}

fn open(filename: &str)->MyResult<Box<dyn BufRead>>{
    match filename{
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn run(config: Config)->MyResult<()>{
    let num_files = config.files.len();

    for file in config.files{
        match open(&file){
            Err(err) => eprintln!("{}: {}", file, err),
            Ok(mut f) => {
                if num_files > 0{
                    println!("{}==> {} <==",
                        if num_files > 0 {"\n"} else {""},
                        file
                    );

                }
                // for line in f.lines().take(config.lines){
                //     println!("{}", line?);
                // }
                // read bytes
                if let Some(num_bytes) = config.bytes{
                    // way 1
                    // let mut handle = f.take(num_bytes as u64);
                    // let mut buffer = vec![0; num_bytes];
                    // let bytes_read = handle.read(&mut buffer)?;
                    // print!("{}", String::from_utf8_lossy(&buffer[..bytes_read])); // convert bytes into string, may not valid utf-8

                    // way 2
                    let bytes: Result<Vec<_>, _> = f.bytes().take(num_bytes).collect();
                    println!("{}", String::from_utf8_lossy(&bytes?));

                }
                else
                {
                    let mut line = String::new();
                    for _ in 0..config.lines{
                        let bytes = f.read_line(&mut line)?;
                        if bytes == 0{
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }


            },
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize>{
    match val.parse(){
        Ok(n) if n>0 => Ok(n),
        _ => Err(From::from(val)),
    }
}
#[test]
fn test_parse_positive_int(){
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());

}