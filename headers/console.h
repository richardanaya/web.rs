#include "js-wasm.h"

void console_log(char *msg){
	static int log;
	char * code = "function(strPtr,strLen){\
            console.log(this.readUtf8FromMemory(strPtr,strLen));\
        }";
	if(log == 0){
		log = js_register_function(code,js_strlen(code));
	}
	js_invoke_function_2(log,(unsigned int)msg,js_strlen(msg));
}