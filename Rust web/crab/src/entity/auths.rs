//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "auths")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub auth_id: i32,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub last_login: Option<DateTime>,
    pub is_active: Option<bool>,
    #[sea_orm(unique)]
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::UserId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
