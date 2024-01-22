use leptos::*;
use leptos_meta::*;

#[component]
fn Button() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let handle_counter = move |_| {
        set_count.update(|n| *n += 2);
    };
    view! {
        <div class="counter">
            <h2>{move || count.get()}</h2>
            <button on:click=handle_counter>"Click me!"</button>
        </div>
    }
}
#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <main>
            <h1>"Rust is Awesome"</h1>
            <Button />
        </main>
    }
}
fn main() {
    mount_to_body(App)
}
