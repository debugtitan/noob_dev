fn main() {
    let mut   x = 5;// can  be reassigned
    let  _x = 5; //can not be reassigned

    // constants are immutable and can't be made mutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("{}",THREE_HOURS_IN_SECONDS);

    //shadowing: redeclare existing variable with let
    let x = x * 2;
    println!("{}",x);
}