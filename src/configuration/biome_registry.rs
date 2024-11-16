use crate::RegistryData;

pub fn get_biome_registry<'a>() -> Vec<RegistryData<'a>> {
    return vec![
        RegistryData {
            entry_id: "minecraft:plains",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:forest",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:grove",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:old_growth_pine_taiga",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:old_growth_spruce_taiga",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:snowy_taiga",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:taiga",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:bamboo_jungle",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:jungle",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:sparse_jungle",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:savanna",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:savanna_plateau",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:windswept_savanna",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:badlands",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:eroded_badlands",
            has_data: false,
            data: None,
        },
        RegistryData {
            entry_id: "minecraft:wooded_badlands",
            has_data: false,
            data: None,
        },
    ];
}
