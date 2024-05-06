use dioxus::prelude::{use_signal, Signal, Readable, ReadableRef, Writable};
use gloo_storage::{LocalStorage, Storage};
use tracing::{info, error};
use serde::{de::DeserializeOwned, Serialize};

/// A persistent storage hook that can be used to store data across application reboots.
///
/// This will load a value from local storage once at startup, then cache it and update
/// it as the application updates it.
///
/// Largely lifted from the Dioxus tutorial on custom hooks.
///
/// Args:
/// * key: The key for the storage entry in browser's local storage.
/// * init: A function to call to initialize the value if it's not found in local storage.
pub fn use_local_storage<T>(key: impl ToString, init: impl FnOnce() -> T) -> UseLocalStorage<T>
    where T: Serialize + DeserializeOwned + 'static
{
    let inner = use_signal(move || {
        let key = key.to_string();
        let value = if let Ok(value) = LocalStorage::get(key.as_str()) {
            value
        } else {
            info!("Didn't find '{}' in local storage.", key.as_str());
            init()
        };
        StorageParams { key, value }
    });
    UseLocalStorage { inner }
}

struct StorageParams<T> {
    key: String,
    value: T,
}

pub struct UseLocalStorage<T>
    where T: Serialize + DeserializeOwned + 'static
{
    inner: Signal<StorageParams<T>>,
}

// For whatever reason, we can't derive Clone and Copy.
impl <T> Clone for UseLocalStorage<T>
    where T: Serialize + DeserializeOwned + 'static
{
    fn clone(&self) -> Self {
        *self
    }
}
impl <T> Copy for UseLocalStorage<T>
    where T: Serialize + DeserializeOwned + 'static {}

impl <T> UseLocalStorage<T>
    where T: Serialize + DeserializeOwned + 'static
{
    /// Returns a reference to the stored value.
    pub fn get_ref<'a>(&'a self) -> LocalStorageRef<'a, T> {
        LocalStorageRef { inner: self.inner.read() }
    }

    /// Sets the value and updates it in local storage.
    pub fn set(&mut self, value: T) {
        // let mut inner = self.inner.write();
        // let inner = &mut *inner;
        // let inner: &mut StorageParams<T> = &mut *inner;
        let inner = &mut *self.inner.write();
        if LocalStorage::set(inner.key.as_str(), &value).is_err() {
            error!("Failed to set '{}' in local storage.", inner.key.as_str());
        }
        // TODO: Should we only update this on success?
        inner.value = value;
    }
}

impl <T> UseLocalStorage<T>
    where T: Clone + Serialize + DeserializeOwned + 'static
{
    /// Returns a clone of the stored value.
    pub fn get(&self) -> T {
        let v = & *self.get_ref();
        v.clone()
    }
}

pub struct LocalStorageRef<'a, T>
    where T: Serialize + DeserializeOwned + 'static
{
    inner: ReadableRef<'a, Signal<StorageParams<T>>>,
}

impl<'a, T> std::ops::Deref for LocalStorageRef<'a, T>
    where T: Serialize + DeserializeOwned + 'static
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.inner).value
    }
}
