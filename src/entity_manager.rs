#[macro_export]
macro_rules! entity_manager {
    ( $($x:ident:$t:ty),* ) => {
        use std::collections::HashMap;
        use uuid::Uuid;
	    type Entity = Uuid;

        struct EntityManager {
        $(
            pub $x: HashMap<Entity, $t>,
        )*
        }

        #[allow(dead_code)]
        impl EntityManager {
            fn new() -> Self {
                EntityManager {
                    $(
                        $x: HashMap::new(),
                    )*
                }
            }

            fn create_entity(&self) -> Uuid {
                Uuid::new_v4()
            }

            fn add_component<T: EntityComponent + Sized>(&mut self, entity: Entity, component: T) {
                component.insert(self, entity);
            }

            fn remove_component<T: EntityComponent + Sized>(&mut self, entity: Entity, component: T) {
                component.remove(self, entity);
            }

            fn remove_entity(&mut self, entity: Entity) {
                $(
                    self.$x.remove(&entity);
                )*
            }
        }

        trait EntityComponent: Sized {
            fn insert(self, em: &mut EntityManager, entity: Entity);
            fn remove(self, em: &mut EntityManager, entity: Entity);
        }

        $(
            impl EntityComponent for $t {
                fn insert(self, em: &mut EntityManager, entity: Entity) {
                    &em.$x.insert(entity, self);
                }
                fn remove(self, em: &mut EntityManager, entity: Entity) {
                    &em.$x.remove(&entity);
                }
            }
        )*

    };
}