use clap::ValueEnum;

pub mod calc;
pub mod even;
pub mod gcd;
pub mod prime;
pub mod progression;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Games {
    /// Brain Calc
    Calc,
    /// Brain Even
    Even,
    /// Brain Gcd
    Gcd,
    /// Brain Prime
    Prime,
    /// Brain Progression
    Progression,
}
