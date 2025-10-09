use crate::{
    auth::CurrentUser,
    data::queries::todo::Todo,
    flash::FlashMessage,
    handlers::dtos::todo::FIELD_TASK,
    paths,
    views::{components::form, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn todos(
    current_user: &CurrentUser,
    flash: &Option<FlashMessage>,
    todos: Vec<Todo>,
    task_value: Option<&str>,
    task_error: Option<&str>,
) -> Markup {
    let content = html! {
        div class="max-w-2xl mx-auto" {
            h1 class="text-2xl font-bold mb-6" { "Todos" }

            form method="POST" action=(paths::forms::CREATE_TODO) class="mb-6 space-y-4" {
                (form::input("text", FIELD_TASK, "New Todo", task_value, task_error))
                (form::submit_button("Add Todo"))
            }

            @if todos.is_empty() {
                p class="text-gray-500 text-center py-8" { "No todos yet. Add one above!" }
            } @else {
                ul class="space-y-2" {
                    @for todo in todos {
                        li class="flex items-center gap-3 p-3 bg-white border rounded-lg" {
                            form method="POST" action=(paths::forms::TOGGLE_TODO) class="flex-shrink-0" {
                                input type="hidden" name="todo_id" value=(todo.todo_id);
                                input
                                    type="checkbox"
                                    checked[todo.is_done]
                                    onchange="this.form.submit()"
                                    class="w-5 h-5 cursor-pointer";
                            }

                            span class={
                                "flex-1 "
                                @if todo.is_done { "line-through text-gray-500" }
                            } {
                                (todo.task)
                            }

                            form method="POST" action=(paths::forms::DELETE_TODO) class="flex-shrink-0" {
                                input type="hidden" name="todo_id" value=(todo.todo_id);
                                button
                                    type="submit"
                                    class="text-red-600 hover:text-red-800 px-2 py-1"
                                {
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    base_layout(current_user, flash, "Todos", "Manage your todos", content)
}