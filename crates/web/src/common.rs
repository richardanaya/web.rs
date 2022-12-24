use core::hash::{Hash, Hasher};
use js::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
pub struct FunctionHandle(pub ExternRef);

impl PartialEq for FunctionHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0.value == other.0.value
    }
}

impl Eq for FunctionHandle {}

impl Hash for FunctionHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.value.hash(state);
    }
}

pub struct EventHandler<T> {
    listeners: Mutex<Option<HashMap<Arc<FunctionHandle>, Box<dyn FnMut(T) + Send + 'static>>>>,
}

impl<T> EventHandler<T> {
    pub const fn new() -> Self {
        Self {
            listeners: Mutex::new(None),
        }
    }

    pub fn add_listener(
        &self,
        id: Arc<FunctionHandle>,
        handler: Box<dyn FnMut(T) + Send + 'static>,
    ) {
        let mut handlers = self.listeners.lock().unwrap();
        if let Some(h) = handlers.as_mut() {
            h.insert(id, handler);
        } else {
            let mut h = HashMap::new();
            h.insert(id, handler);
            *handlers = Some(h);
        }
    }

    pub fn remove_listener(&self, id: &Arc<FunctionHandle>) {
        let mut handlers = self.listeners.lock().unwrap();
        if let Some(h) = handlers.as_mut() {
            h.remove(id);
        }
    }

    pub fn call(&self, id: i64, event: T) {
        let mut handlers = self.listeners.lock().unwrap();
        if let Some(h) = handlers.as_mut() {
            for (key, handler) in h.iter_mut() {
                if key.0.value == id {
                    handler(event);
                }
                return;
            }
        }
    }
}
