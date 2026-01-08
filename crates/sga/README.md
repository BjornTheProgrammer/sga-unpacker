# SGA
What is SGA? SGA is a file format used by Relic games studio for many of their games. This library helps to parse and extract those files for viewing.

## Installation
To install just run `cargo add sga` or add the following to your `Cargo.toml`

```toml
[dependencies]
sga = "0.1"
```

## Usage
To unpack to a specified destination, just use the `extract_all` function.

```rust
use sga::extract_all;

fn main() {
    extract_all("./ArtJapanese.sga", "./ArtJapanese").unwrap();
}
```

If you wish to do something more elaborate, for example only extracting the first folder and files from the table_of_contents, it is possible to construct the file tree, and then write it to disk.

```rust
pub fn extract_toc_folders_only<P: AsRef<Path>>(sga_file: P, out_path: P) -> anyhow::Result<()> {
    // open the file as a buffer
    let mut sga_file = BufReader::new(File::open(sga_file)?);

    // construct the entries from the buffer.
    // contains header, files, folders, and table_of_contents information
    let mut entries = SgaEntries::new(&mut sga_file)?;
    // Just take out the table of contents entries, and replace the taken with an empty vec.
    let toc_entries = std::mem::replace(&mut entries.tocs, Vec::new());
    // Create a new Toc from those entries
    let tocs: Vec<_> = toc_entries
        .into_iter()
        .map(|toc| Toc::initialize_from_entry(&mut sga_file, &entries, toc).unwrap())
        .collect();

    // loop over the tocs
    for toc in tocs {
        // Clone folder for more use.
        let folder = toc.root_folder.clone();
        // read all files from the folder
        let files = FolderNode::read_files_from_folder(folder.clone(), &mut sga_file, &entries)?;
    
        // add all those files into the folder
        for file in files {
            let file = Arc::new(file);
            folder.lock().unwrap().add_child(Node::File(file));
        }

        // write the entire folder to the disk
        write_to_disk(&mut sga_file, folder, &out_path)?;
    }

    Ok(())
}
```

