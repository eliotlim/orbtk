#![crate_name = "orbtk"]
#![crate_type = "lib"]

pub use dces::prelude::*;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod macros;

pub use crate::application::*;
pub use crate::backend::*;
pub use crate::enums::*;
pub use crate::event::*;
pub use crate::layout::*;
pub use crate::properties::*;
pub use crate::render_object::*;
pub use crate::structs::*;
pub use crate::styling::theme::{DEFAULT_THEME_CSS, LIGHT_THEME_EXTENSION_CSS};
pub use crate::styling::vector_graphics::*;
pub use crate::systems::*;
pub use crate::theme::*;
pub use crate::widgets::*;

pub use dces::prelude::*;

pub mod application;
pub mod backend;
pub mod enums;
pub mod event;
pub mod layout;
pub mod properties;
pub mod render_object;
pub mod structs;
pub mod styling;
pub mod systems;
pub mod theme;
pub mod widgets;
pub mod prelude;
