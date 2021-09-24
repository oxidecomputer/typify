use std::{
    env,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Command `rustfmt` could not be found
    #[error("rustfmt is not installed")]
    NoRustfmt,
    /// Error with file IO
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// Error from reading stdin of rustfmt
    #[error(transparent)]
    Conversion(#[from] std::string::FromUtf8Error),
}

pub fn rustfmt<T: ToString>(input: T) -> Result<String, Error> {
    let input = input.to_string();

    let rustfmt = which_rustfmt().ok_or(Error::NoRustfmt)?;

    let mut command = Command::new(&rustfmt)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let mut stdin = command.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = command.wait_with_output()?;

    Ok(String::from_utf8(output.stdout)?)
}

fn which_rustfmt() -> Option<PathBuf> {
    match env::var_os("RUSTFMT") {
        Some(which) => {
            if which.is_empty() {
                None
            } else {
                Some(PathBuf::from(which))
            }
        }
        None => toolchain_find::find_installed_component("rustfmt"),
    }
}

#[cfg(test)]
mod tests {
    use crate::rustfmt;
    use quote::quote;

    #[test]
    fn test_basics() {
        assert_eq!(
            rustfmt(quote! { struct Foo { bar: String }}).unwrap(),
            "struct Foo {\n    bar: String,\n}\n"
        );
    }
}
