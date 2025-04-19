use crate::{Error, Frequency};

impl clap::builder::TypedValueParser for Frequency {
    type Value = Self;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> std::result::Result<Frequency, clap::error::Error> {
        let value = value.to_str().ok_or_else(|| {
            clap::Error::raw(clap::error::ErrorKind::InvalidUtf8, "Invalid UTF-8")
        })?;
        value
            .parse()
            .map_err(|e: Error| clap::Error::raw(clap::error::ErrorKind::InvalidValue, e))
    }
}
