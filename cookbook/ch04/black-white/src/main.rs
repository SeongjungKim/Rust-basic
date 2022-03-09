use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

///
/// A simple enum with only two variations: black and white
///
#[derive(Debug)]
enum Shade {
    Black,
    White,
}

fn new_painter_thread(data: Arc<Mutex<Vec<Shade>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        {
            let mut d = data.lock().unwrap();
            if d.len() > 5 {
                break;
            }

            if d.len() > 0 {
                match d[d.len() - 1] {
                    Shade::Black => d.push(Shade::White),
                    Shade::White => d.push(Shade::Black),
                }
            } else {
                d.push(Shade::Black)
            }
        }

        thread::sleep(Duration::from_secs(1));
        println!("...")
    })
}

fn main() {
    let data = Arc::new(Mutex::new(vec![]));
    let threads: Vec<thread::JoinHandle<()>> = 
        (0..2)
            .map(|_| new_painter_thread(data.clone()))
            .collect();
        
    let _: Vec<()> = threads
                        .into_iter()
                        .map(|t| t.join().unwrap())
                        .collect();
    println!("Result: {:?}", data);
}
