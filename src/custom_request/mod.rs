pub mod custom_request {
    use bytes::Bytes;
    use reqwest::Client;
    use reqwest::Method;
    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct CustomResponse {
        pub headers: HashMap<String, String>,
        pub status_code: String,
        pub version: Option<String>,
        pub url: String,
        pub response_text: String,
        pub cookies: HashMap<String, String>,
    }
    /// This function converts a HeaderMap to a header HashMap
    fn get_response_headers(
        response_headers: &reqwest::header::HeaderMap,
    ) -> HashMap<String, String> {
        let mut headers = HashMap::<String, String>::new();
        for (key, value) in response_headers.iter() {
            headers.insert(key.to_string(), (&value.to_str().unwrap()).to_string());
        }
        return headers;
    }
    /// This function creates a header HashMap from a HashMap  
    fn create_request_header(
        request_headers: HashMap<String, String>,
    ) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        for (k, v) in request_headers.iter() {
            headers.insert(
                reqwest::header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                reqwest::header::HeaderValue::from_str(v).unwrap(),
            );
        }
        return headers;
    }
    /// This function returns a Http Method from an Input String
    fn get_http_method(method: String) -> Method {
        let http_method = match method.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "TRACE" => Method::TRACE,
            "DELETE" => Method::DELETE,
            "PUT" => Method::PUT,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            "CONNECT" => Method::CONNECT,
            "PATCH" => Method::PATCH,
            _ => Method::GET,
        };
        return http_method;
    }
    /// This function builds a http request and returns the request
    pub async fn create_request(
        url: String,
        method: String,
        request_headers: Option<HashMap<String, String>>,
        request_query_params: Option<HashMap<String, String>>,
        request_form: Option<HashMap<String, String>>,
        request_body: Option<Bytes>,
    ) -> Result<CustomResponse, reqwest::Error> {
        let http_method = get_http_method(method);
        let client = Client::new();
        let mut req = client.request(http_method, url);
        if !request_headers.is_none() {
            req = req.headers(create_request_header(request_headers.unwrap()));
        }
        if !request_query_params.is_none() {
            req = req.query(&request_query_params.unwrap());
        }
        if !request_form.is_none() {
            req = req.query(&request_form.unwrap());
        }
        if !request_body.is_none() {
            req = req.body(request_body.unwrap());
        }
        let res = req.send();
        let res = match res.await {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        let res_headers_response_map = res.headers();
        let res_map_headers = get_response_headers(res_headers_response_map);
        let mut res_cookies = HashMap::<String, String>::new();
        for cookie in res.cookies() {
            res_cookies.insert(cookie.name().to_string(), cookie.value().to_string());
        }

        let response = CustomResponse {
            headers: res_map_headers,
            version: None,
            url: res.url().to_string(),
            cookies: res_cookies,
            status_code: res.status().to_string(),
            response_text: res.text().await.unwrap(),
        };
        return Ok(response);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use tokio_test;
    #[test]
    fn test_get_request() {
        let response = tokio_test::block_on(super::custom_request::create_request(
            "https://httpbin.org/get".to_string(),
            "GET".to_string(),
            None,
            None,
            None,
            None,
        ))
        .unwrap();
        assert_eq!("200 OK", response.status_code);
    }

    #[test]
    fn test_post_request() {
        let response = tokio_test::block_on(super::custom_request::create_request(
            "https://httpbin.org/post".to_string(),
            "POST".to_string(),
            None,
            None,
            None,
            None,
        ))
        .unwrap();
        assert_eq!("200 OK", response.status_code);
    }

    #[test]
    fn test_put_request() {
        let response = tokio_test::block_on(super::custom_request::create_request(
            "https://httpbin.org/put".to_string(),
            "PUT".to_string(),
            None,
            None,
            None,
            None,
        ))
        .unwrap();
        assert_eq!("200 OK", response.status_code);
    }

    #[test]
    fn test_delete_request() {
        let response = tokio_test::block_on(super::custom_request::create_request(
            "https://httpbin.org/delete".to_string(),
            "DELETE".to_string(),
            None,
            None,
            None,
            None,
        ))
        .unwrap();
        assert_eq!("200 OK", response.status_code);
    }

    #[test]
    fn test_headers_request() {
        let headers = HashMap::from([
            (String::from("test_1"), String::from("Hallo")),
            (String::from("test_2"), String::from("Hallo 1")),
            (String::from("test_3"), String::from("Hallo 3")),
        ]);
        let response = tokio_test::block_on(super::custom_request::create_request(
            "https://httpbin.org/headers".to_string(),
            "GET".to_string(),
            Some(headers),
            None,
            None,
            None,
        ))
        .unwrap();
        assert_eq!("200 OK", response.status_code);
    }
    #[test]
    fn test_query_params() {
        let query_params = HashMap::from([
            (String::from("test_1"), String::from("Hallo")),
            (String::from("test_2"), String::from("Hallo 1")),
            (String::from("test_3"), String::from("Hallo 3")),
        ]);

        let response = tokio_test::block_on(super::custom_request::create_request(
            "https://httpbin.org/get".to_string(),
            "GET".to_string(),
            None,
            Some(query_params),
            None,
            None,
        ))
        .unwrap();
        assert_eq!("200 OK", response.status_code);
    }

    #[test]
    fn test_form_params() {
        let form_params = HashMap::from([
            (String::from("test_1"), String::from("Hallo")),
            (String::from("test_2"), String::from("Hallo 1")),
            (String::from("test_3"), String::from("Hallo 3")),
        ]);

        let response = tokio_test::block_on(super::custom_request::create_request(
            "https://httpbin.org/put".to_string(),
            "PUT".to_string(),
            None,
            None,
            Some(form_params),
            None,
        ))
        .unwrap();
        assert_eq!("200 OK", response.status_code);
    }
    #[test]
    fn test_xml() {
        let headers = HashMap::from([(String::from("accept"), String::from("application/xml"))]);
        let req_body = "<?xml version=\'1.0\' encoding=\'us-ascii\'?>
        <!--  A SAMPLE set of slides  -->
        <slideshow 
          title=\"Sample Slide Show\"
          date=\"Date of publication\"
          author=\"Yours Truly\"
          >
          <!-- TITLE SLIDE -->
          <slide type=\"all\">
            <title>Wake up to WonderWidgets!</title>
          </slide>
          <!-- OVERVIEW -->
          <slide type=\"all\">
            <title>Overview</title>
            <item>
              Why 
              <em>WonderWidgets</em>
               are great
            </item>
            <item/>
            <item>
              Who 
              <em>buys</em>
               WonderWidgets
            </item>
          </slide>
        </slideshow>
      ";
        let response = tokio_test::block_on(super::custom_request::create_request(
            "https://httpbin.org/put".to_string(),
            "PUT".to_string(),
            Some(headers),
            None,
            None,
            Some(bytes::Bytes::from(req_body)),
        ))
        .unwrap();
        let v: serde_json::Value = serde_json::from_str(&response.response_text).unwrap();
        assert_eq!(v["data"], req_body);
    }
    #[test]
    fn test_json() {
        let headers = HashMap::from([(String::from("accept"), String::from("application/json"))]);
        let req_body = r#"{
            "slideshow": {
                "author": "Yours Truly",
                "date": "date of publication",
                "slides": [
                    {
                    "title": "Wake up to WonderWidgets!",
                    "type": "all"
                    },
                {
                    "items": [
                        "Why <em>WonderWidgets</em> are great",
                        "Who <em>buys</em> WonderWidgets"
                        ],
                        "title": "Overview",
                        "type": "all"
                        }
                        ],
                        "title": "Sample Slide Show"
                        }
          }"#;
        let response = tokio_test::block_on(super::custom_request::create_request(
            "https://httpbin.org/put".to_string(),
            "PUT".to_string(),
            Some(headers),
            None,
            None,
            Some(bytes::Bytes::from(req_body)),
        ))
        .unwrap();
        let v: serde_json::Value = serde_json::from_str(&response.response_text).unwrap();
        assert_eq!(v["data"], req_body);
    }
}
