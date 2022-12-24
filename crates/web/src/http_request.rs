use js::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

static HTTP_LOAD_HANDLERS: Mutex<Option<HashMap<i64, Box<dyn FnMut() + Send + 'static>>>> =
    Mutex::new(None);

fn add_http_load_event_handler(function_handle: i64, handler: Box<dyn FnMut() + Send + 'static>) {
    let mut h = HTTP_LOAD_HANDLERS.lock().unwrap();
    if h.is_none() {
        *h = Some(HashMap::new());
    }
    h.as_mut().unwrap().insert(function_handle, handler);
}

#[no_mangle]
pub extern "C" fn web_handle_http_load_event_handler(id: i64) {
    let mut c = None;
    {
        let mut handlers = HTTP_LOAD_HANDLERS.lock().unwrap();
        if let Some(h) = handlers.as_mut() {
            // remove
            if let Some(handler) = h.remove(&id) {
                c = Some(handler);
            }
        }
    }
    if let Some(mut c) = c {
        c();
    }
}

pub struct XMLHttpRequest(ExternRef);

impl XMLHttpRequest {
    pub fn new() -> XMLHttpRequest {
        let request = js!("
            function() {
                return new XMLHttpRequest();
            }
            ")
        .invoke_and_return_object(&[]);
        XMLHttpRequest(request)
    }

    pub fn open(&self, method: &str, url: &str) {
        js!("
            function(request, method, url) {
                request.open(method, url);
            }
            ")
        .invoke(&[(&(self.0)).into(), method.into(), url.into()]);
    }

    pub fn send(&self) {
        js!("
            function(request) {
                request.send();
            }
            ")
        .invoke(&[(&(self.0)).into()]);
    }

    pub fn send_with_body(&self, body: &str) {
        js!("
            function(request, body) {
                request.send(body);
            }
            ")
        .invoke(&[(&(self.0)).into(), body.into()]);
    }

    pub fn set_request_header(&self, key: &str, value: &str) {
        js!("
            function(request, key, value) {
                request.setRequestHeader(key, value);
            }
            ")
        .invoke(&[(&(self.0)).into(), key.into(), value.into()]);
    }

    pub fn response_status(&self) -> usize {
        js!("
            function(request) {
                return request.status;
            }
            ")
        .invoke(&[(&(self.0)).into()]) as usize
    }

    pub fn response_text(&self) -> String {
        js!("
            function(request) {
                return request.responseText;
            }
            ")
        .invoke_and_return_string(&[(&(self.0)).into()])
    }

    pub fn response_header(&self, key: &str) -> String {
        js!("
            function(request, key) {
                return request.getResponseHeader(key);
            }
            ")
        .invoke_and_return_string(&[(&(self.0)).into(), key.into()])
    }

    pub fn set_on_load(&self, callback: impl FnMut() + Send + 'static) {
        let function_ref = js!(r#"
            function(request){
                const handler = () => {
                    this.module.instance.exports.web_handle_http_load_event_handler(id);
                    this.releaseObject(id);
                };
                const id = this.storeObject(handler);
                request.onload = handler;
                return id;
            }"#)
        .invoke_and_return_bigint(&[(&(self.0)).into()]);
        add_http_load_event_handler(function_ref, Box::new(callback));
    }

    pub fn set_response_type(&self, response_type: &str) {
        js!("
            function(request, response_type) {
                request.responseType = response_type;
            }
            ")
        .invoke(&[(&(self.0)).into(), response_type.into()]);
    }
}

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
}

pub fn fetch(
    url: &str,
    action: HTTPMethod,
    body: Option<&str>,
    headers: Option<HashMap<String, String>>,
    mut callback: impl FnMut(usize, String) + Send + 'static,
) {
    let request = Arc::new(XMLHttpRequest::new());
    let r2 = request.clone();
    let method_str = match action {
        HTTPMethod::GET => "GET",
        HTTPMethod::POST => "POST",
        HTTPMethod::PUT => "PUT",
        HTTPMethod::DELETE => "DELETE",
        HTTPMethod::HEAD => "HEAD",
        HTTPMethod::OPTIONS => "OPTIONS",
        HTTPMethod::PATCH => "PATCH",
    };
    request.open(method_str, url);
    if let Some(body) = body {
        request.send_with_body(body);
    } else {
        request.send();
    }
    if let Some(headers) = headers {
        for (key, value) in headers {
            request.set_request_header(&key, &value);
        }
    }
    request.set_on_load(move || {
        let status = r2.response_status();
        let text = r2.response_text();
        callback(status, text);
    });
}
