use js:**;

{% for namespace in namespaces %}
mod {{namespace.name}} {
    {% for function in namespace.functions %}
    fn {{function.name}}(){
        js!("function(){
                {{namespace.name}}.{{function.name}}()
            }"
        ).invoke_0();
    }
    {% endfor %}
}
{% endfor %}

