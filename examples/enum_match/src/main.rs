#[derive(Debug)]
enum E {
    A, D(i64)
}
use E::*;

fn main() {
    print(A);
    print(D(2));
    print(D(32));
}

fn print(e: E) {
    let s = match e {
        A => "There was A".into(),
        D(i) if i > 5 => {
            format!("Big D: {}", i)
        },
        D(_) => "Not A".into(),
    };

    println!("Hello, {:?}! \t matched: {}", e, s);
}
