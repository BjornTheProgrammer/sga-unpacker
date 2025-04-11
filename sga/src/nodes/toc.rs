use std::{io::{BufRead, Read, Seek}, sync::{Arc, Mutex}};

use anyhow::Result;

use crate::entires::{SgaEntries, SgaToC};

use super::FolderNode;

#[derive(Debug, Clone)]
pub struct Toc {
    pub name: String,
    pub alias: String,

    pub root_folder: Arc<Mutex<FolderNode>>,

    pub toc_entry: SgaToC
}

impl Toc {
    pub fn new(name: String, alias: String, root_folder: Arc<Mutex<FolderNode>>, toc_entry: SgaToC) -> Self {
        Self {
            name,
            alias,
            root_folder,
            toc_entry
        }
    }

    pub fn initialize_from_entry<T: Read + BufRead + Seek>(reader: &mut T, entries: &SgaEntries, toc: SgaToC) -> Result<Self> {
        let root_folder_entry = &entries.folders[toc.folder_root_index as usize];
        let root_folder = FolderNode::folder_from_entry(reader, entries, &root_folder_entry, None)?;

        Ok(Self {
            name: toc.name.clone(),
            alias: toc.alias.clone(),
            root_folder: Arc::new(Mutex::new(root_folder)),
            toc_entry: toc
        })
    }
}
