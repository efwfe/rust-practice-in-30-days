use std::env;
use mingrep::{Config, run};
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments:{}", err);
        process::exit(1);
    });
    
   if let Err(e) = run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
   }
   // 这个地方的unwrap_or_else和if let 起到一样的作用
}

