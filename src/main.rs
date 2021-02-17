use serde_json; use serde::{Serialize, Deserialize}; use std::collections::HashMap; use std::fs; use std::path::Path; use std::process; 
use std::net::UdpSocket; use std::time::{Instant, Duration};
// Define device and config structs for serde
#[derive(Serialize, Deserialize)]
struct Device { name: String, host: String, port: u16, recv_from: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct Config { host: String, listen_port: u16, web_port: u16, devices: Vec<Device>,
}
fn main() {
    // First, check to see if the target JSON file exists. If it doesn't STOP THE PRESSES.
    if !Path::new("prismaroute.json").exists() { println!("No prismaroute.json detected. The program will now exit."); 
        process::exit(-1);
    }
    // Read the contents, then get the config.
    let contents = fs::read_to_string("prismaroute.json").expect("Error reading file"); let config: Config = 
    serde_json::from_str(contents.as_str()).expect("Invalid JSON");
    // Set up the receiving socket. This socket's job is to read UDP packets in. It has a timeout because We still want the loop to run 
    // after a set amount of time.
    let recv_host = format!("{}:{}", config.host, config.web_port); let recv_sock = UdpSocket::bind(recv_host).expect("Unable to bind to 
    RECV host"); recv_sock.set_read_timeout(Some(Duration::from_millis(500))).expect("You should never see this");
    // Set up the sending socket. This socket's job is to repeat the UDP packets out verbatim, but to specific addresses according to a 
    // set of rules.
    let send_host = format!("{}:{}", config.host, config.listen_port); let send_sock = UdpSocket::bind(send_host).expect("Unable to bind 
    to SEND host");
    // buffer
    let mut buf = [0; 1024];
    // Set up emitters/ dir if it doesn't exist. This will eventually by used for cross-process communication between this and the 
    // frontend. (Which doesn't exist yet.)
    if !Path::new("emitters/").exists() { fs::create_dir("emitters/").expect("Unable to create emitters/ directory");
    }
    // Internal storage of emitters.
    let mut emitters = HashMap::new(); loop {
        // Try to read from the socket. If there's no data to read, then there's no one broadcasting, so remove all emitters, and 
        // reinitialize the hashmap. Then continue to the next iteration.
        let (_amt, src) = match recv_sock.recv_from(&mut buf) { Err(_e) => { emitters = HashMap::new(); for direntry in 
                fs::read_dir("emitters/").unwrap() {
                    let path = direntry.expect("IO Error"); fs::remove_file(path.path()).expect("Could not remove emitter");
                }
                continue;
            },
            Ok((_amt, src)) => (_amt, src),
        };
        // Assuming we're broadcasting, get the source address.
        let srcaddr = src.ip().to_string();
        // Add an entry to the emitters with the IP as the key, and an Instant as the value.
        emitters.entry(srcaddr.clone()).or_insert(Instant::now());
        // If the emitter doesn't exist, make it.
        let path = format!("emitters/{}", srcaddr); if !Path::new(&path).exists() { fs::File::create(path).expect("Could not create 
            emitter file");
        }
        // Loop through all devices to see if any of them have the srcaddr as one of their specified receiving devices. If so, repeat 
        // the data out.
        for device in &config.devices { if device.recv_from.contains(&srcaddr) { let remote = format!("{}:{}", device.host, 
                device.port); send_sock.connect(remote).expect("Connection failed"); send_sock.send(&buf).expect("Couldn't send data");
            }
        }
        // Set temporary emitters var
        let mut new_emitters = HashMap::new();
        // Loop through existing emitters and see if any have passed a predefined timeout. If they haven't, add them, then reset the 
        // emitters hashmap to only those that are still active.
        for (src, time) in emitters { if time.elapsed() > Duration::from_secs(5) { fs::remove_file(format!("emitters/{}", 
                src)).expect("Could not remove emitter file");
            } else {
                new_emitters.entry(src).or_insert(time);
            }
        }
        emitters = new_emitters;
    }
}
