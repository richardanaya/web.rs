#include "js-wasm.h"

double document_get_element_by_id(char * id){
    static int fn;
    unsigned int a0 = (unsigned int)id;
    unsigned int a1 = js_strlen(id);
    char *fn_code = "function(idPtr,idLen){ return  this.storeObject(document.getElementById(this.readUtf8FromMemory(idPtr,idLen))); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    return js_invoke_function_2(fn, a0, a1);
}
