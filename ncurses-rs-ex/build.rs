// build.rs

// use std::env;

fn main() {
    // let target = env::var("TARGET").unwrap();    // <arch><sub>-<vendor>-<sys>-<abi>
    // println!("cargo:rustc-target={}", target);

    // Using `brew install ncurses` in Mac OSX, 
    // the library is installed in the path `/usr/local/opt/ncurses/*` .
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local/opt/ncurses/lib");
        println!("cargo:include=/usr/local/opt/ncurses/include");
    }
}
