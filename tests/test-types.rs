use std::io::{self, Read};

fn do_thing(val: Option<Box<i32>>) {
    println!("{}", val.is_some());
}

pub fn main() {
    let boxed_int: Box<i32> = Box::new(999);
    let opt1: Option<Box<i32>> = None;
    let opt2: Option<Box<i32>> = Some(Box::new(123));

    do_thing(opt1);
    do_thing(opt2);

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

}
