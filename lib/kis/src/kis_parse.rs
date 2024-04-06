use std::num::IntErrorKind;
use chrono::NaiveDate;
use serde::de::Visitor;
use serde::Deserializer;

pub(crate) fn parse_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v.parse::<f64>() {
                Ok(v) => Ok(v),
                Err(_) => Ok(0.0)
            }
        }
    }

    deserializer.deserialize_str(TestVisitor)
}

pub(crate) fn parse_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>
{
    struct ParseVisitor;
    impl<'de> Visitor<'de> for ParseVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v.parse::<i32>() {
                Ok(v) => Ok(v),
                Err(e) if *e.kind() == IntErrorKind::Empty => Ok(0),
                _ => Err(serde::de::Error::custom("Failed to parse i32"))
            }
        }
    }

    deserializer.deserialize_str(ParseVisitor)
}

pub(crate) fn parse_to_naive_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>
{
    struct ParseVisitor;
    impl<'de> Visitor<'de> for ParseVisitor {
        type Value = NaiveDate;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match NaiveDate::parse_from_str(v, "%Y%m%d") {
                Ok(v) => Ok(v),
                Err(_) => Ok(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            }
        }
    }

    deserializer.deserialize_str(ParseVisitor)
}