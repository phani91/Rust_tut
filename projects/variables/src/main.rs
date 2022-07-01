fn main() {
    println!("Hello, world!");
    let mut x : u128 = 5;
    println!("x = {}",x);
    x = x + 1;

    {
        let x= x*2;
        println!("x= {x}")
    }
    println!("x = {}",x);

    let mut spaces = "   ";
    let sp = spaces.len();
    print!("sp: {sp}");
}
