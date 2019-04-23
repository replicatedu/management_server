use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{thread, time};
extern crate rand;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

mod command;
const MAX_USERS: i32 = 10;
const TIMEOUT: i32 = 30*60;

//fail safe to preent against some bad states
fn docker_reaper(counter: Arc<Mutex<i32>>){
    let thirty_seconds = time::Duration::from_secs(2);
    loop{
        {
            let mut counter_inc = counter.lock().unwrap();
            if *counter_inc == 0{
                dbg!("REAPING");
            }
        }
        thread::sleep(thirty_seconds);
    }
}


fn handle_client(mut stream: TcpStream, port: u32, counter: Arc<Mutex<i32>>) {
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let rand_path: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();


    let ip: ::std::net::IpAddr = my_public_ip::resolve().unwrap();

    println!("{}", &rand_string);
    dbg!(command::get_output(&rand_string,&rand_path));
    let thirty_seconds = time::Duration::from_secs(1);
    let mut data = [0 as u8; 25]; // using 50 byte buffer
    {
        let mut counter_inc = counter.lock().unwrap();
        *counter_inc += 1;
    }
    let mut timer = TIMEOUT;
    while timer > 0 && match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            match stream.write(&data[0..size]) {
                Ok(_) => true,
                Err(_) => {
                    println!("client disconnected");
                    {
                        let mut counter_inc = counter.lock().unwrap();
                        *counter_inc -= 1;
                        false
                    }
                }
            }
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            {
                let mut counter_inc = counter.lock().unwrap();
                *counter_inc -= 1;
            }
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        {
            let mut counter_inc = counter.lock().unwrap();
            let mut status: String = "".to_owned();
            status.push_str(&format!(
                "{}:{} password {} ,current sessions {}/{}: seconds left: {}",
                &ip,
                port,
                rand_string,
                (*counter_inc).to_string(),
                MAX_USERS,
                timer
            ));
            stream.write(status.as_bytes());
        }
        thread::sleep(thirty_seconds);
        timer-=1;
    }
    stream.write(b"timeout!");
}

fn handle_error(mut stream: TcpStream, counter: Arc<Mutex<i32>>) {
    let thirty_seconds = time::Duration::from_secs(1);
    let mut data = [0 as u8; 50]; // using 50 byte buffer

    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            match stream.write(&data[0..size]) {
                Ok(_) => true,
                Err(_) => {
                    println!("client disconnected");
                    false
                }
            }
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        {
            let mut counter_inc = counter.lock().unwrap();
            let mut status: String = "".to_owned();
            status.push_str(&format!(
                "{} connected...cannot create new session, try again when this counter is below {}",
                (*counter_inc).to_string(),MAX_USERS
            ));
            stream.write(status.as_bytes());
        }
        thread::sleep(thirty_seconds);
    }
}

fn main() {

    // List all of the machine's network interfaces


    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let counter = Arc::new(Mutex::new(0));
    let counter4 = Arc::clone(&counter);
     thread::spawn(move || {
                            // connection succeeded
                            docker_reaper(counter4)
                        });
    let port_start = 8000;
    let mut port_add = 0;
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let counter = Arc::clone(&counter);
                let counter2 = Arc::clone(&counter);
                let counter3 = Arc::clone(&counter);

                println!("New connection: {}", stream.peer_addr().unwrap());

                {
                    let counter_d = counter.lock().expect("error getting mutex");
                    if (*counter_d >= MAX_USERS) {
                        thread::spawn(move || {
                            // connection succeeded
                            handle_error(stream, counter3);
                        });
                    } else {
                        port_add = ((port_add + 1) % 30);
                        thread::spawn(move || {
                            // connection succeeded
                            handle_client(stream, (port_add + port_start), counter2)
                        });
                    }
                }
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
