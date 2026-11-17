use crate::engine::ast::Node;
use crate::engine::runtime::use_resource_async;
use std::future::Future;
use std::panic::Location;

pub struct OTABuilder<F> {
    key: String,
    fetcher: Option<F>,
    fallback: Node,
    error_fallback: Box<dyn Fn(String) -> Node>,
}

impl<F, Fut> OTABuilder<F>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<Node, String>> + crate::engine::runtime::SendBound + 'static,
{
    pub fn error(mut self, err_fn: impl Fn(String) -> Node + 'static) -> Self {
        self.error_fallback = Box::new(err_fn);
        self
    }

    pub fn fallback(mut self, fallback: impl crate::engine::builder::IntoNode) -> Self {
        self.fallback = fallback.into_node();
        self
    }

    pub fn into_node(self) -> Node {
        let resource = use_resource_async(&self.key, self.fetcher.unwrap());
        match resource {
            Some(Ok(node)) => node,
            Some(Err(err)) => (self.error_fallback)(err),
            None => self.fallback,
        }
    }
}

impl<F, Fut> crate::engine::builder::IntoNode for OTABuilder<F>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<Node, String>> + crate::engine::runtime::SendBound + 'static,
{
    fn into_node(self) -> Node {
        self.into_node()
    }
}

#[macro_export]
macro_rules! OTA {
    ($fetcher:expr) => {
        $crate::engine::ota::ota_impl($fetcher)
    }
}

#[allow(non_snake_case)]
#[track_caller]
pub fn ota_impl<Fut, F>(fetcher: F) -> OTABuilder<F>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<Node, String>> + crate::engine::runtime::SendBound + 'static,
{
    let caller = Location::caller();
    let key = format!("{}:{}:{}", caller.file(), caller.line(), caller.column());
    
    OTABuilder {
        key,
        fetcher: Some(fetcher),
        fallback: crate::engine::ast::Node::Element {
            name: "text".to_string(),
            props: vec![("label".to_string(), crate::engine::ast::PropValue::String("Loading...".to_string()))],
            children: vec![],
        },
        error_fallback: Box::new(|e| crate::engine::ast::Node::Element {
            name: "text".to_string(),
            props: vec![("label".to_string(), crate::engine::ast::PropValue::String(format!("Error: {}", e)))],
            children: vec![],
        }),
    }
}
