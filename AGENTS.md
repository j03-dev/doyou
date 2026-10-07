# AGENTS.md - DoYou Development Guide

## Project Overview

DoYou is a cross-platform desktop application for listening to music from YouTube, built with Rust using the [Dioxus](https://dioxuslabs.com/) framework. The application uses Tailwind CSS with DaisyUI for styling.

## Build Commands

### Running the Application

```bash
# Desktop development (default)
dx serve

# Android development
dx serve --android

# Web development
dx serve --web

# Production build
dx build
```

### Testing

No formal test suite currently exists. To run a single test when tests are added:

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run tests in a specific module
cargo test module_name
```

### Linting and Formatting

```bash
# Clippy lints
cargo clippy

# Auto-fix some clippy issues
cargo clippy --fix

# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Dependency Management

```bash
# Update dependencies
cargo update

# Build (compiles without running)
cargo build
```

## Code Style Guidelines

### General Conventions

- **Language**: Rust (2024 edition)
- **Framework**: Dioxus 0.7.x with router
- **State Management**: Dioxus Signals for reactive state
- **Styling**: Tailwind CSS + DaisyUI plugin

### Project Structure

```
src/
├── main.rs              # Entry point, module declarations
├── app.rs               # Root component, provider tree
├── route.rs             # Router definitions
├── pages/               # Route screens
│   ├── home.rs
│   ├── favorite.rs
│   └── setting.rs
├── components/          # Presentational / shared UI (outer layer)
│   ├── alert.rs         # Renders AlertProps from context
│   ├── button.rs
│   ├── dock.rs
│   ├── form.rs          # Form helpers (get_value_from)
│   ├── icons.rs
│   ├── loading.rs
│   ├── music_list.rs
│   ├── music_player.rs  # <audio> element + mini/full player UI
│   ├── navbar.rs
│   └── text_input.rs
├── context/             # Reactive state providers (state layer)
│   ├── alert.rs         # AlertProps / AlertLevel (state payload)
│   ├── favorites.rs
│   ├── home.rs
│   ├── playback.rs      # Audio playback state
│   └── settings.rs
├── core/                # Infra (no DB, no UI)
│   ├── error.rs         # Error type alias
│   └── platform.rs      # Config dir / path resolution (desktop + Android)
└── repository/          # Data access boundary
    ├── db/              # rusql-alchemy models + queries
    │   ├── mod.rs
    │   └── models.rs
    ├── favorites.rs
    ├── settings.rs
    └── youtube.rs       # Wraps the yt crate

yt/                      # Workspace crate for YouTube API
├── src/
│   ├── data_api/        # YouTube Data API v3
│   └── extractor/       # Audio stream URL extraction
└── Cargo.toml
```

**Layering rule**: `context` (state) must not import `components` (UI). `components` may
import `context`. Flow: `pages` → `components` / `context` → `repository` → `db` / `yt`.

### Component Development

Components use Dioxus macros and follow this pattern:

```rust
use dioxus::prelude::*;

#[component]
pub fn ComponentName(props: ComponentProps) -> Element {
    // Use signals for reactive state
    let mut state = use_signal(|| initial_value);
    
    rsx! {
        // JSX-like template
        div { class: "tailwind-classes",
            // Component children
        }
    }
}
```

### Naming Conventions

- **Files**: snake_case (e.g., `music_player.rs`, `text_input.rs`)
- **Modules**: snake_case
- **Types/Structs**: PascalCase (e.g., `Playback`, `ButtonProps`)
- **Functions**: snake_case (e.g., `add_to_favorite`, `get_settings`)
- **Variables**: snake_case
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `FAVICON`, `TAILWIND_CSS`)

### Props and Properties

```rust
#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(default)]           // Optional with default
    pub class: &'static str,
    pub on_click: EventHandler<MouseEvent>,  // Event handlers
    pub children: Element,       // Child elements
}
```

### Error Handling

- Use `anyhow` for the `yt` crate (see `yt/Cargo.toml`)
- `repository/*` functions return `Result<T, String>` — the UI-facing boundary; map
  underlying errors with `.map_err(|err| err.to_string())`
- Use `core::error::Error` (`Box<dyn Error + Send + Sync>`) for infrastructure code
  such as `core::platform`
- Propagate errors with `?` operator
- Display errors to users via the `Alert` component with `AlertProps::error/warning/info`

```rust
// Example error pattern
pub type Error = Box<dyn std::error::Error + Sync + Send>;

// In async functions
match some_operation().await {
    Ok(value) => do_something(value),
    Err(e) => error.set(Some(AlertProps::error(e))),
}
```

### Database

- Uses `rusql-alchemy` ORM
- Database path configured via `core/platform.rs`
- Models defined in `src/repository/db/models.rs`

### Async Operations

- Use `tokio` for async runtime
- Use `spawn(async move { ... })` for background tasks
- Use `use_effect` for side effects in components

```rust
use_effect(move || {
    spawn(async move {
        // Async operations here
    });
});
```

### Tailwind CSS Usage

- Use DaisyUI component classes (`btn`, `btn-ghost`, `btn-circle`)
- Use Tailwind utility classes for custom styling
- Theme configuration in `tailwind.css`

```rust
// Example
rsx! {
    div { class: "btn btn-ghost btn-circle m-2",
        Icon {}
    }
}
```

### Import Organization

Group imports by module:

```rust
use dioxus::prelude::*;

// External crates
use yt::data_api::types::Item;
use yt::extractor::YouTubeExtractor;

// Local modules
use crate::components::button::ButtonGhost;
use crate::components::icons::{DoYouIcon, SearchIcon};
use crate::components::form::get_value_from;
use crate::context::{use_home, use_playback};
use crate::repository;

// Self modules
use self::music_list::MusicList;
```

### Key Files Reference

| File | Purpose |
|------|---------|
| `src/main.rs` | App entry point, module declarations |
| `src/app.rs` | Root component + provider tree |
| `src/context/playback.rs` | Global audio playback state |
| `src/repository/db/mod.rs` | Database operations |
| `yt/src/data_api/` | YouTube search API |
| `yt/src/extractor/` | Audio stream extraction |

## Environment Variables

Create a `.env` file in the project root:

```
GOOGLE_API_KEY=your_google_api_key_here
```

## Additional Resources

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7/)
- [Tailwind CSS](https://tailwindcss.com/)
- [DaisyUI](https://daisyui.com/)
