// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-icu: icu component model.
//!
//! Real implementation: icu_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IcuItem {
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
pub enum IcuError {
    EmptyIcuItemName,
    DuplicateIcuItem(String),
    EmptyEventSubject,
}

impl fmt::Display for IcuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIcuItemName => write!(f, "icu_item name must not be empty"),
            Self::DuplicateIcuItem(id) => write!(f, "duplicate icu_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for IcuError {}

pub struct Icu {
    pub name: String,
    icu_items: BTreeMap<String, IcuItem>,
    events: BTreeMap<String, Event>,
}

impl Icu {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            icu_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_icu_item(&mut self, item: IcuItem) -> Result<(), IcuError> {
        if item.name.trim().is_empty() {
            return Err(IcuError::EmptyIcuItemName);
        }
        let id = item.id.clone();
        if self.icu_items.contains_key(&id) {
            return Err(IcuError::DuplicateIcuItem(id.clone()));
        }
        self.icu_items.insert(id, item);
        Ok(())
    }

    pub fn icu_item_count(&self) -> usize {
        self.icu_items.len()
    }

    pub fn get_icu_item(&self, id: &str) -> Option<&IcuItem> {
        self.icu_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), IcuError> {
        if event.subject.trim().is_empty() {
            return Err(IcuError::EmptyEventSubject);
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

    fn sample_icu_item(id: &str, name: &str) -> IcuItem {
        IcuItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Icu::new("Icu");
        assert_eq!(m.name, "Icu");
        assert_eq!(m.icu_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_icu_item() {
        let mut m = Icu::new("Icu");
        m.add_icu_item(sample_icu_item("a1", "Alpha")).unwrap();
        assert_eq!(m.icu_item_count(), 1);
        assert!(m.get_icu_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Icu::new("Icu");
        assert!(m.add_icu_item(sample_icu_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Icu::new("Icu");
        m.add_icu_item(sample_icu_item("x", "One")).unwrap();
        assert!(m.add_icu_item(sample_icu_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Icu::new("Icu");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
