//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize)]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "beneficiary")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub representant_name: String,
    pub representant_lastname_f: String,
    pub representant_lastname_m: String,
    pub phone: String,
    pub legal_name: String,
    #[sea_orm(unique)]
    pub nif: String,
    pub website: String,
    pub foundation_date: Date,
    pub credentials_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::credentials::Entity",
        from = "Column::CredentialsId",
        to = "super::credentials::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Credentials,
    #[sea_orm(has_many = "super::donation::Entity")]
    Donation,
}

impl Related<super::credentials::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Credentials.def()
    }
}

impl Related<super::donation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Donation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
