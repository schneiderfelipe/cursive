//! # Cursive
//!
//! [Cursive] is a [TUI] library - it lets you easily build rich interfaces
//! for use in a terminal.
//!
//! [Cursive]: https://github.com/gyscos/cursive
//! [TUI]: https://en.wikipedia.org/wiki/Text-based_user_interface
//!
//! ## Getting started
//!
//! * Every application should start with a [`Cursive`] object. It is the main
//!   entry-point to the library.
//! * A declarative phase then describes the structure of the UI by adding
//!   views and configuring their behaviours.
//! * Finally, the event loop is started by calling [`Cursive::run`].
//!
//! ## Examples
//!
//! ```rust,no_run
//! use cursive::{Cursive, CursiveExt};
//! use cursive::views::TextView;
//!
//! let mut siv = Cursive::new();
//!
//! siv.add_layer(TextView::new("Hello World!\nPress q to quit."));
//!
//! siv.add_global_callback('q', |s| s.quit());
//!
//! siv.run();
//! ```
//!
//! ## Views
//!
//! Views are the main components of a cursive interface.
//! The [`views`] module contains many views to use in your
//! application; if you don't find what you need, you may also implement the
//! [`View`] trait and build your own.
//!
//! ## Callbacks
//!
//! Cursive is callback-driven: it reacts to events generated by user input.
//!
//! During the declarative phase, callbacks are set to trigger on specific
//! events. These functions usually take an `&mut Cursive` argument, allowing
//! them to modify the view tree at will.
//!
//! ## Debugging
//!
//! The `Cursive` root initializes the terminal on creation, and does cleanups
//! on drop. While it is alive, printing to the terminal will not work
//! as expected, making debugging a bit harder.
//!
//! One solution is to redirect stderr to a file when running the application,
//! and log to it instead of stdout.
//!
//! Or you can use gdb as usual.
//!
//! ## Themes
//!
//! Cursive supports configuring the feels and looks of your application with
//! custom themes and colors. For details see documentation of the
//! [`cursive::theme`] module.
//!
//! [`cursive::theme`]: ./theme/index.html
#![deny(missing_docs)]
#![cfg_attr(feature = "doc-cfg", feature(doc_cfg))]

pub use cursive_core::*;

mod utf8;

pub mod backends;

mod cursive_ext;
mod cursive_runnable;

pub use cursive_ext::CursiveExt;
pub use cursive_runnable::CursiveRunnable;

/// Creates a new Cursive root using one of the enabled backends.
///
/// Will use the first available backend from this list:
/// * BearLibTerminal
/// * Termion
/// * Crossterm
/// * Pancurses
/// * Ncurses
///
/// If none of these is enabled, it will default to a dummy backend.
///
/// # Panics
///
/// If the backend initialization fails.
pub fn default() -> CursiveRunnable {
    CursiveRunnable::default()
}

/// Creates a new Cursive root using a ncurses backend.
#[cfg(feature = "ncurses-backend")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "ncurses-backend")))]
pub fn ncurses() -> CursiveRunnable {
    CursiveRunnable::ncurses()
}

/// Creates a new Cursive root using a pancurses backend.
#[cfg(feature = "pancurses-backend")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "pancurses-backend")))]
pub fn pancurses() -> CursiveRunnable {
    CursiveRunnable::pancurses()
}

/// Creates a new Cursive root using a termion backend.
#[cfg(feature = "termion-backend")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "termion-backend")))]
pub fn termion() -> CursiveRunnable {
    CursiveRunnable::termion()
}

/// Creates a new Cursive root using a crossterm backend.
#[cfg(feature = "crossterm-backend")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "crossterm-backend")))]
pub fn crossterm() -> CursiveRunnable {
    CursiveRunnable::crossterm()
}

/// Creates a new Cursive root using a bear-lib-terminal backend.
#[cfg(feature = "blt-backend")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "blt-backend")))]
pub fn blt() -> CursiveRunnable {
    CursiveRunnable::blt()
}

/// Creates a new Cursive root using a dummy backend.
///
/// Nothing will be output. This is mostly here for tests.
pub fn dummy() -> CursiveRunnable {
    CursiveRunnable::dummy()
}
