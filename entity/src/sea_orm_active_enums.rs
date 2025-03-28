//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "r_type")]
#[derive(Serialize, Deserialize)]
pub enum RType {
    #[sea_orm(string_value = "donator")]
    Donator,
    #[sea_orm(string_value = "beneficiary")]
    Beneficiary,
}
