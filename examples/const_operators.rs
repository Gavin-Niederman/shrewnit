use shrewnit::prelude::*;

fn main() {
    const LENGTH: Length = something_at_runtime_or_const();
    const TIME: Time = <Seconds as One<f64, _>>::ONE.mul_scalar(2.0);
    const VELOCITY: LinearVelocity = LENGTH.div_time(TIME);

    const FPS: f64 = VELOCITY.to::<FeetPerSecond>();
    dbg!(FPS);
}


const fn something_at_runtime_or_const() -> Length {
    let length = <Inches as One<f64, _>>::ONE;
    length.mul_scalar(2.0)
}