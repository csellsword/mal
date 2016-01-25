extern crate readline;
extern crate libc;
extern crate regex;

mod types;
mod reader;
mod printer;

use std::{ffi};

use readline as rl;

fn read(s: String) -> MalType {
    read_str(s)
}

fn eval(s: MalType) -> MalType {
    s
}

fn print(s: MalType) -> String {
    let pretty_print = pr_str(&s);
    println!("{}", &pretty_print);
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
                println!("readline error");
                break;
            },
        }
    }
}
