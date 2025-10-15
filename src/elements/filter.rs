/// <https://datatracker.ietf.org/doc/html/rfc4791#section-9.7>
#[derive(Debug, Default)]
pub struct Filter {
    comp_filter: Option<CompFilter>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(mut self, comp_filter: CompFilter) -> Self {
        self.comp_filter = Some(comp_filter);

        self
    }
}

impl webdav::ToXml for Filter {
    fn to_xml(&self) -> String {
        format!("<c:filter>{}</c:filter>", self.comp_filter.to_xml())
    }
}

/// <https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.1>
#[derive(Debug)]
pub struct CompFilter {
    name: String,
    is_not_defined: Option<IsNotDefined>,
    children: Vec<Box<dyn webdav::ToXml>>,
}

impl CompFilter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_not_defined: None,
            children: Vec::new(),
        }
    }

    pub fn append(mut self, comp_filter: Self) -> Self {
        self.children.push(Box::new(comp_filter));

        self
    }

    pub fn is_not_defined(mut self, value: bool) -> Self {
        if value {
            self.is_not_defined = Some(IsNotDefined);
        } else {
            self.is_not_defined = None;
        }

        self
    }

    pub fn prop_filter(mut self, prop_filter: PropFilter) -> Self {
        self.children.push(Box::new(prop_filter));

        self
    }

    pub fn time_range(mut self, time_range: super::TimeRange) -> Self {
        self.children.push(Box::new(time_range));

        self
    }
}

impl webdav::ToXml for CompFilter {
    fn to_xml(&self) -> String {
        if self.is_not_defined.is_none() && self.children.is_empty() {
            return format!("<c:comp-filter name=\"{}\" />", self.name);
        }

        format!(
            "<c:comp-filter name=\"{}\">{}{}</c:comp-filter>",
            self.name,
            self.is_not_defined.to_xml(),
            self.children.to_xml(),
        )
    }
}

/// <https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.2>
#[derive(Debug)]
pub struct PropFilter {
    name: String,
    is_not_defined: Option<IsNotDefined>,
    children: Vec<Box<dyn webdav::ToXml>>,
}

impl PropFilter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_not_defined: None,
            children: Vec::new(),
        }
    }

    pub fn append(mut self, param_filter: ParamFilter) -> Self {
        self.children.push(Box::new(param_filter));

        self
    }

    pub fn is_not_defined(mut self, value: bool) -> Self {
        if value {
            self.is_not_defined = Some(IsNotDefined);
        } else {
            self.is_not_defined = None;
        }

        self
    }

    pub fn text_match(mut self, text_match: TextMatch) -> Self {
        self.children.push(Box::new(text_match));

        self
    }

    pub fn time_range(mut self, time_range: super::TimeRange) -> Self {
        self.children.push(Box::new(time_range));

        self
    }
}

impl webdav::ToXml for PropFilter {
    fn to_xml(&self) -> String {
        if self.is_not_defined.is_none() && self.children.is_empty() {
            format!("<c:prop-filter name=\"{}\" />", self.name)
        } else {
            format!(
                "<c:prop-filter name=\"{}\">{}{}</c:prop-filter>",
                self.name,
                self.is_not_defined.to_xml(),
                self.children.to_xml(),
            )
        }
    }
}

/// <https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.3>
#[derive(Debug)]
pub struct ParamFilter {
    name: String,
    params: Vec<Box<dyn Param>>,
}

impl ParamFilter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            params: Vec::new(),
        }
    }

    pub fn append<P: Param + 'static>(mut self, param: P) -> Self {
        self.params.push(Box::new(param));

        self
    }
}

impl webdav::ToXml for ParamFilter {
    fn to_xml(&self) -> String {
        let params = self
            .params
            .iter()
            .map(|x| x.to_xml())
            .collect::<Vec<_>>()
            .join("");

        format!(
            "<c:param-filter name=\"{}\">{params}</c:param-filter>",
            self.name,
        )
    }
}

pub trait Param: std::fmt::Debug + webdav::ToXml {}

/// https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.4
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct IsNotDefined;

impl webdav::ToXml for IsNotDefined {
    fn to_xml(&self) -> String {
        "<c:is-not-defined />".to_string()
    }
}

/// <https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.5>
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextMatch {
    text: String,
    collation: Option<String>,
    negate_condition: bool,
}

impl TextMatch {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            collation: None,
            negate_condition: false,
        }
    }

    pub fn collation(mut self, collation: &str) -> Self {
        self.collation = Some(collation.to_string());

        self
    }

    pub fn negate_condition(mut self, value: bool) -> Self {
        self.negate_condition = value;

        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();

        self
    }
}

impl webdav::ToXml for TextMatch {
    fn to_xml(&self) -> String {
        let collation = match &self.collation {
            Some(collation) => format!(" collation=\"{collation}\""),
            None => String::new(),
        };

        let negate_condition = if self.negate_condition {
            " negate-condition=\"yes\"".to_string()
        } else {
            String::new()
        };

        format!(
            "<c:text-match{collation}{negate_condition}><![CDATA[{}]]></c:text-match>",
            &self.text
        )
    }
}

impl Param for TextMatch {}

#[cfg(test)]
mod test {
    use webdav::ToXml as _;

    #[test]
    fn comp_filter() {
        let filter = crate::filter! {
            CompFilter::new("VEVENT")
        };

        assert_eq!(
            filter.to_xml(),
            "<c:filter><c:comp-filter name=\"VEVENT\" /></c:filter>",
        );
    }

    #[test]
    fn nested() {
        let filter = crate::filter! {
            CompFilter::new("VCALENDAR") {
                CompFilter::new("VEVENT") {
                }
            }
        };

        assert_eq!(
            filter.to_xml(),
            "<c:filter><c:comp-filter name=\"VCALENDAR\"><c:comp-filter name=\"VEVENT\" /></c:comp-filter></c:filter>",
        );
    }

    #[test]
    fn text_match() {
        let filter = crate::filter! {
            CompFilter::new("VEVENT") {
                prop_filter: PropFilter::new("URL") {
                    text_match: TextMatch::new("https://example.org"),
                }
            }
        };

        assert_eq!(
            filter.to_xml(),
            "<c:filter><c:comp-filter name=\"VEVENT\"><c:prop-filter name=\"URL\"><c:text-match><![CDATA[https://example.org]]></c:text-match></c:prop-filter></c:comp-filter></c:filter>",
        );

        let filter = crate::elements::filter::TextMatch::new("https://example.org")
            .collation("i;octet")
            .negate_condition(true);

        assert_eq!(
            filter.to_xml(),
            "<c:text-match collation=\"i;octet\" negate-condition=\"yes\"><![CDATA[https://example.org]]></c:text-match>",
        );
    }

    #[test]
    fn default() {
        let filter = crate::filter! {
            CompFilter::new("VEVENT") {
                prop_filter: PropFilter::new("URL") {
                    text_match: TextMatch::default() {
                        text: "https://example.org",
                    }
                }
            }
        };

        assert_eq!(
            filter.to_xml(),
            "<c:filter><c:comp-filter name=\"VEVENT\"><c:prop-filter name=\"URL\"><c:text-match><![CDATA[https://example.org]]></c:text-match></c:prop-filter></c:comp-filter></c:filter>",
        );
    }

    #[test]
    fn is_not_defined() {
        let filter = crate::filter! {
            CompFilter::new("VCALENDAR") {
                CompFilter::new("VTODO") {
                    prop_filter: PropFilter::new("COMPLETED") {
                        is_not_defined: true,
                    }
                }
            }
        };

        assert_eq!(
            filter.to_xml(),
            "<c:filter><c:comp-filter name=\"VCALENDAR\"><c:comp-filter name=\"VTODO\"><c:prop-filter name=\"COMPLETED\"><c:is-not-defined /></c:prop-filter></c:comp-filter></c:comp-filter></c:filter>",
        );
    }

    #[test]
    fn time_range() {
        use chrono::TimeZone as _;

        let filter = crate::filter! {
            CompFilter::new("VCALENDAR") {
                CompFilter::new("VEVENT") {
                    time_range: TimeRange {
                        start: chrono::Utc.with_ymd_and_hms(2006, 1, 4, 0, 0, 0).earliest(),
                        end: None,
                    }
                }
            }
        };

        assert_eq!(
            filter.to_xml(),
            "<c:filter><c:comp-filter name=\"VCALENDAR\"><c:comp-filter name=\"VEVENT\"><c:time-range start=\"20060104T000000Z\" end=\"+infinity\" /></c:comp-filter></c:comp-filter></c:filter>",
        );
    }
}
