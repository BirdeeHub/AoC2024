use std::io::Result;
mod part1;
mod part2;
mod better;
fn main() -> Result<()> {
    part1::run()?;
    part2::run()?;
    better::run()
}
