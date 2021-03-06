use std::{
    env,
    fmt::Debug,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    str,
};

use anyhow;
use thiserror::Error;

// shortcut macro to include the built macro const file.
#[macro_export]
macro_rules! include_const_file {
    ($file_name: expr) => {
        include!(concat!(
            env!("OUT_DIR"),
            concat!("/", concat!($file_name, ".rs"))
        ));
    };
}

type CResult<T> = anyhow::Result<T, CError>;

#[derive(Error, Debug)]
pub enum CError {
    #[error("Var error: `{0}`")]
    VarError(#[from] env::VarError),
    #[error("IO Error: `{0}`")]
    FileError(#[from] io::Error),
}

pub struct ConstantWriter {
    file: File,
}

impl ConstantWriter {
    // create a new macro const writer with a name: {name}.rs
    pub fn new(name: &str) -> CResult<ConstantWriter> {
        let out_dir = env::var("OUT_DIR")?;
        let name = format!("{}.rs", name);
        let dest = Path::new(&out_dir).join(name);
        let file = File::create(&dest)?;

        Ok(ConstantWriter { file })
    }

    // create a new macro const writer from a given path.
    pub fn new_from_path(path: &Path) -> CResult<ConstantWriter> {
        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;

        Ok(ConstantWriter { file })
    }

    // add an import to the macro const file.
    pub fn add_import(&mut self, lib_name: &str) -> CResult<()> {
        Ok(writeln!(self.file, "pub use {};", lib_name)?)
    }

    // add the value you want to turn into a constant macro. Must be a debug type to be registered as a string.
    pub fn add_const_macro<T: Debug>(&mut self, name: &str, value: T) -> CResult<()> {
        Ok(self.add_const_macro_raw(name, &format!("{:?}", value))?)
    }

    // inner functionality for building a constant macro.
    fn add_const_macro_raw(&mut self, name: &str, raw_const: &String) -> CResult<()> {
        writeln!(self.file, "#[macro_export]")?;

        Ok(writeln!(
            self.file,
            "macro_rules! {} {{() => {{ {} }};}}",
            name, raw_const
        )?)
    }

    pub fn add_public_constant<T: Debug>(
        &mut self,
        name: &str,
        typ: &str,
        value: T,
    ) -> CResult<()> {
        self.add_public_constant_raw(name, typ, &format!("{:?}", value))?;
        Ok(())
    }

    fn add_public_constant_raw(
        &mut self,
        name: &str,
        typ: &str,
        raw_const: &String,
    ) -> CResult<()> {
        Ok(writeln!(
            self.file,
            "pub const {}: {} = {};",
            name.to_uppercase(),
            typ,
            raw_const
        )?)
    }

    pub fn add_static_value<T: Debug>(&mut self, name: &str, typ: &str, value: T) -> CResult<()> {
        self.add_static_value_raw(name, typ, &format!("{:?}", value))?;
        Ok(())
    }

    fn add_static_value_raw(&mut self, name: &str, typ: &str, raw_const: &String) -> CResult<()> {
        Ok(writeln!(
            self.file,
            "pub static {}: {} = {};",
            name.to_uppercase(),
            typ,
            raw_const
        )?)
    }

    // close the file to finish writing the constant macros.
    pub fn close(&mut self) -> CResult<()> {
        Ok(self.file.flush()?)
    }
}
