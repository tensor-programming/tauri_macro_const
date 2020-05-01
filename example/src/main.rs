fn main() {
    // print the two macros
    println!("{}", data_dir!());
    println!("{}", foo_file!());

    // print the data in the included file.
    println!("{:?}", std::str::from_utf8(assets::ASSETS));
}

// simulate a module.  Must export the macros using #[macro_use]
#[macro_use]
pub mod assets {

    use tauri_macro_const::include_const_file;

    // include the test.rs file from the OUT_DIR directory
    include_const_file!("test");

    // define a static asset pointing towards the macro constant string path.  No string literal error
    pub static ASSETS: &'static [u8; 11] = include_bytes!(foo_file!());
}
