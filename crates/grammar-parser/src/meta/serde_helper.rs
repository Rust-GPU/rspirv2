#![cfg(feature = "serde")]

use serde::de;
use std::fmt;

pub fn num_or_hex<'de, D: de::Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    struct NumOrStr;

    impl de::Visitor<'_> for NumOrStr {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "either a number or a hex string")
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
            value
                .try_into()
                .map_err(|_e| de::Error::invalid_value(de::Unexpected::Unsigned(value), &self))
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            u32::from_str_radix(&value[2..], 16)
                .map_err(|_e| de::Error::invalid_value(de::Unexpected::Str(value), &self))
        }
    }

    d.deserialize_any(NumOrStr)
}
