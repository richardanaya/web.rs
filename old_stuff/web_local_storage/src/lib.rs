#![no_std]
extern crate alloc;
use alloc::string::String;

use js::*;

pub fn local_storage_set_item(id: &str, data: &str) {
    let a0 = id.as_ptr() as u32;
    let a1 = id.len() as u32;
    let d0 = data.as_ptr() as u32;
    let d1 = data.len() as u32;
    let func = js!(r###"function(idPtr,idLen,dataPtr,dataLen){
                window.localStorage.setItem(this.readUtf8FromMemory(idPtr,idLen),this.readUtf8FromMemory(dataPtr,dataLen));
        }"###);
    func.invoke_4(a0, a1, d0, d1);
}

pub fn local_storage_get_item(id: &str) -> Option<String> {
    let a0 = id.as_ptr() as u32;
    let a1 = id.len() as u32;
    let func = js!(r###"function(idPtr,idLen){
                const a = window.localStorage.getItem(this.readUtf8FromMemory(idPtr,idLen));
                if(a === null){
                    return -1;
                } 
                return this.writeUtf8ToMemory(a);
        }"###);
    let txt = func.invoke_2(a0, a1);
    if txt == -1.0 {
        return None;
    } else {
        let allocation_id = txt as usize;
        let s = extract_string_from_memory(allocation_id);
        clear_allocation(allocation_id);
        Some(s)
    }
}

pub fn local_storage_remove_item(id: &str) {
    let a0 = id.as_ptr() as u32;
    let a1 = id.len() as u32;
    let func = js!(r###"function(idPtr,idLen){
                window.localStorage.removeItem(this.readUtf8FromMemory(idPtr,idLen));
        }"###);
    func.invoke_2(a0, a1);
}

pub fn local_storage_clear() {
    let func = js!(r###"function(idPtr,idLen,dataPtr,dataLen){
                window.localStorage.clear();
        }"###);
    func.invoke_0();
}
