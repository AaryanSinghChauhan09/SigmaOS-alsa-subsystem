// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-alsa-subsystem: alsa subsystem component model.
//!
//! Real implementation: subsystems, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subsystem {
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
pub enum AlsaSubsystemError {
    EmptySubsystemName,
    DuplicateSubsystem(String),
    EmptyEventSubject,
}

impl fmt::Display for AlsaSubsystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySubsystemName => write!(f, "subsystem name must not be empty"),
            Self::DuplicateSubsystem(id) => write!(f, "duplicate subsystem {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AlsaSubsystemError {}

pub struct AlsaSubsystem {
    pub name: String,
    subsystems: BTreeMap<String, Subsystem>,
    events: BTreeMap<String, Event>,
}

impl AlsaSubsystem {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            subsystems: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_subsystem(&mut self, item: Subsystem) -> Result<(), AlsaSubsystemError> {
        if item.name.trim().is_empty() {
            return Err(AlsaSubsystemError::EmptySubsystemName);
        }
        let id = item.id.clone();
        if self.subsystems.contains_key(&id) {
            return Err(AlsaSubsystemError::DuplicateSubsystem(id.clone()));
        }
        self.subsystems.insert(id, item);
        Ok(())
    }

    pub fn subsystem_count(&self) -> usize {
        self.subsystems.len()
    }

    pub fn get_subsystem(&self, id: &str) -> Option<&Subsystem> {
        self.subsystems.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AlsaSubsystemError> {
        if event.subject.trim().is_empty() {
            return Err(AlsaSubsystemError::EmptyEventSubject);
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

    fn sample_subsystem(id: &str, name: &str) -> Subsystem {
        Subsystem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AlsaSubsystem::new("AlsaSubsystem");
        assert_eq!(m.name, "AlsaSubsystem");
        assert_eq!(m.subsystem_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_subsystem() {
        let mut m = AlsaSubsystem::new("AlsaSubsystem");
        m.add_subsystem(sample_subsystem("a1", "Alpha")).unwrap();
        assert_eq!(m.subsystem_count(), 1);
        assert!(m.get_subsystem("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AlsaSubsystem::new("AlsaSubsystem");
        assert!(m.add_subsystem(sample_subsystem("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AlsaSubsystem::new("AlsaSubsystem");
        m.add_subsystem(sample_subsystem("x", "One")).unwrap();
        assert!(m.add_subsystem(sample_subsystem("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AlsaSubsystem::new("AlsaSubsystem");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
