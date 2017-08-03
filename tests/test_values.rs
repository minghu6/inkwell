extern crate inkwell;

use self::inkwell::context::Context;
use self::inkwell::module::Linkage::*;
use self::inkwell::types::VectorType;
use self::inkwell::values::InstructionOpcode::*;

use std::ffi::CString;

#[test]
fn test_linkage() {
    let context = Context::create();
    let module = context.create_module("testing");

    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);

    let function = module.add_function("free_f32", &fn_type, None);

    assert_eq!(function.get_linkage(), ExternalLinkage);
}

#[test]
fn test_instructions() {
    let context = Context::create();
    let module = context.create_module("testing");
    let builder = context.create_builder();

    let void_type = context.void_type();
    let i64_type = context.i64_type();
    let f32_type = context.f32_type();
    let f32_ptr_type = f32_type.ptr_type(0);
    let fn_type = void_type.fn_type(&[&f32_ptr_type], false);

    let function = module.add_function("free_f32", &fn_type, None);
    let basic_block = context.append_basic_block(&function, "entry");

    builder.position_at_end(&basic_block);

    let arg1 = function.get_first_param().unwrap().into_pointer_value();

    let f32_val = f32_type.const_float(3.14);

    let store_instruction = builder.build_store(&arg1, &f32_val);
    let ptr_val = builder.build_ptr_to_int(&arg1, &i64_type, "ptr_val");
    let ptr = builder.build_int_to_ptr(&ptr_val, &f32_ptr_type, "ptr");
    let free_instruction = builder.build_free(&arg1);
    let return_instruction = builder.build_return(None);

    assert_eq!(store_instruction.get_opcode(), Store);
    assert_eq!(ptr_val.as_instruction().unwrap().get_opcode(), PtrToInt);
    assert_eq!(ptr.as_instruction().unwrap().get_opcode(), IntToPtr);
    assert_eq!(free_instruction.get_opcode(), Call);
    assert_eq!(return_instruction.get_opcode(), Return);
}

#[test]
fn test_tail_call() {
    let context = Context::create();
    let module = context.create_module("testing");
    let builder = context.create_builder();

    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);

    let function = module.add_function("do_nothing", &fn_type, None);

    let call_instruction = builder.build_call(&function, &[], "to_infinity_and_beyond", false);

    assert_eq!(call_instruction.right().unwrap().is_tail_call(), false);

    let call_instruction = builder.build_call(&function, &[], "to_infinity_and_beyond", true);

    assert_eq!(call_instruction.right().unwrap().is_tail_call(), true);
}

#[test]
fn test_const_null_ptr() {
    let context = Context::create();
    let void_type = context.void_type();
    let bool_type = context.bool_type();
    let i8_type = context.i8_type();
    let i16_type = context.i16_type();
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let i128_type = context.i128_type();
    let f16_type = context.f16_type();
    let f32_type = context.f32_type();
    let f64_type = context.f64_type();
    let f128_type = context.f128_type();
    let struct_type = context.struct_type(&[&i8_type, &f128_type], false);
    let ptr_type = f64_type.ptr_type(0);
    let vec_type = f64_type.vec_type(42);
    let array_type = f64_type.array_type(42);

    assert!(void_type.const_null_ptr().is_null());
    assert!(bool_type.const_null_ptr().is_null());
    assert!(i8_type.const_null_ptr().is_null());
    assert!(i16_type.const_null_ptr().is_null());
    assert!(i32_type.const_null_ptr().is_null());
    assert!(i64_type.const_null_ptr().is_null());
    assert!(i128_type.const_null_ptr().is_null());
    assert!(f16_type.const_null_ptr().is_null());
    assert!(f32_type.const_null_ptr().is_null());
    assert!(f64_type.const_null_ptr().is_null());
    assert!(f128_type.const_null_ptr().is_null());
    assert!(struct_type.const_null_ptr().is_null());
    assert!(ptr_type.const_null_ptr().is_null());
    assert!(vec_type.const_null_ptr().is_null());
    assert!(array_type.const_null_ptr().is_null());
}

#[test]
fn test_set_get_name() {
    let context = Context::create();
    let bool_type = context.bool_type();
    let i8_type = context.i8_type();
    let i16_type = context.i16_type();
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let i128_type = context.i128_type();
    let f16_type = context.f16_type();
    let f32_type = context.f32_type();
    let f64_type = context.f64_type();
    let f128_type = context.f128_type();
    let array_type = f64_type.array_type(42);
    let f128_ppc_type = context.f128_type_ppc();

    let bool_val = bool_type.const_int(0, false);
    let i8_val = i8_type.const_int(0, false);
    let i16_val = i16_type.const_int(0, false);
    let i32_val = i32_type.const_int(0, false);
    let i64_val = i64_type.const_int(0, false);
    let i128_val = i128_type.const_int(0, false);
    let f16_val = f16_type.const_float(0.0);
    let f32_val = f32_type.const_float(0.0);
    let f64_val = f64_type.const_float(0.0);
    let f128_val = f128_type.const_float(0.0);
    let ptr_val = bool_type.const_null_ptr();
    let array_val = array_type.const_array(&[&f64_val]);
    let struct_val = context.const_struct(&[&i8_val, &f128_val], false);
    let vec_val = VectorType::const_vector(&[&i8_val]);
    let f128_ppc_val = f128_ppc_type.const_float(0.0);

    assert_eq!(bool_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i8_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i16_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i32_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i64_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i128_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f16_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f32_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f64_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f128_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(ptr_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(array_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(struct_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(vec_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f128_ppc_val.get_name(), &*CString::new("").unwrap());

    // LLVM Gem: You can't set names on constant values, so this doesn't do anything:
    bool_val.set_name("my_val");
    i8_val.set_name("my_val2");
    i16_val.set_name("my_val3");
    i32_val.set_name("my_val4");
    i64_val.set_name("my_val5");
    i128_val.set_name("my_val6");
    f16_val.set_name("my_val7");
    f32_val.set_name("my_val8");
    f64_val.set_name("my_val9");
    f128_val.set_name("my_val10");
    ptr_val.set_name("my_val11");
    array_val.set_name("my_val12");
    struct_val.set_name("my_val13");
    vec_val.set_name("my_val14");
    f128_ppc_val.set_name("my_val14");

    assert_eq!(bool_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i8_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i16_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i32_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i64_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(i128_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f16_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f32_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f64_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f128_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(ptr_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(array_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(struct_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(vec_val.get_name(), &*CString::new("").unwrap());
    assert_eq!(f128_ppc_val.get_name(), &*CString::new("").unwrap());

    let void_type = context.void_type();
    let ptr_type = bool_type.ptr_type(0);
    let struct_type = context.struct_type(&[&bool_type], false);
    let vec_type = bool_type.vec_type(1);

    let module = context.create_module("types");
    let builder = context.create_builder();

    // You can set names on variables, though:
    let fn_type = void_type.fn_type(&[&bool_type, &f32_type, &struct_type, &array_type, &ptr_type, &vec_type], false);

    let function = module.add_function("do_stuff", &fn_type, None);
    let basic_block = context.append_basic_block(&function, "entry");

    builder.position_at_end(&basic_block);

    let int_param = function.get_nth_param(0).unwrap().into_int_value();
    let float_param = function.get_nth_param(1).unwrap().into_float_value();
    let struct_param = function.get_nth_param(2).unwrap().into_struct_value();
    let array_param = function.get_nth_param(3).unwrap().into_array_value();
    let ptr_param = function.get_nth_param(4).unwrap().into_pointer_value();
    let vec_param = function.get_nth_param(5).unwrap().into_vector_value();

    assert_eq!(int_param.get_name(), &*CString::new("").unwrap());
    assert_eq!(float_param.get_name(), &*CString::new("").unwrap());
    assert_eq!(struct_param.get_name(), &*CString::new("").unwrap());
    assert_eq!(array_param.get_name(), &*CString::new("").unwrap());
    assert_eq!(ptr_param.get_name(), &*CString::new("").unwrap());
    assert_eq!(vec_param.get_name(), &*CString::new("").unwrap());

    int_param.set_name("my_val");
    float_param.set_name("my_val2");
    ptr_param.set_name("my_val3");
    array_param.set_name("my_val4");
    struct_param.set_name("my_val5");
    vec_param.set_name("my_val6");

    assert_eq!(int_param.get_name(), &*CString::new("my_val").unwrap());
    assert_eq!(float_param.get_name(), &*CString::new("my_val2").unwrap());
    assert_eq!(ptr_param.get_name(), &*CString::new("my_val3").unwrap());
    assert_eq!(array_param.get_name(), &*CString::new("my_val4").unwrap());
    assert_eq!(struct_param.get_name(), &*CString::new("my_val5").unwrap());
    assert_eq!(vec_param.get_name(), &*CString::new("my_val6").unwrap());

    // TODO: Test globals, supposedly constant globals work?
}