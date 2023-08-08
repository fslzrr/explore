use actix_web::{get, web, HttpResponse, Responder};
use leptos::*;

use crate::providers::Providers;
use crate::todo::components::{TodoItem, Todos};

#[get("/")]
pub async fn todos() -> impl Responder {
    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <Providers>
                <Todos todos=vec![
                    TodoItem { id: 0, title: "hello".to_string(), completed: false },
                    TodoItem { id: 1, title: "world".to_string(), completed: true },
                ]/>
            </Providers>
        }
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

#[get("/todo/{id}")]
pub async fn todo(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    return HttpResponse::Ok().body(format!("Todos: {id}"));
}
