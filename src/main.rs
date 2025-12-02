mod aoc1;
mod aoc2;

fn main() {
    aoc1::solve()
        .map_err(|e| {
            eprintln!("Error: {}", e);
        })
        .ok();

    aoc2::solve();
}
