use crate::Children;
use crate::Requestable;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, crate::Object)]
pub struct Home {
    url: String,
    auth: Option<crate::Authorization>,
}

impl Home {
    pub fn calendars(&self) -> crate::Result<BTreeMap<String, crate::Calendar>> {
        let response = self.propfind(&self.url, r#"
<d:propfind xmlns:d="DAV:" xmlns:cs="http://calendarserver.org/ns/" xmlns:c="urn:ietf:params:xml:ns:caldav" xmlns:x1="http://apple.com/ns/ical/">
  <d:prop>
     <d:resourcetype />
     <d:displayname />
     <cs:getctag />
     <c:supported-calendar-component-set />
     <x1:calendar-color />
  </d:prop>
</d:propfind>
"#)?;

        Ok(self.to_map(
            &response,
            "//d:response//d:displayname/text()",
            "//d:displayname [text() = '{}']/../../../d:href/text()",
            vec![(
                "color",
                "//d:displayname [text() = '{}']/../x1:calendar-color/text()",
            )],
        ))
    }

    pub fn new_calendar(&self, path: &str, config: &crate::elements::Mkcalendar) -> crate::Result {
        use webdav::ToXml as _;

        let url = format!("{}{path}", self.url);

        let mut config = config.clone();
        if config.name.is_none() {
            config.name = Some(path.to_string());
        }
        self.mkcalendar(&url, &config.to_xml())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn new_calendar() -> crate::Result {
        let server = crate::test::server();

        let client = crate::Client::new(server.url(""));
        let mkcalendar = crate::elements::Mkcalendar {
            name: Some("Lisa's Events".to_string()),
            description: Some("Calendar restricted to events.".to_string()),
            timezone: Some(crate::ical::vcalendar! {
                prodid: "-//Example Corp.//CalDAV Client//EN",
                version: "2.0",
                timezones: [
                    crate::ical::vtimezone! {
                        tzid: "US-Eastern",
                        last_modified: "19870101T000000Z",
                        standard: [
                            crate::ical::tz_standard! {
                                dtstart: "19671029T020000",
                                rrule: "FREQ=YEARLY;BYDAY=-1SU;BYMONTH=10",
                                tzoffsetfrom: "-0400",
                                tzoffsetto: "-0500",
                                tzname: ["Eastern Standard Time (US & Canada)"],
                            }?,
                        ],
                        daylight: [
                            crate::ical::tz_daylight! {
                                dtstart: "19870405T020000",
                                rrule: "FREQ=YEARLY;BYDAY=1SU;BYMONTH=4",
                                tzoffsetfrom: "-0500",
                                tzoffsetto: "-0400",
                                tzname: ["Eastern Daylight Time (US & Canada)"],
                            }?,
                        ],
                    }?,
                ],
            }?),
            supported_components: vec![crate::ical::Components::Event],
        };

        assert!(client.new_calendar("events", &mkcalendar).is_ok());

        Ok(())
    }
}
