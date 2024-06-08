use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
      <Stylesheet id="leptos" href="/pkg/discord-rs.css"/>
      <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
      <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
      <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png"/>
      <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png"/>
      <Link rel="manifest" href="/site.webmanifest"/>

      // sets the document title
      <Title text="Discord.rs"/>

      // content for this welcome page
      <Router fallback=|| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(AppError::NotFound);
          view! { <ErrorTemplate outside_errors/> }.into_view()
      }>
          <main class="flex flex-col w-full h-dvh justify-center items-center bg-zinc-950 text-zinc-50">
              <Routes>
                  <Route path="" view=HomePage/>
              </Routes>
          </main>
      </Router>
  }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
  // Creates a reactive value to update the button
  let (count, set_count) = create_signal(0);
  let on_click = move |_| set_count.update(|count| *count += 1);

  view! {
      <h1>"Welcome to Leptos!"</h1>
      <button
          on:click=on_click
          class="bg-orange-600 hover:bg-orange-700 px-5 py-3 text-white rounded-lg"
      >
          "Click Me: "
          {count}
      </button>
  }
}
