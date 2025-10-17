use std::env;
use std::path::PathBuf;
use std::process::Command;

fn bin_path() -> PathBuf {
    if let Ok(exe) = env::var("CARGO_BIN_EXE_phpvm") {
        return PathBuf::from(exe);
    }
    // Fallback to target dir alongside current test exe
    let mut dir = env::current_exe().expect("current_exe");
    // .../target/debug/deps/<testname>
    dir.pop(); // deps
    let mut cand = dir.clone();
    cand.pop(); // debug
    cand.push("phpvm");
    if cfg!(windows) { cand.set_extension("exe"); }
    if cand.exists() { return cand; }
    // try in the same folder as deps (debug/deps/phpvm)
    let mut cand2 = dir.clone();
    cand2.push("phpvm");
    if cfg!(windows) { cand2.set_extension("exe"); }
    cand2
}

fn mk_temp_home(test_name: &str) -> PathBuf {
    let mut p = env::temp_dir();
    p.push(format!("phpvm-test-basic-{}-{}", test_name, std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn layout_exists() {
    let home = mk_temp_home("layout");
    let output = Command::new(bin_path()).env("HOME", &home).arg("doctor").output().unwrap();
    assert!(output.status.success(), "doctor failed: {}", String::from_utf8_lossy(&output.stderr));
}
