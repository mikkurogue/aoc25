mod aoc1;

fn main() {
    aoc1::solve()
        .map_err(|e| {
            eprintln!("Error: {}", e);
        })
        .ok();
}
