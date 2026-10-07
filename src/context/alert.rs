use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AlertProps {
    pub level: AlertLevel,
    pub message: String,
    #[props(default = 3000)]
    pub duration: u64,
}

impl AlertProps {
    pub fn error(message: String) -> Self {
        Self {
            level: AlertLevel::Error,
            message,
            duration: 3000,
        }
    }

    pub fn warning(message: String) -> Self {
        Self {
            level: AlertLevel::Warning,
            message,
            duration: 3000,
        }
    }

    pub fn info(message: String) -> Self {
        Self {
            level: AlertLevel::Info,
            message,
            duration: 3000,
        }
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
