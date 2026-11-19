extern crate self as native_kit;

pub mod engine;

// pub use engine::ffi::*;
pub use engine::core::*;
pub use engine::builder::*;
pub use engine::primitives::*;
pub use engine::components::*;
pub use engine::registry::*;
pub use engine::runtime::*;
pub use engine::ast::*;
pub use engine::router::*;
pub use engine::props::*;
pub use native_kit_macros::{
    component, prop, props, render, server, ui, view, App, Button, Grid, HStack,
    NavigationStack, Page, Return, Section, ToolbarTop, VStack, View,
};

#[allow(non_camel_case_types)]
pub type string = String;
#[allow(non_camel_case_types)]
pub type number = u32;
#[allow(non_camel_case_types)]
pub type uint = u32;
#[allow(non_camel_case_types)]
pub type int = i32;
#[allow(non_camel_case_types)]
pub type float = f32;
#[allow(non_camel_case_types)]
pub type boolean = bool;
pub type Slot = Node;


