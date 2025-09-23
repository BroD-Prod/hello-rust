use std::{io::stdin, process::Command};

fn main(){
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    let command = s.trim();

    Command::new(command).spawn().unwrap();
}