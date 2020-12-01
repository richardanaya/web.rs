#include "js-wasm.h"

void _get_context(double el){
    static int fn;
    
    double a0 = el;
    char *fn_code = "function(el){ .getContext(el); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_1(fn, a0);
}

void _fill_rect(double ctxdouble xdouble ydouble heightdouble width){
    static int fn;
    
    double a0 = ctx;
    
    double a1 = x;
    
    double a2 = y;
    
    double a3 = height;
    
    double a4 = width;
    char *fn_code = "function(ctxxyheightwidth){ .fillRect(ctxxyheightwidth); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_5(fn, a0a0a0a0a0);
}

void _clear_rect(double ctxdouble xdouble ydouble heightdouble width){
    static int fn;
    
    double a0 = ctx;
    
    double a1 = x;
    
    double a2 = y;
    
    double a3 = height;
    
    double a4 = width;
    char *fn_code = "function(ctxxyheightwidth){ .clearRect(ctxxyheightwidth); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_5(fn, a0a0a0a0a0);
}
