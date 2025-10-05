
use relic_chunky::chunky::ChunkFile;

fn main() {
    let mut file = std::fs::File::open("./tests/weapon_war_elephant_spear_3_sul.rgd").unwrap();
    let reader = std::io::BufReader::new(&mut file);

    let file = ChunkFile::parse(reader).unwrap();
    println!("File: {:?}", file);

    // let header = relic_chunky::chunky::ChunkyFileHeader::parse(&mut reader).unwrap();
    // println!("Header: {:?}", header);

    // let chunk_header = relic_chunky::chunky::ChunkHeader::parse(&mut reader).unwrap();

}
