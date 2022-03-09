use std::thread;
use std::time::Duration;

fn start_no_shared_data_thread() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Waiting for three seconds.");
        thread::sleep(Duration::from_secs(3));
        println!("Done")
    })
}

fn start_shared_data_thread(a_number: i32, a_vec: Vec<i32>) -> thread::JoinHandle<Vec<i32>> {
    thread::spawn(move || {
        print!("    a_vec ---> [");
        for i in a_vec.iter() {
            print!(" {} ", i);
        }
        println!("]");
        println!("   A number from inside the thread: {}", a_number);
        a_vec
    })
}

fn main() {
    let no_move_thread = start_no_shared_data_thread();
    for _ in 0..10 {
        print!(":");
    }
    println!("Waiting for the thread to finish ... {:?}", no_move_thread.join());

    let a_number = 42;
    let a_vec = vec![1, 2, 3, 4, 5];
    let move_thread = start_shared_data_thread(a_number, a_vec);
    println!("We can still use a Copy-enabled type: {}", a_number);
    println!("Waiting for the thread to finish ... {:?}", move_thread.join());
}
