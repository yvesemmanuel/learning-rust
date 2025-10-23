fn main() {
    print_labeled_measurement(5, 'h');
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(x);

    println!("The value of x is: {x}");
}