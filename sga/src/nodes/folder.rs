use std::{
    io::{BufRead, Read, Seek, SeekFrom},
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::Result;



use crate::{entires::{SgaEntries, SgaFolderEntry}, utils::read_c_string};

use super::FileNode;

#[derive(Debug, Clone)]
pub enum Node {
    Folder(Arc<Mutex<FolderNode>>),
    File(Arc<FileNode>),
}

#[derive(Debug, Clone)]
pub struct FolderNode {
    pub name: String,

    pub parent: Option<Arc<Mutex<FolderNode>>>,
    pub children: Vec<Node>,
    entry: SgaFolderEntry,
}

fn read_folder_name_from_stream<U: Read + BufRead>(reader: &mut U) -> Result<String> {
    let mut folder_name = read_c_string(reader)?;

    let parsed = Path::new(&folder_name);
    if let Some(name) = parsed.file_name() {
        folder_name = name.to_string_lossy().to_string();
    }

    Ok(folder_name)
}

impl FolderNode {
    pub fn new(
        name: String,
        entry: SgaFolderEntry,
        parent: Option<Arc<Mutex<FolderNode>>>,
        children: Option<Vec<Node>>,
    ) -> Self {
        let children = match children {
            Some(val) => val,
            None => Vec::new(),
        };

        Self {
            name,
            parent,
            children,
            entry,
        }
    }

    pub fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }

    pub fn read_files_from_folder<T: Read + BufRead + Seek>(
        this: Arc<Mutex<Self>>,
        reader: &mut T,
        entries: &SgaEntries,
    ) -> Result<Vec<FileNode>> {
        let entry = &this.lock().unwrap().entry;
        let mut nodes = Vec::new();
        for i in entry.file_start_index..entry.file_end_index {
            let file_entry = &entries.files[i as usize];
            reader.seek(SeekFrom::Start(
                entries.header.header_blob_offset
                    + entries.header.string_offset as u64
                    + file_entry.name_offset as u64,
            ))?;
            let file_name = read_c_string(reader)?;
            let node = FileNode::new(
                file_name,
                entries.header.data_offset + file_entry.data_offset,
                file_entry.compressed_length as usize,
                file_entry.uncompressed_size as usize,
                file_entry.storage_type.clone(),
                this.clone(),
            );

            nodes.push(node);
        }

        Ok(nodes)
    }

    pub fn read_folders_from_folder<T: Read + BufRead + Seek>(
        this: Arc<Mutex<Self>>,
        reader: &mut T,
        entries: &SgaEntries,
    ) -> Result<Vec<FolderNode>> {
        let entry = &this.lock().unwrap().entry;
        let mut nodes = Vec::new();

        for i in entry.folder_start_index..entry.folder_end_index {
            let folder_entry = &entries.folders[i as usize];
            let node = Self::folder_from_entry(reader, entries, folder_entry, Some(this.clone()))?;

            nodes.push(node);
        }

        Ok(nodes)
    }

    pub fn folder_from_entry<T: Read + BufRead + Seek>(
        reader: &mut T,
        entries: &SgaEntries,
        folder_entry: &SgaFolderEntry,
        parent: Option<Arc<Mutex<FolderNode>>>,
    ) -> Result<Self> {
        reader.seek(SeekFrom::Start(
            entries.header.header_blob_offset
                + entries.header.string_offset as u64
                + folder_entry.name_offset as u64,
        ))?;
        let folder_name = read_folder_name_from_stream(reader)?;
        let node = FolderNode::new(folder_name, folder_entry.clone(), parent, None);

        Ok(node)
    }
}
