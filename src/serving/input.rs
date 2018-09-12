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
pub struct ExampleList {
    // message fields
    pub examples: ::protobuf::RepeatedField<super::example::Example>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExampleList {}

impl ExampleList {
    pub fn new() -> ExampleList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExampleList {
        static mut instance: ::protobuf::lazy::Lazy<ExampleList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExampleList,
        };
        unsafe {
            instance.get(ExampleList::new)
        }
    }

    // repeated .tensorflow.Example examples = 1;

    pub fn clear_examples(&mut self) {
        self.examples.clear();
    }

    // Param is passed by value, moved
    pub fn set_examples(&mut self, v: ::protobuf::RepeatedField<super::example::Example>) {
        self.examples = v;
    }

    // Mutable pointer to the field.
    pub fn mut_examples(&mut self) -> &mut ::protobuf::RepeatedField<super::example::Example> {
        &mut self.examples
    }

    // Take field
    pub fn take_examples(&mut self) -> ::protobuf::RepeatedField<super::example::Example> {
        ::std::mem::replace(&mut self.examples, ::protobuf::RepeatedField::new())
    }

    pub fn get_examples(&self) -> &[super::example::Example] {
        &self.examples
    }

    fn get_examples_for_reflect(&self) -> &::protobuf::RepeatedField<super::example::Example> {
        &self.examples
    }

    fn mut_examples_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::example::Example> {
        &mut self.examples
    }
}

impl ::protobuf::Message for ExampleList {
    fn is_initialized(&self) -> bool {
        for v in &self.examples {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.examples)?;
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
        for value in &self.examples {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.examples {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for ExampleList {
    fn new() -> ExampleList {
        ExampleList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExampleList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::example::Example>>(
                    "examples",
                    ExampleList::get_examples_for_reflect,
                    ExampleList::mut_examples_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExampleList>(
                    "ExampleList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExampleList {
    fn clear(&mut self) {
        self.clear_examples();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExampleList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExampleList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExampleListWithContext {
    // message fields
    pub examples: ::protobuf::RepeatedField<super::example::Example>,
    pub context: ::protobuf::SingularPtrField<super::example::Example>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExampleListWithContext {}

impl ExampleListWithContext {
    pub fn new() -> ExampleListWithContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExampleListWithContext {
        static mut instance: ::protobuf::lazy::Lazy<ExampleListWithContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExampleListWithContext,
        };
        unsafe {
            instance.get(ExampleListWithContext::new)
        }
    }

    // repeated .tensorflow.Example examples = 1;

    pub fn clear_examples(&mut self) {
        self.examples.clear();
    }

    // Param is passed by value, moved
    pub fn set_examples(&mut self, v: ::protobuf::RepeatedField<super::example::Example>) {
        self.examples = v;
    }

    // Mutable pointer to the field.
    pub fn mut_examples(&mut self) -> &mut ::protobuf::RepeatedField<super::example::Example> {
        &mut self.examples
    }

    // Take field
    pub fn take_examples(&mut self) -> ::protobuf::RepeatedField<super::example::Example> {
        ::std::mem::replace(&mut self.examples, ::protobuf::RepeatedField::new())
    }

    pub fn get_examples(&self) -> &[super::example::Example] {
        &self.examples
    }

    fn get_examples_for_reflect(&self) -> &::protobuf::RepeatedField<super::example::Example> {
        &self.examples
    }

    fn mut_examples_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::example::Example> {
        &mut self.examples
    }

    // .tensorflow.Example context = 2;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: super::example::Example) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut super::example::Example {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> super::example::Example {
        self.context.take().unwrap_or_else(|| super::example::Example::new())
    }

    pub fn get_context(&self) -> &super::example::Example {
        self.context.as_ref().unwrap_or_else(|| super::example::Example::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<super::example::Example> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::example::Example> {
        &mut self.context
    }
}

impl ::protobuf::Message for ExampleListWithContext {
    fn is_initialized(&self) -> bool {
        for v in &self.examples {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.context {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.examples)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
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
        for value in &self.examples {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.examples {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ExampleListWithContext {
    fn new() -> ExampleListWithContext {
        ExampleListWithContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExampleListWithContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::example::Example>>(
                    "examples",
                    ExampleListWithContext::get_examples_for_reflect,
                    ExampleListWithContext::mut_examples_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::example::Example>>(
                    "context",
                    ExampleListWithContext::get_context_for_reflect,
                    ExampleListWithContext::mut_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExampleListWithContext>(
                    "ExampleListWithContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExampleListWithContext {
    fn clear(&mut self) {
        self.clear_examples();
        self.clear_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExampleListWithContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExampleListWithContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Input {
    // message oneof groups
    kind: ::std::option::Option<Input_oneof_kind>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Input {}

#[derive(Clone,PartialEq)]
pub enum Input_oneof_kind {
    example_list(ExampleList),
    example_list_with_context(ExampleListWithContext),
}

impl Input {
    pub fn new() -> Input {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Input {
        static mut instance: ::protobuf::lazy::Lazy<Input> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Input,
        };
        unsafe {
            instance.get(Input::new)
        }
    }

    // .tensorflow.serving.ExampleList example_list = 1;

    pub fn clear_example_list(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_example_list(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Input_oneof_kind::example_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_example_list(&mut self, v: ExampleList) {
        self.kind = ::std::option::Option::Some(Input_oneof_kind::example_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_example_list(&mut self) -> &mut ExampleList {
        if let ::std::option::Option::Some(Input_oneof_kind::example_list(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Input_oneof_kind::example_list(ExampleList::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Input_oneof_kind::example_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_example_list(&mut self) -> ExampleList {
        if self.has_example_list() {
            match self.kind.take() {
                ::std::option::Option::Some(Input_oneof_kind::example_list(v)) => v,
                _ => panic!(),
            }
        } else {
            ExampleList::new()
        }
    }

    pub fn get_example_list(&self) -> &ExampleList {
        match self.kind {
            ::std::option::Option::Some(Input_oneof_kind::example_list(ref v)) => v,
            _ => ExampleList::default_instance(),
        }
    }

    // .tensorflow.serving.ExampleListWithContext example_list_with_context = 2;

    pub fn clear_example_list_with_context(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_example_list_with_context(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Input_oneof_kind::example_list_with_context(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_example_list_with_context(&mut self, v: ExampleListWithContext) {
        self.kind = ::std::option::Option::Some(Input_oneof_kind::example_list_with_context(v))
    }

    // Mutable pointer to the field.
    pub fn mut_example_list_with_context(&mut self) -> &mut ExampleListWithContext {
        if let ::std::option::Option::Some(Input_oneof_kind::example_list_with_context(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Input_oneof_kind::example_list_with_context(ExampleListWithContext::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Input_oneof_kind::example_list_with_context(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_example_list_with_context(&mut self) -> ExampleListWithContext {
        if self.has_example_list_with_context() {
            match self.kind.take() {
                ::std::option::Option::Some(Input_oneof_kind::example_list_with_context(v)) => v,
                _ => panic!(),
            }
        } else {
            ExampleListWithContext::new()
        }
    }

    pub fn get_example_list_with_context(&self) -> &ExampleListWithContext {
        match self.kind {
            ::std::option::Option::Some(Input_oneof_kind::example_list_with_context(ref v)) => v,
            _ => ExampleListWithContext::default_instance(),
        }
    }
}

impl ::protobuf::Message for Input {
    fn is_initialized(&self) -> bool {
        if let Some(Input_oneof_kind::example_list(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Input_oneof_kind::example_list_with_context(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Input_oneof_kind::example_list(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Input_oneof_kind::example_list_with_context(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &Input_oneof_kind::example_list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Input_oneof_kind::example_list_with_context(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &Input_oneof_kind::example_list(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Input_oneof_kind::example_list_with_context(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for Input {
    fn new() -> Input {
        Input::new()
    }

    fn descriptor_static(_: ::std::option::Option<Input>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ExampleList>(
                    "example_list",
                    Input::has_example_list,
                    Input::get_example_list,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ExampleListWithContext>(
                    "example_list_with_context",
                    Input::has_example_list_with_context,
                    Input::get_example_list_with_context,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Input>(
                    "Input",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Input {
    fn clear(&mut self) {
        self.clear_example_list();
        self.clear_example_list_with_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Input {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Input {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#tensorflow_serving/apis/input.proto\x12\x12tensorflow.serving\x1a%ten\
    sorflow/core/example/example.proto\">\n\x0bExampleList\x12/\n\x08example\
    s\x18\x01\x20\x03(\x0b2\x13.tensorflow.ExampleR\x08examples\"x\n\x16Exam\
    pleListWithContext\x12/\n\x08examples\x18\x01\x20\x03(\x0b2\x13.tensorfl\
    ow.ExampleR\x08examples\x12-\n\x07context\x18\x02\x20\x01(\x0b2\x13.tens\
    orflow.ExampleR\x07context\"\xc6\x01\n\x05Input\x12H\n\x0cexample_list\
    \x18\x01\x20\x01(\x0b2\x1f.tensorflow.serving.ExampleListH\0R\x0bexample\
    ListB\x02(\x01\x12k\n\x19example_list_with_context\x18\x02\x20\x01(\x0b2\
    *.tensorflow.serving.ExampleListWithContextH\0R\x16exampleListWithContex\
    tB\x02(\x01B\x06\n\x04kindB\x03\xf8\x01\x01J\xb8\x10\n\x06\x12\x04\x03\0\
    K\x01\nl\n\x01\x0c\x12\x03\x03\0\x122b\x20Input\x20used\x20in\x20serving\
    \x20APIs.\x20\x20Based\x20on\x20the\x20tensorflow.Example\x20family\x20o\
    f\n\x20feature\x20representations.\n\n\x08\n\x01\x08\x12\x03\x05\0\x1f\n\
    \x0b\n\x04\x08\xe7\x07\0\x12\x03\x05\0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x05\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x05\x07\x17\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x05\x07\x17\n\x0c\n\x05\x08\
    \xe7\x07\0\x03\x12\x03\x05\x1a\x1e\n\t\n\x02\x03\0\x12\x03\x07\x07.\n\
    \x08\n\x01\x02\x12\x03\t\x08\x1a\n\xb8\x01\n\x02\x04\0\x12\x04\x0e\0\x10\
    \x01\x1a\xab\x01\x20Specifies\x20one\x20or\x20more\x20fully\x20independe\
    nt\x20input\x20Examples.\n\x20See\x20examples\x20at:\n\x20\x20\x20\x20\
    \x20https://github.com/tensorflow/tensorflow/blob/master/tensorflow/core\
    /example/example.proto\n\n\n\n\x03\x04\0\x01\x12\x03\x0e\x08\x13\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x0f\x02+\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\
    \x0f\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0f\x0b\x1d\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\x0f\x1e&\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\
    \x0f)*\n\xbf\x08\n\x02\x04\x01\x12\x04A\0D\x01\x1a\xb2\x08\x20Specifies\
    \x20one\x20or\x20more\x20independent\x20input\x20Examples,\x20with\x20a\
    \x20common\x20context\n\x20Example.\n\n\x20The\x20common\x20use\x20case\
    \x20for\x20context\x20is\x20to\x20cleanly\x20and\x20optimally\x20specify\
    \x20some\n\x20features\x20that\x20are\x20common\x20across\x20multiple\
    \x20examples.\n\n\x20See\x20example\x20below\x20with\x20a\x20search\x20q\
    uery\x20as\x20the\x20context\x20and\x20multiple\x20restaurants\n\x20to\
    \x20perform\x20some\x20inference\x20on.\n\n\x20context:\x20{\n\x20\x20\
    \x20feature:\x20{\n\x20\x20\x20\x20\x20key\x20\x20:\x20\"query\"\n\x20\
    \x20\x20\x20\x20value:\x20{\n\x20\x20\x20\x20\x20\x20\x20bytes_list:\x20\
    {\n\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20[\x20\"pizza\"\x20]\n\
    \x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20}\n\
    \x20}\n\x20examples:\x20{\n\x20\x20\x20feature:\x20{\n\x20\x20\x20\x20\
    \x20key\x20\x20:\x20\"cuisine\"\n\x20\x20\x20\x20\x20value:\x20{\n\x20\
    \x20\x20\x20\x20\x20\x20bytes_list:\x20{\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20value:\x20[\x20\"Pizzeria\"\x20]\n\x20\x20\x20\x20\x20\x20\x20}\
    \n\x20\x20\x20\x20\x20}\n\x20\x20\x20}\n\x20}\n\x20examples:\x20{\n\x20\
    \x20\x20feature:\x20{\n\x20\x20\x20\x20\x20key\x20\x20:\x20\"cuisine\"\n\
    \x20\x20\x20\x20\x20value:\x20{\n\x20\x20\x20\x20\x20\x20\x20bytes_list:\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20[\x20\"Taqueria\"\
    \x20]\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20}\n\x20}\n\n\x20Implementations\x20of\x20ExampleListWithContext\x20m\
    erge\x20the\x20context\x20Example\x20into\x20each\n\x20of\x20the\x20Exam\
    ples.\x20Note\x20that\x20feature\x20keys\x20must\x20not\x20be\x20duplica\
    ted\x20between\x20the\n\x20Examples\x20and\x20context\x20Example,\x20or\
    \x20the\x20behavior\x20is\x20undefined.\n\n\x20See\x20also:\n\x20\x20\
    \x20\x20\x20tensorflow/core/example/example.proto\n\x20\x20\x20\x20\x20h\
    ttps://developers.google.com/protocol-buffers/docs/proto3#maps\n\n\n\n\
    \x03\x04\x01\x01\x12\x03A\x08\x1e\n\x0b\n\x04\x04\x01\x02\0\x12\x03B\x02\
    +\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03B\x02\n\n\x0c\n\x05\x04\x01\x02\0\
    \x06\x12\x03B\x0b\x1d\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03B\x1e&\n\x0c\
    \n\x05\x04\x01\x02\0\x03\x12\x03B)*\n\x0b\n\x04\x04\x01\x02\x01\x12\x03C\
    \x02!\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04C\x02B+\n\x0c\n\x05\x04\x01\
    \x02\x01\x06\x12\x03C\x02\x14\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03C\
    \x15\x1c\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03C\x1f\x20\n\n\n\x02\x04\
    \x02\x12\x04F\0K\x01\n\n\n\x03\x04\x02\x01\x12\x03F\x08\r\n\x0c\n\x04\
    \x04\x02\x08\0\x12\x04G\x02J\x03\n\x0c\n\x05\x04\x02\x08\0\x01\x12\x03G\
    \x08\x0c\n\x0b\n\x04\x04\x02\x02\0\x12\x03H\x04/\n\x0c\n\x05\x04\x02\x02\
    \0\x06\x12\x03H\x04\x0f\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03H\x10\x1c\n\
    \x0c\n\x05\x04\x02\x02\0\x03\x12\x03H\x1f\x20\n\x0c\n\x05\x04\x02\x02\0\
    \x08\x12\x03H!.\n\x0f\n\x08\x04\x02\x02\0\x08\xe7\x07\0\x12\x03H\"-\n\
    \x10\n\t\x04\x02\x02\0\x08\xe7\x07\0\x02\x12\x03H\"&\n\x11\n\n\x04\x02\
    \x02\0\x08\xe7\x07\0\x02\0\x12\x03H\"&\n\x12\n\x0b\x04\x02\x02\0\x08\xe7\
    \x07\0\x02\0\x01\x12\x03H\"&\n\x10\n\t\x04\x02\x02\0\x08\xe7\x07\0\x03\
    \x12\x03H)-\n\x0b\n\x04\x04\x02\x02\x01\x12\x03I\x04G\n\x0c\n\x05\x04\
    \x02\x02\x01\x06\x12\x03I\x04\x1a\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\
    \x03I\x1b4\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03I78\n\x0c\n\x05\x04\
    \x02\x02\x01\x08\x12\x03I9F\n\x0f\n\x08\x04\x02\x02\x01\x08\xe7\x07\0\
    \x12\x03I:E\n\x10\n\t\x04\x02\x02\x01\x08\xe7\x07\0\x02\x12\x03I:>\n\x11\
    \n\n\x04\x02\x02\x01\x08\xe7\x07\0\x02\0\x12\x03I:>\n\x12\n\x0b\x04\x02\
    \x02\x01\x08\xe7\x07\0\x02\0\x01\x12\x03I:>\n\x10\n\t\x04\x02\x02\x01\
    \x08\xe7\x07\0\x03\x12\x03IAEb\x06proto3\
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
