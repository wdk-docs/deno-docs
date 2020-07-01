// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use crate::version;
use bytes::Bytes;
use deno_core::ErrBox;
use futures::future::FutureExt;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::IF_NONE_MATCH;
use reqwest::header::LOCATION;
use reqwest::header::USER_AGENT;
use reqwest::redirect::Policy;
use reqwest::Client;
use reqwest::Response;
use reqwest::StatusCode;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::future::Future;
use std::io;
use std::io::Read;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio::io::AsyncRead;
use url::Url;

/// Create new instance of async reqwest::Client. This client supports
/// proxies and doesn't follow redirects.
pub fn create_http_client(ca_file: Option<String>) -> Result<Client, ErrBox> {
  let mut headers = HeaderMap::new();
  headers.insert(
    USER_AGENT,
    format!("Deno/{}", version::DENO).parse().unwrap(),
  );
  let mut builder = Client::builder()
    .redirect(Policy::none())
    .default_headers(headers)
    .use_rustls_tls();

  if let Some(ca_file) = ca_file {
    let mut buf = Vec::new();
    File::open(ca_file)?.read_to_end(&mut buf)?;
    let cert = reqwest::Certificate::from_pem(&buf)?;
    builder = builder.add_root_certificate(cert);
  }

  builder.build().map_err(|_| {
    ErrBox::from(io::Error::new(
      io::ErrorKind::Other,
      "Unable to build http client".to_string(),
    ))
  })
}
/// Construct the next uri based on base uri and location header fragment
/// See <https://tools.ietf.org/html/rfc3986#section-4.2>
fn resolve_url_from_location(base_url: &Url, location: &str) -> Url {
  if location.starts_with("http://") || location.starts_with("https://") {
    // absolute uri
    Url::parse(location).expect("provided redirect url should be a valid url")
  } else if location.starts_with("//") {
    // "//" authority path-abempty
    Url::parse(&format!("{}:{}", base_url.scheme(), location))
      .expect("provided redirect url should be a valid url")
  } else if location.starts_with('/') {
    // path-absolute
    base_url
      .join(location)
      .expect("provided redirect url should be a valid url")
  } else {
    // assuming path-noscheme | path-empty
    let base_url_path_str = base_url.path().to_owned();
    // Pop last part or url (after last slash)
    let segs: Vec<&str> = base_url_path_str.rsplitn(2, '/').collect();
    let new_path = format!("{}/{}", segs.last().unwrap_or(&""), location);
    base_url
      .join(&new_path)
      .expect("provided redirect url should be a valid url")
  }
}

// TODO(ry) HTTP headers are not unique key, value pairs. There may be more than
// one header line with the same key. This should be changed to something like
// Vec<(String, String)>
pub type HeadersMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub enum FetchOnceResult {
  Code(Vec<u8>, HeadersMap),
  NotModified,
  Redirect(Url, HeadersMap),
}

/// Asynchronously fetches the given HTTP URL one pass only.
/// If no redirect is present and no error occurs,
/// yields Code(ResultPayload).
/// If redirect occurs, does not follow and
/// yields Redirect(url).
pub fn fetch_once(
  client: Client,
  url: &Url,
  cached_etag: Option<String>,
) -> impl Future<Output = Result<FetchOnceResult, ErrBox>> {
  let url = url.clone();

  let fut = async move {
    let mut request = client.get(url.clone());

    if let Some(etag) = cached_etag {
      let if_none_match_val = HeaderValue::from_str(&etag).unwrap();
      request = request.header(IF_NONE_MATCH, if_none_match_val);
    }
    let response = request.send().await?;

    if response.status() == StatusCode::NOT_MODIFIED {
      return Ok(FetchOnceResult::NotModified);
    }

    let mut headers_: HashMap<String, String> = HashMap::new();
    let headers = response.headers();

    if let Some(warning) = headers.get("X-Deno-Warning") {
      eprintln!(
        "{} {}",
        crate::colors::yellow("Warning"),
        warning.to_str().unwrap()
      );
    }

    for key in headers.keys() {
      let key_str = key.to_string();
      let values = headers.get_all(key);
      let values_str = values
        .iter()
        .map(|e| e.to_str().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
      headers_.insert(key_str, values_str);
    }

    if response.status().is_redirection() {
      let location_string = response
        .headers()
        .get(LOCATION)
        .expect("url redirection should provide 'location' header")
        .to_str()
        .unwrap();

      debug!("Redirecting to {:?}...", &location_string);
      let new_url = resolve_url_from_location(&url, location_string);
      return Ok(FetchOnceResult::Redirect(new_url, headers_));
    }

    if response.status().is_client_error()
      || response.status().is_server_error()
    {
      let err = io::Error::new(
        io::ErrorKind::Other,
        format!("Import '{}' failed: {}", &url, response.status()),
      );
      return Err(err.into());
    }

    let body = response.bytes().await?.to_vec();

    return Ok(FetchOnceResult::Code(body, headers_));
  };

  fut.boxed()
}

/// Wraps reqwest `Response` so that it can be exposed as an `AsyncRead` and integrated
/// into resources more easily.
pub struct HttpBody {
  response: Response,
  chunk: Option<Bytes>,
  pos: usize,
}

impl HttpBody {
  pub fn from(body: Response) -> Self {
    Self {
      response: body,
      chunk: None,
      pos: 0,
    }
  }
}

impl AsyncRead for HttpBody {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context,
    buf: &mut [u8],
  ) -> Poll<Result<usize, io::Error>> {
    let mut inner = self.get_mut();
    if let Some(chunk) = inner.chunk.take() {
      debug!(
        "HttpBody Fake Read buf {} chunk {} pos {}",
        buf.len(),
        chunk.len(),
        inner.pos
      );
      let n = min(buf.len(), chunk.len() - inner.pos);
      {
        let rest = &chunk[inner.pos..];
        buf[..n].copy_from_slice(&rest[..n]);
      }
      inner.pos += n;
      if inner.pos == chunk.len() {
        inner.pos = 0;
      } else {
        inner.chunk = Some(chunk);
      }
      return Poll::Ready(Ok(n));
    } else {
      assert_eq!(inner.pos, 0);
    }

    let chunk_future = inner.response.chunk();
    futures::pin_mut!(chunk_future);

    let result = match futures::ready!(chunk_future.poll(cx)) {
      Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
      Ok(Some(chunk)) => {
        debug!(
          "HttpBody Real Read buf {} chunk {} pos {}",
          buf.len(),
          chunk.len(),
          inner.pos
        );
        let n = min(buf.len(), chunk.len());
        buf[..n].copy_from_slice(&chunk[..n]);
        if buf.len() < chunk.len() {
          inner.pos = n;
          inner.chunk = Some(chunk);
        }
        Ok(n)
      }
      Ok(None) => Ok(0),
    };
    result.into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_fetch_string() {
    let http_server_guard = test_util::http_server();
    // Relies on external http server. See tools/http_server.py
    let url =
      Url::parse("http://127.0.0.1:4545/cli/tests/fixture.json").unwrap();
    let client = create_http_client(None).unwrap();
    let result = fetch_once(client, &url, None).await;
    if let Ok(FetchOnceResult::Code(body, headers)) = result {
      assert!(!body.is_empty());
      assert_eq!(headers.get("content-type").unwrap(), "application/json");
      assert_eq!(headers.get("etag"), None);
      assert_eq!(headers.get("x-typescript-types"), None);
    } else {
      panic!();
    }
    drop(http_server_guard);
  }

  #[tokio::test]
  async fn test_fetch_gzip() {
    let http_server_guard = test_util::http_server();
    // Relies on external http server. See tools/http_server.py
    let url = Url::parse(
      "http://127.0.0.1:4545/cli/tests/053_import_compression/gziped",
    )
    .unwrap();
    let client = create_http_client(None).unwrap();
    let result = fetch_once(client, &url, None).await;
    if let Ok(FetchOnceResult::Code(body, headers)) = result {
      assert_eq!(String::from_utf8(body).unwrap(), "console.log('gzip')");
      assert_eq!(
        headers.get("content-type").unwrap(),
        "application/javascript"
      );
      assert_eq!(headers.get("etag"), None);
      assert_eq!(headers.get("x-typescript-types"), None);
    } else {
      panic!();
    }
    drop(http_server_guard);
  }

  #[tokio::test]
  async fn test_fetch_with_etag() {
    let http_server_guard = test_util::http_server();
    let url = Url::parse("http://127.0.0.1:4545/etag_script.ts").unwrap();
    let client = create_http_client(None).unwrap();
    let result = fetch_once(client.clone(), &url, None).await;
    if let Ok(FetchOnceResult::Code(body, headers)) = result {
      assert!(!body.is_empty());
      assert_eq!(String::from_utf8(body).unwrap(), "console.log('etag')");
      assert_eq!(
        headers.get("content-type").unwrap(),
        "application/typescript"
      );
      assert_eq!(headers.get("etag").unwrap(), "33a64df551425fcc55e");
    } else {
      panic!();
    }

    let res =
      fetch_once(client, &url, Some("33a64df551425fcc55e".to_string())).await;
    assert_eq!(res.unwrap(), FetchOnceResult::NotModified);

    drop(http_server_guard);
  }

  #[tokio::test]
  async fn test_fetch_brotli() {
    let http_server_guard = test_util::http_server();
    // Relies on external http server. See tools/http_server.py
    let url = Url::parse(
      "http://127.0.0.1:4545/cli/tests/053_import_compression/brotli",
    )
    .unwrap();
    let client = create_http_client(None).unwrap();
    let result = fetch_once(client, &url, None).await;
    if let Ok(FetchOnceResult::Code(body, headers)) = result {
      assert!(!body.is_empty());
      assert_eq!(String::from_utf8(body).unwrap(), "console.log('brotli');");
      assert_eq!(
        headers.get("content-type").unwrap(),
        "application/javascript"
      );
      assert_eq!(headers.get("etag"), None);
      assert_eq!(headers.get("x-typescript-types"), None);
    } else {
      panic!();
    }
    drop(http_server_guard);
  }

  #[tokio::test]
  async fn test_fetch_once_with_redirect() {
    let http_server_guard = test_util::http_server();
    // Relies on external http server. See tools/http_server.py
    let url =
      Url::parse("http://127.0.0.1:4546/cli/tests/fixture.json").unwrap();
    // Dns resolver substitutes `127.0.0.1` with `localhost`
    let target_url =
      Url::parse("http://localhost:4545/cli/tests/fixture.json").unwrap();
    let client = create_http_client(None).unwrap();
    let result = fetch_once(client, &url, None).await;
    if let Ok(FetchOnceResult::Redirect(url, _)) = result {
      assert_eq!(url, target_url);
    } else {
      panic!();
    }
    drop(http_server_guard);
  }

  #[test]
  fn test_resolve_url_from_location_full_1() {
    let url = "http://deno.land".parse::<Url>().unwrap();
    let new_uri = resolve_url_from_location(&url, "http://golang.org");
    assert_eq!(new_uri.host_str().unwrap(), "golang.org");
  }

  #[test]
  fn test_resolve_url_from_location_full_2() {
    let url = "https://deno.land".parse::<Url>().unwrap();
    let new_uri = resolve_url_from_location(&url, "https://golang.org");
    assert_eq!(new_uri.host_str().unwrap(), "golang.org");
  }

  #[test]
  fn test_resolve_url_from_location_relative_1() {
    let url = "http://deno.land/x".parse::<Url>().unwrap();
    let new_uri = resolve_url_from_location(&url, "//rust-lang.org/en-US");
    assert_eq!(new_uri.host_str().unwrap(), "rust-lang.org");
    assert_eq!(new_uri.path(), "/en-US");
  }

  #[test]
  fn test_resolve_url_from_location_relative_2() {
    let url = "http://deno.land/x".parse::<Url>().unwrap();
    let new_uri = resolve_url_from_location(&url, "/y");
    assert_eq!(new_uri.host_str().unwrap(), "deno.land");
    assert_eq!(new_uri.path(), "/y");
  }

  #[test]
  fn test_resolve_url_from_location_relative_3() {
    let url = "http://deno.land/x".parse::<Url>().unwrap();
    let new_uri = resolve_url_from_location(&url, "z");
    assert_eq!(new_uri.host_str().unwrap(), "deno.land");
    assert_eq!(new_uri.path(), "/z");
  }

  #[tokio::test]
  async fn test_fetch_with_cafile_string() {
    let http_server_guard = test_util::http_server();
    // Relies on external http server. See tools/http_server.py
    let url =
      Url::parse("https://localhost:5545/cli/tests/fixture.json").unwrap();

    let client = create_http_client(Some(String::from(
      test_util::root_path()
        .join("std/http/testdata/tls/RootCA.pem")
        .to_str()
        .unwrap(),
    )))
    .unwrap();
    let result = fetch_once(client, &url, None).await;
    if let Ok(FetchOnceResult::Code(body, headers)) = result {
      assert!(!body.is_empty());
      assert_eq!(headers.get("content-type").unwrap(), "application/json");
      assert_eq!(headers.get("etag"), None);
      assert_eq!(headers.get("x-typescript-types"), None);
    } else {
      panic!();
    }
    drop(http_server_guard);
  }

  #[tokio::test]
  async fn test_fetch_with_cafile_gzip() {
    let http_server_guard = test_util::http_server();
    // Relies on external http server. See tools/http_server.py
    let url = Url::parse(
      "https://localhost:5545/cli/tests/053_import_compression/gziped",
    )
    .unwrap();
    let client = create_http_client(Some(String::from(
      test_util::root_path()
        .join("std/http/testdata/tls/RootCA.pem")
        .to_str()
        .unwrap(),
    )))
    .unwrap();
    let result = fetch_once(client, &url, None).await;
    if let Ok(FetchOnceResult::Code(body, headers)) = result {
      assert_eq!(String::from_utf8(body).unwrap(), "console.log('gzip')");
      assert_eq!(
        headers.get("content-type").unwrap(),
        "application/javascript"
      );
      assert_eq!(headers.get("etag"), None);
      assert_eq!(headers.get("x-typescript-types"), None);
    } else {
      panic!();
    }
    drop(http_server_guard);
  }

  #[tokio::test]
  async fn test_fetch_with_cafile_with_etag() {
    let http_server_guard = test_util::http_server();
    let url = Url::parse("https://localhost:5545/etag_script.ts").unwrap();
    let client = create_http_client(Some(String::from(
      test_util::root_path()
        .join("std/http/testdata/tls/RootCA.pem")
        .to_str()
        .unwrap(),
    )))
    .unwrap();
    let result = fetch_once(client.clone(), &url, None).await;
    if let Ok(FetchOnceResult::Code(body, headers)) = result {
      assert!(!body.is_empty());
      assert_eq!(String::from_utf8(body).unwrap(), "console.log('etag')");
      assert_eq!(
        headers.get("content-type").unwrap(),
        "application/typescript"
      );
      assert_eq!(headers.get("etag").unwrap(), "33a64df551425fcc55e");
      assert_eq!(headers.get("x-typescript-types"), None);
    } else {
      panic!();
    }

    let res =
      fetch_once(client, &url, Some("33a64df551425fcc55e".to_string())).await;
    assert_eq!(res.unwrap(), FetchOnceResult::NotModified);

    drop(http_server_guard);
  }

  #[tokio::test]
  async fn test_fetch_with_cafile_brotli() {
    let http_server_guard = test_util::http_server();
    // Relies on external http server. See tools/http_server.py
    let url = Url::parse(
      "https://localhost:5545/cli/tests/053_import_compression/brotli",
    )
    .unwrap();
    let client = create_http_client(Some(String::from(
      test_util::root_path()
        .join("std/http/testdata/tls/RootCA.pem")
        .to_str()
        .unwrap(),
    )))
    .unwrap();
    let result = fetch_once(client, &url, None).await;
    if let Ok(FetchOnceResult::Code(body, headers)) = result {
      assert!(!body.is_empty());
      assert_eq!(String::from_utf8(body).unwrap(), "console.log('brotli');");
      assert_eq!(
        headers.get("content-type").unwrap(),
        "application/javascript"
      );
      assert_eq!(headers.get("etag"), None);
      assert_eq!(headers.get("x-typescript-types"), None);
    } else {
      panic!();
    }
    drop(http_server_guard);
  }
}
