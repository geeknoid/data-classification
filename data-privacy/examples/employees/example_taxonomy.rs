use data_privacy::taxonomy;

#[taxonomy(example)]
pub enum ExampleTaxonomy {
    PersonallyIdentifiableInformation,
    OrganizationallyIdentifiableInformation,
}
