fn main() {
    println!(".bat file generator for mp3gain.exe tool.");

    let arg: Vec<String> = std::env::args().collect();

    if arg.len() != 3 {
        println!("Wrong parameters!");
        println!("Usage: mp3GainBatRust.exe BAT_FILE SCAN_DIRECTORY");
        println!("example: mp3GainBatRust.exe process.bat c:\\wolololo");
        return;
    }

    let files: Vec<String> = get_files(std::path::Path::new(&arg[2]), &0);

    let mut output_file: String = "".to_string();
    add_line(&mut output_file, &"@echo off".to_string());
    let mut cnt = files.len();
    println!("found {} mp3 files...", cnt);

    if cnt == 0 {
        println!("File \"{}\" not created.", arg[1]);
        return;
    }

    for plik in files {
        cnt -= 1;
        add_line(&mut output_file, &format!("mp3gain /r /c \"{}\"", plik));
        add_line(&mut output_file, &format!("echo files left:{}", cnt));
    }

    if std::fs::write(std::path::Path::new(&arg[1]), output_file).is_err() {
        println!(
            "Could not write to \"{}\". Is this correct file name?",
            arg[1]
        );
    } else {
        println!("File \"{}\" created.", arg[1]);
    }
}

fn add_line(so: &mut String, si: &String) {
    so.push_str(format!("{}\r\n", &si).as_str());
}

fn get_files(dir: &std::path::Path, sub_level: &usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if dir.is_dir() {
        //let format_str = ; //format!("{: >1$}", "|_", sub_level);
        println!(
            "{}{}",
            gen_dist_str(sub_level),
            osstr_2_string(&dir.file_name())
        );
        let dir_entries = std::fs::read_dir(dir).unwrap();
        for dir_entry in dir_entries {
            let dir_entry = dir_entry.unwrap().path();
            if dir_entry.is_dir() {
                out.append(&mut get_files(dir_entry.as_path(), &(sub_level + 1)));
            } else {
                if check_extension(&dir_entry) {
                    out.push(dir_entry.to_str().unwrap_or("").to_string());
                }
            }
        }
    } else {
        println!("error: {} not a directory", dir.display());
    }
    out
}

fn check_extension(path_buf: &std::path::PathBuf) -> bool {
    osstr_2_string(&path_buf.extension()).to_lowercase() == "mp3"
}

fn osstr_2_string(input: &std::option::Option<&std::ffi::OsStr>) -> String {
    input
        .unwrap_or(std::ffi::OsStr::new(""))
        .to_str()
        .unwrap_or("")
        .to_string()
}

fn gen_dist_str(input: &usize) -> String {
    let mut out: String = "".to_string();
    if *input < 1 {
        return "".to_string();
    }
    if *input >= 2 {
        for _ in 0..((*input) - 2) {
            out += "| ";
        }
    }
    out += "|-";
    out
}
