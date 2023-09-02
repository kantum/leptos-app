use leptos::*;
use leptos_meta::*;
// use leptos_router::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

static BODY_STYLE: &str = "max-width: max-content; margin: auto; padding: 20px";

static BUTTON_CLASS: &str = "\
    bg-blue-500 \
    hover:bg-blue-700 \
    text-white \
    font-bold \
    py-2 \
    px-4 \
    rounded\
";

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (inverted_count, set_inverted_count) = create_signal(cx, 100);
    let double_count = move || count() * 2;

    view! { cx,
        <body style=BODY_STYLE>
            <p class="bg-blue-700">
            hello
            </p>
                <button on:click=move |_| { set_count.update(|n| *n += 1); set_inverted_count.update(|n| *n -= 1) }
                class=BUTTON_CLASS
                      >
                    {count}
                </button>
                <br/>
                <ProgressBar progress=count/><br/>
                <ProgressBar progress={inverted_count}/><br/>
                <ProgressBar progress={double_count}/><br/>
                <ProgressBar progress={move || 100 - double_count()}/><br/>
                <DynamicList initial_length = 5 />
                <Home/>
        </body>
    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar<F>(
    cx: Scope,
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    progress: F,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! { cx,
    <progress
        max=max
        value=progress
        />
    }
}

/// Shows a List
#[component]
fn List(cx: Scope) -> impl IntoView {
    // create a list of N signals
    let counters = (1..=10).map(|idx| create_signal(cx, idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
            <li>
                <button
                on:click=move |_| set_count.update(|n| *n += 1)
                class=BUTTON_CLASS
                >
                {count}
            </button>
                </li>
            }
        })
        .collect_view(cx);

    view! { cx,
    <ul>{counter_buttons}</ul>
    }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
    cx: Scope,
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(cx, initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(cx, next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! { cx,
    <div>
        <button on:click=add_counter>
        "Add Counter"
        </button>

        <ul>
        // The <For/> component is central here
        // This allows for efficient, key list rendering
        <For
            // `each` takes any function that returns an iterator
            // this should usually be a signal or derived signal
            // if it's not reactive, just render a Vec<_> instead of <For/>
            each=counters
            // the key should be unique and stable for each row
            // using an index is usually a bad idea, unless your list
            // can only grow, because moving items around inside the list
            // means their indices will change and they will all rerender
            key=|counter| counter.0
            // the view function receives each item from your `each` iterator
            // and returns a view
            view=move |cx, (id, (count, set_count))| {
                view! { cx,
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                    {count}
                    </button>

                    <button
                    on:click=move |_| {
                        set_counters.update(|counters| {
                            counters.retain(|(counter_id, _)| counter_id != &id)
                        });
                    }
                    >
                    "Remove"
                    </button>

                    </li>
                }
            }
        />
        </ul>

        <Todo todo = Todo {
            id: 1,
            name: String::from("bob")
        } />

        <Todos todos = vec![
            Todo {
                id: 2,
                name: String::from("George")
            },
            Todo {
                id: 3,
                name: String::from("George")
            },
            Todo {
                id: 4,
                name: String::from("George")
            }
        ] />
        <Input />
        </div>
    }
}

#[derive(Clone, Hash)]
pub struct Todo {
    id: usize,
    name: String,
}

#[component]
pub fn Todos(cx: Scope, todos: Vec<Todo>) -> impl IntoView {
    let (todos, set_todos) = create_signal(cx, todos);

    return view! {cx,
        <ul class="bg-blue-500 rounded p-2">
            <For
                each=todos
                key = | todo | todo.id
                view = move | cx, todo | {
                    view! { cx,
                        <li class="bg-blue-100 m-2 p-2 rounded">
                            "wtf "
                            { todo.id }
                            " "
                            { todo.name }
                        </li>
                    }
                }
            />
        </ul>
    };
}

#[component]
pub fn Todo(cx: Scope, todo: Todo) -> impl IntoView {
    return view! {cx,
    <div>
        <p> "I am todo named " { todo.name } </p>
    </div>
    };
}

#[component]
pub fn Input(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    view! { cx,
        <input type="text"
            class="border-2 my-2 border-rose-500 rounded focus-visible:border-green-500"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
            </button>
        </main>
    }
}
