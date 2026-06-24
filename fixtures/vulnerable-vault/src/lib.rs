//! Intentionally vulnerable Soroban-style contract for RytScan fixture tests.

#![allow(dead_code)]

pub struct Env;
pub struct Address;

pub enum DataKey {
    Balance(Address),
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
pub struct PersistentStorage;
pub struct TemporaryStorage;

impl StorageAccessor {
    pub fn instance(&self) -> InstanceStorage {
        InstanceStorage
    }
    pub fn persistent(&self) -> PersistentStorage {
        PersistentStorage
    }
    pub fn temporary(&self) -> TemporaryStorage {
        TemporaryStorage
    }
}

impl InstanceStorage {
    pub fn set<T>(&self, _key: &DataKey, _value: &T) {}
}

impl PersistentStorage {
    pub fn set<T>(&self, _key: &DataKey, _value: &T) {}
}

impl TemporaryStorage {
    pub fn set<T>(&self, _key: &DataKey, _value: &T) {}
}

pub struct TokenClient;

impl TokenClient {
    pub fn transfer(&self, _to: &Address, _amount: i128) -> bool {
        true
    }
}

pub fn withdraw(env: Env, user: Address, amount: i128) {
    let token = TokenClient;
    token.transfer(&user, amount);
    env.storage().persistent().set(&DataKey::Balance(user), &amount);
}

pub fn set_admin(env: Env, admin: Address) {
    env.storage().temporary().set(&DataKey::Admin, &admin);
}

pub fn risky_mint(env: Env, to: Address, amount: i128) {
    let value = amount.checked_add(1).unwrap();
    env.storage().instance().set(&DataKey::Balance(to), &value);
}
