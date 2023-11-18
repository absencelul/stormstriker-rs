use std::path::PathBuf;
use memory::process::Process;

const DLL_NAME: &str = "stormstriker_rs.dll";
const EXE_NAME: &str = "StormStrikerClient-Win64-Shipping.exe";

fn dll_path<'a>() -> String {
    let mut path = PathBuf::from("dummy");
    for profile in ["release", "debug"] {
        let formatted = format!("target/{}/{}", profile, DLL_NAME);
        let temp_path: PathBuf = formatted.into();
        if temp_path.exists() {
            path = temp_path;
            break;
        }
    }

    if !path.exists() {
        path = PathBuf::from(DLL_NAME);
        if !path.exists() {
            eprintln!("Could not find DLL");
            std::process::exit(1);
        }
        return path
            .to_str()
            .expect("Could not convert DLL path to string")
            .to_string();
    }

    std::fs::copy(path, DLL_NAME).unwrap();
    path = PathBuf::from(DLL_NAME);
    if !path.exists() {
        eprintln!("Could not copy DLL to current directory");
        std::process::exit(0);
    }

    path.to_str()
        .expect("Could not convert DLL path to string")
        .to_string()
}

fn main() {
    println!("Finding process...");
    let path = dll_path();
    let process = Process::by_name(EXE_NAME).expect("Could not find process");

    println!("Injecting DLL...");
    process.inject(path.into()).expect("Could not inject DLL");
}
