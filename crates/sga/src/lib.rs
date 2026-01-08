use std::{fs::{self, File}, io::{BufRead, BufReader, Read, Seek}, path::Path, sync::{Arc, Mutex}};

use anyhow::Result;
use entires::{FileStorageType, SgaEntries};
use nodes::{FolderNode, Node, Toc};

pub mod nodes;
pub mod entires;
pub(crate) mod utils;

/// This function visits all the files and folders from the specified folder.
/// It then adds the files and folders it finds on the way as children to the parent
fn visit_folder<T: Read + Seek + BufRead>(
    reader: &mut T,
    folder: Arc<Mutex<FolderNode>>,
    entries: &SgaEntries,
) -> Result<()> {
    let files = FolderNode::read_files_from_folder(folder.clone(), reader, entries)?;
    let subfolders = FolderNode::read_folders_from_folder(folder.clone(), reader, entries)?;

    for file in files {
        let file = Arc::new(file);
        folder.lock().unwrap().add_child(Node::File(file));
    }

    for subfolder in subfolders {
        let subfolder = Arc::new(Mutex::new(subfolder));
        folder.lock().unwrap().add_child(Node::Folder(subfolder.clone()));

        visit_folder(reader, subfolder, entries)?;
    }

    Ok(())
}

/// This function writes the files and folders to the disk at the specified path.
pub fn write_to_disk<T: Read + Seek, P: AsRef<Path>>(
    reader: &mut T,
    folder: Arc<Mutex<FolderNode>>,
    base_path: P,
) -> Result<()> {
    let folder_guard = folder.lock().unwrap();

    // Create the directory on disk
    let folder_path = base_path.as_ref().join(&folder_guard.name);
    fs::create_dir_all(&folder_path)?;

    for child in &folder_guard.children {
        match child {
            Node::File(file_node) => {
                let data = file_node.read_data(reader)?;
                let file_path = folder_path.join(&file_node.name);

                if let FileStorageType::Unknown(n) = file_node.storage_type {
                    println!("The storage type of '{:?}' is unknown with value of '{}', it will be unpacked as raw bytes!", file_path, n);
                }

                fs::write(file_path, &data)?;
            }
            Node::Folder(subfolder) => {
                write_to_disk(reader, subfolder.clone(), &folder_path)?;
            }
        }
    }

    Ok(())
}

/// This function extracts all files from the sga into the specified out path.
pub fn extract_all<P: AsRef<Path>>(sga_file: P, out_path: P) -> Result<()> {
    let mut sga_file = BufReader::new(File::open(sga_file)?);

    let mut entries = SgaEntries::new(&mut sga_file)?;
    let toc_entries = std::mem::replace(&mut entries.tocs, Vec::new());
    let tocs: Vec<_> = toc_entries
        .into_iter()
        .map(|toc| Toc::initialize_from_entry(&mut sga_file, &entries, toc).unwrap())
        .collect();

    for toc in tocs {
        visit_folder(&mut sga_file, toc.root_folder.clone(), &entries)?;
        write_to_disk(&mut sga_file, toc.root_folder, out_path.as_ref())?;
    }


    Ok(())
}


/// This function extracts all files from the sga into the specified out path.
pub fn extract_toc_folders_only<P: AsRef<Path>>(sga_file: P, out_path: P) -> Result<()> {
    let mut sga_file = BufReader::new(File::open(sga_file)?);

    let mut entries = SgaEntries::new(&mut sga_file)?;
    let toc_entries = std::mem::replace(&mut entries.tocs, Vec::new());
    let tocs: Vec<_> = toc_entries
        .into_iter()
        .map(|toc| Toc::initialize_from_entry(&mut sga_file, &entries, toc).unwrap())
        .collect();

    for toc in tocs {
        let folder = toc.root_folder.clone();
        let files = FolderNode::read_files_from_folder(folder.clone(), &mut sga_file, &entries)?;
    
        for file in files {
            let file = Arc::new(file);
            folder.lock().unwrap().add_child(Node::File(file));
        }

        write_to_disk(&mut sga_file, folder, &out_path)?;
    }

    Ok(())
}