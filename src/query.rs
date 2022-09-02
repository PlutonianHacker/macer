use std::marker::PhantomData;

use crate::World;

pub struct Query<'a, T: ComponentsQuery> {
    world: &'a World,
    fetch: T::Fetch,
    _marker: PhantomData<T>,
}

impl<'a, T: ComponentsQuery> Query<'a, T> {
    pub fn new(world: &'a World) -> Self {
        let fetch = T::init(world);

        Self {
            world,
            fetch,
            _marker: PhantomData,
        }
    }
}

pub trait ComponentsQuery {
    type Fetch;

    fn init(world: &World) -> Self::Fetch;
}

/*
impl<T> ComponentsQuery for T {
    type Fetch = Vec<T>;

    fn init(world: &World) -> Self::Fetch {
        //world.archetypes.

    }
}*/

/*
impl<T: ComponentsQuery> Query<T> for T {
    fn new() -> Self {
        todo!()
    }
}*/

pub trait State: Sized {}
