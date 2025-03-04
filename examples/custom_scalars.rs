use shrewnit::{Dimension, Feet, Inches, Length, ScalarExt, Seconds};

fn print_in_inches(distance: Length<i32>) {
    println!("{} inches", distance.to::<Inches>());
}

fn main() {
    // Create a new measure with a scalar type of i32.
    let distance = 300i32.feet();

    // Unit math works with measures of the same scalar type.
    // Mixing scalar types will result in a compile error.
    let velocity = distance / (3.seconds());

    println!("{:?}", velocity);
    print_in_inches(distance);
}
