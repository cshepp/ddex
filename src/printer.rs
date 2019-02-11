use std::fmt::Write;
use crate::dex_types::*;
use crate::util::to_hex_string;
use crate::instructions::*;
use crate::binary_parser::BinaryParser;

pub struct Printer {
    pub strings: Vec<String>,
    pub types: Vec<DexType>,
    pub protos: Vec<DexProto>,
    pub fields: Vec<DexField>,
    pub methods: Vec<DexMethod>,
    pub classes: Vec<DexClassDef>,
    pub parser: BinaryParser,
}

impl Printer {
    pub fn print_classes(&mut self) {
        for i in 0..self.classes.len() {
            self.print_class_idx(i);
        }
    }

    pub fn print_class_idx(&mut self, i: usize) {
        let class = &self.classes[i];
        let class_type = &self.types[class.class_idx as usize];
        let class_name = match &class_type.parsed {
            TypeDescriptor::Class(s) => &s,
            _ => "",
        };

        let access_level = get_class_access_level_string(class.access_flags);

        let mut result = String::new();
        write!(&mut result, "{} class {} {{\n", access_level, class_name.replace("/", ".")).expect("");

        for encoded_field in &class.instance_fields {
            let field = &self.fields[encoded_field.field_idx as usize];
            let field_type = &self.types[field.type_idx as usize].parsed;
            let field_name = &self.strings[field.name_idx as usize];
            let access_level = get_field_access_level_string(encoded_field.access_flags);
            let type_string = get_type_descriptor_string(field_type);
            write!(&mut result, "\t{} {} {};\n", access_level, type_string, field_name).expect("");
        }

        for encoded_method in &class.direct_methods {
            let method = &self.methods[encoded_method.method_idx as usize];
            let method_proto = &self.protos[method.proto_idx as usize];
            let method_name = &self.strings[method.name_idx as usize];
            let access_level = get_method_access_level_string(encoded_method.access_flags);

            let return_type = &self.types[method_proto.return_type_idx as usize];
            let return_type_string = get_type_descriptor_string(&return_type.parsed);
            let param_types = method_proto.parameter_type_idx_list.iter()
                .map(|idx: &TypeIndex| {
                    let t = &self.types[*idx].parsed;
                    return get_type_descriptor_string(t);
                })
                .collect::<Vec<String>>()
                .join(", ");

            write!(&mut result, "\t{} {} {}({}) {{\n", access_level, return_type_string, method_name, param_types).expect("");
            match &encoded_method.code_item {
                Some(c) => {
                    write!(&mut result, "\t\t// Offset: {:x}\n", c.addr).expect("");
                    write!(&mut result, "\t\t// Ins: {:?}\n", c.ins_size).expect("");
                    //println!("+++++++++++ {} {}", c.addr + 16, c.addr);
                    let instructions = parse_bytecode(&mut self.parser, (c.addr + 16) as usize, c.instructions_size as usize);
                    for i in instructions {
                        write!(&mut result, "\t\t{:?}\n", i).expect("");
                    }
                },
                None => {},
            };
            write!(&mut result, "\t}}\n").expect("");
        }

        write!(&mut result, "}}\n").expect("");
        print!("{}", result);
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
        .filter(|(access_level, _)| access_level & x >= 1)
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
        .filter(|(access_level, _)| access_level & x >= 1)
        .map(|(_, s)| *s)
        .collect::<Vec<&str>>()
        .join(" ");
}

fn get_method_access_level_string(x: u32) -> String {
    let access_levels: Vec<(u32, &str)> = vec![
        (MethodAccessLevel::Public       as u32, "public"),
        (MethodAccessLevel::Private      as u32, "private"),
        (MethodAccessLevel::Protected    as u32, "protected"),
        (MethodAccessLevel::Static       as u32, "static"),
        (MethodAccessLevel::Final        as u32, "final"),
        (MethodAccessLevel::Synchronized as u32, ""),
        (MethodAccessLevel::Bridge       as u32, ""),
        (MethodAccessLevel::VarArgs      as u32, ""),
        (MethodAccessLevel::Native       as u32, ""),
        (MethodAccessLevel::Abstract     as u32, "abstract"),
        (MethodAccessLevel::Strict       as u32, ""),
        (MethodAccessLevel::Synthetic    as u32, ""),
        (MethodAccessLevel::Constructor  as u32, ""),
        (MethodAccessLevel::DeclaredSynchronized as u32, ""),
    ];

    return access_levels.iter()
        .filter(|(access_level, _)| access_level & x >= 1)
        .map(|(_, s)| *s)
        .collect::<Vec<&str>>()
        .join(" ");
}

fn get_type_descriptor_string(t: &TypeDescriptor) -> String {
    match t {
        TypeDescriptor::Void => String::from("void"),
        TypeDescriptor::Boolean => String::from("boolean"),
        TypeDescriptor::Byte => String::from("byte"),
        TypeDescriptor::Short => String::from("short"),
        TypeDescriptor::Char => String::from("char"),
        TypeDescriptor::Int => String::from("int"),
        TypeDescriptor::Long => String::from("long"),
        TypeDescriptor::Float => String::from("float"),
        TypeDescriptor::Double => String::from("double"),
        TypeDescriptor::Class(x) => x.to_string().replace("/", "."),
        TypeDescriptor::Array(b) => {
            format!("{}[]", get_type_descriptor_string(b))
        },
    }
}