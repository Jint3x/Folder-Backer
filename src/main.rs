use std::path::PathBuf;

use env_plus;
mod helpers;
use helpers::{fetch_files, filter_files};

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

    create_backup(mfolder, bfolder, &to_filter)
}


/// The main function that creates a backup of the files in each folder.
///
fn create_backup(mfolder: String, bfolder: String, to_filter: &Vec<&str>) {
    let mut mfiles = fetch_files(&mfolder);
    let mut bfiles = fetch_files(&bfolder);
    filter_files(&mut mfiles.dirs, &mut bfiles.dirs, to_filter);

    // Go to the folder and clear all differences between the backup and the main dir.
    // Then, a list of sub dirs will be given and the same function will be called on all of them.
    clear_inconsistencies_files(mfiles.files, bfiles.files, &bfolder);
    let remaining_dirs = clear_inconsistencies_dirs(mfiles.dirs, bfiles.dirs, &bfolder);

    for (main_dir, backup_dir) in remaining_dirs {
        create_backup(main_dir, backup_dir, to_filter);
    }
}

/// Clear inconsistencies between files:
/// (M = mfolder, B = bfolder)
/// 
/// 1. If file exists in B, but doesn't exist in M, remove it.
/// 2. If a file exists in M, but doesn't exist in B, copy it.
/// 3. If file exists in both M and B, check it's metadata and copy it to B if there's a difference.
fn clear_inconsistencies_files(mfiles: Vec<PathBuf>, bfiles: Vec<PathBuf>, bpath: &String) {
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
        let bpathbuf = PathBuf::from(&bpath);
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
/// 3. If dir exists in both M and B, check if the main folder's modified date is different than the backup one. If so, copy the file.
fn clear_inconsistencies_dirs(mdirs: Vec<PathBuf>, bdirs: Vec<PathBuf>, bpath: &String) -> Vec<(String, String)> {
    let mut to_explore = vec!();

    let exists_in_m_only = mdirs.iter().filter(|mdir| {
        bdirs.iter().all(|bdir| {
            bdir.file_name().unwrap() != mdir.file_name().unwrap()
        })
    });

    let exists_in_b_only = bdirs.iter().filter(|bdir| {
        mdirs.iter().all(|mdir| {
            mdir.file_name().unwrap() != bdir.file_name().unwrap()
        })
    });


    let exists_in_m_and_b = bdirs.iter().filter_map(|bdir| {
        for mdir in mdirs.iter() {
            if mdir.file_name().unwrap() == bdir.file_name().unwrap() {
                return Some((mdir, bdir))
            }
        }

        None
    });


    // Step 1
    exists_in_m_only.for_each(|dir| {
        let new_path = PathBuf::from(bpath).join(dir.file_name().unwrap());
        std::fs::create_dir(&new_path).unwrap();

        to_explore.push((
            dir.to_str().unwrap().to_string(),
            new_path.to_str().unwrap().to_string()
        ));
    });

    // Step 2
    exists_in_b_only.for_each(|dir| {
        std::fs::remove_dir_all(dir).unwrap();
    });

    // Step 3
    exists_in_m_and_b.for_each(|(mdir, bdir)| {
        let data_main = mdir.metadata().unwrap().modified().unwrap();
        let data_backup = bdir.metadata().unwrap().modified().unwrap();

        if data_main != data_backup {
            to_explore.push((
                mdir.to_str().unwrap().to_string(),
                bdir.to_str().unwrap().to_string()
            ));
        }
    });


    to_explore
}