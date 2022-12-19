#![no_std]
use js::*;
#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

pub type HTMLElement = f64;

#[derive(Copy, Clone)]
struct Destructable {
    function: Option<JSFunction>,
}

pub trait CustomElement {
    fn new(element: HTMLElement) -> Self
    where
        Self: core::marker::Sized + core::marker::Sync + core::marker::Send + 'static;
    fn register(name: &str)
    where
        Self: core::marker::Sized + core::marker::Sync + core::marker::Send + 'static,
    {
        let construct = create_callback_1(|element| {
            let el = Arc::new(Mutex::new(Self::new(element.into())));
            let el1 = el.clone();
            let el2 = el.clone();
            let el3 = el.clone();

            let destruct_connect = Arc::new(Mutex::new(Destructable { function: None }));
            let connect = create_callback_0(move || {
                el1.lock().connected();
            });
            destruct_connect.lock().function = Some(connect.into());

            let destruct_attribute_change = Arc::new(Mutex::new(Destructable { function: None }));
            let attribute_change = create_callback_3(move |name_obj, old_obj, new_obj| {
                let name = cstr_to_string(name_obj as i32);
                let old = if old_obj == -1.0 {
                    None
                } else {
                    Some(cstr_to_string(old_obj as i32))
                };
                let new = if new_obj == -1.0 {
                    None
                } else {
                    Some(cstr_to_string(new_obj as i32))
                };
                el3.lock().attribute_changed(name, old, new);
            });

            destruct_attribute_change.lock().function = Some(connect.into());

            let destruct_disconnect = Arc::new(Mutex::new(Destructable { function: None }));
            let destruct_disconnect2 = destruct_disconnect.clone();
            let disconnect = create_callback_0(move || {
                el2.lock().disconnected();
                remove_callback(destruct_connect.lock().function.as_ref().unwrap().into());
                remove_callback(destruct_disconnect.lock().function.as_ref().unwrap().into());
                remove_callback(
                    destruct_attribute_change
                        .lock()
                        .function
                        .as_ref()
                        .unwrap()
                        .into(),
                );
            });
            destruct_disconnect2.lock().function = Some(disconnect.into());

            lazy_static::lazy_static! {
                static ref FN: JSFunction= {
                register_function(
                    r#"function(e,a,b,c){
                        e.addHooks(a,b,c);
                    }"#,
                )
            };};
            FN.invoke_4(element, connect, disconnect, attribute_change);
        });
        let attrs = Self::observed_attributes().join(",");
        lazy_static::lazy_static! {
            static ref FN: JSFunction= {
            register_function(
                r#"function(construct,elementNamePtr, elementNameLen,attrNamesPtr,attrNamesLen){
                    const elementName = this.readUtf8FromMemory(elementNamePtr,elementNameLen);
                    const attrNames = this.readUtf8FromMemory(attrNamesPtr,attrNamesLen);
                    let attrs = attrNames.split(",");
                    class GeneratedCustomElement extends HTMLElement {
                      constructor() {
                          super();
                          construct(this);
                      }
    
                      static get observedAttributes() {
                        return attrs;
                      }
    
                      connectedCallback() {
                        self.connect();
                      }
    
                      disconnectedCallback() {
                        self.disconnect();
                      }
    
                      attributeChangedCallback(attributeName, oldValue, newValue) {
                        self.attributeChange(attributeName,oldValue,newValue)
                      }
    
                      addHooks(connect,disconnect,attributeChange){
                        self.connect = connect;
                        self.disconnect = disconnect;
                        self.attributeChange = attributeChange;
                      }
                    }
    
                    // tell the dom to associate it with an html tag name
                    customElements.define(elementName, GeneratedCustomElement);
                  }"#,
            )
        };};
        FN.invoke_5(
            construct,
            name.as_ptr() as u32,
            name.len() as u32,
            attrs.as_ptr() as u32,
            attrs.len() as u32,
        );
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec![]
    }

    fn created(&mut self) {}
    fn connected(&mut self) {}
    fn disconnected(&mut self) {}
    fn attribute_changed(
        &mut self,
        _name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
    }
}
