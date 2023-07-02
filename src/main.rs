use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

/// 
#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (inverted_count, set_inverted_count) = create_signal(cx, 100);
    let double_count = move || count() * 2;

    view! { cx,
    <button on:click=move |_| { set_count.update(|n| *n += 1); set_inverted_count.update(|n| *n -= 1) }
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
          >
        {count}
    </button>
        <br/>
        <ProgressBar progress=count/><br/>
        <ProgressBar progress={inverted_count}/><br/>
        <ProgressBar progress={double_count}/><br/>
        <ProgressBar progress={move || 100 - double_count()}/><br/>
        <DynamicList initial_length = 5/>
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
    progress: F
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
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
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
    // let values = vec![0, 1, 2];
    // view! { cx,
    // // this will just render "012"
    // <p>{values.clone()}</p>
    //     // or we can wrap them in <li>
    //     <ul>
    //     {values.into_iter()
    //         .map(|n| view! { cx, <li>{n}</li>})
    //             .collect_view(cx)}
    // </ul>
    // }
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
        </div>
    }
}
