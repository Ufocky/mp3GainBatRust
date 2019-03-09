fn main() {
    println!(".bat file generator for mp3gain.exe tool.");

    let arg: Vec<String> = std::env::args().collect();

    if arg.len() != 3 || !std::path::Path::new(&arg[1]).is_dir() {
        println!("Wrong parameters!");
        println!("Usage: mp3GainBatRust.exe BAT_FILE SCAN_DIRECTORY");
        println!("example: mp3GainBatRust.exe process.bat c:\\wolololo");
        return;
    }

    let mut output_file: String = String::from("");
    add_line(&mut output_file, &String::from("@echo off"));

    let mut files: Vec<String> = Vec::new();
    get_files(std::path::Path::new(&arg[2]), &mut files);

    let mut cnt = files.len();
    for plik in files {
        cnt -= 1;
        add_line(
            &mut output_file,
            &String::from(format!("mp3gain /r /c \"{}\"", plik)),
        );
        add_line(
            &mut output_file,
            &String::from(format!("echo files left:{}", cnt)),
        );
    }

    std::fs::write(std::path::Path::new(&arg[1]), output_file)
        .expect(format!("Write error to {}", arg[1]).as_str());
}

fn add_line(so: &mut String, si: &String) {
    so.push_str(&si);
    so.push_str("\r\n");
}

fn get_files(dir: &std::path::Path, out: &mut Vec<String>) {
    if dir.is_dir() {
        let dir_entries = std::fs::read_dir(dir).unwrap();
        for dir_entry in dir_entries {
            let dir_entry = dir_entry.unwrap().path();
            if dir_entry.is_dir() {
                get_files(dir_entry.as_path(), out);
            } else {
                if check_extension(&dir_entry) {
                    out.push(String::from(dir_entry.to_str().unwrap_or("")));
                }
            }
        }
    }
}

fn check_extension(path_buf: &std::path::PathBuf) -> bool {
    if path_buf
        .extension()
        .unwrap_or(std::ffi::OsStr::new(""))
        .to_str()
        .unwrap_or("")
        .to_lowercase()
        == "mp3"
    {
        true
    } else {
        false
    }
}
