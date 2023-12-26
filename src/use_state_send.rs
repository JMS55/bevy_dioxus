// https://github.com/DioxusLabs/dioxus-std/blob/8db5b1e8a3b8c81f3174a0c9cb951c87058289ca/std/src/utils/rw/use_rw.rs

use dioxus::prelude::ScopeState;
use std::sync::{Arc, RwLock, RwLockReadGuard};

pub fn use_state_send<T: Send + Sync + 'static>(
    cx: &ScopeState,
    init_rw: impl FnOnce() -> T,
) -> &mut UseStateSend<T> {
    let hook = cx.use_hook(|| UseStateSend {
        update: cx.schedule_update(),
        value: Arc::new(RwLock::new(init_rw())),
    });

    hook
}

pub struct UseStateSend<T> {
    update: Arc<dyn Fn() + Send + Sync + 'static>,
    value: Arc<RwLock<T>>,
}

impl<T> Clone for UseStateSend<T> {
    fn clone(&self) -> Self {
        Self {
            update: self.update.clone(),
            value: self.value.clone(),
        }
    }
}

impl<T> UseStateSend<T> {
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.value.read().expect("Lock poisoned")
    }

    pub fn write(&self, new_value: T) {
        let mut lock = self.value.write().expect("Lock poisoned");
        *lock = new_value;
        self.needs_update();
    }

    pub fn needs_update(&self) {
        (self.update)()
    }
}
