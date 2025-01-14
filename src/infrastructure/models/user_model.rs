use sqlx::FromRow;
use crate::domain::entities::user_entity::UserEntity;
use crate::infrastructure::models::model::Model;

#[derive(FromRow)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub num: String,
}

impl Model<UserEntity> for UserModel {
    fn to_entity(self) -> UserEntity {
        UserEntity {
            id: self.id ,
            name: self.username,
            email: self.email,
            num: self.num,
        }
    }

    fn from_entity(entity: UserEntity) -> Self {
        UserModel {
            id: entity.id,
            username: entity.name,
            email: entity.email,
            num: entity.num,
        }
    }
}