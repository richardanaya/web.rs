#include "js-wasm.h"

{%- for binding in bindings %}
{%- for function in binding.functions %}

{% if function.output -%}double{% else %}void{% endif %} {{binding.namespace}}_{% if function.friendly_name -%}
{{function.friendly_name}}
{%- else -%}
{{function.name}}
{%- endif -%}(
    {%- for param in function.parameters -%} {% if param.parameter_type == "string" -%}
        char *
    {%- else -%}
        double
    {%- endif %} {{param.friendly_name}}
    {%- if not loop.last -%}
    , {% endif %} 
    {%- endfor -%}){
    static int fn;
    {% set_global i = 0 %}
    {%- for param in function.parameters -%}
    {%- if param.parameter_type == "string" -%}
    unsigned int a{{i}} = (unsigned int){{param.friendly_name}};
    {% set_global i = i + 1 -%}
    unsigned int a{{i}} = js_strlen({{param.friendly_name}});
    {% else %}
    double a{{i}} = {{param.friendly_name}};
    {% endif -%}
    {% set_global i = i + 1 %}
    {%- endfor -%}
    char *fn_code = "function({% for param in function.parameters -%}
    {%- if param.parameter_type == "string" -%}
    {{param.name}}Ptr,{{param.name}}Len
    {%- else -%}
    {{param.name}}
    {%- endif -%}
    {%- if not loop.last -%}
    , {% endif -%}
    {%- endfor -%}){ {% if function.output -%}return {% endif %}{% if function.output == "object" %} this.storeObject({% endif %}{{binding.namespace}}.{{function.name}}({% for param in function.parameters -%}
                    {%- if param.parameter_type == "string" -%}
                    this.readUtf8FromMemory({{param.name}}Ptr,{{param.name}}Len)
                    {%- elif param.parameter_type == "object" -%}this.getObject({{param.name}})
                    {%- else -%}{{param.name}}
                    {%- endif -%}{%- if not loop.last -%}, {% endif -%}{%- endfor -%}){% if function.output == "object" %}){% endif %}; }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    {% if function.output -%}return {% endif %}js_invoke_function_{{i}}(fn{% if function.parameters | length > 0 %}, {% endif %}{% set i = 0 -%}
    {%- for param in function.parameters -%}
        {%- if param.parameter_type == "string" -%}
        a{{i}}, {% set i = i + 1 -%}
        a{{i}}
        {%- else -%}
        a{{i}}
        {%- endif -%}
    {%- if not loop.last -%}
    , {% endif -%}
    {%- endfor -%});
}
{%- endfor %}
{%- endfor -%}