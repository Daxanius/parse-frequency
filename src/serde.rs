use super::*;
use ::serde::{Deserialize, Deserializer, Serialize, Serializer, de};

impl<'de> Deserialize<'de> for Frequency {
    fn deserialize<D>(
        deserializer: D,
    ) -> std::result::Result<Frequency, <D as serde::Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_frequency(&s).map_err(de::Error::custom)
    }
}

impl Serialize for Frequency {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
