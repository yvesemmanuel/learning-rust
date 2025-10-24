fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM
    let r3 = &s;
    println!("{r1}, {r2}, and {r3}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
