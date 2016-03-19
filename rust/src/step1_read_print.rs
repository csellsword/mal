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

fn read(s: String) -> Result<MalType, MalErr> {
    read_str(s.as_str())
}

fn eval(s: Result<MalType, MalErr>) -> Result<MalType, MalErr> {
    s
}

fn print(s: Result<MalType, MalErr>){
    let pretty_print = match s {
			Ok(good) => pr_str(&good),
			Err(nogood) => match nogood {
				MalErr::ParseErr(msg) => msg,
				MalErr::Comment => return,
			},
		};
    println!("{}",&pretty_print);
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
