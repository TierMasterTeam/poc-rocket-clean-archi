pub trait Model<Entity> {
    fn to_entity(self) -> Entity;

    fn from_entity(entity: Entity) -> Self;
}