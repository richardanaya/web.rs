#include "js-wasm.h"

double _get_element_by_id(double id){
    static int fn;
    
    double a0 = id;
    char *fn_code = "function(id){ return  this.storeObject(.getElementById(id)); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    return js_invoke_function_1(fn, a0);
}
