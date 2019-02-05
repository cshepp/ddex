use crate::binary_parser::BinaryParser;
use crate::dex_types::*;
use crate::util::{to_decimal, to_decimal_short, to_utf8, to_hex_string};

pub fn parse_header(parser: &mut BinaryParser) -> DexHeader {
    parser.seek_to(0);
    let dex_magic = parser.take(8);
    assert_eq!(dex_magic[0..3], [0x64, 0x65, 0x78]);
    assert_eq!(dex_magic[3], 0x0a);
    assert_eq!(dex_magic[7], 0x00);
    let dex_version = dex_magic[4..7].to_vec();
    let checksum = parser.take(4);
    let sha1 = parser.take(20);
    let file_size = parser.take(4);
    let header_size = parser.take(4);
    let endian_constant = parser.take(4);
    let link_size = parser.take(4);
    let link_offset = parser.take(4);
    let map_offset = parser.take(4);
    let string_ids_size = parser.take(4);
    let string_ids_offset = parser.take(4);
    let type_ids_size = parser.take(4);
    let type_ids_offset = parser.take(4);
    let proto_ids_size = parser.take(4);
    let proto_ids_offset = parser.take(4);
    let field_ids_size = parser.take(4);
    let field_ids_offset = parser.take(4);
    let method_ids_size = parser.take(4);
    let method_ids_offset = parser.take(4);
    let class_defs_size = parser.take(4);
    let class_defs_offset = parser.take(4);
    let data_size = parser.take(4);
    let data_offset = parser.take(4);

    let endianness = match endian_constant.as_slice() {
        [0x78, 0x56, 0x34, 0x12] => Endianness::LittleEndian, 
        _ => Endianness::BigEndian,
    };

    DexHeader {
        dex_version: to_utf8(&dex_version),
        checksum: to_decimal(&checksum),
        sha1: to_hex_string(&sha1).replace(" ", ""),
        file_size: to_decimal(&file_size),
        header_size: to_decimal(&header_size),
        endianness,
        link_size: to_decimal(&link_size),
        link_offset: to_decimal(&link_offset),
        map_offset: to_decimal(&map_offset),
        string_ids_size: to_decimal(&string_ids_size),
        string_ids_offset: to_decimal(&string_ids_offset),
        type_ids_size: to_decimal(&type_ids_size),
        type_ids_offset: to_decimal(&type_ids_offset),
        proto_ids_size: to_decimal(&proto_ids_size),
        proto_ids_offset: to_decimal(&proto_ids_offset),
        field_ids_size: to_decimal(&field_ids_size),
        field_ids_offset: to_decimal(&field_ids_offset),
        method_ids_size: to_decimal(&method_ids_size),
        method_ids_offset: to_decimal(&method_ids_offset),
        class_defs_size: to_decimal(&class_defs_size),
        class_defs_offset: to_decimal(&class_defs_offset),
        data_size: to_decimal(&data_size),
        data_offset: to_decimal(&data_offset),
    }
}

pub fn parse_strings(parser: &mut BinaryParser, offset: usize, list_size: usize) -> Vec<String> {
    let parse_item = Box::new(|p: &mut BinaryParser| {
        let start_hex = &p.take(4);
        let start_addr = to_decimal(start_hex) as usize;

        p.seek_to(start_addr as usize);
        let _length = p.parse_uleb128();

        let s = p.take_until(0x00);
        p.expect(0x00);

        return to_utf8(&s);
    });
        
    return parse_list_items(parser, offset, list_size, 4, parse_item);
}

pub fn parse_types(parser: &mut BinaryParser, offset: usize, list_size: usize, strings: &Vec<String>) -> Vec<DexType> {
    let mut result: Vec<DexType> = Vec::new();
    let size_in_bytes = list_size * 4; // each type_id is 4 bytes
    parser.seek_to(offset);
    loop {
        let addr = parser.current_location();
        if addr >= offset + size_in_bytes {
            break;
        }

        let idx = to_decimal(&parser.take(4));
        let s = strings[idx as usize].clone();
        let t = DexType {
            raw: s.clone(),
            parsed: parse_type_descriptor(s),
        };

        result.push(t);
        parser.seek_to(addr + 4);
    }

    return result;
}

pub fn parse_protos(parser: &mut BinaryParser, offset: usize, list_size: usize) -> Vec<DexProto> {
    let parse_item = Box::new(|p: &mut BinaryParser| {
        let shorty_idx = to_decimal(&p.take(4)) as StringIndex;
        let return_type_idx = to_decimal(&p.take(4)) as TypeIndex;
        let parameters_offset = to_decimal(&p.take(4));
        
        let mut parameter_type_idx_list: Vec<TypeIndex> = Vec::new();
        if parameters_offset != 0 { // offset of 0 indicates no parameter_type_idx_list
            p.seek_to(parameters_offset as usize);
            let parameter_count = to_decimal(&p.take(4));

            for _ in 0..parameter_count {
                let param_idx = to_decimal_short(&p.take(2));
                parameter_type_idx_list.push(param_idx as TypeIndex);
            }
        }

        DexProto {
            shorty_idx,
            return_type_idx,
            parameters_offset,
            parameter_type_idx_list,
        }
    });

    return parse_list_items(parser, offset, list_size, 12, parse_item);
}

pub fn parse_fields(parser: &mut BinaryParser, offset: usize, list_size: usize) -> Vec<DexField> {
    let parse_item = Box::new(|p: &mut BinaryParser| {

        let class_idx = to_decimal_short(&p.take(2));
        let type_idx = to_decimal_short(&p.take(2));
        let name_idx = to_decimal(&p.take(4));

        DexField {
            class_idx: class_idx as ClassIndex,
            type_idx: type_idx as TypeIndex,
            name_idx: name_idx as StringIndex,
        }
    });

    return parse_list_items(parser, offset, list_size, 8, parse_item);
}

pub fn parse_methods(parser: &mut BinaryParser, offset: usize, list_size: usize) -> Vec<DexMethod> {
    let parse_item = Box::new(|p: &mut BinaryParser| {
        let class_idx = to_decimal_short(&p.take(2));
        let proto_idx = to_decimal_short(&p.take(2));
        let name_idx = to_decimal(&p.take(4));

        DexMethod {
            class_idx: class_idx as ClassIndex,
            proto_idx: proto_idx as ProtoIndex,
            name_idx: name_idx as StringIndex,
        }
    });

    return parse_list_items(parser, offset, list_size, 8, parse_item);
}

pub fn parse_class_defs(parser: &mut BinaryParser, offset: usize, list_size: usize) -> Vec<DexClassDef> {
    let parse_item = Box::new(|p: &mut BinaryParser| {
        let class_idx = to_decimal(&p.take(4)) as TypeIndex;
        let access_flags = to_decimal(&p.take(4));
        let superclass_idx = to_decimal(&p.take(4)) as TypeIndex;
        let interfaces_offset = to_decimal(&p.take(4));
        let source_file_idx = to_decimal(&p.take(4)) as StringIndex;
        let annotations_offset = to_decimal(&p.take(4));
        let class_data_offset = to_decimal(&p.take(4));
        let static_values_offset = to_decimal(&p.take(4));

        if class_data_offset == 0 {
            return DexClassDef {
                class_idx,
                access_flags,
                superclass_idx,
                interfaces_offset,
                source_file_idx,
                annotations_offset,
                class_data_offset,
                static_values_offset,
                static_fields: Vec::new(),
                instance_fields: Vec::new(),
                direct_methods: Vec::new(),
                virtual_methods: Vec::new(),
            };
        }

        p.seek_to(class_data_offset as usize);
        let static_fields_list_size = p.parse_uleb128();
        let instance_fields_list_size = p.parse_uleb128();
        let direct_methods_list_size = p.parse_uleb128();
        let virtual_methods_list_size = p.parse_uleb128();

        let static_fields = parse_encoded_fields(p, static_fields_list_size as usize);
        let instance_fields = parse_encoded_fields(p, instance_fields_list_size as usize);
        let direct_methods = parse_encoded_methods(p, direct_methods_list_size as usize);
        let virtual_methods = parse_encoded_methods(p, virtual_methods_list_size as usize);

        DexClassDef {
            class_idx,
            access_flags,
            superclass_idx,
            interfaces_offset,
            source_file_idx,
            annotations_offset,
            class_data_offset,
            static_values_offset,
            static_fields,
            instance_fields,
            direct_methods,
            virtual_methods,
        }
    });

    return parse_list_items(parser, offset, list_size, 32, parse_item);
}

fn parse_list_items<T>(
    parser: &mut BinaryParser,
    offset: usize,
    list_size: usize,
    list_item_size: usize,
    parse_item: Box<(Fn(&mut BinaryParser) -> T)>
) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let size_in_bytes = list_size * list_item_size;
    parser.seek_to(offset);
    loop {
        let addr = parser.current_location();
        if addr >= offset + size_in_bytes {
            break;
        }

        let item = parse_item(parser);
        result.push(item);
        parser.seek_to(addr + list_item_size);
    }

    return result;
}

fn parse_encoded_fields(p: &mut BinaryParser, list_size: usize) -> Vec<EncodedField> {
    let mut last_field_idx: Option<FieldIndex> = None;
    let mut fields: Vec<EncodedField> = Vec::new();
    for _ in 0..list_size {
        let field_idx_diff = p.parse_uleb128() as FieldIndex;
        let field_idx = match last_field_idx {
            Some(idx) => idx + field_idx_diff,
            None => field_idx_diff as FieldIndex,
        };
        let access_flags = p.parse_uleb128();
        fields.push(EncodedField{ field_idx: (field_idx as FieldIndex), access_flags });
        last_field_idx = Some(field_idx as FieldIndex);
    }
    return fields;
}

fn parse_encoded_methods(p: &mut BinaryParser, list_size: usize) -> Vec<EncodedMethod> {
    let mut last_method_idx: Option<u32> = None;
    let mut methods: Vec<EncodedMethod> = Vec::new();
    for _ in 0..list_size {
        let method_idx_diff = p.parse_uleb128();
        let method_idx = match last_method_idx {
            Some(idx) => idx + method_idx_diff,
            None => method_idx_diff,
        };
        let access_flags = p.parse_uleb128();
        let code_offset = p.parse_uleb128();
        let mut code_item = None;
        if code_offset != 0 {
            let addr = p.current_location();
            p.seek_to(code_offset as usize);
            let registers_size = to_decimal_short(&p.take(2));
            let ins_size = to_decimal_short(&p.take(2));
            let outs_size = to_decimal_short(&p.take(2));
            let tries_size = to_decimal_short(&p.take(2));
            let debug_info_offset = to_decimal(&p.take(4));
            let instructions_size = to_decimal(&p.take(4));
            let instructions = p.take(instructions_size as usize);

            code_item = Some(CodeItem {
                addr: code_offset,
                registers_size,
                ins_size,
                outs_size,
                tries_size,
                debug_info_offset,
                instructions_size,
                instructions,
            });

            p.seek_to(addr);
        }

        methods.push(EncodedMethod{ method_idx: (method_idx as MethodIndex), access_flags, code_offset, code_item });
        last_method_idx = Some(method_idx);
    }

    return methods;
}

fn parse_type_descriptor(s: String) -> TypeDescriptor {
    match s.chars().map(|x| x.clone()).collect::<Vec<char>>().as_slice() {
        ['V'] => TypeDescriptor::Void,
        ['Z'] => TypeDescriptor::Boolean,
        ['B'] => TypeDescriptor::Byte,
        ['S'] => TypeDescriptor::Short,
        ['C'] => TypeDescriptor::Char,
        ['I'] => TypeDescriptor::Int,
        ['J'] => TypeDescriptor::Long,
        ['F'] => TypeDescriptor::Float,
        ['D'] => TypeDescriptor::Double,
        c if *c.first().unwrap() == '[' => {
            let rest = c.iter().skip(1).collect::<String>();
            let nested_descriptor = parse_type_descriptor(rest);
            TypeDescriptor::Array(Box::new(nested_descriptor))
        },
        c if *c.first().unwrap() == 'L' => {
            let mut class_name = c.iter().skip(1).collect::<String>();
            class_name.pop(); // last char is always ';' so drop it
            TypeDescriptor::Class(class_name)
        }
        _ => TypeDescriptor:: Void,
    }
}