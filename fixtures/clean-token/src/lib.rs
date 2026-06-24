//! Minimal clean Soroban-style contract fixture.

#![allow(dead_code)]

pub struct Env;
pub struct Address;

pub enum DataKey {
    Admin,
}

impl Env {
    pub fn storage(&self) -> StorageAccessor {
        StorageAccessor
    }
    pub fn events(&self) -> EventPublisher {
        EventPublisher
    }
}

pub struct StorageAccessor;
pub struct EventPublisher;
pub struct InstanceStorage;

impl StorageAccessor {
    pub fn instance(&self) -> InstanceStorage {
        InstanceStorage
    }
}

impl InstanceStorage {
    pub fn get<T>(&self, _key: &DataKey) -> Option<T> {
        None
    }
    pub fn set<T>(&self, _key: &DataKey, _value: &T) {}
    pub fn extend_ttl(&self, _threshold: u32, _extend_to: u32) {}
}

impl Address {
    pub fn require_auth(&self) {}
}

pub fn initialize(env: Env, admin: Address) {
    admin.require_auth();
    env.storage().instance().set(&DataKey::Admin, &admin);
    env.storage().instance().extend_ttl(100, 1000);
    env.events().publish(("init",), admin);
}

impl EventPublisher {
    pub fn publish<T>(&self, _topics: (&str,), _data: T) {}
}
