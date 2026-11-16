// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-tmp-configs: tmp configs component model.
//!
//! Real implementation: configss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configs {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TmpConfigsError {
    EmptyConfigsName,
    DuplicateConfigs(String),
    EmptyEventSubject,
}

impl fmt::Display for TmpConfigsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyConfigsName => write!(f, "configs name must not be empty"),
            Self::DuplicateConfigs(id) => write!(f, "duplicate configs {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for TmpConfigsError {}

pub struct TmpConfigs {
    pub name: String,
    configss: BTreeMap<String, Configs>,
    events: BTreeMap<String, Event>,
}

impl TmpConfigs {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            configss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_configs(&mut self, item: Configs) -> Result<(), TmpConfigsError> {
        if item.name.trim().is_empty() {
            return Err(TmpConfigsError::EmptyConfigsName);
        }
        let id = item.id.clone();
        if self.configss.contains_key(&id) {
            return Err(TmpConfigsError::DuplicateConfigs(id.clone()));
        }
        self.configss.insert(id, item);
        Ok(())
    }

    pub fn configs_count(&self) -> usize {
        self.configss.len()
    }

    pub fn get_configs(&self, id: &str) -> Option<&Configs> {
        self.configss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), TmpConfigsError> {
        if event.subject.trim().is_empty() {
            return Err(TmpConfigsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_configs(id: &str, name: &str) -> Configs {
        Configs {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = TmpConfigs::new("TmpConfigs");
        assert_eq!(m.name, "TmpConfigs");
        assert_eq!(m.configs_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_configs() {
        let mut m = TmpConfigs::new("TmpConfigs");
        m.add_configs(sample_configs("a1", "Alpha")).unwrap();
        assert_eq!(m.configs_count(), 1);
        assert!(m.get_configs("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = TmpConfigs::new("TmpConfigs");
        assert!(m.add_configs(sample_configs("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = TmpConfigs::new("TmpConfigs");
        m.add_configs(sample_configs("x", "One")).unwrap();
        assert!(m.add_configs(sample_configs("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = TmpConfigs::new("TmpConfigs");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
