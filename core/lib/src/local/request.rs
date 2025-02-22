macro_rules! pub_request_impl {
    ($import:literal $($prefix:tt $suffix:tt)?) =>
{
    /// Borrows the inner `Request` as seen by Rocket.
    ///
    /// Note that no routing has occurred and that there is no remote
    /// address unless one has been explicitly set with
    /// [`set_remote()`](Request::set_remote()).
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = $import]
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let inner: &rocket::Request = request.inner();
    /// # });
    /// ```
    #[inline(always)]
    pub fn inner(&self) -> &Request<'c> {
        self._request()
    }

    /// Mutably borrows the inner `Request` as seen by Rocket.
    ///
    /// Note that no routing has occurred and that there is no remote
    /// address unless one has been explicitly set with
    /// [`set_remote()`](Request::set_remote()).
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = $import]
    ///
    /// # Client::_test(|_, request, _| {
    /// let mut request: LocalRequest = request;
    /// let inner: &mut rocket::Request = request.inner_mut();
    /// # });
    /// ```
    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut Request<'c> {
        self._request_mut()
    }

    /// Add a header to this request.
    ///
    /// Any type that implements `Into<Header>` can be used here. Among
    /// others, this includes [`ContentType`] and [`Accept`].
    ///
    /// [`ContentType`]: crate::http::ContentType
    /// [`Accept`]: crate::http::Accept
    ///
    /// # Examples
    ///
    /// Add the Content-Type header:
    ///
    /// ```rust
    #[doc = $import]
    /// use rocket::http::Header;
    /// use rocket::http::ContentType;
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let req = request
    ///     .header(ContentType::JSON)
    ///     .header(Header::new("X-Custom", "custom-value"));
    /// # });
    /// ```
    #[inline]
    pub fn header<H>(mut self, header: H) -> Self
        where H: Into<crate::http::Header<'static>>
    {
        self._request_mut().add_header(header.into());
        self
    }

    /// Adds a header to this request without consuming `self`.
    ///
    /// # Examples
    ///
    /// Add the Content-Type header:
    ///
    /// ```rust
    #[doc = $import]
    /// use rocket::http::ContentType;
    ///
    /// # Client::_test(|_, mut request, _| {
    /// let mut request: LocalRequest = request;
    /// request.add_header(ContentType::JSON);
    /// # });
    /// ```
    #[inline]
    pub fn add_header<H>(&mut self, header: H)
        where H: Into<crate::http::Header<'static>>
    {
        self._request_mut().add_header(header.into());
    }

    /// Set the remote address of this request.
    ///
    /// # Examples
    ///
    /// Set the remote address to "8.8.8.8:80":
    ///
    /// ```rust
    #[doc = $import]
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let address = "8.8.8.8:80".parse().unwrap();
    /// let req = request.remote(address);
    /// # });
    /// ```
    #[inline]
    pub fn remote(mut self, address: std::net::SocketAddr) -> Self {
        self.set_remote(address);
        self
    }

    /// Add a cookie to this request.
    ///
    /// # Examples
    ///
    /// Add `user_id` cookie:
    ///
    /// ```rust
    #[doc = $import]
    /// use rocket::http::Cookie;
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let req = request
    ///     .cookie(("username", "sb"))
    ///     .cookie(("user_id", "12"));
    /// # });
    /// ```
    #[inline]
    pub fn cookie<'a, C>(mut self, cookie: C) -> Self
        where C: Into<crate::http::Cookie<'a>>
    {
        self._request_mut().cookies_mut().add_original(cookie.into().into_owned());
        self
    }

    /// Add all of the cookies in `cookies` to this request.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = $import]
    /// use rocket::http::Cookie;
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let cookies = vec![("a", "b"), ("c", "d")];
    /// let req = request.cookies(cookies);
    /// # });
    /// ```
    #[inline]
    pub fn cookies<'a, C, I>(mut self, cookies: I) -> Self
        where C: Into<crate::http::Cookie<'a>>,
              I: IntoIterator<Item = C>
    {
        for cookie in cookies {
            let cookie: crate::http::Cookie<'_> = cookie.into();
            self._request_mut().cookies_mut().add_original(cookie.into_owned());
        }

        self
    }

    /// Add a [private cookie] to this request.
    ///
    /// [private cookie]: crate::http::CookieJar::add_private()
    ///
    /// # Examples
    ///
    /// Add `user_id` as a private cookie:
    ///
    /// ```rust
    #[doc = $import]
    /// use rocket::http::Cookie;
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let req = request.private_cookie(("user_id", "sb"));
    /// # });
    /// ```
    #[cfg(feature = "secrets")]
    #[cfg_attr(nightly, doc(cfg(feature = "secrets")))]
    #[inline]
    pub fn private_cookie<C>(mut self, cookie: C) -> Self
        where C: Into<crate::http::Cookie<'static>>
    {
        self._request_mut().cookies_mut().add_original_private(cookie.into());
        self
    }

    /// Set mTLS client certificates to send along with the request.
    ///
    /// If the request already contained certificates, they are replaced with
    /// thsoe in `reader.`
    ///
    /// `reader` is expected to be PEM-formatted and contain X509 certificates.
    /// If it contains more than one certificate, the entire chain is set on the
    /// request. If it contains items other than certificates, the certificate
    /// chain up to the first non-certificate item is set on the request. If
    /// `reader` is syntactically invalid PEM, certificates are cleared on the
    /// request.
    ///
    /// The type `C` can be anything that implements [`std::io::Read`]. This
    /// includes: `&[u8]`, `File`, `&File`, `Stdin`, and so on. To read a file
    /// in at compile-time, use [`include_bytes!()`].
    ///
    /// ```rust
    /// use std::fs::File;
    ///
    #[doc = $import]
    /// use rocket::fs::relative;
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let path = relative!("../../examples/tls/private/ed25519_cert.pem");
    /// let req = request.identity(File::open(path).unwrap());
    /// # });
    /// ```
    #[cfg(feature = "mtls")]
    #[cfg_attr(nightly, doc(cfg(feature = "mtls")))]
    pub fn identity<C: std::io::Read>(mut self, reader: C) -> Self {
        use crate::http::{tls::util::load_certs, private::Certificates};

        let mut reader = std::io::BufReader::new(reader);
        let certs = load_certs(&mut reader).map(Certificates::from);
        self._request_mut().connection.client_certificates = certs.ok();
        self
    }

    /// Sets the body data of the request.
    ///core/lib/src/local/request.rs
    /// # Examples
    ///
    /// ```rust
    #[doc = $import]
    /// use rocket::http::ContentType;
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let req = request
    ///     .header(ContentType::Text)
    ///     .body("Hello, world!");
    /// # });
    /// ```
    #[inline]
    pub fn body<S: AsRef<[u8]>>(mut self, body: S) -> Self {
        // TODO: For CGI, we want to be able to set the body to be stdin
        // without actually reading everything into a vector. Can we allow
        // that here while keeping the simplicity? Looks like it would
        // require us to reintroduce a NetStream::Local(Box<Read>) or
        // something like that.
        *self._body_mut() = body.as_ref().into();
        self
    }

    /// Sets the body to `value` serialized as JSON with `Content-Type`
    /// [`ContentType::JSON`](crate::http::ContentType::JSON).
    ///
    /// If `value` fails to serialize, the body is set to empty. The
    /// `Content-Type` header is _always_ set.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = $import]
    /// use rocket::serde::Serialize;
    /// use rocket::http::ContentType;
    ///
    /// #[derive(Serialize)]
    /// struct Task {
    ///     id: usize,
    ///     complete: bool,
    /// }
    ///
    /// # Client::_test(|_, request, _| {
    /// let task = Task { id: 10, complete: false };
    ///
    /// let request: LocalRequest = request;
    /// let req = request.json(&task);
    /// assert_eq!(req.content_type(), Some(&ContentType::JSON));
    /// # });
    /// ```
    #[cfg(feature = "json")]
    #[cfg_attr(nightly, doc(cfg(feature = "json")))]
    pub fn json<T: crate::serde::Serialize>(self, value: &T) -> Self {
        let json = serde_json::to_vec(&value).unwrap_or_default();
        self.header(crate::http::ContentType::JSON).body(json)
    }

    /// Sets the body to `value` serialized as MessagePack with `Content-Type`
    /// [`ContentType::MsgPack`](crate::http::ContentType::MsgPack).
    ///
    /// If `value` fails to serialize, the body is set to empty. The
    /// `Content-Type` header is _always_ set.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = $import]
    /// use rocket::serde::Serialize;
    /// use rocket::http::ContentType;
    ///
    /// #[derive(Serialize)]
    /// struct Task {
    ///     id: usize,
    ///     complete: bool,
    /// }
    ///
    /// # Client::_test(|_, request, _| {
    /// let task = Task { id: 10, complete: false };
    ///
    /// let request: LocalRequest = request;
    /// let req = request.msgpack(&task);
    /// assert_eq!(req.content_type(), Some(&ContentType::MsgPack));
    /// # });
    /// ```
    #[cfg(feature = "msgpack")]
    #[cfg_attr(nightly, doc(cfg(feature = "msgpack")))]
    pub fn msgpack<T: crate::serde::Serialize>(self, value: &T) -> Self {
        let msgpack = rmp_serde::to_vec(value).unwrap_or_default();
        self.header(crate::http::ContentType::MsgPack).body(msgpack)
    }

    /// Set the body (data) of the request without consuming `self`.
    ///
    /// # Examples
    ///
    /// Set the body to be a JSON structure; also sets the Content-Type.
    ///
    /// ```rust
    #[doc = $import]
    /// use rocket::http::ContentType;
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let mut request = request.header(ContentType::JSON);
    /// request.set_body(r#"{ "key": "value", "array": [1, 2, 3] }"#);
    /// # });
    /// ```
    #[inline]
    pub fn set_body<S: AsRef<[u8]>>(&mut self, body: S) {
        *self._body_mut() = body.as_ref().into();
    }

    /// Dispatches the request, returning the response.
    ///
    /// This method consumes `self` and is the preferred mechanism for
    /// dispatching.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = $import]
    ///
    /// # Client::_test(|_, request, _| {
    /// let request: LocalRequest = request;
    /// let response = request.dispatch();
    /// # });
    /// ```
    #[inline(always)]
    pub $($prefix)? fn dispatch(self) -> LocalResponse<'c> {
        self._dispatch()$(.$suffix)?
    }

    #[cfg(test)]
    #[allow(dead_code)]
    fn _ensure_impls_exist() {
        fn is_clone_debug<T: Clone + std::fmt::Debug>() {}
        is_clone_debug::<Self>();

        fn is_deref_req<'a, T: std::ops::Deref<Target = Request<'a>>>() {}
        is_deref_req::<Self>();

        fn is_deref_mut_req<'a, T: std::ops::DerefMut<Target = Request<'a>>>() {}
        is_deref_mut_req::<Self>();
    }
}}
