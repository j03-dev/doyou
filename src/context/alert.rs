use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;

/// Monotonic id used to make every alert instance distinct, even when two
/// alerts carry the same message. The UI marks ids as seen so a dismissed
/// alert is never rendered again after leaving and re-entering a screen.
static NEXT_ALERT_ID: AtomicU64 = AtomicU64::new(1);

/// Reserved id for the offline notice: it is shown at most once per app
/// session, no matter which screen or action notices the connection is down.
const OFFLINE_ALERT_ID: u64 = 0;

#[derive(Props, PartialEq, Clone)]
pub struct AlertProps {
    pub id: u64,
    pub level: AlertLevel,
    pub message: String,
    #[props(default = 3000)]
    pub duration: u64,
}

impl AlertProps {
    pub fn error(message: impl ToString) -> Self {
        Self::new(AlertLevel::Error, message)
    }

    pub fn warning(message: impl ToString) -> Self {
        Self::new(AlertLevel::Warning, message)
    }

    pub fn info(message: impl ToString) -> Self {
        Self::new(AlertLevel::Info, message)
    }

    fn new(level: AlertLevel, message: impl ToString) -> Self {
        Self {
            id: NEXT_ALERT_ID.fetch_add(1, Ordering::Relaxed),
            level,
            message: message.to_string(),
            duration: 3000,
        }
    }
}

/// Shared offline notice. Uses the same one-shot mechanism as every other
/// alert, so it is shown at most once per occurrence.
pub fn offline_alert() -> AlertProps {
    AlertProps {
        id: OFFLINE_ALERT_ID,
        level: AlertLevel::Info,
        message: "You're offline – local music only".to_string(),
        duration: 3000,
    }
}

#[derive(Clone, PartialEq)]
pub enum AlertLevel {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = match self {
            AlertLevel::Error => "error",
            AlertLevel::Warning => "warning",
            AlertLevel::Info => "info",
        };
        write!(f, "alert-{}", level)
    }
}

/// Remembers which alerts have already been displayed. Lives above the router
/// so the state survives screen changes.
#[derive(Clone, Copy)]
pub struct AlertSeen {
    ids: Signal<Vec<u64>>,
}

const MAX_SEEN: usize = 128;

#[component]
pub fn AlertProvider(children: Element) -> Element {
    use_context_provider(|| AlertSeen {
        ids: Signal::new(Vec::new()),
    });

    rsx! {
        {children}
    }
}

pub fn use_alerts() -> AlertSeen {
    use_context::<AlertSeen>()
}

impl AlertSeen {
    pub fn is_seen(&self, id: u64) -> bool {
        self.ids.peek().contains(&id)
    }

    pub fn mark_seen(&self, id: u64) {
        let mut signal = self.ids;
        let mut ids = signal.write();
        if ids.contains(&id) {
            return;
        }
        if ids.len() >= MAX_SEEN
            && let Some(pos) = ids.iter().position(|seen| *seen != OFFLINE_ALERT_ID)
        {
            ids.remove(pos);
        }
        ids.push(id);
    }
}
