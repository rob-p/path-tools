use anyhow::anyhow;
use std::ffi::{CString, OsStr, OsString};
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

trait IntoCString {
    fn into_c_string(&self) -> CString;
}

impl IntoCString for PathBuf {
    fn into_c_string(&self) -> CString {
        #[cfg(not(target_os = "windows"))]
        {
            CString::new(std::os::unix::ffi::OsStrExt::as_bytes(
                <PathBuf as Clone>::clone(self).into_os_string().as_os_str(),
            ))
            .unwrap()
        }
        #[cfg(target_os = "windows")]
        CString::new(self.into_os_string().to_str().unwrap()).unwrap()
    }
}

trait TryIntoCString {
    fn try_into_c_string(&self) -> anyhow::Result<CString>;
}

impl TryIntoCString for PathBuf {
    fn try_into_c_string(&self) -> anyhow::Result<CString> {
        #[cfg(not(target_os = "windows"))]
        {
            CString::new(std::os::unix::ffi::OsStrExt::as_bytes(
                <PathBuf as Clone>::clone(self).into_os_string().as_os_str(),
            ))
            .map_err(|_e| anyhow!("encountered NulError converting PathBuf to CString"))
        }
        #[cfg(target_os = "windows")]
        CString::new(self.into_os_string().to_str()?)
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
