#include "js-wasm.h"

void canvas_get_context(double el){
    static int fn;
    
    double a0 = el;
    char *fn_code = "function(el){ canvas.getContext(el); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_1(fn, a0);
}

void canvas_fill_rect(double ctx, double x, double y, double height, double width){
    static int fn;
    
    double a0 = ctx;
    
    double a1 = x;
    
    double a2 = y;
    
    double a3 = height;
    
    double a4 = width;
    char *fn_code = "function(ctx, x, y, height, width){ canvas.fillRect(ctx, x, y, height, width); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_5(fn, a0, a0, a0, a0, a0);
}

void canvas_clear_rect(double ctx, double x, double y, double height, double width){
    static int fn;
    
    double a0 = ctx;
    
    double a1 = x;
    
    double a2 = y;
    
    double a3 = height;
    
    double a4 = width;
    char *fn_code = "function(ctx, x, y, height, width){ canvas.clearRect(ctx, x, y, height, width); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_5(fn, a0, a0, a0, a0, a0);
}
