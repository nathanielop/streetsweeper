use regex::Regex;
use std::env::args;
use std::fs::{read_dir, File, ReadDir};
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

fn get_files(dir: &Path, root_alias: &String) -> Vec<String> {
    let dir_entries: ReadDir = read_dir(dir).unwrap();
    let mut paths: Vec<String> = vec![];
    dir_entries.for_each(|dir| {
        let path_buf: PathBuf = dir.unwrap().path();
        if path_buf.is_dir() {
            get_files(path_buf.as_path(), root_alias)
                .iter()
                .for_each(|str| paths.push(String::from(str)));
        } else {
            let str_path = path_buf.to_str().unwrap();
            if str_path.ends_with(".js") {
                paths.push(String::from(str_path).replace("./", root_alias));
            }
        }
    });
    paths
}

fn get_file_imports(path: &String) -> Vec<String> {
    let mut file = File::open(path).unwrap();
    let regex: Regex = Regex::new(r#"(import [A-z0-9{}\s,]{1,} from [A-z0-9"'./]{1,};)"#).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut imports: Vec<String> = vec![];
    for cap in regex.captures_iter(&contents) {
        let import_path = cap[0].to_string().replace("./", "").replace("\"", "'");
        let path_begin_index = import_path.find("'").unwrap() + 1;
        let path_end_index = import_path.rfind("'").unwrap();
        imports.push(unsafe {
            import_path
                .get_unchecked(path_begin_index..path_end_index)
                .to_string()
        })
    }

    return imports;
}

fn main() -> Result<()> {
    let root_alias = args().nth(0).unwrap_or(String::from(""));
    let files = get_files(Path::new("."), &root_alias);
    let imports = files.iter().map(|str| get_file_imports(str)).flatten();
    files.iter().for_each(|file| {
        if imports.clone().find(|import| import == file) == None {
            println!("Dead file found, {}", file);
        }
    });
    Ok(())
}
