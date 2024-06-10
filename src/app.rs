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
      <main class="flex flex-col w-full h-dvh bg-zinc-700 text-zinc-50">
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
  view! { <GuildsSidebar/> }
}

#[component]
fn GuildsSidebar() -> impl IntoView {
  view! {
    <aside class="flex flex-col w-20 h-full bg-zinc-950 border-r border-zinc-900 px-4 py-2 gap-2">
      <button class="flex items-center justify-center w-full h-12 p-2 rounded-[50px] bg-zinc-700 text-zinc-50 hover:rounded-lg hover:bg-orange-500 transition-all duration-300 ease-out">
        <img src="/assets/discord-mark-white.svg" alt="Add a server" class="self-center w-8 h-8"/>
      </button>

      <hr class="border-zinc-700"/>

      <div class="flex w-full items-center justify-between">
        <button class="w-full h-12 p-2 rounded-[50px] bg-zinc-700 text-zinc-50 hover:rounded-lg hover:bg-orange-500 transition-all duration-300 ease-out text-2xl">
          +
        </button>
      </div>
    </aside>
  }
}
