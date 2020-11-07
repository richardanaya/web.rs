#ifndef export
#define export __attribute__((visibility("default")))
#endif

#ifndef JS_H
typedef double JSValue;
typedef int JSFunction;
extern JSFunction js_register_function(char*,unsigned int);
extern JSValue js_invoke_function(JSFunction,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue);

JSValue const JS_NULL = 0.0;
JSValue const JS_UNDEFINED = 1.0;
JSValue const DOM_SELF = 2.0;
JSValue const DOM_WINDOW = 2.0;
JSValue const DOM_DOCUMENT = 3.0;
JSValue const DOM_BODY = 4.0;
#endif
