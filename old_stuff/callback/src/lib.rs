#![no_std]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use spin::{Mutex, MutexGuard};

pub enum CallbackHandler {
    Callback0(Box<dyn FnMut() + Send + 'static>),
    Callback1(Box<dyn FnMut(f64) + Send + 'static>),
    Callback2(Box<dyn FnMut(f64, f64) + Send + 'static>),
    Callback3(Box<dyn FnMut(f64, f64, f64) + Send + 'static>),
    Callback4(Box<dyn FnMut(f64, f64, f64, f64) + Send + 'static>),
    Callback5(Box<dyn FnMut(f64, f64, f64, f64, f64) + Send + 'static>),
    Callback6(Box<dyn FnMut(f64, f64, f64, f64, f64, f64) + Send + 'static>),
    Callback7(Box<dyn FnMut(f64, f64, f64, f64, f64, f64, f64) + Send + 'static>),
    Callback8(Box<dyn FnMut(f64, f64, f64, f64, f64, f64, f64, f64) + Send + 'static>),
    Callback9(Box<dyn FnMut(f64, f64, f64, f64, f64, f64, f64, f64, f64) + Send + 'static>),
    Callback10(Box<dyn FnMut(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) + Send + 'static>),
}

type CallbackHandle = f64;

pub struct CallbackManager {
    cur_id: CallbackHandle,
    pub keys: Vec<CallbackHandle>,
    pub handlers: Vec<Arc<Mutex<CallbackHandler>>>,
}

fn get_callbacks() -> MutexGuard<'static, CallbackManager> {
    static SINGLETON: Mutex<CallbackManager> = {
        Mutex::new(CallbackManager {
            cur_id: 0.0,
            keys: Vec::new(),
            handlers: Vec::new(),
        })
    };
    SINGLETON.lock()
}

pub fn get_callback(id: CallbackHandle) -> Option<Arc<Mutex<CallbackHandler>>> {
    let cbs = get_callbacks();
    let index = cbs.keys.iter().position(|&r| r == id);
    if let Some(i) = index {
        let handler_ref = cbs.handlers.get(i).unwrap().clone();
        core::mem::drop(cbs);
        Some(handler_ref)
    } else {
        None
    }
}

pub fn remove_callback(id: CallbackHandle) {
    let mut cbs = get_callbacks();
    let index = cbs.keys.iter().position(|&r| r == id);
    if let Some(i) = index {
        cbs.keys.remove(i);
        cbs.handlers.remove(i);
    }
}

fn create_callback(cb: CallbackHandler) -> f64 {
    let mut h = get_callbacks();
    h.cur_id += 1.0;
    let id = h.cur_id;
    h.keys.push(id);
    h.handlers.push(Arc::new(Mutex::new(cb)));
    id
}

pub fn create_callback_0(cb: impl FnMut() + Send + 'static) -> f64 {
    create_callback(CallbackHandler::Callback0(Box::new(cb)))
}

pub fn create_callback_1(cb: impl FnMut(f64) + Send + 'static) -> f64 {
    create_callback(CallbackHandler::Callback1(Box::new(cb)))
}

pub fn create_callback_2(cb: impl FnMut(f64, f64) + Send + 'static) -> f64 {
    create_callback(CallbackHandler::Callback2(Box::new(cb)))
}

pub fn create_callback_3(cb: impl FnMut(f64, f64, f64) + Send + 'static) -> f64 {
    create_callback(CallbackHandler::Callback3(Box::new(cb)))
}

pub fn create_callback_4(cb: impl FnMut(f64, f64, f64, f64) + Send + 'static) -> f64 {
    create_callback(CallbackHandler::Callback4(Box::new(cb)))
}

pub fn create_callback_5(cb: impl FnMut(f64, f64, f64, f64, f64) + Send + 'static) -> f64 {
    create_callback(CallbackHandler::Callback5(Box::new(cb)))
}

pub fn create_callback_6(cb: impl FnMut(f64, f64, f64, f64, f64, f64) + Send + 'static) -> f64 {
    create_callback(CallbackHandler::Callback6(Box::new(cb)))
}

pub fn create_callback_7(
    cb: impl FnMut(f64, f64, f64, f64, f64, f64, f64) + Send + 'static,
) -> f64 {
    create_callback(CallbackHandler::Callback7(Box::new(cb)))
}

pub fn create_callback_8(
    cb: impl FnMut(f64, f64, f64, f64, f64, f64, f64, f64) + Send + 'static,
) -> f64 {
    create_callback(CallbackHandler::Callback8(Box::new(cb)))
}

pub fn create_callback_9(
    cb: impl FnMut(f64, f64, f64, f64, f64, f64, f64, f64, f64) + Send + 'static,
) -> f64 {
    create_callback(CallbackHandler::Callback9(Box::new(cb)))
}

pub fn create_callback_10(
    cb: impl FnMut(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) + Send + 'static,
) -> f64 {
    create_callback(CallbackHandler::Callback10(Box::new(cb)))
}

pub struct CallbackFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
    result: Option<f64>,
}

impl Future for CallbackFuture {
    type Output = Option<f64>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture {
    pub fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
            result: None,
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback1(Box::new(move |v: f64| {
            let mut shared_state = thread_shared_state.lock();
            shared_state.completed = true;
            shared_state.result = Some(v);
            if let Some(waker) = shared_state.waker.take() {
                core::mem::drop(shared_state);
                waker.wake()
            }
        })));
        (CallbackFuture { shared_state }, id as f64)
    }
}

struct CallbackFuture0 {
    shared_state: Arc<Mutex<SharedState0>>,
}

/// Shared state between the future and the waiting thread
struct SharedState0 {
    completed: bool,
    waker: Option<Waker>,
    result: (),
}

impl Future for CallbackFuture0 {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture0 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState0 {
            completed: false,
            waker: None,
            result: (),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback0(Box::new(move || {
            let mut shared_state = thread_shared_state.lock();
            shared_state.completed = true;
            shared_state.result = ();
            if let Some(waker) = shared_state.waker.take() {
                core::mem::drop(shared_state);
                waker.wake()
            }
        })));
        (CallbackFuture0 { shared_state }, id)
    }
}

pub fn create_callback_future_0() -> (impl Future, f64) {
    CallbackFuture0::new()
}

struct CallbackFuture1 {
    shared_state: Arc<Mutex<SharedState1>>,
}

/// Shared state between the future and the waiting thread
struct SharedState1 {
    completed: bool,
    waker: Option<Waker>,
    result: f64,
}

impl Future for CallbackFuture1 {
    type Output = f64;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture1 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState1 {
            completed: false,
            waker: None,
            result: 0.0,
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback1(Box::new(move |v: f64| {
            let mut shared_state = thread_shared_state.lock();
            shared_state.completed = true;
            shared_state.result = v;
            if let Some(waker) = shared_state.waker.take() {
                core::mem::drop(shared_state);
                waker.wake()
            }
        })));
        (CallbackFuture1 { shared_state }, id)
    }
}

pub fn create_callback_future_1() -> (impl Future, f64) {
    CallbackFuture1::new()
}

struct CallbackFuture2 {
    shared_state: Arc<Mutex<SharedState2>>,
}

/// Shared state between the future and the waiting thread
struct SharedState2 {
    completed: bool,
    waker: Option<Waker>,
    result: (f64, f64),
}

impl Future for CallbackFuture2 {
    type Output = (f64, f64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture2 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState2 {
            completed: false,
            waker: None,
            result: (0.0, 0.0),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback2(Box::new(
            move |a1: f64, a2: f64| {
                let mut shared_state = thread_shared_state.lock();
                shared_state.completed = true;
                shared_state.result = (a1, a2);
                if let Some(waker) = shared_state.waker.take() {
                    core::mem::drop(shared_state);
                    waker.wake()
                }
            },
        )));
        (CallbackFuture2 { shared_state }, id)
    }
}

pub fn create_callback_future_2() -> (impl Future, f64) {
    CallbackFuture2::new()
}

struct CallbackFuture3 {
    shared_state: Arc<Mutex<SharedState3>>,
}

/// Shared state between the future and the waiting thread
struct SharedState3 {
    completed: bool,
    waker: Option<Waker>,
    result: (f64, f64, f64),
}

impl Future for CallbackFuture3 {
    type Output = (f64, f64, f64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture3 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState3 {
            completed: false,
            waker: None,
            result: (0.0, 0.0, 0.0),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback3(Box::new(
            move |a1: f64, a2: f64, a3: f64| {
                let mut shared_state = thread_shared_state.lock();
                shared_state.completed = true;
                shared_state.result = (a1, a2, a3);
                if let Some(waker) = shared_state.waker.take() {
                    core::mem::drop(shared_state);
                    waker.wake()
                }
            },
        )));
        (CallbackFuture3 { shared_state }, id)
    }
}

pub fn create_callback_future_3() -> (impl Future, f64) {
    CallbackFuture3::new()
}

struct CallbackFuture4 {
    shared_state: Arc<Mutex<SharedState4>>,
}

/// Shared state between the future and the waiting thread
struct SharedState4 {
    completed: bool,
    waker: Option<Waker>,
    result: (f64, f64, f64, f64),
}

impl Future for CallbackFuture4 {
    type Output = (f64, f64, f64, f64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture4 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState4 {
            completed: false,
            waker: None,
            result: (0.0, 0.0, 0.0, 0.0),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback4(Box::new(
            move |a1: f64, a2: f64, a3: f64, a4: f64| {
                let mut shared_state = thread_shared_state.lock();
                shared_state.completed = true;
                shared_state.result = (a1, a2, a3, a4);
                if let Some(waker) = shared_state.waker.take() {
                    core::mem::drop(shared_state);
                    waker.wake()
                }
            },
        )));
        (CallbackFuture4 { shared_state }, id)
    }
}

pub fn create_callback_future_4() -> (impl Future, f64) {
    CallbackFuture4::new()
}

struct CallbackFuture5 {
    shared_state: Arc<Mutex<SharedState5>>,
}

/// Shared state between the future and the waiting thread
struct SharedState5 {
    completed: bool,
    waker: Option<Waker>,
    result: (f64, f64, f64, f64, f64),
}

impl Future for CallbackFuture5 {
    type Output = (f64, f64, f64, f64, f64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture5 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState5 {
            completed: false,
            waker: None,
            result: (0.0, 0.0, 0.0, 0.0, 0.0),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback5(Box::new(
            move |a1: f64, a2: f64, a3: f64, a4: f64, a5: f64| {
                let mut shared_state = thread_shared_state.lock();
                shared_state.completed = true;
                shared_state.result = (a1, a2, a3, a4, a5);
                if let Some(waker) = shared_state.waker.take() {
                    core::mem::drop(shared_state);
                    waker.wake()
                }
            },
        )));
        (CallbackFuture5 { shared_state }, id)
    }
}

pub fn create_callback_future_5() -> (impl Future, f64) {
    CallbackFuture5::new()
}

struct CallbackFuture6 {
    shared_state: Arc<Mutex<SharedState6>>,
}

/// Shared state between the future and the waiting thread
struct SharedState6 {
    completed: bool,
    waker: Option<Waker>,
    result: (f64, f64, f64, f64, f64, f64),
}

impl Future for CallbackFuture6 {
    type Output = (f64, f64, f64, f64, f64, f64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture6 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState6 {
            completed: false,
            waker: None,
            result: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback6(Box::new(
            move |a1: f64, a2: f64, a3: f64, a4: f64, a5: f64, a6: f64| {
                let mut shared_state = thread_shared_state.lock();
                shared_state.completed = true;
                shared_state.result = (a1, a2, a3, a4, a5, a6);
                if let Some(waker) = shared_state.waker.take() {
                    core::mem::drop(shared_state);
                    waker.wake()
                }
            },
        )));
        (CallbackFuture6 { shared_state }, id)
    }
}

pub fn create_callback_future_6() -> (impl Future, f64) {
    CallbackFuture6::new()
}

struct CallbackFuture7 {
    shared_state: Arc<Mutex<SharedState7>>,
}

/// Shared state between the future and the waiting thread
struct SharedState7 {
    completed: bool,
    waker: Option<Waker>,
    result: (f64, f64, f64, f64, f64, f64, f64),
}

impl Future for CallbackFuture7 {
    type Output = (f64, f64, f64, f64, f64, f64, f64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture7 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState7 {
            completed: false,
            waker: None,
            result: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback7(Box::new(
            move |a1: f64, a2: f64, a3: f64, a4: f64, a5: f64, a6: f64, a7: f64| {
                let mut shared_state = thread_shared_state.lock();
                shared_state.completed = true;
                shared_state.result = (a1, a2, a3, a4, a5, a6, a7);
                if let Some(waker) = shared_state.waker.take() {
                    core::mem::drop(shared_state);
                    waker.wake()
                }
            },
        )));
        (CallbackFuture7 { shared_state }, id)
    }
}

pub fn create_callback_future_7() -> (impl Future, f64) {
    CallbackFuture7::new()
}

struct CallbackFuture8 {
    shared_state: Arc<Mutex<SharedState8>>,
}

/// Shared state between the future and the waiting thread
struct SharedState8 {
    completed: bool,
    waker: Option<Waker>,
    result: (f64, f64, f64, f64, f64, f64, f64, f64),
}

impl Future for CallbackFuture8 {
    type Output = (f64, f64, f64, f64, f64, f64, f64, f64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture8 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState8 {
            completed: false,
            waker: None,
            result: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback8(Box::new(
            move |a1: f64, a2: f64, a3: f64, a4: f64, a5: f64, a6: f64, a7: f64, a8: f64| {
                let mut shared_state = thread_shared_state.lock();
                shared_state.completed = true;
                shared_state.result = (a1, a2, a3, a4, a5, a6, a7, a8);
                if let Some(waker) = shared_state.waker.take() {
                    core::mem::drop(shared_state);
                    waker.wake()
                }
            },
        )));
        (CallbackFuture8 { shared_state }, id)
    }
}

pub fn create_callback_future_8() -> (impl Future, f64) {
    CallbackFuture8::new()
}

struct CallbackFuture9 {
    shared_state: Arc<Mutex<SharedState9>>,
}

/// Shared state between the future and the waiting thread
struct SharedState9 {
    completed: bool,
    waker: Option<Waker>,
    result: (f64, f64, f64, f64, f64, f64, f64, f64, f64),
}

impl Future for CallbackFuture9 {
    type Output = (f64, f64, f64, f64, f64, f64, f64, f64, f64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture9 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState9 {
            completed: false,
            waker: None,
            result: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback9(Box::new(
            move |a1: f64,
                  a2: f64,
                  a3: f64,
                  a4: f64,
                  a5: f64,
                  a6: f64,
                  a7: f64,
                  a8: f64,
                  a9: f64| {
                let mut shared_state = thread_shared_state.lock();
                shared_state.completed = true;
                shared_state.result = (a1, a2, a3, a4, a5, a6, a7, a8, a9);
                if let Some(waker) = shared_state.waker.take() {
                    core::mem::drop(shared_state);
                    waker.wake()
                }
            },
        )));
        (CallbackFuture9 { shared_state }, id)
    }
}

pub fn create_callback_future_9() -> (impl Future, f64) {
    CallbackFuture9::new()
}

struct CallbackFuture10 {
    shared_state: Arc<Mutex<SharedState10>>,
}

/// Shared state between the future and the waiting thread
struct SharedState10 {
    completed: bool,
    waker: Option<Waker>,
    result: (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64),
}

impl Future for CallbackFuture10 {
    type Output = (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock();
        if shared_state.completed {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl CallbackFuture10 {
    fn new() -> (Self, f64) {
        let shared_state = Arc::new(Mutex::new(SharedState10 {
            completed: false,
            waker: None,
            result: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }));

        let thread_shared_state = shared_state.clone();
        let id = create_callback(CallbackHandler::Callback10(Box::new(
            move |a1: f64,
                  a2: f64,
                  a3: f64,
                  a4: f64,
                  a5: f64,
                  a6: f64,
                  a7: f64,
                  a8: f64,
                  a9: f64,
                  a10: f64| {
                let mut shared_state = thread_shared_state.lock();
                shared_state.completed = true;
                shared_state.result = (a1, a2, a3, a4, a5, a6, a7, a8, a9, a10);
                if let Some(waker) = shared_state.waker.take() {
                    core::mem::drop(shared_state);
                    waker.wake()
                }
            },
        )));
        (CallbackFuture10 { shared_state }, id)
    }
}

pub fn create_callback_future_10() -> (impl Future, f64) {
    CallbackFuture10::new()
}

#[no_mangle]
fn handle_callback(
    id: f64,
    a1: f64,
    a2: f64,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    a7: f64,
    a8: f64,
    a9: f64,
    a10: f64,
) {
    let h = get_callback(id);
    let handler_ref = h.unwrap();
    let mut handler = handler_ref.lock();
    match &mut *handler {
        CallbackHandler::Callback0(c) => c(),
        CallbackHandler::Callback1(c) => c(a1),
        CallbackHandler::Callback2(c) => c(a1, a2),
        CallbackHandler::Callback3(c) => c(a1, a2, a3),
        CallbackHandler::Callback4(c) => c(a1, a2, a3, a4),
        CallbackHandler::Callback5(c) => c(a1, a2, a3, a4, a5),
        CallbackHandler::Callback6(c) => c(a1, a2, a3, a4, a5, a6),
        CallbackHandler::Callback7(c) => c(a1, a2, a3, a4, a5, a6, a7),
        CallbackHandler::Callback8(c) => c(a1, a2, a3, a4, a5, a6, a7, a8),
        CallbackHandler::Callback9(c) => c(a1, a2, a3, a4, a5, a6, a7, a8, a9),
        CallbackHandler::Callback10(c) => c(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10),
    }
}
