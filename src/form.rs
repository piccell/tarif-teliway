use axum::body::Bytes;

#[derive(Debug, Clone)]
pub enum FieldType {
    Integer,
    Float,
    Text
}

#[derive(Debug, Clone)]
pub struct InputField {
    pub code: String,
    pub label: String,
    pub the_type: FieldType,
    pub value: Option<String>,
    pub is_required: bool,   
    pub error: Option<String>, 
}

impl InputField {
    pub fn new(
        the_type: FieldType,
        code: &str,
        label: &str,
        value: Option<String>,
        is_required: bool,
        error: Option<String>,
    ) -> Self {
        InputField { 
            code: code.to_owned(),
            label: label.to_owned(),
            the_type,
            value,
            is_required,
            error,
        }
    }

    fn check_required(&self) -> bool {
        if let Some(value) = self.value.clone() {
            !value.is_empty()
        }        
        else {
            false
        }
    }
    
    fn check_integer(&self) -> bool {
        if let Some(value) = self.value.clone() {
            value.parse::<u16>().is_ok()
        }
        else {
            false
        }
    }
    
    fn check_float(&self) -> bool {
        if let Some(value) = self.value.clone() {
            value.parse::<f32>().is_ok()
        }
        else {
            false
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut valid = true;

        if self.is_required {
            valid &= self.check_required();
        }

        if self.value.is_some() {
            let check_type = match self.the_type {
                FieldType::Integer => self.check_integer(),
                FieldType::Float => self.check_float(),
                _ => true
            };

            valid &= check_type;            
        }

        valid
    }
}

pub fn extract_key_values(value: Bytes) -> Vec<(String, String)> {
    let args = value.slice(..);
    let body_str = String::from_utf8(args.to_vec()).unwrap(); 
       
    body_str
        .split('&')
        .collect::<Vec<_>>()
        .iter()
        .map(|x| {
            let parts = x.split('=').collect::<Vec<_>>();
            (parts[0].to_owned(), parts[1].to_owned())
        }) 
        .filter(|x| !x.1.is_empty())
        .collect::<Vec<_>>()   
}

#[cfg(test)]
mod tests {
    use super::{InputField, FieldType};

    #[test]
    fn test_is_required() {
        let field = InputField::new(FieldType::Integer, "", "", Some("5".to_string()), true, None);
        
        assert!(field.is_valid())
    }

    #[test]
    fn test_is_not_required() {
        let field = InputField::new(FieldType::Integer, "", "", None, false, None);
        
        assert!(field.is_valid())
    }    
}
