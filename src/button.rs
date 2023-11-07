use leptos::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button
          class="
            px-4 py-1 text-sm text-purple-600 font-semibold rounded-full border-2 border-purple-200
            dark:text-white dark:border-purple-800
            hover:text-white hover:bg-purple-600 hover:border-transparent
            focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2
            dark:focus:ring-offset-slate-800 dark:focus:ring-purple-900
          "
          >
          {children()}
        </button>
    }
}

#[component]
pub fn IconButton(children: Children) -> impl IntoView {
    view! {
        <button
          class="
            px-1 py-1 text-sm text-purple-600 font-semibold rounded-full border-2 border-purple-200
            dark:text-white dark:border-purple-800
            hover:text-white hover:bg-purple-600 hover:border-transparent
            focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2
            dark:focus:ring-offset-slate-800 dark:focus:ring-purple-900
          "
          >
          {children()}
        </button>
    }
}
