#include "js-wasm.h"

void console_clear(){
    static int fn;
    char *fn_code = "function(){ console.clear(); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_0(fn);
}

void console_log(char * msg){
    static int fn;
    unsigned int a0 = (unsigned int)msg;
    unsigned int a1 = js_strlen(msg);
    char *fn_code = "function(msgPtr,msgLen){ console.log(this.readUtf8FromMemory(msgPtr,msgLen)); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_2(fn, a0, a1);
}

void console_warning(char * msg){
    static int fn;
    unsigned int a0 = (unsigned int)msg;
    unsigned int a1 = js_strlen(msg);
    char *fn_code = "function(msgPtr,msgLen){ console.warn(this.readUtf8FromMemory(msgPtr,msgLen)); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_2(fn, a0, a1);
}

void console_error(char * msg){
    static int fn;
    unsigned int a0 = (unsigned int)msg;
    unsigned int a1 = js_strlen(msg);
    char *fn_code = "function(msgPtr,msgLen){ console.error(this.readUtf8FromMemory(msgPtr,msgLen)); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_2(fn, a0, a1);
}

void console_time(char * msg){
    static int fn;
    unsigned int a0 = (unsigned int)msg;
    unsigned int a1 = js_strlen(msg);
    char *fn_code = "function(msgPtr,msgLen){ console.time(this.readUtf8FromMemory(msgPtr,msgLen)); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_2(fn, a0, a1);
}

void console_time_end(char * msg){
    static int fn;
    unsigned int a0 = (unsigned int)msg;
    unsigned int a1 = js_strlen(msg);
    char *fn_code = "function(msgPtr,msgLen){ console.timeEnd(this.readUtf8FromMemory(msgPtr,msgLen)); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_2(fn, a0, a1);
}
