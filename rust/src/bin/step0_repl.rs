extern crate readline;
extern crate libc;

use std::{ffi};

use readline as rl;

fn read(s: String) -> String {
    s
}

fn eval(s: String) -> String {
    s
}

fn print(s: String) -> String {
    println!("{}", &s);
    s
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
