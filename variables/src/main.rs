// Use "const" to declare a constant value.

const MAX_POINTS: u32 = 100_000;

fn main() {
    // Rust declares variables as immutable by default
    // let x = 5;

    // To declare a variable as modifiable, add "mut" after "let".
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Type inference. If there is a decimal point, it is declared as a "float" type.
    // let int_i = 3;
    // let float_i = 0.1;

    // Use "_" to inform the compiler that the variable is unused.
    let _int_l = 10;

    // Display the number of bits of the currently used OS
    println!("{}", usize::BITS);

    // Display memory addresses.
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    // Check the stack address.
    let i2: i64 = 2;
    let i3: i64 = 3;

    // Stacked with variables defined in 64-bit, so there can be an 8-byte spacing.
    println!("Stack address of i2 is : {:p}", &i2);
    println!("Stack address of i3 is : {:p}", &i3);

    // Try using Rust shadowing.
    let y = 5;
    println!("Stack address of y is : {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is : {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is : {:p}", &y);
    println!("The value of y is {}", y);

    // Change the scope
    {
        // 表示される数値は"0"
        let y = 0;
        println!("Stack address of y is : {:p}", &y);
        println!("The value of y is {}", y);
    }
    // The number displayed is "12".
    println!("The value of y is {}", y);

    // Try using tuple
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    // tuple in tuple
    let mut t2 = ((0, 1), (2, 3));

    // Change the value of a variable using a pointer
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    // Try to handle arrays
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a2[3]);
}
