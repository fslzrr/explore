use leptos::*;

#[component]
pub fn Providers(cx: Scope, children: Children) -> impl IntoView {
    return view! { cx,
        <head>
            <script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV" crossorigin="anonymous"></script>
        </head>
        <body>
            {children(cx)}
        </body>
    };
}