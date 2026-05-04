mod two_nums;
use two_nums::two_sum;

fn main() {
    println!("Hello, world!");
    println!("How are you?");

    let x: u32 = 10;
    println!("The value of x is: {}", x);
    eprintln!("{} days", 31);
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1); // 00001
    println!("{number:0<5}", number=1); // 10000
    println!("{number:0>width$}", number=1, width=5);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let x = vec![1,2,3,4,5];
    let target = 5;
    let y = two_sum(x,target);
    println!("Two sums at : {}, {}, {}", y[0], y[1], target);

}

