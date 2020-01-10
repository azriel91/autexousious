use std::convert::TryFrom;

use variant_struct::VariantStruct;

#[derive(Debug, VariantStruct, PartialEq)]
pub enum MyEnum {
    /// Unit variant.
    #[variant_struct_attrs(derive(Clone, Copy, Debug, PartialEq))]
    Unit,
    /// Tuple variant.
    #[variant_struct_attrs(derive(Clone, Copy, Debug, PartialEq))]
    Tuple(u32, u64),
    /// Struct variant.
    #[variant_struct_attrs(derive(Clone, Copy, Debug, PartialEq))]
    Struct { field_0: u32, field_1: u64 },
}

#[test]
fn enum_from_unit_struct() {
    assert_eq!(MyEnum::Unit, MyEnum::from(Unit));
}

#[test]
fn unit_struct_try_from_enum_ok() {
    assert_eq!(Ok(Unit), Unit::try_from(MyEnum::Unit));
}

#[test]
fn unit_struct_try_from_enum_err() {
    assert_eq!(
        Err(MyEnum::Tuple(1, 2)),
        Unit::try_from(MyEnum::Tuple(1, 2))
    );
}

#[test]
fn enum_from_tuple_struct() {
    assert_eq!(MyEnum::Tuple(1, 2), MyEnum::from(Tuple(1, 2)));
}

#[test]
fn tuple_struct_try_from_enum_ok() {
    assert_eq!(Ok(Tuple(1, 2)), Tuple::try_from(MyEnum::Tuple(1, 2)));
}

#[test]
fn tuple_struct_try_from_enum_err() {
    assert_eq!(Err(MyEnum::Unit), Tuple::try_from(MyEnum::Unit));
}

#[test]
fn enum_from_named_struct() {
    assert_eq!(
        MyEnum::Struct {
            field_0: 1,
            field_1: 2
        },
        MyEnum::from(Struct {
            field_0: 1,
            field_1: 2
        })
    );
}

#[test]
fn named_struct_try_from_enum_ok() {
    assert_eq!(
        Ok(Struct {
            field_0: 1,
            field_1: 2
        }),
        Struct::try_from(MyEnum::Struct {
            field_0: 1,
            field_1: 2
        })
    );
}

#[test]
fn named_struct_try_from_enum_err() {
    assert_eq!(Err(MyEnum::Unit), Struct::try_from(MyEnum::Unit));
}
