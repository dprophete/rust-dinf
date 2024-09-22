use anyhow::Result;
use clap::Parser;
use file_entry::FileEntry;
use std::{collections::BinaryHeap, path::PathBuf};
use utils::{pp_path, pp_size};
use walkdir::WalkDir;

mod file_entry;
mod utils;

#[derive(clap::Parser, Debug)]
#[command(about = "A basic directory info", long_about = None)]
pub struct MainArgs {
    #[clap(short)]
    n: Option<u8>,
    #[clap(long)]
    path: Option<String>,
}

fn main() -> Result<()> {
    let args = MainArgs::parse();
    let n = args.n.unwrap_or(3);
    let path = args.path.unwrap_or(".".to_string());
    traverse_current_directory(&path, n)?;
    Ok(())
}

fn traverse_current_directory(path: &str, n: u8) -> Result<()> {
    let mut file_size_heap = BinaryHeap::<FileEntry>::new();
    let mut dir_size_heap = BinaryHeap::<FileEntry>::new();

    let mut cur_dir_path = Option::<PathBuf>::None;
    let mut cur_dir_size = 0;

    let mut nb_files: u64 = 0;
    let mut total_size: u64 = 0;

    let handle_dir = |dir_size_heap: &mut BinaryHeap<FileEntry>,
                      cur_dir_path: &Option<PathBuf>,
                      cur_dir_size: u64| {
        if let Some(cur_dir_path2) = cur_dir_path {
            let dir_entry = FileEntry {
                path: cur_dir_path2.clone(),
                size: cur_dir_size,
            };
            dir_size_heap.push(dir_entry);
            if (dir_size_heap.len() as u8) > n {
                dir_size_heap.pop();
            }
        }
    };

    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_dir() && entry.depth() == 1 {
            handle_dir(&mut dir_size_heap, &cur_dir_path, cur_dir_size);
            cur_dir_size = 0;
            cur_dir_path = Some(entry.path().to_path_buf());
        }

        if entry.file_type().is_file() {
            nb_files += 1;

            let metadata = entry.metadata()?;
            let size = metadata.len();
            total_size += size;
            let file_entry = FileEntry {
                size,
                path: entry.path().to_path_buf(),
            };
            file_size_heap.push(file_entry);
            if (file_size_heap.len() as u8) > n {
                file_size_heap.pop();
            }
            cur_dir_size += size;
        }
    }

    if cur_dir_size > 0 {
        handle_dir(&mut dir_size_heap, &cur_dir_path, cur_dir_size);
    }

    println!("== summary");
    println!(" - nb files {}", nb_files);
    println!(" - total size {}", pp_size(total_size));

    println!("\n== top {} biggest extensions:", n);
    // let top_dir_sizes = dir_size_heap.into_sorted_vec();
    // for dir in top_dir_sizes.iter() {
    //     println!(" - {}: {}", pp_path(&dir.path), pp_size(dir.size));
    // }

    println!("\n== top {} largest dirs:", n);
    let top_dir_sizes = dir_size_heap.into_sorted_vec();
    for dir in top_dir_sizes.iter() {
        println!(" - {}: {}", pp_path(&dir.path), pp_size(dir.size));
    }

    println!("\n== top {} largest files:", n);
    let top_file_sizes = file_size_heap.into_sorted_vec();
    for file in top_file_sizes.iter() {
        println!(" - {}: {}", pp_path(&file.path), pp_size(file.size));
    }
    Ok(())
}
