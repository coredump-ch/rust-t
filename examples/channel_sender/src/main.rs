// Start with RUST_BACKTRACE=1 cargo run

extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

extern crate ansi_term;
use ansi_term::Colour::*;

fn main() {

  println!("\n\n\n\n  {}\n\n", Red.bold().paint("running simple_channel():"));
  simple_channel();

  sleep(5);
  println!("\n\n\n\n  {}\n\n", Red.bold().paint("running complex_channel():"));

  complex_channel();
}

fn simple_channel() {
  let pool = ThreadPool::new(4);

  let rx = {
    let (tx, rx) = channel();
    for i in 0..8 {
      let tx = tx.clone();
      pool.execute(move|| {
        if i == 4 {panic!("unexpected panic");} // -- unexpected failure added here --
        tx.send(i).unwrap();
      });
    }
    rx
  };

  // And now this code waits for all the senders to be destructed or the first 8 values:
  assert_eq!(24, rx.iter().take(8).fold(0, |a, b| a + b));
}

fn complex_channel() {
  let pool = ThreadPool::new(4);
  let send_n_commands = 13;
  let now = Instant::now();

  // Map
  let rx = {
    let (tx, rx) = channel::<MyCommands<u64>>();

    for i in 0..send_n_commands {
      let tx = tx.clone();
      pool.execute(move|| {
        sleep(i);

        tx.send( i.into() ).unwrap();
      });
    }
    rx
  };

  // Reduce
  let sum = rx.iter().take(send_n_commands as usize).fold(0, |base, current_element| {
    println!("recv: {:?} after {} seconds", current_element, now.elapsed().as_secs());
    match current_element {
      Add(n) => base + n as i64,
      Sub(n) => base - n as i64,
    }
  });
  assert_eq!(6, sum);
  println!("calculated Sum: {} in {} seconds", sum, now.elapsed().as_secs());
}

fn sleep(s : u64) {
  std::thread::sleep( Duration::from_secs(s) )
}

#[derive(Debug)]
enum MyCommands<T> {
  Add(T),
  Sub(T),
}
use MyCommands::*;


impl Into<MyCommands<u64>> for u64 {
  fn into(self) -> MyCommands<u64> {
    if self % 2 == 0 {
      Add(self)
    } else {
      Sub(self)
    }
  }
}
