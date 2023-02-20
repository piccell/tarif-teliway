use maud::{Markup, html};
use crate::{form::{InputField, FieldType}};

pub fn input_field(field: &InputField , error: Option<String>) -> Markup {
        let star = if field.is_required {" *" } else { "" };
        let label = format!("{}{star}", field.label);
        let value = field.value.clone().unwrap_or_default();

        let error_tag = match error {
            Some(_) => Some("true"),
            _ => None
        };

        let the_type = match field.the_type {
            FieldType::Float | FieldType::Integer => "number",
            FieldType::Text => "text"
        };

        html! {
            label for=(field.code) { (label) 
                input type=(the_type)
                    id=(field.code) 
                    name=(field.code) 
                    value=(value)
                    aria-invalid=[error_tag];
            }
        }
}