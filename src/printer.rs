use crate::dex_types::*;

pub struct Printer {
    pub strings: Vec<String>,
    pub types: Vec<DexType>,
    pub protos: Vec<DexProto>,
    pub fields: Vec<DexField>,
    pub methods: Vec<DexMethod>,
    pub classes: Vec<DexClassDef>,
}

impl Printer {
    pub fn print_class_idx(self, i: usize) {
        let class = &self.classes[i];
        let class_type = &self.types[class.class_idx as usize];
        let class_name = match &class_type.parsed {
            TypeDescriptor::Class(s) => &s,
            _ => "",
        };

        let access_level = get_class_access_level_string(class.access_flags);

        println!("{} class {} {{ }}", access_level, class_name.replace("/", "."));
    }
}

fn get_class_access_level_string(x: u32) -> String {
    let access_levels: Vec<(u32, &str)> = vec![
        (ClassAccessLevel::Public     as u32, "public"),
        (ClassAccessLevel::Private    as u32, "private"),
        (ClassAccessLevel::Protected  as u32, "protected"),
        (ClassAccessLevel::Static     as u32, "static"),
        (ClassAccessLevel::Final      as u32, "final"),
        (ClassAccessLevel::Interface  as u32, "interface"),
        (ClassAccessLevel::Abstract   as u32, "abstract"),
        (ClassAccessLevel::Synthetic  as u32, ""),
        (ClassAccessLevel::Annotation as u32, ""),
        (ClassAccessLevel::Enum       as u32, "enum"),
    ];

    return access_levels.iter()
        .filter(|(access_level, _)| access_level & x == 1)
        .map(|(_, s)| *s)
        .collect::<Vec<&str>>()
        .join(" ");
}

fn get_field_access_level_string(x: u32) -> String {
    let access_levels: Vec<(u32, &str)> = vec![
        (FieldAccessLevel::Public    as u32, "public"),
        (FieldAccessLevel::Private   as u32, "private"),
        (FieldAccessLevel::Protected as u32, "protected"),
        (FieldAccessLevel::Static    as u32, "static"),
        (FieldAccessLevel::Final     as u32, "final"),
        (FieldAccessLevel::Volatile  as u32, "volatile"),
        (FieldAccessLevel::Transient as u32, ""),
        (FieldAccessLevel::Synthetic as u32, ""),
        (FieldAccessLevel::Enum      as u32, "enum"),
    ];

    return access_levels.iter()
        .filter(|(access_level, _)| access_level & x == 1)
        .map(|(_, s)| *s)
        .collect::<Vec<&str>>()
        .join(" ");
}