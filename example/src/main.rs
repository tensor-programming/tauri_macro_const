fn main() {
    // print the two macros
    println!("{}", data_dir!());
    println!("{}", foo_file!());

    // print the data in the included file.
    println!("{:?}", std::str::from_utf8(assets::FOO_FILE));
    // print the const and static strings
    println!("{}", assets::S_STR);
    println!("{}", assets::SS_STR);

    // confirm that these values are still equal.  You must get these ones via their module paths unlike the macros.
    assert_eq!(assets::S_STR, assets::SOME_STRING);
    assert_eq!(assets::SS_STR, assets::SOME_STATIC_STRING);
}

// simulate a module.  Must export the macros using #[macro_use]
#[macro_use]
pub mod assets {

    use tauri_macro_const::include_const_file;

    // include the test.rs file from the OUT_DIR directory
    include_const_file!("test");

    // define a static asset pointing towards the macro constant string path.  No string literal error
    // Notice type annotation defines 11 bytes because that is the length of the file contents.
    pub static FOO_FILE: &'static [u8; 11] = include_bytes!(foo_file!());
    // map SOME_STRING to S_STR
    pub const S_STR: &'static str = SOME_STRING;
    // map SOME_STATIC_STRING to SS_STR
    pub static SS_STR: &'static str = SOME_STATIC_STRING;
}
