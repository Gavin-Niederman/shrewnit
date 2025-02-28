use shrewnit::{Distance, Feet, Measure, Seconds};

shrewnit::simple_unit!(
    pub HalfInches of measure Distance = 78.740158 per canonical
);

fn main() {
    let distance = 30.0 * Feet;
    let time = 3.5 * Seconds;

    let average_velocity = distance / time;
    let acceleration = average_velocity / time;

    println!("{:?} {:?}", average_velocity, acceleration);

    let half_inches = distance.to::<HalfInches>();
    println!("{:?}", half_inches);
}
