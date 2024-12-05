use std::io::Result;
mod part1;
mod part2;
mod part1try2;
fn main() -> Result<()> {
    part1::run()?;
    part2::run()?;
    part1try2::run()
}
