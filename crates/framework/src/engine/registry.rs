use std::sync::{OnceLock, Mutex, Arc};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde_json::Value;

pub type ActionFn = Arc<dyn Fn(&mut dyn std::any::Any, Option<Value>) + Send + Sync>;

#[derive(Clone, Default)]
pub struct Action(Option<ActionFn>);

impl Action {
    pub fn new<H, Args, S>(handler: H) -> Self
    where
        H: ActionHandler<Args, S>,
        S: 'static,
    {
        Self(Some(Arc::new(move |any_state, payload| {
            handler.invoke(any_state, payload);
        })))
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn emit(&self) {
        if let Some(ref f) = self.0 {
            let mut unit = ();
            f(&mut unit, None);
        }
    }

    pub fn emit_with(&self, val: impl Into<Value>) {
        if let Some(ref f) = self.0 {
            let mut unit = ();
            f(&mut unit, Some(val.into()));
        }
    }
}

impl ActionHandler<(), ()> for Action {
    fn invoke(&self, state: &mut dyn std::any::Any, payload: Option<Value>) {
        if let Some(ref f) = self.0 {
            f(state, payload);
        }
    }
}

impl ActionHandler<(), ()> for Option<Action> {
    fn invoke(&self, state: &mut dyn std::any::Any, payload: Option<Value>) {
        if let Some(ref act) = self {
            act.invoke(state, payload);
        }
    }
}

pub fn action_registry() -> &'static Mutex<HashMap<String, ActionFn>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, ActionFn>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

static RENDER_ACTION_SEQ: AtomicUsize = AtomicUsize::new(0);
static EPHEMERAL_ACTION_SEQ: AtomicUsize = AtomicUsize::new(0);
static IN_ACTION_DISPATCH: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn reset_render_action_seq() {
    RENDER_ACTION_SEQ.store(0, Ordering::SeqCst);
}

fn next_action_id() -> String {
    if IN_ACTION_DISPATCH.load(Ordering::SeqCst) {
        let seq = EPHEMERAL_ACTION_SEQ.fetch_add(1, Ordering::SeqCst);
        format!("action_ephem_{:x}", seq)
    } else {
        let seq = RENDER_ACTION_SEQ.fetch_add(1, Ordering::SeqCst);
        format!("action_{:x}", seq)
    }
}

pub trait ActionHandler<Args, S>: Send + Sync + 'static {
    fn invoke(&self, state: &mut dyn std::any::Any, payload: Option<Value>);
}

impl<F> ActionHandler<(), ()> for F
where
    F: Fn() + Send + Sync + 'static,
{
    fn invoke(&self, _state: &mut dyn std::any::Any, _payload: Option<Value>) {
        self();
    }
}

impl<F, S: 'static> ActionHandler<(&mut S,), S> for F
where
    F: Fn(&mut S) + Send + Sync + 'static,
{
    fn invoke(&self, any_state: &mut dyn std::any::Any, _payload: Option<Value>) {
        if let Some(state) = any_state.downcast_mut::<S>() {
            self(state);
        }
    }
}

impl<F> ActionHandler<(&mut (dyn std::any::Any + 'static),), ()> for F
where
    F: Fn(&mut dyn std::any::Any) + Send + Sync + 'static,
{
    fn invoke(&self, any_state: &mut dyn std::any::Any, _payload: Option<Value>) {
        self(any_state);
    }
}

impl<F, S: 'static> ActionHandler<(&mut S, Option<Value>), S> for F
where
    F: Fn(&mut S, Option<Value>) + Send + Sync + 'static,
{
    fn invoke(&self, any_state: &mut dyn std::any::Any, payload: Option<Value>) {
        if let Some(state) = any_state.downcast_mut::<S>() {
            self(state, payload);
        }
    }
}


fn extract_string_payload(payload: &Option<Value>) -> String {
    match payload {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Object(map)) => map.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        Some(other) => other.to_string(),
        None => String::new(),
    }
}

impl<F, S: 'static> ActionHandler<(&mut S, String), S> for F
where
    F: Fn(&mut S, String) + Send + Sync + 'static,
{
    fn invoke(&self, any_state: &mut dyn std::any::Any, payload: Option<Value>) {
        if let Some(state) = any_state.downcast_mut::<S>() {
            self(state, extract_string_payload(&payload));
        }
    }
}

impl<F, S: 'static> ActionHandler<(&mut S, bool), S> for F
where
    F: Fn(&mut S, bool) + Send + Sync + 'static,
{
    fn invoke(&self, any_state: &mut dyn std::any::Any, payload: Option<Value>) {
        if let Some(state) = any_state.downcast_mut::<S>() {
            let b = match &payload {
                Some(Value::Bool(v)) => *v,
                Some(Value::Object(m)) => m.get("value").and_then(|v| v.as_bool()).unwrap_or(false),
                _ => false,
            };
            self(state, b);
        }
    }
}

pub fn register_handler<H, Args, S>(handler: H) -> String
where
    H: ActionHandler<Args, S>,
    S: 'static,
{
    let id = next_action_id();

    let wrapper = move |any_state: &mut dyn std::any::Any, payload: Option<Value>| {
        handler.invoke(any_state, payload);
    };

    let mut reg = action_registry().lock().unwrap();
    reg.insert(id.clone(), Arc::new(wrapper));
    id
}

pub fn register_action<F, S: 'static>(f: F) -> String
where
    F: Fn(&mut S, Option<Value>) + Send + Sync + 'static,
{
    let id = next_action_id();

    let wrapper = move |any_state: &mut dyn std::any::Any, payload: Option<Value>| {
        if let Some(state) = any_state.downcast_mut::<S>() {
            f(state, payload);
        }
    };

    let mut reg = action_registry().lock().unwrap();
    reg.insert(id.clone(), Arc::new(wrapper));
    id
}

pub fn execute_action(id: &str, state: &mut dyn std::any::Any, payload: Option<Value>) -> bool {
    let func = {
        let reg = action_registry().lock().unwrap();
        reg.get(id).cloned()
    };

    if let Some(f) = func {
        IN_ACTION_DISPATCH.store(true, Ordering::SeqCst);
        f(state, payload);
        IN_ACTION_DISPATCH.store(false, Ordering::SeqCst);
        true
    } else {
        false
    }
}
