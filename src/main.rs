use env_plus;


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
}
