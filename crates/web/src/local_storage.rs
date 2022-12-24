use js::*;

pub fn local_storage_set(key: &str, value: &str) {
    let local_storage_set = js!(r#"
        function(key, value){
            localStorage.setItem(key, value);
        }"#);
    local_storage_set.invoke(&[key.into(), value.into()]);
}

pub fn local_storage_remove(key: &str) {
    let local_storage_remove = js!(r#"
        function(key){
            localStorage.removeItem(key);
        }"#);
    local_storage_remove.invoke(&[key.into()]);
}

pub fn local_storage_get(key: &str) -> Option<String> {
    let local_storage_get = js!(r#"
        function(key){
            const text = localStorage.getItem(key);
            if(text === null){
                return 0;
            }
            const allocationId = this.writeUtf8ToMemory(text);
            return allocationId;
        }"#);
    let text_allocation_id = local_storage_get.invoke(&[key.into()]);
    if text_allocation_id == 0.0 {
        return None;
    }
    let text = extract_string_from_memory(text_allocation_id as usize);
    Some(text)
}

pub fn local_storage_clear() {
    let local_storage_clear = js!(r#"
        function(){
            localStorage.clear();
        }"#);
    local_storage_clear.invoke(&[]);
}
