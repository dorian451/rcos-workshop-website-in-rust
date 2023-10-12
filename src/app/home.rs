use leptos::*;
use leptos_router::ActionForm;
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

/// Renders the home page of your application.
#[component]
pub(crate) fn HomePage() -> impl IntoView {
    let print_msg_on_server = create_server_action::<HandleThingy>();
    let (count, set_count) = create_signal(0);

    view! {
        <ActionForm action=print_msg_on_server>
            <button>"Print message on server"</button>

            <input name="message"/>

            <p>
                {move || {
                    print_msg_on_server
                        .value()
                        .get()
                        .map(|v| {
                            match v {
                                Ok(d) => d.to_string(),
                                Err(d) => d.to_string(),
                            }
                        })
                }}
            </p>
        </ActionForm>

        <br/>

        <button
            on:click=move |_| {
                set_count.set(count.get() + 1);
            }

            on:auxclick=move |e| {
                if e.button() == 2 {
                    set_count.set(count.get() - 1);
                }
            }

            on:contextmenu=move |e| {
                e.prevent_default();
            }
        >

            {move || format!("Counter: {}", count.get())}
        </button>
    }
}

#[server]
async fn handle_thingy(message: String) -> Result<u64, ServerFnError> {
    println!("{}", message);

    let mut uwu = DefaultHasher::new();
    uwu.write(message.as_bytes());

    Ok(uwu.finish())
}
