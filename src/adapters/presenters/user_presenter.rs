use serde::{Deserialize, Serialize};
use crate::adapters::presenters::presenter::Presenter;
use crate::domain::entities::user_entity::UserEntity;

#[derive(Serialize, Deserialize)]
pub struct UserPresenter {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub num: String,
}

impl Presenter<UserEntity> for UserPresenter {
    fn to_entity(self) -> UserEntity {
        UserEntity {
            id: self.id,
            name: self.name,
            email: self.email,
            num: self.num,
        }
    }

    fn from_entity(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            email: entity.email,
            num: entity.num,
        }
    }
}