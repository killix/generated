pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CustomApp {
        #[doc = "Default listing language in BCP 47 format."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
        #[doc = "Title for the Android app."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for CustomApp {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
            }
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the accounts resource"]
    pub fn accounts(&self) -> crate::resources::accounts::AccountsActions<A> {
        crate::resources::accounts::AccountsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
mod resources {
    pub mod accounts {
        pub mod params {}
        pub struct AccountsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AccountsActions<'a, A> {
            #[doc = "Actions that can be performed on the custom_apps resource"]
            pub fn custom_apps(
                &self,
            ) -> crate::resources::accounts::custom_apps::CustomAppsActions<A> {
                crate::resources::accounts::custom_apps::CustomAppsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod custom_apps {
            pub mod params {}
            pub struct CustomAppsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> CustomAppsActions<'a, A> {
                #[doc = "Create and publish a new custom app."]
                pub fn create(
                    &self,
                    request: crate::schemas::CustomApp,
                    account: i64,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        account,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::CustomApp,
                account: i64,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                #[doc = "Data format for the response."]
                pub fn alt(mut self, value: crate::params::Alt) -> Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(mut self, value: impl Into<String>) -> Self {
                    self.fields = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Deprecated. Please use quotaUser instead."]
                pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                    self.user_ip = Some(value.into());
                    self
                }
                fn _simple_upload_path(&self) -> String {
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("upload/playcustomapp/v1/accounts/");
                    {
                        let var_as_string = self.account.to_string();
                        let var_as_str = &var_as_string;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/customApps");
                    output
                }
                pub fn upload<T, R>(
                    mut self,
                    content: R,
                    mime_type: ::mime::Mime,
                ) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    R: ::std::io::Read + ::std::io::Seek + Send + 'static,
                {
                    if self.fields.is_none() {
                        self.fields = Some(T::field_selector());
                    }
                    let req = self._request(&self._simple_upload_path());
                    let req = req.query(&[("uploadType", "multipart")]);
                    use crate::multipart::{Part, RelatedMultiPart};
                    let mut multipart = RelatedMultiPart::new();
                    let request_json = ::serde_json::to_vec(&self.request)?;
                    multipart.new_part(Part::new(
                        ::mime::APPLICATION_JSON,
                        Box::new(::std::io::Cursor::new(request_json)),
                    ));
                    multipart.new_part(Part::new(mime_type, Box::new(content)));
                    let req = req.header(
                        ::reqwest::header::CONTENT_TYPE,
                        format!("multipart/related; boundary={}", multipart.boundary()),
                    );
                    let req = req.body(reqwest::Body::new(multipart.into_reader()));
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _resumable_upload_path(&self) -> String {
                    let mut output = "https://www.googleapis.com/".to_owned();
                    output.push_str("resumable/upload/playcustomapp/v1/accounts/");
                    {
                        let var_as_string = self.account.to_string();
                        let var_as_str = &var_as_string;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/customApps");
                    output
                }
                pub fn start_resumable_upload(
                    self,
                    mime_type: ::mime::Mime,
                ) -> Result<crate::ResumableUpload, Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._resumable_upload_path());
                    let req = req.query(&[("uploadType", "resumable")]);
                    let req = req.header(
                        ::reqwest::header::HeaderName::from_static("x-upload-content-type"),
                        mime_type.to_string(),
                    );
                    let req = req.json(&self.request);
                    let resp = req.send()?.error_for_status()?;
                    let location_header = resp
                        .headers()
                        .get(::reqwest::header::LOCATION)
                        .ok_or_else(|| {
                            format!("No LOCATION header returned when initiating resumable upload")
                        })?;
                    let upload_url = ::std::str::from_utf8(location_header.as_bytes())?.to_owned();
                    Ok(crate::ResumableUpload::new(
                        self.reqwest.clone(),
                        upload_url,
                    ))
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let fields = T::field_selector();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_standard(
                    self,
                ) -> Result<crate::schemas::CustomApp, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::CustomApp, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/playcustomapp/v1/accounts/".to_owned();
                    {
                        let var_as_string = self.account.to_string();
                        let var_as_str = &var_as_string;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/customApps");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/androidpublisher"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
        }
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
pub struct ResumableUpload {
    reqwest: ::reqwest::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::Client, url: String) -> Self {
        ResumableUpload {
            reqwest,
            url,
            progress: None,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn upload<R>(&mut self, mut reader: R) -> Result<(), Box<dyn ::std::error::Error>>
    where
        R: ::std::io::Read + ::std::io::Seek + Send + 'static,
    {
        let reader_len = {
            let start = reader.seek(::std::io::SeekFrom::Current(0))?;
            let end = reader.seek(::std::io::SeekFrom::End(0))?;
            reader.seek(::std::io::SeekFrom::Start(start))?;
            end
        };
        let progress = match self.progress {
            Some(progress) => progress,
            None => {
                let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
                let req = req.header(::reqwest::header::CONTENT_LENGTH, 0);
                let req = req.header(
                    ::reqwest::header::CONTENT_RANGE,
                    format!("bytes */{}", reader_len),
                );
                let resp = req.send()?.error_for_status()?;
                match resp.headers().get(::reqwest::header::RANGE) {
                    Some(range_header) => {
                        let (_, progress) = parse_range_header(range_header)
                            .map_err(|e| format!("invalid RANGE header: {}", e))?;
                        progress + 1
                    }
                    None => 0,
                }
            }
        };

        reader.seek(::std::io::SeekFrom::Start(progress as u64))?;
        let content_length = reader_len - progress as u64;
        let content_range = format!("bytes {}-{}/{}", progress, reader_len - 1, reader_len);
        let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
        let req = req.header(::reqwest::header::CONTENT_RANGE, content_range);
        let req = req.body(::reqwest::Body::sized(reader, content_length));
        req.send()?.error_for_status()?;
        Ok(())
    }
}

fn parse_range_header(
    range: &::reqwest::header::HeaderValue,
) -> Result<(i64, i64), Box<dyn ::std::error::Error>> {
    let range = range.to_str()?;
    if !range.starts_with("bytes ") {
        return Err(r#"does not begin with "bytes""#.to_owned().into());
    }
    let range = &range[6..];
    let slash_idx = range
        .find('/')
        .ok_or_else(|| r#"does not contain"#.to_owned())?;
    let (begin, end) = range.split_at(slash_idx);
    let end = &end[1..]; // remove '/'
    let begin: i64 = begin.parse()?;
    let end: i64 = end.parse()?;
    Ok((begin, end))
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
mod parsed_string {
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}

trait IterableMethod {
    fn set_page_token(&mut self, value: String);
    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector;
}

#[allow(dead_code)]
struct PageIter<M, T> {
    method: M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<M, T> Iterator for PageIter<M, T>
where
    M: IterableMethod,
    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
{
    type Item = Result<T, Box<dyn ::std::error::Error>>;

    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
        use ::field_selector::FieldSelector;
        #[derive(::serde::Deserialize, FieldSelector)]
        struct PaginatedResult<T>
        where
            T: FieldSelector,
        {
            #[serde(rename = "nextPageToken")]
            next_page_token: Option<String>,

            #[serde(flatten)]
            page_contents: T,
        }

        if self.finished {
            return None;
        }

        let paginated_result: PaginatedResult<T> = match self.method.execute() {
            Ok(r) => r,
            Err(err) => return Some(Err(err)),
        };

        if let Some(next_page_token) = paginated_result.next_page_token {
            self.method.set_page_token(next_page_token);
        } else {
            self.finished = true;
        }

        Some(Ok(paginated_result.page_contents))
    }
}
