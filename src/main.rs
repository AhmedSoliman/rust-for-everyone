use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let shutdown = Arc::new(Mutex::new(false));

    let t1 = std::thread::Builder::new()
        .name("listener".to_owned())
        .spawn({
            let shutdown = Arc::clone(&shutdown);
            move || {
                println!("Starting server...");
                let listener = TcpListener::bind("0.0.0.0:9999").unwrap();
                println!("Starting listener");

                let mut i = 0;
                while !*shutdown.lock().unwrap() {
                    i += 1;
                    let (stream, socket) = listener.accept().unwrap();
                    println!("Received connection...from {:?}", socket);
                    handle_connection(i, stream, socket);
                }
            }
        })
        .unwrap();

    let t2 = std::thread::spawn({
        let shutdown = Arc::clone(&shutdown);
        move || {
            while !*shutdown.lock().unwrap() {
                println!("Hello from the second thread");
                std::thread::sleep(Duration::from_secs(2));
            }
        }
    });

    let stream = t1.join().expect("thread is supposed to finish");
    println!("Thread finished");
    // set shutdown....
    {
        let mut guard = shutdown.lock().unwrap();
        *guard = true;
        // lock dropped.
    }
    println!("Shutdown initiated");

    loop {
        println!("Should have stopped....");
        std::thread::sleep(Duration::from_secs(5));
    }
}

fn handle_connection(i: usize, mut stream: TcpStream, addr: SocketAddr) -> JoinHandle<()> {
    std::thread::Builder::new()
        .name(format!("t{}", i))
        .spawn(move || {
            println!("Handling connection....{:?}", addr);
            loop {
                stream.write(b"Hello from the server\n").unwrap();
                std::thread::sleep(Duration::from_secs(1));
            }
        })
        .unwrap()
}
