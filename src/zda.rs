use crate::datetime::{Date, Time};
use crate::{common, Source};

/// Geographic latitude ang longitude sentence with time of fix and receiver state.
#[derive(Debug, PartialEq, Clone)]
pub struct ZDA {
    /// Navigational system.
    pub source: Source,
    /// Current Time  in UTC.
    pub time: Time,
    /// Current Day
    pub day: u8,
    /// Current Month
    pub month: u8,
    /// Current Year
    pub year: u16,
    /// Offset in hours from UTC.
    pub offset_hours: Option<i8>,
    /// Offset in minutes from UTC,
    pub offset_minutes: Option<u8>,
}

impl ZDA {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let time = Time::parse_from_hhmmss(fields.next())?;
        let day = common::parse_u8(fields.next())?;
        let month = common::parse_u8(fields.next())?;
        let year = common::parse_u16(fields.next())?;
        let offset_hours = common::parse_i8(fields.next())?;
        let offset_minutes = common::parse_u8(fields.next())?;

        if let (Some(time), Some(day), Some(month), Some(year), offset_hours, offset_minutes) =
            (time, day, month, year, offset_hours, offset_minutes)
        {
            Ok(Some(ZDA {
                source,
                time,
                day,
                month,
                year,
                offset_hours,
                offset_minutes,
            }))
        } else {
            Ok(None)
        }
    }
}
