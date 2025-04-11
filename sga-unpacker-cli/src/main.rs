use anyhow::Result;
use sga::extract_all;

fn main() -> Result<()> {
    extract_all("./test/ArtJapanese.sga", "./test/out2")?;

    Ok(())
}
