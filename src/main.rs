use anyhow::Result;
use sga_unpacker::extract_all;

fn main() -> Result<()> {
    extract_all("./test/ArtJapanese.sga", "./test/out2")?;

    Ok(())
}
