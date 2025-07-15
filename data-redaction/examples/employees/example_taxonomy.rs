use data_classification::taxonomy;

#[taxonomy(example)]
pub enum ExampleTaxonomy {
    PersonallyIdentifiableInformation,
    OrganizationallyIdentifiableInformation,
}
