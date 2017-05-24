use std::marker::PhantomData;
use std::fmt::{Display, Result as FmtResult, Formatter};
use chrono::{UTC, NaiveDateTime, NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, Error};
use super::ChronoDateTime;
use super::format::{DateFormat, ParseError};
use super::formats::ChronoFormat;
use super::mapping::{DateFieldType, DateMapping, DefaultDateMapping};

pub use chrono::{Datelike, Timelike};

impl DateFieldType<DefaultDateMapping<ChronoFormat>, ChronoFormat> for ChronoDateTime {}

/// An Elasticsearch `date` type with a required `time` component.
///
/// The [format](format/index.html) is provided as a generic parameter.
/// This struct wraps up a `chrono::DateTime<UTC>` struct, meaning storing time in `UTC` is required.
///
/// # Examples
///
/// Defining a date using the default format:
///
/// ```
/// # use elastic_types::prelude::*;
/// let date: Date<DefaultDateFormat> = Date::now();
/// ```
///
/// Defining a date using a named format:
///
/// ```
/// # use elastic_types::prelude::*;
/// let date = Date::<BasicDateTime>::now();
/// ```
///
/// Defining a date using a custom mapping:
///
/// ```
/// # use elastic_types::prelude::*;
/// let date: Date<BasicDateTime, DefaultDateMapping<_>> = Date::now();
/// ```
///
/// Accessing the values of a date:
///
/// ```
/// # use elastic_types::prelude::*;
/// let date = Date::<BasicDateTime>::now();
///
/// //eg: 2010/04/30 13:56:59.372
/// println!("{}/{}/{} {}:{}:{}.{}",
///     date.year(),
///     date.month(),
///     date.day(),
///     date.hour(),
///     date.minute(),
///     date.second(),
///     date.nanosecond() / 1000000
/// );
/// ```
///
/// # Links
///
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)
#[derive(Debug, Clone, PartialEq)]
pub struct Date<F, M = DefaultDateMapping<F>>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    value: ChronoDateTime,
    _t: PhantomData<(M, F)>,
}

impl<F, M> Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    /// Creates a new `Date` from the given `chrono::DateTime<UTC>`.
    ///
    /// This function will consume the provided `chrono` date.
    ///
    /// # Examples
    ///
    /// Create an `Date` from the given `chrono::DateTime`:
    ///
    /// ```
    /// # extern crate elastic_types;
    /// # extern crate chrono;
    /// # fn main() {
    /// use chrono::UTC;
    /// use elastic_types::date::{ Date, DefaultDateFormat };
    ///
    /// //Create a chrono DateTime struct
    /// let chronoDate = UTC::now();
    ///
    /// //Give it to the Date struct
    /// let esDate: Date<DefaultDateFormat> = Date::new(chronoDate);
    /// # }
    /// ```
    pub fn new(date: ChronoDateTime) -> Date<F, M> {
        Date {
            value: date,
            _t: PhantomData,
        }
    }

    /// Creates an `Date` from the given UTC primitives:
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// let esDate: Date<DefaultDateFormat> = Date::build(
    ///     2015,
    ///     5,
    ///     14,
    ///     16,
    ///     45,
    ///     8,
    ///     886
    /// );
    /// ```
    pub fn build(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, milli: u32) -> Date<F, M> {
        Date {
            value: ChronoDateTime::from_utc(NaiveDateTime::new(NaiveDate::from_ymd(year, month, day),
                                                               NaiveTime::from_hms_milli(hour, minute, second, milli)),
                                            UTC),
            _t: PhantomData,
        }
    }

    /// Gets the current system time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// let date: Date<DefaultDateFormat> = Date::now();
    /// ```
    pub fn now() -> Date<F, M> {
        Date {
            value: UTC::now(),
            _t: PhantomData,
        }
    }

    /// Parse the date and time from a string.
    ///
    /// The format of the string must match the given `DateFormat`.
    ///
    /// # Examples
    ///
    /// Parsing from a specified `DateFormat`.
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// let date = Date::<BasicDateTime>::parse("20151126T145543.778Z").unwrap();
    /// ```
    pub fn parse(date: &str) -> Result<Date<F, M>, ParseError> {
        F::parse(date).map(Date::new)
    }

    /// Format the date and time as a string.
    ///
    /// The format of the string is specified by the given `DateFormat`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// let date: Date<BasicDateTime> = Date::now();
    /// let fmt = date.format();
    ///
    /// //eg: 20151126T145543.778Z
    /// println!("{}", fmt);
    /// ```
    pub fn format(&self) -> String {
        F::format(&self.value).into()
    }

    /// Change the format/mapping of this date.
    ///
    /// # Examples
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// //Get the current datetime formatted as basic_date_time
    /// let date: Date<BasicDateTime> = Date::now();
    ///
    /// //Change the format to epoch_millis
    /// let otherdate: Date<EpochMillis> = date.remap();
    /// ```
    pub fn remap<FInto, MInto>(self) -> Date<FInto, MInto>
        where FInto: DateFormat,
              MInto: DateMapping<Format = FInto>
    {
        Date::<FInto, MInto>::new(self.value)
    }
}

impl<F, M> DateFieldType<M, F> for Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
}

impl_mapping_type!(ChronoDateTime, Date, DateMapping, DateFormat);

impl<F, M> Default for Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn default() -> Date<F, M> {
        Date::<F, M>::now()
    }
}

impl<F, M> Display for Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", F::format(&self.value))
    }
}

impl<F, M> Serialize for Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(&F::format(&self.value))
    }
}

impl<'de, F, M> Deserialize<'de> for Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn deserialize<D>(deserializer: D) -> Result<Date<F, M>, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Default)]
        struct DateTimeVisitor<F, M>
            where F: DateFormat,
                  M: DateMapping<Format = F>
        {
            _t: PhantomData<(M, F)>,
        }

        impl<'de, F, M> Visitor<'de> for DateTimeVisitor<F, M>
            where F: DateFormat,
                  M: DateMapping<Format = F>
        {
            type Value = Date<F, M>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter,
                       "a json string or number containing a formatted date")
            }

            fn visit_str<E>(self, v: &str) -> Result<Date<F, M>, E>
                where E: Error
            {
                let result = Date::<F, M>::parse(v);
                result.map_err(|err| Error::custom(format!("{}", err)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Date<F, M>, E>
                where E: Error
            {
                let result = Date::<F, M>::parse(&v.to_string());
                result.map_err(|err| Error::custom(format!("{}", err)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Date<F, M>, E>
                where E: Error
            {
                let result = Date::<F, M>::parse(&v.to_string());
                result.map_err(|err| Error::custom(format!("{}", err)))
            }
        }

        deserializer.deserialize_any(DateTimeVisitor::<F, M>::default())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct DateBrw<'a, F, M = DefaultDateMapping<F>>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    value: &'a ChronoDateTime,
    _t: PhantomData<(M, F)>,
}

impl<'a, F, M> DateBrw<'a, F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    #[doc(hidden)]
    pub fn new(date: &'a ChronoDateTime) -> DateBrw<'a, F, M> {
        DateBrw {
            value: date,
            _t: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn format(&self) -> String {
        F::format(&self.value).into()
    }
}

impl<'a, F, M> Display for DateBrw<'a, F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", F::format(&self.value))
    }
}

impl<'a, F, M> DateFieldType<M, F> for DateBrw<'a, F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
}

impl<'a, F, M> Serialize for DateBrw<'a, F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(&F::format(&self.value))
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use chrono;
    use chrono::offset::TimeZone;

    use prelude::*;

    #[derive(ElasticDateFormat, Default, Clone)]
    #[elastic(date_format="yyyy/MM/dd HH:mm:ss", date_format_name="test_date_1")]
    pub struct NamedDateFormat;

    #[derive(ElasticDateFormat, Default, Clone, Copy)]
    #[elastic(date_format="yyyyMMdd")]
    pub struct UnNamedDateFormat;

    #[test]
    fn date_format_uses_name_if_supplied() {
        assert_eq!("test_date_1", NamedDateFormat::name());
    }

    #[test]
    fn date_format_uses_format_if_name_not_supplied() {
        assert_eq!("yyyyMMdd", UnNamedDateFormat::name());
    }

    #[test]
    fn dates_should_use_chrono_format() {
        let dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let expected = dt.format("%Y/%m/%d %H:%M:%S").to_string();

        let dt = Date::<NamedDateFormat>::new(dt.clone());
        let actual = dt.format();

        assert_eq!(expected, actual);
    }

    #[test]
    fn dates_should_use_es_format() {
        let dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let expected = "20150513".to_string();

        let dt = Date::<UnNamedDateFormat>::new(dt.clone());
        let actual = dt.format();

        assert_eq!(expected, actual);
    }

    #[test]
    fn can_change_date_mapping() {
        fn takes_epoch_millis(_: Date<EpochMillis>) -> bool {
            true
        }

        let date: Date<BasicDateTime> = Date::now();

        assert!(takes_epoch_millis(date.remap()));
    }

    #[test]
    fn can_build_date_from_chrono() {
        let date: Date<DefaultDateFormat> = Date::new(chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap());

        assert_eq!((2015, 5, 13, 0, 0, 0), (
            date.year(),
            date.month(),
            date.day(),
            date.hour(),
            date.minute(),
            date.second()
        ));
    }

    #[test]
    fn can_build_date_from_prim() {
        let date: Date<DefaultDateFormat> = Date::build(2015, 5, 13, 0, 0, 0, 0);

        assert_eq!((2015, 5, 13, 0, 0, 0), (
            date.year(),
            date.month(),
            date.day(),
            date.hour(),
            date.minute(),
            date.second()
        ));
    }

    #[test]
    fn serialise_elastic_date() {
        let date = Date::<BasicDateTime>::new(chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S")
            .unwrap());

        let ser = serde_json::to_string(&date).unwrap();

        assert_eq!(r#""20150513T000000.000Z""#, ser);
    }

    #[test]
    fn deserialise_elastic_date() {
        let date: Date<BasicDateTime> = serde_json::from_str(r#""20150513T000000.000Z""#).unwrap();

        assert_eq!((2015, 5, 13), (
            date.year(),
            date.month(),
            date.day()
        ));
    }

    #[test]
    fn serialise_elastic_date_brw() {
        let chrono_date = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S")
            .unwrap();

        let date = DateBrw::<BasicDateTime>::new(&chrono_date);

        let ser = serde_json::to_string(&date).unwrap();

        assert_eq!(r#""20150513T000000.000Z""#, ser);
    }

}
