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
pub struct Regression {
    // message fields
    pub value: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Regression {}

impl Regression {
    pub fn new() -> Regression {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Regression {
        static mut instance: ::protobuf::lazy::Lazy<Regression> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Regression,
        };
        unsafe {
            instance.get(Regression::new)
        }
    }

    // float value = 1;

    pub fn clear_value(&mut self) {
        self.value = 0.;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f32) {
        self.value = v;
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &f32 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut f32 {
        &mut self.value
    }
}

impl ::protobuf::Message for Regression {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.value = tmp;
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
        if self.value != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != 0. {
            os.write_float(1, self.value)?;
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

impl ::protobuf::MessageStatic for Regression {
    fn new() -> Regression {
        Regression::new()
    }

    fn descriptor_static(_: ::std::option::Option<Regression>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "value",
                    Regression::get_value_for_reflect,
                    Regression::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Regression>(
                    "Regression",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Regression {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Regression {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Regression {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegressionResult {
    // message fields
    pub regressions: ::protobuf::RepeatedField<Regression>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegressionResult {}

impl RegressionResult {
    pub fn new() -> RegressionResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegressionResult {
        static mut instance: ::protobuf::lazy::Lazy<RegressionResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegressionResult,
        };
        unsafe {
            instance.get(RegressionResult::new)
        }
    }

    // repeated .tensorflow.serving.Regression regressions = 1;

    pub fn clear_regressions(&mut self) {
        self.regressions.clear();
    }

    // Param is passed by value, moved
    pub fn set_regressions(&mut self, v: ::protobuf::RepeatedField<Regression>) {
        self.regressions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_regressions(&mut self) -> &mut ::protobuf::RepeatedField<Regression> {
        &mut self.regressions
    }

    // Take field
    pub fn take_regressions(&mut self) -> ::protobuf::RepeatedField<Regression> {
        ::std::mem::replace(&mut self.regressions, ::protobuf::RepeatedField::new())
    }

    pub fn get_regressions(&self) -> &[Regression] {
        &self.regressions
    }

    fn get_regressions_for_reflect(&self) -> &::protobuf::RepeatedField<Regression> {
        &self.regressions
    }

    fn mut_regressions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Regression> {
        &mut self.regressions
    }
}

impl ::protobuf::Message for RegressionResult {
    fn is_initialized(&self) -> bool {
        for v in &self.regressions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.regressions)?;
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
        for value in &self.regressions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.regressions {
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

impl ::protobuf::MessageStatic for RegressionResult {
    fn new() -> RegressionResult {
        RegressionResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegressionResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Regression>>(
                    "regressions",
                    RegressionResult::get_regressions_for_reflect,
                    RegressionResult::mut_regressions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegressionResult>(
                    "RegressionResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegressionResult {
    fn clear(&mut self) {
        self.clear_regressions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegressionResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegressionResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegressionRequest {
    // message fields
    pub model_spec: ::protobuf::SingularPtrField<super::model::ModelSpec>,
    pub input: ::protobuf::SingularPtrField<super::input::Input>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegressionRequest {}

impl RegressionRequest {
    pub fn new() -> RegressionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegressionRequest {
        static mut instance: ::protobuf::lazy::Lazy<RegressionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegressionRequest,
        };
        unsafe {
            instance.get(RegressionRequest::new)
        }
    }

    // .tensorflow.serving.ModelSpec model_spec = 1;

    pub fn clear_model_spec(&mut self) {
        self.model_spec.clear();
    }

    pub fn has_model_spec(&self) -> bool {
        self.model_spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_spec(&mut self, v: super::model::ModelSpec) {
        self.model_spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_model_spec(&mut self) -> &mut super::model::ModelSpec {
        if self.model_spec.is_none() {
            self.model_spec.set_default();
        }
        self.model_spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_model_spec(&mut self) -> super::model::ModelSpec {
        self.model_spec.take().unwrap_or_else(|| super::model::ModelSpec::new())
    }

    pub fn get_model_spec(&self) -> &super::model::ModelSpec {
        self.model_spec.as_ref().unwrap_or_else(|| super::model::ModelSpec::default_instance())
    }

    fn get_model_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<super::model::ModelSpec> {
        &self.model_spec
    }

    fn mut_model_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::model::ModelSpec> {
        &mut self.model_spec
    }

    // .tensorflow.serving.Input input = 2;

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    pub fn has_input(&self) -> bool {
        self.input.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input(&mut self, v: super::input::Input) {
        self.input = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_input(&mut self) -> &mut super::input::Input {
        if self.input.is_none() {
            self.input.set_default();
        }
        self.input.as_mut().unwrap()
    }

    // Take field
    pub fn take_input(&mut self) -> super::input::Input {
        self.input.take().unwrap_or_else(|| super::input::Input::new())
    }

    pub fn get_input(&self) -> &super::input::Input {
        self.input.as_ref().unwrap_or_else(|| super::input::Input::default_instance())
    }

    fn get_input_for_reflect(&self) -> &::protobuf::SingularPtrField<super::input::Input> {
        &self.input
    }

    fn mut_input_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::input::Input> {
        &mut self.input
    }
}

impl ::protobuf::Message for RegressionRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.model_spec {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.input {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.model_spec)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.input)?;
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
        if let Some(ref v) = self.model_spec.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.input.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.model_spec.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.input.as_ref() {
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

impl ::protobuf::MessageStatic for RegressionRequest {
    fn new() -> RegressionRequest {
        RegressionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegressionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::model::ModelSpec>>(
                    "model_spec",
                    RegressionRequest::get_model_spec_for_reflect,
                    RegressionRequest::mut_model_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::input::Input>>(
                    "input",
                    RegressionRequest::get_input_for_reflect,
                    RegressionRequest::mut_input_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegressionRequest>(
                    "RegressionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegressionRequest {
    fn clear(&mut self) {
        self.clear_model_spec();
        self.clear_input();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegressionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegressionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegressionResponse {
    // message fields
    pub result: ::protobuf::SingularPtrField<RegressionResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegressionResponse {}

impl RegressionResponse {
    pub fn new() -> RegressionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegressionResponse {
        static mut instance: ::protobuf::lazy::Lazy<RegressionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegressionResponse,
        };
        unsafe {
            instance.get(RegressionResponse::new)
        }
    }

    // .tensorflow.serving.RegressionResult result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: RegressionResult) {
        self.result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut RegressionResult {
        if self.result.is_none() {
            self.result.set_default();
        }
        self.result.as_mut().unwrap()
    }

    // Take field
    pub fn take_result(&mut self) -> RegressionResult {
        self.result.take().unwrap_or_else(|| RegressionResult::new())
    }

    pub fn get_result(&self) -> &RegressionResult {
        self.result.as_ref().unwrap_or_else(|| RegressionResult::default_instance())
    }

    fn get_result_for_reflect(&self) -> &::protobuf::SingularPtrField<RegressionResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RegressionResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for RegressionResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.result {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.result)?;
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
        if let Some(ref v) = self.result.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.result.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RegressionResponse {
    fn new() -> RegressionResponse {
        RegressionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegressionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RegressionResult>>(
                    "result",
                    RegressionResponse::get_result_for_reflect,
                    RegressionResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegressionResponse>(
                    "RegressionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegressionResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegressionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegressionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(tensorflow_serving/apis/regression.proto\x12\x12tensorflow.serving\
    \x1a#tensorflow_serving/apis/input.proto\x1a#tensorflow_serving/apis/mod\
    el.proto\"\"\n\nRegression\x12\x14\n\x05value\x18\x01\x20\x01(\x02R\x05v\
    alue\"T\n\x10RegressionResult\x12@\n\x0bregressions\x18\x01\x20\x03(\x0b\
    2\x1e.tensorflow.serving.RegressionR\x0bregressions\"\x82\x01\n\x11Regre\
    ssionRequest\x12<\n\nmodel_spec\x18\x01\x20\x01(\x0b2\x1d.tensorflow.ser\
    ving.ModelSpecR\tmodelSpec\x12/\n\x05input\x18\x02\x20\x01(\x0b2\x19.ten\
    sorflow.serving.InputR\x05input\"R\n\x12RegressionResponse\x12<\n\x06res\
    ult\x18\x01\x20\x01(\x0b2$.tensorflow.serving.RegressionResultR\x06resul\
    tB\x03\xf8\x01\x01J\x98\x06\n\x06\x12\x04\0\0\x20\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\x08\n\x01\x08\x12\x03\x02\0\x1f\n\x0b\n\x04\x08\xe7\
    \x07\0\x12\x03\x02\0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x02\x07\
    \x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x02\x07\x17\n\x0e\n\x07\x08\
    \xe7\x07\0\x02\0\x01\x12\x03\x02\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\
    \x12\x03\x02\x1a\x1e\n\t\n\x02\x03\0\x12\x03\x04\x07,\n\t\n\x02\x03\x01\
    \x12\x03\x05\x07,\n\x08\n\x01\x02\x12\x03\x07\x08\x1a\nG\n\x02\x04\0\x12\
    \x04\n\0\x0c\x01\x1a;\x20Regression\x20result\x20for\x20a\x20single\x20i\
    tem\x20(tensorflow.Example).\n\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x12\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x0b\x02\x12\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x04\x0b\x02\n\x14\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0b\x02\x07\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x0b\x08\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x0b\x10\x11\nj\n\x02\x04\x01\x12\x04\x10\0\x12\x01\x1a^\x20Contains\
    \x20one\x20result\x20per\x20input\x20example,\x20in\x20the\x20same\x20or\
    der\x20as\x20the\x20input\x20in\n\x20RegressionRequest.\n\n\n\n\x03\x04\
    \x01\x01\x12\x03\x10\x08\x18\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x11\x02&\
    \n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x11\x02\n\n\x0c\n\x05\x04\x01\x02\
    \0\x06\x12\x03\x11\x0b\x15\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x11\x16\
    !\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x11$%\n\x1d\n\x02\x04\x02\x12\
    \x04\x16\0\x1c\x012\x11\x20RPC\x20interfaces.\n\n\n\n\x03\x04\x02\x01\
    \x12\x03\x16\x08\x19\n#\n\x04\x04\x02\x02\0\x12\x03\x18\x02\x1b\x1a\x16\
    \x20Model\x20Specification.\n\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x18\
    \x02\x16\x1b\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x18\x02\x0b\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03\x18\x0c\x16\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x18\x19\x1a\n\x1a\n\x04\x04\x02\x02\x01\x12\x03\x1b\x02%\x1a\r\
    \x20Input\x20data.\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x1b\x02\x18\
    \x1b\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x1b\x02\x1a\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03\x1b\x1b\x20\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\x1b#$\n\n\n\x02\x04\x03\x12\x04\x1e\0\x20\x01\n\n\n\x03\x04\x03\x01\
    \x12\x03\x1e\x08\x1a\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1f\x02\x1e\n\r\n\
    \x05\x04\x03\x02\0\x04\x12\x04\x1f\x02\x1e\x1c\n\x0c\n\x05\x04\x03\x02\0\
    \x06\x12\x03\x1f\x02\x12\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1f\x13\
    \x19\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1f\x1c\x1db\x06proto3\
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
