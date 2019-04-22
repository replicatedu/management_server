
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::{thread, time};
use std::sync::{Mutex, Arc};


fn handle_client(mut stream: TcpStream,counter:Arc<Mutex<i32>>) {

    let thirty_seconds = time::Duration::from_secs(1);
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    {
        let mut counter_inc = counter.lock().unwrap();
        *counter_inc+=1;
    }

    loop{
        while match stream.read(&mut data) {
            Ok(size) => {
                // echo everything!
                stream.write(&data[0..size]).unwrap();
                true
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {
            {
                let mut counter_inc = counter.lock().unwrap();

                stream.write((*counter_inc).to_string().as_bytes());
            }
            thread::sleep(thirty_seconds);
        }

    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let counter = Arc::new(Mutex::new(0));
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let counter = Arc::clone(&counter);
                println!("New connection: {}", stream.peer_addr().unwrap());

                {
                    let counter_d = counter.lock().expect("error getting mutex");
                    dbg!(*counter_d);
                }
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream,counter)
                });

            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}