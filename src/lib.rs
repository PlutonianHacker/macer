#![feature(negative_impls)]

mod archetype;
mod bundle;
mod component;
mod entity;
mod entity_ref;
mod query;
mod sparse_set;
mod world;

pub use archetype::{Archetype, ArchetypeId, Archetypes, Edge};
pub use component::Component;
pub use entity::{Entities, Entity, EntityBuilder, Location};
pub use entity_ref::{EntityMut, EntityRef};
pub use sparse_set::{SparseSet, SparseArray};
pub use world::World;

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use super::*;

    #[derive(Debug, Copy, Clone)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    #[derive(Debug, Copy, Clone)]
    struct Rotation(f32);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Label(String);

    #[derive(Debug, Copy, Clone)]
    struct Marker;

    #[derive(Debug, PartialEq, Eq)]
    struct A(usize);

    #[test]
    fn it_works() {
        let mut world = World::new();

        let components = (
            Velocity { x: 32.0, y: 7.5 },
            Rotation(90.0),
            Label("Tag".to_string()),
        );

        let a = world
            .create_entity()
            .with(Velocity {
                x: 10.9187,
                y: 9.1234,
            })
            .with(components.1)
            //.with(components.2.clone())
            .build();

        let b = world
            .create_entity()
            .with(components.0)
            .with(components.2.clone())
            .with(components.1)
            .build();
        let c = world
            .create_entity()
            .with(components.0)
            .with(components.1)
            .build();

        println!("{:?}", world.get::<Velocity>(a));
        println!("{:?}", world.get::<Velocity>(b));
        println!("{:?}", world.get::<Label>(c));
        //println!("{world:#?}");

        world.add_component(a, Label("Hello".into()));
        world.add_component(a, Marker);

        //world.remove_component::<Velocity>(c);

        println!("{world:#?}");

        //world.query::<>();

        //let mut x = &mut world.archetypes.archetypes[0].types;
        //let y = &mut world.archetypes.archetypes[1].types;

        /*println!("{:?}",  {
            world.archetypes.archetypes[0].types.sort();
            &world.archetypes.archetypes[0].types
        });
        println!("{:?}",  {
            world.archetypes.archetypes[1].types.sort();
            &world.archetypes.archetypes[1].types
        });*/

        //let mut x: Vec<Box<dyn Component>> = vec![Box::new(components.0), Box::new(components.1), Box::new(components.2.clone())];
        //let mut y: Vec<Box<dyn Component>> = vec![Box::new(components.1), Box::new(components.0), Box::new(components.2.clone())];

        /*let mut i = 0.0;

        for _ in 0..1_000_000 {
            let a = world.get_component::<Velocity>(a).unwrap();
            let b = world.get_component::<Velocity>(b).unwrap();
            let _ = world.get_component::<Label>(c);

            i += a.x + b.y;
        }

        println!("{i}");*/
    }

    //#[test]
    fn remove_entities() {
        let mut world = World::new();

        let a = world.create_entity().with(Label("Steve".into())).build();
        let b = world.create_entity().with(Label("Tag".into())).build();

        world.remove_entity(a);
        world.remove_entity(b);

        println!("{:?}", world.get::<Label>(a));

        world.create_entity().with(Rotation(45.0)).build();

        println!("{world:#?}");
    }

    //#[test]
    fn create_entities() {
        let mut world = World::new();

        let components = (Velocity { x: 32.0, y: 7.5 }, Rotation(90.0), Marker);

        world.spawn(components);

        //let mut i = 0;

        for _ in 0..1_000 {
            let _a = world.spawn(components);
            let _b = world.spawn((components.0, components.1));
            let _c = world.spawn((components.0,));

            //i += a.id() + b.id() + c.id();
        }

        //println!("{i}");

        /*let mut i = 0;

        for _ in 0..1_000_000 {
            let a = world
                .create_entity()
                .with(components.0)
                .with(components.1)
                .with(components.2)
                .build();

            let b = world
                .create_entity()
                .with(components.0)
                .with(components.1)
                .build();
            let c = world.create_entity().with(components.0).build();

            i += a.id() + b.id() + c.id();
        }

        println!("{i}");*/
    }

    #[test]
    fn get_components() {
        let mut world = World::new();

        world
            .create_entity()
            .with(Velocity { x: 10.9, y: 0.2 })
            .build();
        world
            .create_entity()
            .with(Velocity { x: 0.67, y: 12.3 })
            .with(Rotation(90.0))
            .build();
        world
            .create_entity()
            .with(Velocity { x: 0.67, y: 12.3 })
            .with(Rotation(45.0))
            .build();
        world
            .create_entity()
            .with(Velocity { x: 5.5, y: 8.9 })
            .build();

        let archetype = &world.archetypes.archetypes[0];

        println!("{:?}", archetype.has_component::<Velocity>());

        println!("{:#?}", world.query::<Velocity>());
        println!("{:#?}", world.query::<Rotation>());
        println!("{:#?}", world.query::<Label>());

        //let vels = world.query::<Velocity>();

        let mut rotations = world.query_mut::<Rotation>();

        rotations[0].0 *= 0.2;

        println!("{:#?}", world.query::<Rotation>());

        //for (velocity, rotation) in vels.iter().zip(rotations.iter()) {
        //println!("{:?}", (velocity, rotation));
        //}

        //println!("{world:#?}");
        //println!("{:?}", world.archetypes.archetypes.len());
    }

    #[test]
    fn query() {
        let mut world = World::new();

        world.create_entity().with(A(10)).build();

        assert_eq!(world.query::<A>(), [&A(10)]);

        world.create_entity().with(A(21)).build();
        world.create_entity().with(A(42)).build();

        assert_eq!(world.query::<A>(), [&A(10), &A(21), &A(42)]);
    }

    #[test]
    fn query_mut() {
        let mut world = World::new();

        world.create_entity().with(A(10)).build();
        world.create_entity().with(Label("abc".into())).build();

        assert_eq!(world.query::<A>()[0], &A(10));

        let mut a = &mut world.query_mut::<A>()[0];

        a.0 += 5;

        assert_eq!(world.query::<A>()[0], &A(15));

        assert_eq!(world.query::<Label>()[0], &Label("abc".into()));

        world.query_mut::<Label>()[0].0 += "efg";

        assert_eq!(world.query::<Label>()[0], &Label("abcefg".into()));
    }
}
