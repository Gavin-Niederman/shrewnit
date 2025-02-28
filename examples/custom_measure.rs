use shrewnit::Measure;

shrewnit::measure!(
    Current {
        si: Watts,
        canonical: Watts,

        Watts: 1.0 per canonical,
    }
);

fn main() {
    let current = Watts * 1.0;

    println!("{}", current.to::<Watts>())
}
