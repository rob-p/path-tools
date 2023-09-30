use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

pub trait WithAdditionalExtension
where
    Self: Into<OsString>,
{
    fn with_additional_extension(&self, s: impl AsRef<OsStr>) -> Self;
}

impl WithAdditionalExtension for PathBuf {
    fn with_additional_extension(&self, s: impl AsRef<OsStr>) -> Self {
        // from: https://stackoverflow.com/questions/74322541/how-to-append-to-pathbuf
        let mut p: OsString = self.into();
        p.push(s);
        p.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn can_append_extension() {
        let initial = PathBuf::from_str("foo.bar").unwrap();
        let updated = initial.with_additional_extension(".baz");
        eprintln!("u : {:?}", updated);
        assert_eq!(updated, PathBuf::from_str("foo.bar.baz").unwrap());
    }
}
