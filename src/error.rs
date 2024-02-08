use external_memory_tools::ExternalMemory;
use substrate_parser::{error::RegistryError, traits::AsMetadata};

#[derive(Debug)]
pub enum ErrorFixMe<E: ExternalMemory, M: AsMetadata<E>> {
    MetaStructure(M::MetaStructureError),
    Registry(RegistryError),
    UnexpectedVariantIndex,
}

impl<E: ExternalMemory, M: AsMetadata<E>> From<RegistryError> for ErrorFixMe<E, M> {
    fn from(registry_error: RegistryError) -> Self {
        ErrorFixMe::Registry(registry_error)
    }
}