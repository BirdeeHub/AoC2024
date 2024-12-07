use std::io::Result;
mod part1;
mod part2bad;
mod part2;
mod types;
fn main() -> Result<()> {
    part1::run()?;
    let expected = part2bad::run()?;
    part2::run(expected)
}
