//! Error and result types for the library.
use std::error::Error as StdError;
use std::fmt;

error_chain! {
    errors {
        /// Failed to convert a value to bytes.
        Serialize {
            description("unable to serialize a value")
        }
        /// A duplicate commit or secret was added.
        AlreadyInserted {
            description("a commit or secret has already been inserted for this key")
        }
        /// A `Secret` was added where no matching `Commit` exists.
        NotPresent {
            description("no commit exists for this key")
        }
        /// Random value generation cannot proceed without at least one participant.
        Empty {
            description("no commits added to exchange")
        }
    }
}

/// What a particular participant did wrong when revealing the generated value.
#[derive(Debug)]
pub enum RevealErrorKind {
    /// The participant submitted a `Commit`, but no `Secret` was added.
    MissingSecret,

    /// The participant's `Commit` did not match their `Secret`.
    ValidationFailed,
}

/// The type of error returned when revealing the generated value.
pub struct RevealError<I> {
    failed: Vec<(RevealErrorKind, I)>,
}

impl<I> fmt::Debug for RevealError<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "RevealError {{")?;
        writeln!(f, "    failed: [")?;
        for (kind, _) in self.failed.iter() {
            writeln!(f, "        ({:?}, _),", kind)?;
        }
        writeln!(f, "    ]")?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

impl<I> fmt::Display for RevealError<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unable to reveal, {} participant(s) failed",
            self.failed.len()
        )
    }
}

impl<I> StdError for RevealError<I> {}

impl<I> RevealError<I> {
    pub(crate) fn new(failed: Vec<(RevealErrorKind, I)>) -> Self {
        Self { failed }
    }

    /// Iterator over the participants who caused the reveal to fail.
    pub fn failed(self) -> impl Iterator<Item = (RevealErrorKind, I)> {
        self.failed.into_iter()
    }
}

/// Result of `Reveal::get`.
pub type RevealResult<T, I> = ::std::result::Result<T, RevealError<I>>;
