use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::str;
use crossterm::{terminal,ClearType};

fn main() {
    let mut terminal = terminal();
    match TcpStream::connect("0.0.0.0:8080") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8080");

            let msg = b"update";
            loop{

                    let mut data = [0 as u8; 256]; // using 6 byte buffer
                    stream.write(msg).unwrap();
                    //println!("Sent Hello, awaiting reply...");
                    match stream.read(&mut data) {

                        Ok(size) => {
                            terminal.clear(ClearType::All);
                            println!("{:}",str::from_utf8(&data[0..size]).expect("error "));
                        },
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}