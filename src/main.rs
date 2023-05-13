#[cfg(feature = "matrix")]
mod matrix;

#[cfg(feature = "race")]
mod race;

fn main() {
    #[cfg(feature = "matrix")]
    matrix::run();

    #[cfg(feature = "race")]
    race::run();
}
