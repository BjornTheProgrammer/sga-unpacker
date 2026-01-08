use relic_chunky::{chunky::ChunkFile, rgd::RelicGameData};

fn main() {
    let mut file = std::fs::File::open("./tests/weapon_war_elephant_spear_3_sul.rgd").unwrap();
    let reader = std::io::BufReader::new(&mut file);

    let mut file = ChunkFile::parse(reader).unwrap();
    let rgd = RelicGameData::parse(&mut file);
    println!("rdg: {:?}", rgd);
    // println!("File: {:?}", file);

    // let header = relic_chunky::chunky::ChunkyFileHeader::parse(&mut reader).unwrap();
    // println!("Header: {:?}", header);

    // let chunk_header = relic_chunky::chunky::ChunkHeader::parse(&mut reader).unwrap();
}
