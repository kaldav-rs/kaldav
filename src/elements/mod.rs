pub mod filter;
pub mod mkcalendar;

pub use filter::Filter;
pub use mkcalendar::Mkcalendar;

/// <https://datatracker.ietf.org/doc/html/rfc4791#section-9.9>
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TimeRange {
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
}

impl webdav::ToXml for TimeRange {
    fn to_xml(&self) -> String {
        let date_format = "%Y%m%dT%H%M%SZ";

        let start = self
            .start
            .as_ref()
            .map(|x| x.naive_utc().format(date_format).to_string())
            .unwrap_or_else(|| "-infinity".to_string());

        let end = self
            .end
            .as_ref()
            .map(|x| x.naive_utc().format(date_format).to_string())
            .unwrap_or_else(|| "+infinity".to_string());

        format!("<c:time-range start=\"{start}\" end=\"{end}\" />")
    }
}
