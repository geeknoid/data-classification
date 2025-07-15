use crate::example_taxonomy::*;
use serde::{Deserialize, Serialize};

/// Holds info about a single corporate employee.
#[derive(Serialize, Deserialize, Clone)]
pub struct Employee {
    pub name: PersonallyIdentifiableInformation<String>,
    pub address: PersonallyIdentifiableInformation<String>,
    pub employee_id: OrganizationallyIdentifiableInformation<String>,
    pub age: u32,
}
