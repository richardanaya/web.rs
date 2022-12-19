#![no_std]

use js::*;

{%- for binding in bindings %}

{%- for function in binding.functions %}

pub fn {{binding.namespace}}_{% if function.friendly_name -%}
    {{function.friendly_name}}
    {%- else -%}
    {{function.name}}
    {%- endif -%}(
    {%- for param in function.parameters -%}{{param.friendly_name}}: {% if param.parameter_type == "string" -%}
            &str
        {%- else -%}
            impl Into<f64>
        {%- endif -%}
        {%- if not loop.last -%}
        , {% endif -%}
    {%- endfor -%}){% if function.output %} -> f64{% endif %} {
    {% set_global i = 0 %}
    {%- for param in function.parameters -%}
    {%- if param.parameter_type == "string" -%}
    let a{{i}} = {{param.friendly_name}}.as_ptr() as u32;
    {% set_global i = i + 1 -%}
    let a{{i}} = {{param.friendly_name}}.len() as u32;
    {% else -%}
    let a{{i}} = {{param.friendly_name}}.into();
    {% endif -%}
    {% set_global i = i + 1 %}
    {%- endfor -%}
    let func = js!(r###"function({% for param in function.parameters -%}
        {%- if param.parameter_type == "string" -%}
        {{param.name}}Ptr,{{param.name}}Len
        {%- else -%}
        {{param.name}}
        {%- endif -%}
        {%- if not loop.last -%}
        , {% endif -%}
        {%- endfor -%}){
                {% if function.output -%}return {% endif %}{% if function.output == "object" %} this.storeObject({% endif %}{{binding.namespace}}.{{function.name}}({% for param in function.parameters -%}
                {%- if param.parameter_type == "string" -%}
                this.readUtf8FromMemory({{param.name}}Ptr,{{param.name}}Len)
                {%- elif param.parameter_type == "object" -%}
                this.getObject({{param.name}})
                {%- else -%}
                {{param.name}}
                {%- endif -%}
                {%- if not loop.last -%}
                , {% endif -%}
                {%- endfor -%}){% if function.output == "object" %}){% endif %};
        }"###);
    func.invoke_{{i}}(
        {%- set i = 0 -%}
        {%- for param in function.parameters -%}
            {%- if param.parameter_type == "string" -%}
            a{{i}}, {% set i = i + 1 -%}
            a{{i}}
            {%- else -%}
            a{{i}}
            {%- endif -%}
        {%- if not loop.last -%}
        , {% endif -%}
        {%- endfor -%}){%- if not function.output -%};{%- endif %}
}
{%- endfor %}
{%- endfor -%}