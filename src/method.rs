pub struct Method(Inner);

pub enum Inner {
    /// https://datatracker.ietf.org/doc/html/rfc4791#section-5.3.1
    MkCalendar,
    /// https://datatracker.ietf.org/doc/html/rfc3253#section-3.6
    Report,
    Webdav(webdav::Method),
}

impl Method {
    pub const MKCALENDAR: Method = Method(Inner::MkCalendar);
    pub const REPORT: Method = Method(Inner::Report);

    pub const PROPFIND: Method = Method(Inner::Webdav(webdav::Method::PROPFIND));
    pub const PROPPATCH: Method = Method(Inner::Webdav(webdav::Method::PROPPATCH));
    pub const MKCOL: Method = Method(Inner::Webdav(webdav::Method::MKCOL));
    pub const COPY: Method = Method(Inner::Webdav(webdav::Method::COPY));
    pub const MOVE: Method = Method(Inner::Webdav(webdav::Method::MOVE));
    pub const LOCK: Method = Method(Inner::Webdav(webdav::Method::LOCK));
    pub const UNLOCK: Method = Method(Inner::Webdav(webdav::Method::UNLOCK));

    pub const GET: Method = Method(Inner::Webdav(webdav::Method::GET));
    pub const POST: Method = Method(Inner::Webdav(webdav::Method::POST));
    pub const PUT: Method = Method(Inner::Webdav(webdav::Method::PUT));
    pub const DELETE: Method = Method(Inner::Webdav(webdav::Method::DELETE));
    pub const HEAD: Method = Method(Inner::Webdav(webdav::Method::HEAD));
    pub const OPTIONS: Method = Method(Inner::Webdav(webdav::Method::OPTIONS));
    pub const CONNECT: Method = Method(Inner::Webdav(webdav::Method::CONNECT));
    pub const PATCH: Method = Method(Inner::Webdav(webdav::Method::PATCH));
    pub const TRACE: Method = Method(Inner::Webdav(webdav::Method::TRACE));
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self.0 {
            Inner::MkCalendar => "MKCALENDAR",
            Inner::Report => "REPORT",
            Inner::Webdav(method) => {
                return write!(f, "{method}");
            }
        };

        f.write_str(s)
    }
}

impl std::ops::Deref for Method {
    type Target = webdav::Method;

    fn deref(&self) -> &Self::Target {
        match self.0 {
            Inner::MkCalendar => todo!(),
            Inner::Report => todo!(),
            Inner::Webdav(ref method) => method,
        }
    }
}
