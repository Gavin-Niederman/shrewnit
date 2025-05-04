use shrewnit::{Inches, Length, LinearVelocity, Seconds, Time, One};

fn main() {
    const LENGTH: Length<f32> = Inches::ONE;
    const TIME: Time<f32> = <Seconds as One<f32, _>>::ONE.mul_scalar(2.0);
    const VELOCITY: LinearVelocity<f32> = LENGTH.div_time(TIME);

    dbg!(VELOCITY);
}