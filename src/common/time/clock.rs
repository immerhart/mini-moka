use std::{
    sync::{Arc, RwLock},
    time::Instant as StdInstant,
};

use std::time::Duration;

pub type Instant = StdInstant;

#[derive(Clone)]
pub struct Clock {
    mock: Option<Arc<Mock>>,
}

impl Clock {
    pub fn mock() -> (Clock, Arc<Mock>) {
        let mock = Arc::new(Mock::default());
        let clock = Clock {
            mock: Some(Arc::clone(&mock)),
        };
        (clock, mock)
    }

    pub fn now(&self) -> Instant {
        if let Some(mock) = &self.mock {
            *mock.now.read().expect("lock poisoned")
        } else {
            StdInstant::now()
        }
    }
}

pub struct Mock {
    now: RwLock<Instant>,
}

impl Default for Mock {
    fn default() -> Self {
        Self {
            now: RwLock::new(StdInstant::now()),
        }
    }
}

impl Mock {
    pub fn increment(&self, amount: Duration) {
        *self.now.write().expect("lock poisoned") += amount;
    }
}
