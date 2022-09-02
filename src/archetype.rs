use std::any::TypeId;

use rustc_hash::FxHashMap;

use crate::component::Component;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ArchetypeId(usize);

impl ArchetypeId {
    pub const INVALID: Self = ArchetypeId(usize::MAX);
}

#[derive(Debug)]
pub struct Archetype {
    pub id: ArchetypeId,
    pub types: Box<[TypeId]>,
    pub components: Vec<Vec<Box<dyn Component>>>, // Vec<SparseSet<Box<dyn Component>>>, //Vec<Vec<Box<dyn Component>>>,
    add: FxHashMap<TypeId, ArchetypeId>,
    info: FxHashMap<TypeId, ComponentInfo>,
}

impl Archetype {
    pub fn new(id: ArchetypeId, types: Box<[TypeId]>) -> Self {
        Self {
            id,
            components: Vec::from_iter(types.iter().map(|_| Vec::new())),
            info: FxHashMap::from_iter(
                types
                    .iter()
                    .enumerate()
                    .map(|(pos, id)| (*id, ComponentInfo { column: pos })),
            ),
            types,
            add: FxHashMap::default(),
        }
    }

    pub fn has_component<C: Component>(&self) -> bool {
        let id = TypeId::of::<C>();
        self.types.iter().any(|typ| *typ == id)
    }

    pub fn get_component<C: Component>(&self) -> Vec<&C> {
        let index = self.get_component_index(&TypeId::of::<C>()).unwrap().column;

        self.components[index]
            .iter()
            .map(|c| (**c).as_any().downcast_ref::<C>().unwrap())
            .collect::<Vec<_>>()
    }

    pub fn get_component_mut<C: Component>(&mut self) -> Vec<&mut C> {
        let index = self.get_component_index(&TypeId::of::<C>()).unwrap().column;

        self.components[index]
            .iter_mut()
            .map(|c| (**c).as_any_mut().downcast_mut::<C>().unwrap())
            .collect::<Vec<_>>()
    }

    pub fn get_single<C: Component>(&self) -> &C {
        let index = self.get_component_index(&TypeId::of::<C>()).unwrap().column;

        self.components[index]
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<C>()
            .unwrap()
    }

    pub fn get_single_mut<C: Component>(&mut self) -> &mut C {
        let index = self.get_component_index(&TypeId::of::<C>()).unwrap().column;

        self.components[index]
            .first_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut::<C>()
            .unwrap()
    }

    pub fn get_component_index(&self, id: &TypeId) -> Option<ComponentInfo> {
        self.info.get(id).copied()
    }

    pub fn edges(&self) -> &FxHashMap<TypeId, ArchetypeId> {
        &self.add
    }

    pub fn edges_mut(&mut self) -> &mut FxHashMap<TypeId, ArchetypeId> {
        &mut self.add
    }

    pub fn id(&self) -> ArchetypeId {
        self.id
    }

    pub(crate) fn row(&self) -> usize {
        self.components[0].len() - 1
    }
}

#[derive(Debug, Default)]
pub struct Archetypes {
    pub(crate) archetypes: Vec<Archetype>,
    types: FxHashMap<Box<[TypeId]>, usize>,
    ids: FxHashMap<ArchetypeId, usize>,
    count: usize,
}

impl Archetypes {
    pub fn has_archetype(&self, types: &[TypeId]) -> bool {
        self.types.contains_key(types)
    }

    pub fn create_archetype(&mut self, types: &mut Box<[TypeId]>) -> ArchetypeId {
        types.sort_unstable();

        let id = ArchetypeId(self.count);
        self.count += 1;

        let archetype = Archetype::new(id, types.clone());
        let index = self.archetypes.len();

        self.types.insert(types.clone(), index);
        self.ids.insert(id, index);

        self.archetypes.push(archetype);

        id
    }

    pub fn get_by_types_mut(&mut self, types: &[TypeId]) -> Option<&mut Archetype> {
        let index = *self.types.get(types)?;

        self.archetypes.get_mut(index)
    }

    pub fn get_by_id(&self, id: ArchetypeId) -> &Archetype {
        let index = self.ids[&id];

        &self.archetypes[index]
    }

    pub fn get_by_id_mut(&mut self, id: ArchetypeId) -> &mut Archetype {
        let index = self.ids[&id];

        &mut self.archetypes[index]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ComponentInfo {
    pub(crate) column: usize,
}

#[derive(Debug, Default)]
pub struct Edge {
    pub add: Option<ArchetypeId>,
    pub remove: Option<ArchetypeId>,
}

impl Edge {
    pub fn new() -> Self {
        Self {
            add: None,
            remove: None,
        }
    }
}
