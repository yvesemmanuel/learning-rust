fn fibonnaci(n: u32) -> u32 {
    if n == 1 { 1 }
    else if n == 2 { 1 }
    else {fibonnaci(n - 1) + fibonnaci(n - 2)}
}

fn main() {
    let n = 10;
    let m = fibonnaci(n);
    println!("The element {n} of the Fibonacci sequence is {m}");
}
