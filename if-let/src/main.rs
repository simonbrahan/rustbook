enum CoinToss {
    Heads(bool),
    Tails,
}

fn main() {
    let toss = CoinToss::Heads(true);

    if let CoinToss::Heads(state) = toss {
        println!("Was heads! {:?}", state);
    }
}
