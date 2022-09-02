use std::any::TypeId;

use crate::Component;

pub trait Bundle {
    fn types(&self) -> Box<[TypeId]>;

    fn components(self) -> Box<[Box<dyn Component>]>;
}

macro_rules! impl_bundle {
    ($($name: ident),*) => {
        impl<$($name: Component),*> Bundle for ($($name,)*) {
            fn types(&self) -> Box<[TypeId]> {
                vec![$(TypeId::of::<$name>()),*].into_boxed_slice()
            }

            #[allow(non_snake_case)]
            #[allow(unused_parens)]
            fn components(self) -> Box<[Box<dyn Component>]> {
                let ($($name),*) = self;
                $(let $name: Box<dyn Component> = Box::new($name);)*

                vec![$($name),*].into_boxed_slice()
            }          
        }
    };
}

impl_bundle!(A);
impl_bundle!(A, B);
impl_bundle!(A, B, C);
impl_bundle!(A, B, C, D);
impl_bundle!(A, B, C, D, E);
impl_bundle!(A, B, C, D, E, F);
impl_bundle!(A, B, C, D, E, F, G);
impl_bundle!(A, B, C, D, E, F, G, H);