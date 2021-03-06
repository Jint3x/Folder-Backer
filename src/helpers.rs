use std::path::PathBuf;

#[derive(Debug)]
pub struct DirItems {
    pub files: Vec<PathBuf>,
    pub dirs: Vec<PathBuf>
}

/// Extract all dirs/files from a given dir path. 
pub fn fetch_files(dir: &String) -> DirItems {
    let dir_items = std::fs::read_dir(dir).expect("Couldn't read a path passed to this function");
    let mut dir_files = DirItems {
        files: Vec::new(),
        dirs: Vec::new(),
    };

    dir_items.for_each(|entity| {
        if let Err(err) = entity {
            println!("An error has occured visiting a file, the error is: {:?}", err)
        } else {
            let entity = entity.unwrap();

            if entity.path().is_dir() {
                dir_files.dirs.push(entity.path());
            } else {
                dir_files.files.push(entity.path());
            }
        }
    });


    dir_files
}


// Given a list of the directories in the main and backup folder, filter them so the ones in the main_files will be ignored 
// and the ones in the backup folder will be completely removed if found. 
pub fn filter_files(main_dirs: &mut Vec<PathBuf>, backup_dirs: &mut Vec<PathBuf>, to_filter: &Vec<&str>) {
    // Remove all filtered items from the vec.
    *main_dirs = main_dirs
    .iter()
    .filter(|name| !to_filter.contains(&name.file_name().unwrap().to_str().unwrap()))
    .map(|path| path.clone())
    .collect::<Vec<PathBuf>>();

    // Remove all filtered items from the vec and delete the directories themselves.
    *backup_dirs = backup_dirs
    .iter()
    .filter(|name| {
        let file_name = name.file_name().unwrap().to_str().unwrap();

        if to_filter.contains(&file_name) {
            std::fs::remove_dir_all(name).unwrap();
            false
        } else {
            true
        }
    })
    .map(|path| path.clone())
    .collect::<Vec<PathBuf>>();
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn fetch_files_get_files() {
        let path = "./test-folder/fetch-files";

        let folder_test3 = format!("{}{}", path, "/test3/");
        let file_test = format!("{}{}", path, "/test.txt");
        let file_test2 = format!("{}{}", path, "/test2.txt");

        fs::create_dir(path).unwrap();

        fs::create_dir(folder_test3).unwrap();
        fs::File::create(file_test).unwrap();
        fs::File::create(file_test2).unwrap();

        let items = fetch_files(&path.to_string());
        
        assert_eq!(items.files.len(), 2);
        assert_eq!(items.dirs.len(), 1);

        fs::remove_dir_all(path).unwrap();
    }


    #[test]
    fn filter_files_filter() {
        let path = "./test-folder/filter-files";
        let folder_test3 = format!("{}{}", path, "/test3/");
        let file_test = format!("{}{}", path, "/test.txt");
        let file_test2 = format!("{}{}", path, "/test2.txt");

        fs::create_dir(path).unwrap();
        fs::create_dir(folder_test3).unwrap();
        fs::File::create(file_test).unwrap();
        fs::File::create(file_test2).unwrap();

        let items = fetch_files(&path.to_string());

        let mut bdirs = items.dirs;
        let mut mdirs = vec!(
            PathBuf::from("C:/special-folder/special/test5/"),
            PathBuf::from("C:/special-folder/special/test6/"),
        );

        filter_files(&mut mdirs, &mut bdirs, &vec!("test5", "test3"));

        assert_eq!(mdirs.len(), 1);
        assert_eq!(bdirs.len(), 0);

        fs::remove_dir_all(path).unwrap();
    }
}