use std::{io::{stdin, stdout, Write}, process::Command};

fn main(){
    loop{
        print!(">");
        let _ = stdout().flush();

        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();

        let mut command_parts = s.trim().split_whitespace();
        let command = command_parts.next().unwrap();
        let args = command_parts;

        match command {
                "exit" => break,
                command => {
                let mut child = Command::new(command).args(args).spawn().unwrap();
                let _ = child.wait();
            }
        }
    }
}