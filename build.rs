use std::process::Command;

fn main() {
  let tag = Command::new("git").args(&["describe", "--tags"]).output().unwrap().stdout;
  println!("cargo:rustc-env=GIT_TAG={}", String::from_utf8(tag).unwrap());
}