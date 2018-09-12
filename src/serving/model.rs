// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ModelSpec {
    // message fields
    pub name: ::std::string::String,
    pub version: ::protobuf::SingularPtrField<::protobuf::well_known_types::Int64Value>,
    pub signature_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ModelSpec {}

impl ModelSpec {
    pub fn new() -> ModelSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ModelSpec {
        static mut instance: ::protobuf::lazy::Lazy<ModelSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ModelSpec,
        };
        unsafe {
            instance.get(ModelSpec::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .google.protobuf.Int64Value version = 2;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::protobuf::well_known_types::Int64Value) {
        self.version = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::protobuf::well_known_types::Int64Value {
        if self.version.is_none() {
            self.version.set_default();
        }
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::protobuf::well_known_types::Int64Value {
        self.version.take().unwrap_or_else(|| ::protobuf::well_known_types::Int64Value::new())
    }

    pub fn get_version(&self) -> &::protobuf::well_known_types::Int64Value {
        self.version.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Int64Value::default_instance())
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Int64Value> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Int64Value> {
        &mut self.version
    }

    // string signature_name = 3;

    pub fn clear_signature_name(&mut self) {
        self.signature_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature_name(&mut self, v: ::std::string::String) {
        self.signature_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature_name(&mut self) -> &mut ::std::string::String {
        &mut self.signature_name
    }

    // Take field
    pub fn take_signature_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.signature_name, ::std::string::String::new())
    }

    pub fn get_signature_name(&self) -> &str {
        &self.signature_name
    }

    fn get_signature_name_for_reflect(&self) -> &::std::string::String {
        &self.signature_name
    }

    fn mut_signature_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.signature_name
    }
}

impl ::protobuf::Message for ModelSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.version {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.version)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.signature_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.version.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.signature_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.signature_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.version.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.signature_name.is_empty() {
            os.write_string(3, &self.signature_name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ModelSpec {
    fn new() -> ModelSpec {
        ModelSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<ModelSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ModelSpec::get_name_for_reflect,
                    ModelSpec::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Int64Value>>(
                    "version",
                    ModelSpec::get_version_for_reflect,
                    ModelSpec::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "signature_name",
                    ModelSpec::get_signature_name_for_reflect,
                    ModelSpec::mut_signature_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ModelSpec>(
                    "ModelSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ModelSpec {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_version();
        self.clear_signature_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ModelSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ModelSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#tensorflow_serving/apis/model.proto\x12\x12tensorflow.serving\x1a\x1e\
    google/protobuf/wrappers.proto\"}\n\tModelSpec\x12\x12\n\x04name\x18\x01\
    \x20\x01(\tR\x04name\x125\n\x07version\x18\x02\x20\x01(\x0b2\x1b.google.\
    protobuf.Int64ValueR\x07version\x12%\n\x0esignature_name\x18\x03\x20\x01\
    (\tR\rsignatureNameB\x03\xf8\x01\x01J\x8d\x06\n\x06\x12\x04\0\0\x15\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x1a\n\
    \x08\n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\
    \x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\
    \x07\0\x02\0\x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\
    \x03\x03\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\t\n\
    \x02\x03\0\x12\x03\x05\x07'\nS\n\x02\x04\0\x12\x04\x08\0\x15\x01\x1aG\
    \x20Metadata\x20for\x20an\x20inference\x20request\x20such\x20as\x20the\
    \x20model\x20name\x20and\x20version.\n\n\n\n\x03\x04\0\x01\x12\x03\x08\
    \x08\x11\n&\n\x04\x04\0\x02\0\x12\x03\n\x02\x12\x1a\x19\x20Required\x20s\
    ervable\x20name.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\n\x02\x08\x13\n\x0c\
    \n\x05\x04\0\x02\0\x05\x12\x03\n\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\n\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\n\x10\x11\n\xf9\x01\n\x04\
    \x04\0\x02\x01\x12\x03\x10\x02)\x1a\xeb\x01\x20Optional\x20version.\x20I\
    f\x20unspecified,\x20will\x20use\x20the\x20latest\x20(numerical)\x20vers\
    ion.\n\x20Typically\x20not\x20needed\x20unless\x20coordinating\x20across\
    \x20multiple\x20models\x20that\x20were\n\x20co-trained\x20and/or\x20have\
    \x20inter-dependencies\x20on\x20the\x20versions\x20used\x20at\x20inferen\
    ce\n\x20time.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x10\x02\n\x12\n\x0c\
    \n\x05\x04\0\x02\x01\x06\x12\x03\x10\x02\x1c\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\x10\x1d$\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x10'(\nb\n\
    \x04\x04\0\x02\x02\x12\x03\x14\x02\x1c\x1aU\x20A\x20named\x20signature\
    \x20to\x20evaluate.\x20If\x20unspecified,\x20the\x20default\x20signature\
    \x20will\n\x20be\x20used.\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x14\x02\
    \x10)\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x14\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x02\x01\x12\x03\x14\t\x17\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\
    \x14\x1a\x1bb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
