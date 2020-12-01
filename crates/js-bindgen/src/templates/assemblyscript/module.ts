import * as jswasm from "./js-wasm"

{%- for binding in bindings %}

{%- for function in binding.functions %}
let {{binding.namespace}}_{% if function.friendly_name -%}
{{function.friendly_name}}
{%- else -%}
{{function.name}}
{%- endif -%}_fn:f64 = 0;
export function {{binding.namespace}}_{% if function.friendly_name -%}
    {{function.friendly_name}}
    {%- else -%}
    {{function.name}}
    {%- endif -%}(
    {%- for param in function.parameters -%}{{param.friendly_name}}: {% if param.parameter_type == "string" -%}
            string
        {%- else -%}
            f64
        {%- endif -%}
        {%- if not loop.last -%}
        , {% endif -%}
    {%- endfor -%}){% if function.output %} : f64{% else %} : void{% endif %} {
    {% set_global i = 0 %}
    {%- for param in function.parameters -%}
    {%- if param.parameter_type == "string" -%}
    const a{{i}}: f64 = <f64>changetype<usize>({{param.friendly_name}});
    {% set_global i = i + 1 -%}
    const a{{i}}: f64 = {{param.friendly_name}}.length*2;
    {% else %}
    const a{{i}}: f64 = {{param.friendly_name}};
    {% endif -%}
    {% set_global i = i + 1 %}
    {%- endfor -%}
    if( {{binding.namespace}}_{% if function.friendly_name -%}
        {{function.friendly_name}}
        {%- else -%}
        {{function.name}}
        {%- endif -%}_fn === 0) {
        const code = `function({% for param in function.parameters -%}
            {%- if param.parameter_type == "string" -%}
            {{param.name}}Ptr,{{param.name}}Len
            {%- else -%}
            {{param.name}}
            {%- endif -%}
            {%- if not loop.last -%}
            , {% endif -%}
            {%- endfor -%}){ {% if function.output -%}return {% endif %}{% if function.output == "object" %} this.storeObject({% endif %}{{binding.namespace}}.{{function.name}}({% for param in function.parameters -%}
                            {%- if param.parameter_type == "string" -%}
                            this.readUtf16FromMemory({{param.name}}Ptr,{{param.name}}Len)
                            {%- elif param.parameter_type == "object" -%}this.getObject({{param.name}})
                            {%- else -%}{{param.name}}
                            {%- endif -%}{%- if not loop.last -%}, {% endif -%}{%- endfor -%}){% if function.output == "object" %}){% endif %}; }`;
        {{binding.namespace}}_{% if function.friendly_name -%}
        {{function.friendly_name}}
        {%- else -%}
        {{function.name}}
        {%- endif -%}_fn = <f64>jswasm.js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
    }
    {% if function.output -%}return {% endif %}jswasm.js_invoke_function_{{i}}({{binding.namespace}}_{% if function.friendly_name -%}
        {{function.friendly_name}}
        {%- else -%}
        {{function.name}}
        {%- endif -%}_fn{% if function.parameters | length > 0 %}, {% endif %}{% set i = 0 -%}
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
{% endfor %}
{%- endfor -%}