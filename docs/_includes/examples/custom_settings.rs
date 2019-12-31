use schemars::{gen::SchemaSettings, JsonSchema};

#[derive(JsonSchema)]
pub struct MyStruct {
    pub my_int: i32,
    pub my_bool: bool,
    pub my_nullable_enum: Option<MyEnum>,
}

#[derive(JsonSchema)]
pub enum MyEnum {
    StringNewType(String),
    StructVariant { floats: Vec<f32> },
}

fn main() {
    // Pure draft 07 schemas use a `definitions` property instead of `$defs`.
    // We'll also make `Option<T>` schemas have a `nullable` property set.
    let settings = SchemaSettings::draft07_pure().with(|s| {
        s.option_nullable = true;
        s.option_add_null_type = false;
    });
    let gen = settings.into_generator();
    let schema = gen.into_root_schema_for::<MyStruct>();
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
