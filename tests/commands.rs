// use std::env;
// use std::fs;
// use std::path::{Path, PathBuf};
// use std::process::Command;

// fn bin_path() -> PathBuf {
//     if let Ok(exe) = env::var("CARGO_BIN_EXE_phpvm") {
//         return PathBuf::from(exe);
//     }
//     let mut dir = env::current_exe().expect("current_exe");
//     dir.pop();
//     let mut cand = dir.clone();
//     cand.pop();
//     cand.push("phpvm");
//     if cfg!(windows) { cand.set_extension("exe"); }
//     if cand.exists() { return cand; }
//     let mut cand2 = dir.clone();
//     cand2.push("phpvm");
//     if cfg!(windows) { cand2.set_extension("exe"); }
//     cand2
// }

// fn mk_temp_home(test_name: &str) -> PathBuf {
//     let mut p = env::temp_dir();
//     p.push(format!("phpvm-test-{}-{}", test_name, std::process::id()));
//     if p.exists() { let _ = fs::remove_dir_all(&p); }
//     fs::create_dir_all(&p).unwrap();
//     p
// }

// fn phpvm_with_home(home: &Path) -> Command {
//     let mut cmd = Command::new(bin_path());
//     cmd.env("HOME", home);
//     cmd
// }

// #[test]
// fn doctor_runs() {
//     let home = mk_temp_home("doctor");
//     let output = phpvm_with_home(&home).arg("doctor").output().unwrap();
//     assert!(output.status.success(), "doctor failed: {}", String::from_utf8_lossy(&output.stderr));
//     let out = String::from_utf8_lossy(&output.stdout);
//     assert!(out.contains("root:"));
//     assert!(out.contains("versions:"));
// }

// #[test]
// fn cache_prune_idempotent() {
//     let home = mk_temp_home("cache_prune");
//     // run twice
//     for _ in 0..2 {
//         let output = phpvm_with_home(&home).args(["cache", "prune"]).output().unwrap();
//         assert!(output.status.success(), "cache prune failed: {}", String::from_utf8_lossy(&output.stderr));
//     }
//     let cache = home.join(".phpvm").join("cache");
//     // Ensure cache directory exists (it should be created by doctor command)
//     if !cache.exists() {
//         let _ = phpvm_with_home(&home).arg("doctor").output();
//     }
//     assert!(cache.exists());
// }

// #[test]
// fn alias_add_list_delete() {
//     let home = mk_temp_home("alias");
//     // add
//     let add = phpvm_with_home(&home).args(["alias", "foo", "1.2.3"]).output().unwrap();
//     assert!(add.status.success());
//     // list
//     let list = phpvm_with_home(&home).arg("alias").output().unwrap();
//     let out = String::from_utf8_lossy(&list.stdout);
//     assert!(out.contains("foo\t1.2.3"), "alias list missing entry: {}", out);
//     // delete
//     let del = phpvm_with_home(&home).args(["alias", "--delete", "foo"]).output().unwrap();
//     assert!(del.status.success());
//     // list again
//     let list2 = phpvm_with_home(&home).arg("alias").output().unwrap();
//     let out2 = String::from_utf8_lossy(&list2.stdout);
//     assert!(!out2.contains("foo\t1.2.3"));
// }

// #[test]
// fn set_global_and_local() {
//     let home = mk_temp_home("set_versions");
//     // global
//     let g = phpvm_with_home(&home).args(["global", "1.2.3"]).output().unwrap();
//     assert!(g.status.success(), "global failed: {}", String::from_utf8_lossy(&g.stderr));
//     let global_file = home.join(".phpvm").join("global");
//     let global = fs::read_to_string(global_file).unwrap();
//     assert_eq!(global.trim(), "1.2.3");
//     // local (in project dir)
//     let project = home.join("project");
//     fs::create_dir_all(&project).unwrap();
//     let l = phpvm_with_home(&home).current_dir(&project).args(["local", "2.0.0"]).output().unwrap();
//     assert!(l.status.success(), "local failed: {}", String::from_utf8_lossy(&l.stderr));
//     let local_file = project.join(".php-version");
//     assert!(local_file.exists(), "local file not created: {}", local_file.display());
//     let local = fs::read_to_string(local_file).unwrap();
//     assert_eq!(local.trim(), "2.0.0");
// }

// #[test]
// fn which_fallback_system() {
//     let home = mk_temp_home("which");
//     // choose a ubiquitous program (cmd on Windows, sh on Unix)
//     let program = if cfg!(windows) { "cmd" } else { "sh" };
//     let output = phpvm_with_home(&home).args(["which", program]).output().unwrap();
//     assert!(output.status.success());
//     let out = String::from_utf8_lossy(&output.stdout);
//     let path = out.trim();
//     assert!(!path.is_empty());
//     assert!(Path::new(path).exists());
// }

// #[test]
// fn init_outputs_for_zsh() {
//     let home = mk_temp_home("init");
//     let output = phpvm_with_home(&home).args(["init", "--shell", "zsh"]).output().unwrap();
//     assert!(output.status.success());
//     let out = String::from_utf8_lossy(&output.stdout);
//     assert!(out.contains("export PATH="));
// }

// #[test]
// fn list_runs() {
//     let home = mk_temp_home("list");
//     let output = phpvm_with_home(&home).arg("list").output().unwrap();
//     assert!(output.status.success());
// }


