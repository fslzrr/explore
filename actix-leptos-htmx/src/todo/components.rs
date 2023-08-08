use leptos::*;

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

#[component]
pub fn Todo(cx: Scope, todo: TodoItem) -> impl IntoView {
    return view! { cx,
        <div>
            <p>{todo.title}</p>
            <p>{todo.completed}</p>
        </div>
    };
}

#[component]
pub fn Todos(cx: Scope, todos: Vec<TodoItem>) -> impl IntoView {
    return view! {cx,
        <ul>
            {todos.into_iter().map(|todo| {
                view! {cx,
                    <li>
                        <Todo todo=todo/>
                    </li>
                }
            }).collect_view(cx)}
        </ul>
    };
}
