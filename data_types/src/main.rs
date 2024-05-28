fn main() {
    /*Scalar Types
    A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.  */
    //u for int
    //f for floats
    //bool for booleans
    let _guess: u32 = "42".parse().expect("Not a number!");
    const SUM_DIGIT: u32 = 5;
    let num1: u32 = 5;
    let num3: u32 = 2;
    let mut result = num1 * SUM_DIGIT;
    result = result * num3;
    println!("{}", result);
    //understood mut and use of int types
    let is_exact: bool = result == 50;
    println!("{}", is_exact);
    //characters
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    /*Compound Types
    Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays. */

    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let xy: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = xy.0;

    let _six_point_four = xy.1;

    let _one = xy.2;

    //ARRAYS: every element of an array must have the same type----- seems would go on with tuple
    // canset the type and length of the array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
}
