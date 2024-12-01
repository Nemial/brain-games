use brain_games::games::{calc, even, gcd, prime, progression, Games};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct App {
    #[arg(value_enum)]
    game: Games,
}

fn main() {
    let app = App::parse();

    match app.game {
        Games::Calc => calc::start(),
        Games::Even => even::start(),
        Games::Gcd => gcd::start(),
        Games::Prime => prime::start(),
        Games::Progression => progression::start(),
    }
}
