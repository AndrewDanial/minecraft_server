use crate::RegistryData;
pub fn get_dimension_registry<'a>() -> Vec<RegistryData<'a>> {
    return vec![
        RegistryData {
            entry_id: "minecraft:overworld",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:the_nether",
            has_data: false,
            data: None,
        },
    ];
}
