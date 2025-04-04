//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize)]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "aliments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub r_type: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub lots: i32,
    pub caducity_date: Date,
    pub id_donator: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::aliment_per_donation::Entity")]
    AlimentPerDonation,
    #[sea_orm(
        belongs_to = "super::donator::Entity",
        from = "Column::IdDonator",
        to = "super::donator::Column::Id",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Donator,
}

impl Related<super::aliment_per_donation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AlimentPerDonation.def()
    }
}

impl Related<super::donator::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Donator.def()
    }
}

impl Related<super::donation::Entity> for Entity {
    fn to() -> RelationDef {
        super::aliment_per_donation::Relation::Donation.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::aliment_per_donation::Relation::Aliments.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
