extern crate readline;
extern crate libc;
extern crate regex;

mod types;
mod reader;
mod printer;

use types::*;
use reader::*;
use printer::*;

use std::{ffi};
use readline as rl;

fn read(s: String) -> Result<MalType, String> {
    read_str(s.as_str())
}

fn eval(s: Result<MalType, String>) -> Result<MalType, String> {
    s
}

fn print(s: Result<MalType, String>) -> String {
    //let pretty_print = pr_str(&s);
    let pretty_print = match s {
			Ok(good) => pr_str(&good),
			Err(nogood) => nogood,
		};
    println!("{}",&pretty_print);
    pretty_print
}

fn rep(s: String){
   print(eval(read(s)));
}

fn main(){
    let prompt: ffi::CString = ffi::CString::new("user> ").unwrap();
    loop {
        match rl::readline(&prompt) {
            Ok(s) => {
                let input = (*s).to_str().unwrap().to_string();
                rep(input);
            },
            _ => {
                break;
            },
        }
    }
}
