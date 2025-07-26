use std::fmt::Display;

// TODO 7/19/2025
// Move to its own file?
#[derive(Debug, Clone)]
pub struct Error {
    errors: Vec<ErrorKind>,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// The schema includes a (currently) unsupported schema construction. This
    /// is (likely) a current limitation rather than a fundamental one. If you
    /// encounter this error, you are encouraged to submit a issue that
    /// includes the failing schema (or a simpler one if you can narrow it
    /// down).
    UnsupportedSchemaConstruction {
        // TODO
        // Could this be a URL or something?
        /// The filly qualified id associated with the unsupported schema
        /// construction.
        id: String,
        ///
        message: String,
    },
}

impl Error {
    pub(crate) fn unsupported_schema_construction(
        id: impl AsRef<str>,
        message: impl AsRef<str>,
    ) -> Self {
        Self {
            errors: vec![ErrorKind::UnsupportedSchemaConstruction {
                id: id.as_ref().to_string(),
                message: message.as_ref().to_string(),
            }],
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            match error {
                ErrorKind::UnsupportedSchemaConstruction { id, message } => {
                    writeln!(f, "Unsupported schema construction: {}: {}", id, message)?;
                }
            }
        }
        Ok(())
    }
}

impl std::error::Error for Error {}
