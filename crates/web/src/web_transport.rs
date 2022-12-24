use crate::common::*;
use js::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

struct WebTransport{
    handle:ExternRef
}

impl WebTransport {
    pub fn new(endpoint: &str){
        let transport = js!("
            function(endpoint) {
                return new WebTransport(endpoint);
            }
            ")
        .invoke_and_return_object(&[endpoint.into()]);
        WebTransport{handle:transport}
    }
}

