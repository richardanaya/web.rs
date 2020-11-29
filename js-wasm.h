#ifndef export
#define export __attribute__((visibility("default")))
#endif

#ifndef JS_H

unsigned int
js_strlen(const char *str)
{
        const char *s;

        for (s = str; *s; ++s)
                ;
        return (s - str);
}

typedef double JSValue;
typedef int JSFunction;
extern JSFunction js_register_function(char*,unsigned int);
extern JSValue js_invoke_function(JSFunction,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue,JSValue);
extern void js_release(JSValue);

JSValue const JS_NULL = 0.0;
JSValue const JS_UNDEFINED = 1.0;
JSValue const DOM_SELF = 2.0;
JSValue const DOM_WINDOW = 2.0;
JSValue const DOM_DOCUMENT = 3.0;
JSValue const DOM_BODY = 4.0;

JSValue js_invoke_function_0(JSFunction fn){
  return js_invoke_function(fn,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0);
}
JSValue js_invoke_function_1(JSFunction fn, JSValue a){
  return js_invoke_function(fn,a,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0);
}
JSValue js_invoke_function_2(JSFunction fn, JSValue a, JSValue b){
  return js_invoke_function(fn,a,b,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0);
}
JSValue js_invoke_function_3(JSFunction fn, JSValue a, JSValue b, JSValue c){
  return js_invoke_function(fn,a,b,c,0.0,0.0,0.0,0.0,0.0,0.0,0.0); 
}
JSValue js_invoke_function_4(JSFunction fn, JSValue a, JSValue b, JSValue c, JSValue d){
  return js_invoke_function(fn,a,b,c,d,0.0,0.0,0.0,0.0,0.0,0.0);
}
JSValue js_invoke_function_5(JSFunction fn, JSValue a, JSValue b, JSValue c, JSValue d, JSValue e){
  return js_invoke_function(fn,a,b,c,d,e,0.0,0.0,0.0,0.0,0.0);
}
JSValue js_invoke_function_6(JSFunction fn, JSValue a, JSValue b, JSValue c, JSValue d, JSValue e, JSValue f){
  return js_invoke_function(fn,a,b,c,d,e,f,0.0,0.0,0.0,0.0);
}
JSValue js_invoke_function_7(JSFunction fn, JSValue a, JSValue b, JSValue c, JSValue d, JSValue e, JSValue f, JSValue g){
  return js_invoke_function(fn,a,b,c,d,e,f,g,0.0,0.0,0.0);
}
JSValue js_invoke_function_8(JSFunction fn, JSValue a, JSValue b, JSValue c, JSValue d, JSValue e, JSValue f, JSValue g, JSValue h){
  return js_invoke_function(fn,a,b,c,d,e,f,g,h,0.0,0.0);
}
JSValue js_invoke_function_9(JSFunction fn, JSValue a, JSValue b, JSValue c, JSValue d, JSValue e, JSValue f, JSValue g, JSValue h, JSValue i){
  return js_invoke_function(fn,a,b,c,d,e,f,g,h,i,0.0);
}
JSValue js_invoke_function_10(JSFunction fn, JSValue a, JSValue b, JSValue c, JSValue d, JSValue e, JSValue f, JSValue g, JSValue h, JSValue i, JSValue j){
  return js_invoke_function(fn,a,b,c,d,e,f,g,h,i,j);
}
#endif
