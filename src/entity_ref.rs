use std::any::TypeId;

use crate::{ArchetypeId, Component, Entity, Location, World};

#[derive(Debug, Clone)]
pub struct EntityRef<'a> {
    world: &'a World,
    entity: Entity,
    location: Location,
}

impl<'a> EntityRef<'a> {
    pub fn new(world: &'a World, entity: Entity, location: Location) -> Self {
        Self {
            world,
            entity,
            location,
        }
    }

    pub fn id(&self) -> Entity {
        self.entity
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn archetype(&self) -> ArchetypeId {
        self.location.id
    }

    pub fn row(&self) -> usize {
        self.location.row
    }

    pub fn world(&self) -> &'a World {
        self.world
    }

    pub fn get<T: Component>(&self) -> Option<&'a T> {
        get_component(self.world, self.entity, self.location)
    }

    pub fn has_component<T: Component>(&self) -> bool {
        self.world.archetypes.get_by_id(self.archetype()).has_component::<T>()
    }
}

#[derive(Debug)]
pub struct EntityMut<'a> {
    world: &'a mut World,
    entity: Entity,
    location: Location,
}

impl<'a> EntityMut<'a> {
    pub fn new(world: &'a mut World, entity: Entity, location: Location) -> Self {
        Self {
            world,
            entity,
            location,
        }
    }

    pub fn id(&self) -> Entity {
        self.entity
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn archetype(&self) -> ArchetypeId {
        self.location.id
    }

    pub fn row(&self) -> usize {
        self.location.row
    }

    pub fn world(&'a mut self) -> &'a mut World {
        self.world
    }

    pub fn get<T: Component>(&'a self) -> Option<&'a T> {
        get_component(self.world, self.entity, self.location)
    }

    pub fn get_mut<T: Component>(&'a mut self) -> Option<&'a mut T> {
        get_component_mut(self.world, self.entity, self.location)
    }

    pub fn has_component<T: Component>(&self) -> bool {
        self.world.archetypes.get_by_id(self.archetype()).has_component::<T>()
    }
}

fn get_component<T: Component>(world: &World, _entity: Entity, location: Location) -> Option<&'_ T> {
    let archetype = world.archetypes.get_by_id(location.archetype());
    let info = archetype.get_component_index(&TypeId::of::<T>())?;

    (*archetype.components[info.column].get(location.row)?)
        .as_any()
        .downcast_ref::<T>()
}

pub fn get_component_mut<T: Component>(world: &mut World, _entity: Entity, location: Location) -> Option<&mut T> {
    let archetype = world.archetypes.get_by_id_mut(location.archetype());
    let info = archetype.get_component_index(&TypeId::of::<T>())?;

    (*archetype.components[info.column].get_mut(location.row)?)
        .as_any_mut()
        .downcast_mut::<T>()
}
