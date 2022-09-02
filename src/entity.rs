use std::{any::TypeId, mem, ops::Range};

use crate::{archetype::ArchetypeId, Component, World};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Entity(usize);

impl Entity {
    pub fn from_raw(n: usize) -> Self {
        Self(n)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct Entities {
    pub(crate) entities: Vec<Location>,
    freed: Vec<usize>,
    count: usize,
    range: Range<usize>,
}

impl Entities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_range(&mut self, range: Range<usize>) {
        self.range = range;
    }

    pub fn reserve_entity(&mut self) -> Entity {
        if self.count < self.range.start {
            self.count = self.range.start;
        }

        let id = self.freed.pop().unwrap_or_else(|| {
            self.count = (self.count + 1).min(self.range.end);
            self.count - 1
        });

        if id >= self.entities.len() {
            self.entities.resize(id + 1, Location::EMPTY);
        }

        Entity(id)
    }

    pub fn free(&mut self, entity: Entity) -> Location {
        let location = mem::replace(&mut self.entities[entity.0], Location::EMPTY);

        self.freed.push(entity.0);

        location
    }

    pub fn get(&self, entity: Entity) -> Option<Location> {
        if entity.0 < self.entities.len() {
            let location = self.entities[entity.0];

            if location.id != ArchetypeId::INVALID || location.row != usize::MAX {
                Some(location)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Default for Entities {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            freed: Vec::new(),
            count: 0,
            range: 0..usize::MAX,
        }
    }
}

pub struct EntityBuilder<'a> {
    pub world: &'a mut World,
    pub entity: Entity,
    pub components: Vec<(TypeId, Box<dyn Component>)>,
}

impl<'a> EntityBuilder<'a> {
    pub fn build(self) -> Entity {
        let entity = self.entity;

        let mut types = self
            .components
            .iter()
            .map(|c| c.0)
            .collect::<Vec<TypeId>>()
            .into_boxed_slice();

        types.sort_unstable();

        let archetype = self.world.get_archetype_mut(&mut types);

        for (id, component) in self.components.into_iter() {
            let info = archetype.get_component_index(&id).unwrap();

            archetype.components[info.column].push(component);
        }

        let id = archetype.id();
        let row = archetype.components[0].len() - 1;

        self.world.entities.entities[entity.id()] = Location { row, id };

        entity
    }

    pub fn with<T: Component + 'static>(mut self, component: T) -> Self {
        self.components
            .push((TypeId::of::<T>(), Box::new(component)));

        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub(crate) row: usize,
    pub(crate) id: ArchetypeId,
}

impl Location {
    pub const EMPTY: Self = Location {
        row: usize::MAX,
        id: ArchetypeId::INVALID,
    };

    pub fn archetype(&self) -> ArchetypeId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = Entity::from_raw(0xDEADBEEF);

        assert_eq!(entity.id(), 0xDEADBEEF);
    }

    #[test]
    fn test_free() {
        let mut entities = Entities::new();

        let entity1 = entities.reserve_entity();
        let entity2 = entities.reserve_entity();

        assert_eq!(entity1.id(), 0);
        assert_eq!(entity2.id(), 1);

        entities.free(entity1);

        let entity3 = entities.reserve_entity();

        assert_eq!(entity3.id(), 0);
    }

    #[test]
    fn test_range() {
        let mut entities = Entities::new();

        entities.set_range(10..256);

        let entity1 = entities.reserve_entity();

        assert_eq!(entity1.id(), 10);
    }
}
