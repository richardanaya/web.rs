#ifndef export
#define export __attribute__((visibility("default")))
#endif

#ifndef RAMEN_H
typedef double JSValue;
extern int js_register_function(char*,usize len);
extern JSValue js_register_function(int,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue);

JSValue const JS_NULL = 0.0;
JSValue const JS_UNDEFINED = 1.0;
JSValue const JS_SELF = 2.0;
JSValue const JS_WINDOW = 2.0;
JSValue const JS_DOCUMENT = 3.0;
#endif