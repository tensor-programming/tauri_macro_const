use tauri_macro_const::ConstantWriter;

// example build.rs file
fn main() {
    // create a mutable constant macro writer
    let mut const_writer = ConstantWriter::new("test").unwrap();

    // add constant called data_dir pointing to the relative path ../data
    const_writer.add_const_macro("data_dir", "../data").unwrap();

    // add constant called foo_file pointing to the file foo in the data folder
    const_writer
        .add_const_macro("foo_file", "../data/foo")
        .unwrap();

    // close the constant writer
    const_writer.close().unwrap();
}
