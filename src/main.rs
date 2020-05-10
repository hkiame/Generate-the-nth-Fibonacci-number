use std::io;
fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed o read line.");

    let num: i64 = match num.trim().parse(){
        Ok(num) => num,
        Err(_) => 0
    };

    println!("{}", nth_fib(num));
    
}

fn nth_fib(value: i64) -> i64{
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    let mut c: i64;
    
    if value == 0 {
        return a;
    }

    for _number in 2..=value {
        c = a + b;
        a = b;
        b = c;
    }

    return b;

}
