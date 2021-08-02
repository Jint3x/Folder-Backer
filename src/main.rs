use std::path::PathBuf;

use env_plus;
mod helpers;
use helpers::{DirItems, fetch_files, filter_files};

fn main() {
    env_plus::EnvLoader::new()
    .change_file("./saver.conf".into())
    .change_delimiter("=".into())
    .change_comment("#".into())
    .activate();

    // m = main, b = backup
    let mfolder = std::env::var("main_folder").expect("Didn't find a main_folder entry in the config");
    let bfolder = std::env::var("backup_folder").expect("Didn't find a backup_folder entry in the config");

    // Collect all dirs that need to be filtered (removed) when copying. This is usually folders that you don't care about or are too big
    // to copy. 
    let to_filter = std::env::var("filter_dirs").unwrap_or("".into());
    let to_filter = to_filter
    .split(",")
    .map(|str| str.trim())
    .collect::<Vec<&str>>();

    create_backup(mfolder, bfolder, to_filter)
}


/// The main function that creates a backup of the files in each folder.
///
fn create_backup(mfolder: String, bfolder: String, to_filter: Vec<&str>) {
    let mut mfiles = fetch_files(mfolder.clone());
    let mut bfiles = fetch_files(bfolder.clone());
    filter_files(&mut mfiles.dirs, &mut bfiles.dirs, to_filter.clone());

    // Go to the folder and clear all differences between the backup and the main dir.
    // Then, a list of sub dirs will be given and the same function will be called on all of them.
    clear_inconsistencies_files(mfiles.files, bfiles.files, mfolder.clone(), bfolder.clone());
    let remaining_dirs = clear_inconsistencies_dirs(mfiles.dirs, bfiles.dirs);

    for (main_dir, backup_dir) in remaining_dirs {
        // create_backup(main_dir, backup_dir, to_filter.clone());
    }
}

/// Clear inconsistencies between files:
/// (M = mfolder, B = bfolder)
/// 
/// 1. If file exists in B, but doesn't exist in M, remove it.
/// 2. If a file exists in M, but doesn't exist in B, copy it.
/// 3. If file exists in both M and B, check it's metadata and copy it to B if there's a difference.
fn clear_inconsistencies_files(mfiles: Vec<PathBuf>, bfiles: Vec<PathBuf>, mpath: String, bpath: String) {
    let exists_in_m_only = mfiles.iter().filter(|mfile| {
        bfiles.iter().all(|bfile| {
            bfile.file_name().unwrap() != mfile.file_name().unwrap()
        })
    });

    let exists_in_b_only = bfiles.iter().filter(|bfile| {
        mfiles.iter().all(|mfile| {
            mfile.file_name().unwrap() != bfile.file_name().unwrap()
        })
    });

    let exists_in_m_and_b = bfiles.iter().filter_map(|bfile| {
        for mfile in mfiles.iter() {
            if mfile.file_name().unwrap() == bfile.file_name().unwrap() {
                return Some((mfile, bfile))
            }
        }

        None
    });

    // Handle error later
    // Run step 1
    exists_in_b_only.for_each(|file| {
        std::fs::remove_file(file).unwrap()
    });

    // Run step 2
    exists_in_m_only.for_each(|file| {
        let bpathbuf = PathBuf::from(bpath.clone());
        let bpathbuf = bpathbuf.join(file.file_name().unwrap());

        std::fs::copy(file, bpathbuf).unwrap(); 
    });

    // Run step 3
    exists_in_m_and_b.for_each(|(mfile, bfile)| {
        let data_main = mfile.metadata().unwrap().modified().unwrap();
        let data_backup = bfile.metadata().unwrap().modified().unwrap();

        if data_main != data_backup {
            std::fs::remove_file(bfile).unwrap();
            std::fs::copy(mfile, bfile).unwrap();
        }
    })
}


/// Clear inconsistencies between dirs:
/// (M = mfolder, B = bfolder)
/// 
/// 1. If dir exists in M, but doesn't exist in B, create it.
/// 2. If dir exists in B, but doesn't exist in M, remove all of its contents and remove it.
/// 3. If dir exists in both M and B, do nothing.  
fn clear_inconsistencies_dirs(mdirs: Vec<PathBuf>, bdirs: Vec<PathBuf>) -> Vec<(String, String)> {
    vec!(("".into(), "".into()))
}