use core::slice;
use std::fmt::{self, Debug};

use crate::{Component, Entity};

pub struct SparseSet<T> {
    sparse: SparseArray<usize>,
    dense: Vec<Entity>,
    data: Vec<T>,
}

impl<T: Component> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            sparse: SparseArray::new(),
            dense: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sparse: SparseArray::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
            dense: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.sparse.len()
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        self.sparse.insert(entity.id(), self.len());

        self.dense.push(entity);
        self.data.push(component);
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let index = self.sparse.get(entity.id());

        if let Some(index) = index.copied() {
            let component = self.data.swap_remove(index);
            let _entity = self.dense.swap_remove(index);

            self.sparse.remove(entity.id());

            if index < self.len() {
                let swapped_entry = self.dense[index];

                self.sparse.insert(swapped_entry.id(), index);
            }

            Some(component)
        } else {
            None
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.sparse.get(entity.id())?;

        self.data.get(*index)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = self.sparse.get(entity.id())?;

        self.data.get_mut(*index)
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity.id())
    }

    pub fn components(&self) -> &[T] {
        &self.data
    }

    pub fn components_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<T: Debug> fmt::Debug for SparseSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SparseArray")
            .field("dense", &self.data)
            .finish()
    }
}

#[derive(Debug)]
pub struct SparseArray<T> {
    values: Vec<Option<T>>,
}

impl<T> SparseArray<T> {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut values = Vec::new();
        values.resize_with(capacity, || None);

        Self { values }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if index >= self.values.len() {
            self.values.resize_with(index + 1, || None);
        }
        self.values[index] = Some(value);
    }

    pub fn contains(&self, index: usize) -> bool {
        self.values.get(index).map(|v| v.is_some()).is_some()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index).map(|v| v.as_ref()).unwrap_or(None)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.values
            .get_mut(index)
            .map(|v| v.as_mut())
            .unwrap_or(None)
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.values.get_mut(index).and_then(|v| v.take())
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Component;

    #[test]
    fn test_new() {
        let mut sparse_set = SparseSet::<Component>::new();

        sparse_set.insert(Entity::from_raw(20), Component);
        sparse_set.insert(Entity::from_raw(100), Component);
        sparse_set.insert(Entity::from_raw(40), Component);

        assert_eq!(sparse_set.dense.len(), 3);
        assert_eq!(sparse_set.sparse.len(), 101);
    }

    #[test]
    fn test_insert() {
        let mut sparse_set = SparseSet::<Component>::new();

        assert_eq!(sparse_set.len(), 0);

        sparse_set.insert(Entity::from_raw(0), Component);

        assert_eq!(sparse_set.len(), 1);

        sparse_set.insert(Entity::from_raw(10), Component);

        assert_eq!(sparse_set.len(), 2);
        assert_eq!(sparse_set.sparse.len(), 11);

        sparse_set.insert(Entity::from_raw(9), Component);

        assert_eq!(sparse_set.len(), 3);
        assert_eq!(sparse_set.sparse.len(), 11);
    }

    #[test]
    fn test_remove() {
        let mut sparse_set = SparseSet::<Component>::new();

        sparse_set.insert(Entity::from_raw(5), Component);
        sparse_set.insert(Entity::from_raw(1), Component);

        assert!(sparse_set.remove(Entity::from_raw(1)).is_some());

        assert!(sparse_set.get(Entity::from_raw(1)).is_none());
        assert!(sparse_set.get(Entity::from_raw(5)).is_some());
    }

    #[test]
    fn test_remove_none() {
        let mut sparse_set = SparseSet::<Component>::new();

        assert!(sparse_set.remove(Entity::from_raw(0xDEADBEEF)).is_none());
    }

    #[test]
    fn test_get() {
        let mut sparse_set = SparseSet::<u32>::new();

        sparse_set.insert(Entity::from_raw(0), 6);

        assert_eq!(sparse_set.get(Entity::from_raw(0)), Some(&6));
    }

    #[test]
    fn test_get_mut() {
        let mut sparse_set = SparseSet::<u32>::new();

        sparse_set.insert(Entity::from_raw(0), 6);

        *sparse_set.get_mut(Entity::from_raw(0)).unwrap() = 12;

        assert_eq!(sparse_set.get(Entity::from_raw(0)), Some(&12));
    }

    #[test]
    fn test_contains() {
        let mut sparse_set = SparseSet::<u32>::new();

        sparse_set.insert(Entity::from_raw(0), 1);
        sparse_set.insert(Entity::from_raw(1), 12);
        sparse_set.insert(Entity::from_raw(2), 42);

        assert!(sparse_set.contains(Entity::from_raw(0)));
        assert!(sparse_set.contains(Entity::from_raw(1)));
        assert!(sparse_set.contains(Entity::from_raw(2)));

        assert!(sparse_set.contains(Entity::from_raw(3)) == false);
    }

    #[test]
    fn test_capacity() {
        let sparse_set = SparseSet::<Component>::with_capacity(100);

        assert_eq!(sparse_set.capacity(), 100);
    }
}
