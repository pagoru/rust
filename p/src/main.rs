use std::collections::HashMap;

#[derive(Clone)]
struct Entity {
    id: u32,
    component_data: HashMap<Component, i32>
}

impl Entity {
    fn new(id: u32, component_data: HashMap<Component, i32>) -> Entity {
        Entity {
            id,
            component_data
        }
    }

}

#[allow(dead_code)]
struct System {
    id: u8,
    components: Vec<Component>
}

impl System {
    fn new(id: u8, components: Vec<Component>) -> System {
        System {
            id,
            components
        }
    }
}

#[allow(dead_code)]
#[derive(Hash, Eq, PartialEq, Clone)]
enum Component {
    DisplayObject,
    Sprite
}

#[allow(dead_code)]
struct Engine {
    systems: Option<Vec<System>>,
    entities: Vec<Entity>
}

impl Engine {
    fn new() -> Engine {
        Engine {
            systems: None,
            entities: Vec::new()
        }
    }

    fn set_systems(&mut self, systems: Vec<System>) {
        self.systems = Some(systems);
    }

    fn add_entity(&mut self, entity: Entity) {
        let entity_id = entity.id.clone();
        let component_data = entity.component_data.clone();
        self.entities.push(entity);

        let systems = self.systems.as_ref().expect("Not to be None");
        let found_systems: Vec<&System> = systems.iter().filter(|system|
            (component_data.keys().all(|item| system.components.contains(item)))
        ).collect();

        for system in &found_systems {
            println!("entity: {} - system -> {}", entity_id, system.id)
        }
    }


}

fn main() {
    let mut engine = Engine::new();

    let system = System::new(0, vec![Component::Sprite, Component::DisplayObject]);
    let system1 = System::new(1, vec![Component::DisplayObject]);
    engine.set_systems(vec![system, system1]);

    let entity = Entity::new(0, HashMap::from(
        [
            (Component::DisplayObject, 123_000)
        ]
    ));
    engine.add_entity(entity);

    let entity1 = Entity::new(1, HashMap::from(
        [
            (Component::Sprite, 123_000)
        ]
    ));
    engine.add_entity(entity1);



    /*


    let systems = engine.systems.as_mut().expect("Not to be None");
    let picked_system = systems.get(0).expect("Not to be None");


    println!("{}", picked_system.components.iter()
        .any(|s|
            match s  {
                Component::DisplayObject => true,
                _ => false
            }
        )
    );
     */

}