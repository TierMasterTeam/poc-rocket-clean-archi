use derive_new::new;

#[derive(new)]
pub struct UserEntity {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub num: String,
}