use shrewnit::prelude::*;

shrewnit::simple_unit!(
    pub HalfInches of dimension Length = 78.740158 per canonical
);

fn main() {
    let distance = 30.0f64 * Feet;
    let time = 3.5 * Seconds;

    let average_velocity = distance / time;
    let acceleration = average_velocity / time;

    println!("{:?} {:?}", average_velocity, acceleration);

    let half_inches = distance.to::<HalfInches>();
    println!("{:?}", half_inches);
}
