use shrewnit::prelude::*;

fn main() {
    const LENGTH: Length = Inches::ONE;
    const TIME: Time = <Seconds as One<f64, _>>::ONE.mul_scalar(2.0);
    const VELOCITY: LinearVelocity = LENGTH.div_time(TIME);

    const FPS: f64 = VELOCITY.to::<FeetPerSecond>();
    dbg!(FPS);
}