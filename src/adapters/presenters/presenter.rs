pub trait Presenter<Entity> {
    fn to_entity(self) -> Entity;
    fn from_entity(entity: Entity) -> Self;
}