#[derive(Debug)]
pub struct Mkcalendar {
    pub name: String,
    pub description: Option<String>,
    pub timezone: Option<ikal::VCalendar>,
    pub supported_components: Vec<ikal::Components>,
}

impl webdav::ToXml for Mkcalendar {
    fn to_xml(&self) -> String {
        let description = self
            .description
            .as_ref()
            .map(|x| format!("<c:calendar-description>{x}</c:calendar-description>"))
            .unwrap_or_default();

        let timezone = if let Some(timezone) = &self.timezone {
            format!(
                "<c:calendar-timezone><![CDATA[{}]]></c:calendar-timezone>",
                ikal::ser::ical(timezone)
            )
        } else {
            String::new()
        };

        let mut components = self
            .supported_components
            .iter()
            .map(|x| format!("<c:comp name=\"{x}\"/>"))
            .collect::<Vec<_>>();
        components.insert(0, "<c:supported-calendar-component-set>".to_string());
        components.push("</c:supported-calendar-component-set>".to_string());

        let xml = format!(
            r#"<?xml version="1.0" encoding="utf-8" ?>
<c:mkcalendar xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav">
    <d:set>
        <d:prop>
            <d:displayname>{name}</d:displayname>
            {description}
            {components}
            {timezone}
        </d:prop>
    </d:set>
</c:mkcalendar>
"#,
            name = self.name,
            components = components.join("\n")
        );

        xml
    }
}
