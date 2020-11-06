#ifndef export
#define export __attribute__((visibility("default")))
#endif

#ifndef JS_H
typedef double JSValue;
typedef int JSFunction;
extern JSFunction js_register_function(char*,uint len);
extern JSValue js_invoke_function(JSFunction,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue);

JSValue const JS_NULL = 0.0;
JSValue const JS_UNDEFINED = 1.0;
JSValue const JS_SELF = 2.0;
JSValue const JS_WINDOW = 2.0;
JSValue const JS_DOCUMENT = 3.0;
#endif