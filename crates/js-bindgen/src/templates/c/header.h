#include "js-wasm.h"

{%- for namespace in namespaces %}
{%- for function in namespace.functions %}

void {{namespace.name}}_{% if function.friendly_name -%}
{{function.friendly_name}}
{%- else -%}
{{function.name}}
{%- endif -%}(
    {%- for param in function.parameters -%} {% if param.parameter_type == "string" -%}
        char *
    {%- else -%}
        float64
    {%- endif -%}
    {%- if loop.index != loop.last -%}
    ,
    {%- endif %} {{param.friendly_name}}
    {%- endfor -%}){
    static int fn;
    {% set_global i = 0 %}
    {%- for param in function.parameters -%}
    {%- if param.parameter_type == "string" -%}
    unsigned int a{{i}} = (unsigned int){{param.friendly_name}};
    {% set_global i = i + 1 -%}
    unsigned int a{{i}} = js_strlen({{param.friendly_name}});
    {% else %}
    float64 a{{i}} = {{param.friendly_name}};
    {% endif -%}
    {% set_global i = i + 1 %}
    {%- endfor -%}
    char *fn_code = "function({% for param in function.parameters -%}
    {%- if param.parameter_type == "string" -%}
    {{param.name}}Ptr,{{param.name}}Len
    {%- else -%}
    {{param.name}}
    {% endif -%}
    {%- if loop.index != loop.last -%}
    ,
    {%- endif -%}
    {%- endfor -%}){ {{namespace.name}}.{{function.name}}({% for param in function.parameters -%}
            {%- if param.parameter_type == "string" -%}
            this.readUtf8FromMemory({{param.name}}Ptr,{{param.name}}Len)
            {%- else -%}
            {{param.name}}
            {%- endif -%}
            {%- if loop.index != loop.last -%}
            ,
            {%- endif -%}
            {%- endfor -%}); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_{{i}}(fn{% if function.parameters | length > 0 %}, {% endif %}{% set i = 0 -%}
    {%- for param in function.parameters -%}
        {%- if param.parameter_type == "string" -%}
        a{{i}}, {% set i = i + 1 -%}
        a{{i}}
        {%- else -%}
        a{{i}}
        {%- endif -%}
    {%- if loop.index != loop.last -%}
    , {% endif -%}
    {%- endfor -%});
}
{%- endfor %}
{%- endfor -%}