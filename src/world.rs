use std::any::TypeId;

use rustc_hash::FxHashMap;

use crate::{
    archetype::ComponentInfo, bundle::Bundle, entity_ref::get_component_mut, Archetype,
    ArchetypeId, Archetypes, Component, Entities, Entity, EntityBuilder, EntityMut, EntityRef,
    Location,
};

#[derive(Debug, Default)]
pub struct World {
    pub archetypes: Archetypes,
    pub entities: Entities,
    pub _components: FxHashMap<TypeId, FxHashMap<Archetypes, ComponentInfo>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            archetypes: Archetypes::default(),
            entities: Default::default(),
            _components: Default::default(),
        }
    }

    pub fn spawn<B: Bundle>(&mut self, b: B) -> Entity {
        let entity = self.entities.reserve_entity();

        let mut types = b.types();
        let components = b.components().into_vec();

        let archetype = self.get_archetype_mut(&mut types);

        for (component, id) in components.into_iter().zip(types.iter()) {
            let info = archetype.get_component_index(id).unwrap();

            archetype.components[info.column].push(component);
        }

        let id = archetype.id();
        let row = archetype.row();

        self.entities.entities[entity.id()] = Location { row, id };

        entity
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        EntityBuilder {
            entity: self.entities.reserve_entity(),
            world: self,
            components: vec![],
        }
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        let _location = self.entities.free(entity);
    }

    pub fn get_archetype_mut(&mut self, types: &mut Box<[TypeId]>) -> &mut Archetype {
        if !self.archetypes.has_archetype(types) {
            self.create_archetype(types);
        }

        self.archetypes.get_by_types_mut(types).unwrap()
    }

    pub fn create_archetype(&mut self, types: &mut Box<[TypeId]>) -> ArchetypeId {
        self.archetypes.create_archetype(types)
    }

    pub fn entity(&self, entity: Entity) -> EntityRef<'_> {
        let location = self.entities.get(entity).unwrap();
        EntityRef::new(self, entity, location)
    }

    pub fn entity_mut(&mut self, entity: Entity) -> EntityMut<'_> {
        let location = self.entities.get(entity).unwrap();
        EntityMut::new(self, entity, location)
    }

    pub fn get_entity(&self, entity: Entity) -> Option<EntityRef<'_>> {
        let location = self.entities.get(entity)?;
        Some(EntityRef::new(self, entity, location))
    }

    pub fn get_entity_mut(&mut self, entity: Entity) -> Option<EntityMut<'_>> {
        let location = self.entities.get(entity)?;
        Some(EntityMut::new(self, entity, location))
    }

    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.entity(entity).get::<T>()
    }

    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&'_ mut T> {
        get_component_mut(self, entity, self.entities.get(entity)?)
    }

    // TODO: This is clearly a performance nightmare and a dreadful hack
    pub fn add_component<T: Component>(&mut self, entity: Entity, c: T) {
        let location = self.entities.get(entity).unwrap();

        let old_archetype = self.archetypes.get_by_id_mut(location.id);

        let mut new_type = old_archetype.types.to_vec();
        new_type.push(TypeId::of::<T>());
        new_type.sort();

        //let index = location.row;
        let mut components = Vec::new();

        for c in old_archetype.components.iter_mut() {
            components.push(c.remove(location.row));
        }

        components.push(Box::new(c));

        let new_archetype_id = if old_archetype.edges().contains_key(&TypeId::of::<T>()) {
            old_archetype.edges()[&TypeId::of::<T>()]
        } else {
            self.create_archetype(&mut new_type.into_boxed_slice())
        };

        self.archetypes
            .get_by_id_mut(location.id)
            .edges_mut()
            .insert(TypeId::of::<T>(), new_archetype_id);

        let new_archetype = self.archetypes.get_by_id_mut(new_archetype_id);

        for c in new_archetype.components.iter_mut() {
            c.push(components.pop().unwrap());
        }

        let id = new_archetype.id();
        let row = new_archetype.row();

        self.entities.entities[entity.id()] = Location { row, id };
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        //self.archetypes.get_by_id()
        let _location = self.entities.get(entity).unwrap();

        /*let old_archetype = self.archetypes.get_by_id_mut(location.id);

        let mut new_type = old_archetype.types.to_vec();
        let index = new_type.iter().position(|x| x == &TypeId::of::<T>()).unwrap();

        new_type.remove(index);

        //new_type.(TypeId::of::<T>());
        new_type.sort();

        let index = location.row;
        let mut components = Vec::new();

        for c in old_archetype.components.iter_mut() {
            let value = c.remove(index);
            components.push(value);
        }

        let new_archetype = self.get_archetype_mut(&mut new_type.into_boxed_slice());

        for c in new_archetype.components.iter_mut() {
            c.push(components.pop().unwrap());
        }

        let id = new_archetype.id();
        let row = new_archetype.row();

        self.entities.entities[entity.id()] = Location { row, id };*/
        todo!()
    }

    pub fn query<T: Component>(&self) -> Vec<&T> {
        self.archetypes
            .archetypes
            .iter()
            .filter(|a| a.has_component::<T>())
            .flat_map(|a| a.get_component::<T>())
            .collect::<Vec<_>>()
        //Query::<T>::new(self);
        //todo!()
    }

    pub fn query_mut<C: Component>(&mut self) -> Vec<&mut C> {
        self.archetypes
            .archetypes
            .iter_mut()
            .filter(|a| a.has_component::<C>())
            .flat_map(|a| a.get_component_mut::<C>())
            .collect::<Vec<_>>()
    }

    pub fn query_single<C: Component>(&self) -> &C {
        self.archetypes
            .archetypes
            .iter()
            .find(|a| a.has_component::<C>())
            .unwrap()
            .get_single::<C>()
    }

    pub fn query_single_mut<C: Component>(&mut self) -> &mut C {
        self.archetypes
            .archetypes
            .iter_mut()
            .find(|a| a.has_component::<C>())
            .unwrap()
            .get_single_mut::<C>()
    }
}
