//! Date data types.  Supported by [chrono].
//!
//! This lib does not support the OSCAL date-with-timezone data type. It is
//! not used in any FedRAMP OSCAL schema, and it is not supported by [chrono].
//!
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};

use super::StringType;
use crate::{Error, SchemaConstraint};

/// A Naive date with no timezone.
/// All dates are UTC based.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateDatatype(NaiveDate);

impl DateDatatype {
    /// Create a new date.
    /// The date is created from the current UTC date.
    pub fn new() -> Self {
        Self(Utc::now().date_naive())
    }
}

impl Default for DateDatatype {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for DateDatatype {
    type Target = NaiveDate;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for DateDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")?;
        Ok(Self(result))
    }
}

impl ToString for DateDatatype {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl StringType for DateDatatype {
    fn pattern() -> &'static str {
        r"^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{2})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{2})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{2})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))(Z|[+-][0-9]{2}:[0-9]{2})?$"
    }
}

impl SchemaConstraint for DateDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#"A string representing a 24-hour period, optionally qualified by a timezone. This is the same as date-with-timezone, except the timezone portion is optional."#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "date"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateTimeDatatype(NaiveDateTime);

impl TryFrom<&str> for DateTimeDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = value.parse::<NaiveDateTime>()?;
        Ok(Self(result))
    }
}

impl FromStr for DateTimeDatatype {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.parse::<NaiveDateTime>()?;
        Ok(Self(result))
    }
}

impl StringType for DateTimeDatatype {
    fn pattern() -> &'static str {
        r#"^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{2})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{2})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{2})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])(\\.[0-9]+)?(Z|(-((0[0-9]|1[0-2]):00|0[39]:30)|\\+((0[0-9]|1[0-4]):00|(0[34569]|10):30|(0[58]|12):45)))?$"#
    }
}

impl SchemaConstraint for DateTimeDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#"A string representing a point in time, optionally qualified by a timezone. This date and time value is formatted according to “date-time” as defined RFC3339, except the timezone (time-offset) is optional."#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "date-time"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateTimeWithTimezoneDatatype(DateTime<Utc>);

impl DateTimeWithTimezoneDatatype {
    pub fn new() -> Self {
        let utc: DateTime<Utc> = Utc::now();
        Self(utc)
    }
}

impl Default for DateTimeWithTimezoneDatatype {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for DateTimeWithTimezoneDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = value.parse::<DateTime<Utc>>()?;
        Ok(Self(result))
    }
}

impl FromStr for DateTimeWithTimezoneDatatype {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.parse::<DateTime<Utc>>()?;
        Ok(Self(result))
    }
}

impl StringType for DateTimeWithTimezoneDatatype {
    fn pattern() -> &'static str {
        r"^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{2})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{2})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{2})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])(\.[0-9]*[1-9])?(Z|(-((0[0-9]|1[0-2]):00|0[39]:30)|\+((0[0-9]|1[0-4]):00|(0[34569]|10):30|(0[58]|12):45)))$"
    }
}

impl SchemaConstraint for DateTimeWithTimezoneDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#"A string representing a point in time in a given timezone. This date and time value is formatted according to “date-time” as defined RFC3339"#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "date-time-with-timezone"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateTimeDuration(String);

impl DateTimeDuration {
    pub fn new() -> Self {
        let utc: DateTime<Utc> = Utc::now();
        Self(utc.to_string())
    }
}
impl Default for DateTimeDuration {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for DateTimeDuration {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for DateTimeDuration {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl StringType for DateTimeDuration {
    fn pattern() -> &'static str {
        r#"^-?P([0-9]+D(T(([0-9]+H([0-9]+M)?(([0-9]+|[0-9]+(\\.[0-9]+)?)S)?)|([0-9]+M(([0-9]+|[0-9]+(\\.[0-9]+)?)S)?)|([0-9]+|[0-9]+(\\.[0-9]+)?)S))?)|T(([0-9]+H([0-9]+M)?(([0-9]+|[0-9]+(\\.[0-9]+)?)S)?)|([0-9]+M(([0-9]+|[0-9]+(\\.[0-9]+)?)S)?)|([0-9]+|[0-9]+(\\.[0-9]+)?)S)$"#
    }

    fn format() -> Option<&'static str> {
        Some("date")
    }
}

impl SchemaConstraint for DateTimeDuration {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#"An amount of time quantified in days, hours, minutes, and seconds based on ISO-8601 durations (see also RFC3339 appendix A)."#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "date-time-duration"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_to_string() {
        let value = "2024-02-10";
        let json = format!("\"{}\"", value);
        let date = serde_json::from_str::<DateDatatype>(&json).expect("Failed to deserialize");
        assert_eq!(&date.to_string(), value);
        let result = serde_json::to_string(&date).expect("Failed to serialize");
        assert_eq!(&result, &json);
    }

    #[test]
    fn test_validate_date_time() {
        let input = "2015-02-18";
        let result = chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d");
        assert!(result.is_ok());

        let input = "2015-02-18-05:00";
        let result = chrono::DateTime::parse_from_rfc3339(input);
        println!("{:?}", &result);
    }
}
