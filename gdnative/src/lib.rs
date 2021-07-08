//! # Rust bindings for the Godot game engine
//!
//! This crate contains high-level wrappers around the Godot game engine's gdnative API.
//! Some of the types were automatically generated from the engine's JSON API description,
//! and some other types are hand made wrappers around the core C types.
//!
//! ## Memory management for core types
//!
//! Wrappers for most core types expose safe Rust interfaces, and it's unnecessary to mind
//! memory management most of the times. The exceptions are `VariantArray` and `Dictionary`,
//! internally reference-counted collections with "interior mutability" in Rust parlance. These
//! types are modelled using the "typestate" pattern to enforce that the official
//! [thread-safety guidelines][thread-safety]. For more information, read the type-level
//! documentation for these types.
//!
//! Since it is easy to expect containers and other types to allocate a copy of their
//! content when using the `Clone` trait, some types do not implement `Clone` and instead
//! implement [`NewRef`](./trait.NewRef.html) which provides a `new_ref(&self) -> Self` method
//! to create references to the same collection or object.
//!
//! ## Generated API types
//!
//! The `api` module contains high-level wrappers for all the API types generated from a
//! JSON description of the API. The generated types are tied to a specific version, which
//! is currently `3.2.3-stable` for the crates.io version. If you want to use the bindings
//! with another version of the engine, see the instructions [here][custom-version] on
//! generating custom bindings.
//!
//! ### Memory management
//!
//! API types may be reference-counted or manually-managed. This is indicated by the
//! `RefCounted` and `ManuallyManaged` marker traits.
//!
//! The API types can exist in three reference forms: bare, `TRef` and `Ref`. Bare references
//! to API types, like `&'a Node`, represent valid and safe references to Godot objects.
//! As such, API methods may be called safely on them. `TRef` adds typestate tracking, which
//! enable additional abilities like being able to be passed to the engine. `Ref`, or
//! "persistent" references, have `'static` lifetime, but are not always safe to use. For more
//! information on how to use persistent references safely, see the type-level documentation
//! on `Ref`.
//!
//! ## Feature flags
//!
//! ### `bindings`
//!
//! *Enabled* by default. Includes the crates.io version of the bindings in the `api` module.
//!
//! [thread-safety]: https://docs.godotengine.org/en/stable/tutorials/threads/thread_safe_apis.html
//! [custom-version]: https://github.com/godot-rust/godot-rust/#other-versions-or-custom-builds
//!

// TODO: add logo using #![doc(html_logo_url = "https://<url>")]

// Workaround: rustdoc currently shows hidden items in the original crate when they are
//             re-exported. Manually re-exporting the public items works around that.
#[doc(inline)]
pub use gdnative_core::{
    core_types, godot_dbg, godot_error, godot_gdnative_init, godot_gdnative_terminate, godot_init,
    godot_nativescript_init, godot_print, godot_warn, godot_wrap_method, nativescript, object,
    ref_kind, thread_access, GodotObject, GodotResult, NewRef, Null, Ref, TRef,
};

#[doc(hidden)]
pub use gdnative_core::*;

#[doc(inline)]
pub use gdnative_derive::*;

/// Curated re-exports of common items.
pub mod prelude;

#[doc(inline)]
#[cfg(feature = "bindings")]
/// Bindings for the Godot Class API.
pub use gdnative_bindings as api;
