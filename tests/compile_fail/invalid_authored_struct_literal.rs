use surgeist_style::{
    AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue, Color, Property,
    Value,
};

fn main() {
    let _declaration = AuthoredDeclaration {
        property: AuthoredProperty::Property(Property::Color),
        value: AuthoredValue::Value(Value::Color(Color::BLACK)),
    };
    let _declarations = AuthoredDeclarations { values: Vec::new() };
}
