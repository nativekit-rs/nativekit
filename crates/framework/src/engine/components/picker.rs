use std::sync::Arc;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::IntoNode;

pub type PickerSelectHandler = Arc<dyn Fn(&mut dyn std::any::Any, &str) + Send + Sync>;

pub struct Picker {
    pub title: String,
    pub options: Vec<String>,
    pub selected: Option<String>,
    pub style: String,
    pub on_select: Option<PickerSelectHandler>,
}

impl Picker {
    pub fn new<I, S>(options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            title: "Select".to_string(),
            options: options.into_iter().map(|s| s.into()).collect(),
            selected: None,
            style: "drill-in".to_string(),
            on_select: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn selected(mut self, selected: impl Into<String>) -> Self {
        self.selected = Some(selected.into());
        self
    }

    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = style.into();
        self
    }

    pub fn on_select<S: 'static>(mut self, handler: impl Fn(&mut S, &str) + Send + Sync + 'static) -> Self {
        self.on_select = Some(Arc::new(move |any_state, val| {
            if let Some(state) = any_state.downcast_mut::<S>() {
                handler(state, val);
            }
        }));
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_select = Some(Arc::new(move |_any_state, val| {
            handler(val);
        }));
        self
    }
}

impl IntoNode for Picker {
    fn into_node(self) -> Node {
        let display_title = self.title.clone();
        let current_selected = self.selected.clone().unwrap_or_else(|| {
            self.options.first().cloned().unwrap_or_default()
        });
        let options = self.options.clone();
        let select_handler = self.on_select.clone();
        let picker_title = self.title.clone();

        // The inline row button
        let row_btn = crate::engine::primitives::Button::new()
            .variant("secondary")
            .action(move || {
                let opts = options.clone();
                let sh = select_handler.clone();
                let sel = current_selected.clone();

                let mut option_buttons = Vec::new();
                for opt in opts {
                    let opt_val_for_click = opt.clone();
                    let opt_sh = sh.clone();
                    let is_selected = opt == sel;
                    let label = if is_selected {
                        format!("✓  {}", opt)
                    } else {
                        format!("   {}", opt)
                    };

                    let btn = crate::engine::primitives::Button::new()
                        .variant(if is_selected { "primary" } else { "text" })
                        .child(crate::engine::primitives::Text {
                            props: vec![("label".to_string(), PropValue::String(label))],
                            children: vec![],
                        })
                        .action(move |any_state: &mut dyn std::any::Any| {
                            if let Some(ref handler) = opt_sh {
                                handler(any_state, &opt_val_for_click);
                            }
                            crate::engine::router::navigate(-1);
                        });
                    option_buttons.push(btn.into_node());
                }

                let page = crate::engine::primitives::Page::new()
                    .title(picker_title.clone())
                    .child(crate::engine::primitives::VStack::new()
                        .spacing(8)
                        .children(option_buttons)
                    );

                crate::engine::router::navigate(page.into_node());
            })
            .child(
                crate::engine::primitives::HStack::new()
                    .spacing(12)
                    .child(crate::engine::primitives::Text {
                        props: vec![("label".to_string(), PropValue::String(display_title))],
                        children: vec![],
                    })
                    .child(crate::engine::primitives::Text {
                        props: vec![
                            ("label".to_string(), PropValue::String(format!("{} ›", self.selected.as_deref().unwrap_or("Select")))),
                            ("style".to_string(), PropValue::String("caption1".to_string())),
                        ],
                        children: vec![],
                    })
            );

        row_btn.into_node()
    }
}

#[macro_export]
macro_rules! Picker {
    ($options:expr) => {
        $crate::engine::components::picker::Picker::new($options)
    };
}
