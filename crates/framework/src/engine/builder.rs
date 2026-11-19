#![allow(dead_code)]

use crate::engine::ast::Node;


pub trait IntoNode {
    fn into_node(self) -> Node;
}

impl IntoNode for Node {
    fn into_node(self) -> Node { self }
}

impl IntoNode for String {
    fn into_node(self) -> Node { crate::engine::primitives::Text { props: vec![("label".to_string(), crate::engine::ast::PropValue::String(self))], children: vec![] }.into_node() }
}
impl IntoNode for &str {
    fn into_node(self) -> Node { crate::engine::primitives::Text { props: vec![("label".to_string(), crate::engine::ast::PropValue::String(self.to_string()))], children: vec![] }.into_node() }
}

pub trait IntoChildren {
    fn into_children(self) -> Vec<Node>;
}

// Concrete Vec impl instead of generic IntoIterator to avoid overlap with tuples!
impl<T: IntoNode> IntoChildren for Vec<T> {
    fn into_children(self) -> Vec<Node> {
        self.into_iter().map(|item| item.into_node()).collect()
    }
}

impl<T: IntoChildren> IntoChildren for Option<T> {
    fn into_children(self) -> Vec<Node> {
        match self {
            Some(inner) => inner.into_children(),
            None => Vec::new(),
        }
    }
}

impl IntoChildren for () {
    fn into_children(self) -> Vec<Node> {
        Vec::new()
    }
}

pub type Children = Vec<Node>;

pub trait IntoBoolProp {
    fn into_bool_prop(self) -> bool;
}

impl IntoBoolProp for bool {
    fn into_bool_prop(self) -> bool {
        self
    }
}

impl IntoBoolProp for () {
    fn into_bool_prop(self) -> bool {
        true
    }
}

pub trait IntoOptionalProp<T> {
    fn into_optional_prop(self) -> Option<T>;
}

impl<T, U: Into<T>> IntoOptionalProp<T> for U {
    fn into_optional_prop(self) -> Option<T> {
        Some(self.into())
    }
}

pub enum PaddingSpec {
    System,
    Sides(u32, u32, u32, u32),
}

pub trait IntoPaddingProp {
    fn into_padding_prop(self) -> PaddingSpec;
}

impl IntoPaddingProp for () {
    fn into_padding_prop(self) -> PaddingSpec {
        PaddingSpec::System
    }
}

impl IntoPaddingProp for u32 {
    fn into_padding_prop(self) -> PaddingSpec {
        PaddingSpec::Sides(self, self, self, self)
    }
}

impl IntoPaddingProp for i32 {
    fn into_padding_prop(self) -> PaddingSpec {
        let v = self.max(0) as u32;
        PaddingSpec::Sides(v, v, v, v)
    }
}

impl IntoPaddingProp for (u32, u32) {
    fn into_padding_prop(self) -> PaddingSpec {
        PaddingSpec::Sides(self.0, self.1, self.0, self.1)
    }
}

impl IntoPaddingProp for (i32, i32) {
    fn into_padding_prop(self) -> PaddingSpec {
        let v = self.0.max(0) as u32;
        let h = self.1.max(0) as u32;
        PaddingSpec::Sides(v, h, v, h)
    }
}

impl IntoPaddingProp for (u32, u32, u32, u32) {
    fn into_padding_prop(self) -> PaddingSpec {
        PaddingSpec::Sides(self.0, self.1, self.2, self.3)
    }
}

impl IntoPaddingProp for (i32, i32, i32, i32) {
    fn into_padding_prop(self) -> PaddingSpec {
        PaddingSpec::Sides(
            self.0.max(0) as u32,
            self.1.max(0) as u32,
            self.2.max(0) as u32,
            self.3.max(0) as u32,
        )
    }
}

// Tuples up to 10 elements
macro_rules! impl_into_children_tuple {
    ( $($idx:tt => $name:ident),+ ) => {
        impl<$($name: IntoNode),+> IntoChildren for ($($name,)+) {
            fn into_children(self) -> Vec<Node> {
                vec![ $(self.$idx.into_node()),+ ]
            }
        }
    };
}

impl_into_children_tuple!(0 => A);
impl_into_children_tuple!(0 => A, 1 => B);
impl_into_children_tuple!(0 => A, 1 => B, 2 => C);
impl_into_children_tuple!(0 => A, 1 => B, 2 => C, 3 => D);
impl_into_children_tuple!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E);
impl_into_children_tuple!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F);
impl_into_children_tuple!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G);
impl_into_children_tuple!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H);
impl_into_children_tuple!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I);
impl_into_children_tuple!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J);


impl<T: IntoNode> IntoChildren for T {
    fn into_children(self) -> Vec<Node> { vec![self.into_node()] }
}

pub enum DimensionValue {
    Mode(String),
    Px(u32),
}

pub trait IntoDimension {
    fn into_dimension(self) -> DimensionValue;
}

impl IntoDimension for &str {
    fn into_dimension(self) -> DimensionValue {
        DimensionValue::Mode(self.to_string())
    }
}

impl IntoDimension for String {
    fn into_dimension(self) -> DimensionValue {
        DimensionValue::Mode(self)
    }
}

impl IntoDimension for u32 {
    fn into_dimension(self) -> DimensionValue {
        DimensionValue::Px(self)
    }
}

impl IntoDimension for i32 {
    fn into_dimension(self) -> DimensionValue {
        DimensionValue::Px(self.max(0) as u32)
    }
}

impl IntoDimension for usize {
    fn into_dimension(self) -> DimensionValue {
        DimensionValue::Px(self as u32)
    }
}

pub trait IntoAspectRatio {
    fn into_aspect_ratio(self) -> f32;
}

impl IntoAspectRatio for f32 {
    fn into_aspect_ratio(self) -> f32 {
        self
    }
}

impl IntoAspectRatio for f64 {
    fn into_aspect_ratio(self) -> f32 {
        self as f32
    }
}

impl IntoAspectRatio for (u32, u32) {
    fn into_aspect_ratio(self) -> f32 {
        if self.1 == 0 { 1.0 } else { (self.0 as f32) / (self.1 as f32) }
    }
}

impl IntoAspectRatio for (i32, i32) {
    fn into_aspect_ratio(self) -> f32 {
        if self.1 == 0 { 1.0 } else { (self.0 as f32) / (self.1 as f32) }
    }
}

impl IntoAspectRatio for &str {
    fn into_aspect_ratio(self) -> f32 {
        if let Some((w, h)) = self.split_once('/') {
            let wf = w.trim().parse::<f32>().unwrap_or(1.0);
            let hf = h.trim().parse::<f32>().unwrap_or(1.0);
            if hf == 0.0 { 1.0 } else { wf / hf }
        } else {
            self.trim().parse::<f32>().unwrap_or(1.0)
        }
    }
}

