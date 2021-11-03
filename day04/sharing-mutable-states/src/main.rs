use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {}

fn mixer(data: Arc<Mutex<Vec<Number>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        println!("Thread {:?}", thread::current().id());

        let mut d = data.lock().unwrap();

        if d.len() > 0 {
            match d[d.len() - 1] {
                Number::Odd => d.push(Number::Even),
                Number::Even => d.push(Number::Odd),
            }
        } else {
            d.push(Number::Odd)
        }
        if d.len() > 10 {
            break;
        }

        thread::sleep(Duration::from_secs(2));
    })
}
enum Number {
    Odd,  //tex
    Even, //Çift
}
