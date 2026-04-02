use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

static ACTIVE_TRACKERS: OnceLock<Mutex<HashMap<String, Arc<AtomicBool>>>> = OnceLock::new();

fn tracker_registry() -> &'static Mutex<HashMap<String, Arc<AtomicBool>>> {
    ACTIVE_TRACKERS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn tracker_key(game_id: &Option<String>, exe_path: &str) -> String {
    game_id
        .as_ref()
        .map(|id| format!("game:{id}"))
        .unwrap_or_else(|| format!("exe:{}", exe_path.trim().replace('/', "\\").to_ascii_lowercase()))
}

pub fn replace_tracker(key: &str) -> Arc<AtomicBool> {
    let token = Arc::new(AtomicBool::new(false));
    let mut registry = tracker_registry()
        .lock()
        .expect("game tracker registry lock poisoned");

    if let Some(previous) = registry.insert(key.to_string(), Arc::clone(&token)) {
        previous.store(true, Ordering::SeqCst);
        println!("[launch_game] cancelled existing tracker key={key}");
    }

    token
}

pub fn unregister_tracker(key: &str, token: &Arc<AtomicBool>) {
    let mut registry = tracker_registry()
        .lock()
        .expect("game tracker registry lock poisoned");

    let should_remove = registry
        .get(key)
        .map(|current| Arc::ptr_eq(current, token))
        .unwrap_or(false);

    if should_remove {
        registry.remove(key);
        println!("[launch_game] unregistered tracker key={key}");
    }
}
