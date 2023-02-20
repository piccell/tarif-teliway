use std::collections::HashMap;

use maud::{Markup, html};

use crate::{form::{InputField, FieldType}, html::inputs::input_field};
pub mod search;
pub mod form;
/*  name: uuid
*         description: votre clé d'identification
*         required: true
*         type: String
*       - name: weight
*         description: poids
*         required: true
*         type: Float
*       - name: nbUM
*         description: nombre d'unité de manutention
*         required: true
*         type: Number
*       - name: nbPalette
*         description: nombre de palette
*         required: false
*         type: Number
*       - name: nbTaxableUnit
*         description: nombre d'unité taxable
*         required: false
*         type: Number
*       - name: volume
*         description: Volume (m3)
*         required: false
*         type: Float 
*       - name: shippingDate
*         description: Date d'expédition (aaaa-mm-jj)
*         required: false
*         type: String
*       - name: portType
*         description: Type de port (P=Payé, C=Dû)
*         required: false
*         type: String
*       - name: declaredValue
*         description: Valeur déclarée (en euros)
*         required: false
*         type: Float
*       - name: cashOnDeliveryAmount
*         description: Montant contre remboursement
*         required: false
*         type: Float
*       - name: recipientType
*         description: Type destinataire (P=Particulier)
*         required: false
*         type: String
*       - name: productCode
*         description: Code produit
*         required: false
*         type: String
*       - name: townFrom
*         description: Ville d'expédition
*         required: false
*         type: String
*       - name: zipcodeFrom
*         description: Code postal d'expédition
*         required: true
*         type: String
*       - name: countryFrom
*         description: Pays d'expédition (code ISO 2 caractères) (par défault FR)
*         required: true
*         type: String
*         default: "FR"
*       - name: townTo
*         description: Ville de destination
*         required: false
*         type: String
*       - name: zipcodeTo
*         description: Code postal de destination
*         required: true
*         type: String
*       - name: countryTo
*         description: Pays de destination (code ISO 2 caractères) (par défault FR)
*         required: true
*         type: String
*         default: "FR" 
 */

 
pub trait Form {
    fn add(&mut self, field: InputField);
    fn get(&self, field_code: &str) -> Option<&InputField>;
    fn validate(&mut self);
    fn get_error(&self, field_code: &str) -> Option<String>;
    fn get_value_for_key(&self, key:&str, key_values: &[(String, String)]) -> Option<String> {
        key_values
            .iter()
            .find(|x| x.0 == key)
            .map(|x| x.1.clone())
    }
}

#[derive(Debug, Default, Clone)]
pub struct TarifForm {
    pub fields: HashMap<String, InputField>,
    pub errors: HashMap<String, String>
}

impl TarifForm {
    pub fn init(default_values: Vec<(String, String)>) -> Self {
        let mut form = TarifForm { 
            fields: HashMap::new(), 
            ..Default::default()
        };

        let value = form.get_value_for_key("weight", &default_values);        
        form.add(InputField::new(FieldType::Float, "weight", "Poids", value, true, None));

        let value = form.get_value_for_key("nb_palette", &default_values);        
        form.add(InputField::new(FieldType::Integer, "nb_palette", "Nb Palette", value, false, None));

        let value = form.get_value_for_key("zipcode_from", &default_values);        
        form.add(InputField::new(FieldType::Text, "zipcode_from", "Code Postal Exp.", value, true, None));

        let value = form.get_value_for_key("town_from", &default_values);        
        form.add(InputField::new(FieldType::Text, "town_from", "Ville Exp.", value, true, None));

        let value = form.get_value_for_key("country_from", &default_values);        
        form.add(InputField::new(FieldType::Text, "country_from", "Pays Exp.", value, true, None));

        let value = form.get_value_for_key("zipcode_to", &default_values);        
        form.add(InputField::new(FieldType::Text, "zipcode_to", "Code Postal Dest.", value, true, None));

        let value = form.get_value_for_key("town_to", &default_values);        
        form.add(InputField::new(FieldType::Text, "town_to", "Ville Dest.", value, true, None));

        let value = form.get_value_for_key("country_to", &default_values);        
        form.add(InputField::new(FieldType::Text, "country_to", "Pays Dest.", value, true, None));

        form
    }
}

impl Form for TarifForm {
    fn add(&mut self, field: InputField) {
        self.fields.insert(field.code.clone(), field);
    }

    fn get(&self, field_code: &str) -> Option<&InputField> {
        self.fields.get(field_code)
    }
    
    fn validate(&mut self) {
        let mut errors = HashMap::new();

        for key in self.fields.keys() {
            if let Some(field) = self.get(key) {
                if !field.is_valid() {
                    errors.insert(key.to_string(), "Error".to_string());
                }
            };            
        };

        self.errors = errors
    }

    fn get_error(&self, field_code: &str) -> Option<String> {
        self.errors.get(field_code).cloned()
    }
}

pub fn mk_html(form:TarifForm) -> Markup {
    html! {
        form method="post" { 
            div class="grid" {
                (input_field(form.get("weight").unwrap(), form.get_error("weight")))
                (input_field(form.get("nb_palette").unwrap(), form.get_error("nb_palette")))
                // (input_field(form.get("number", "Nb Palette", "nb_palette", "", false))
                // (input_field(form.get("number", "Nb Unité Taxable", "nb_taxable_unit", "", false))
            }
            div class="grid" {
                (input_field(form.get("zipcode_from").unwrap(), form.get_error("zipcode_from")))
                (input_field(form.get("town_from").unwrap(), form.get_error("town_from")))
                (input_field(form.get("country_from").unwrap(), form.get_error("country_from")))
            }
            div class="grid" {
                (input_field(form.get("zipcode_to").unwrap(), form.get_error("zipcode_to")))
                (input_field(form.get("town_to").unwrap(), form.get_error("town_to")))
                (input_field(form.get("country_to").unwrap(), form.get_error("country_to")))
            }        
            input type="submit" value="Calculer";
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::form::{InputField, FieldType};

    use super::{TarifForm, Form};

    #[test]
    fn test_required_field() {
        let mut form = TarifForm { 
            fields: HashMap::new(), 
            ..Default::default()
        };
        let default_values = vec![("weight".to_string(), "5".to_string())];
        let value = form.get_value_for_key("weight", &default_values);        
        form.add(InputField::new(FieldType::Integer, "weight", "Poids", value, true, None));

        form.validate();

        assert!(form.errors.is_empty())
    }

    #[test]
    fn test_optional_field() {
        let mut form = TarifForm { 
            fields: HashMap::new(), 
            ..Default::default()
        };
   
        form.add(InputField::new(FieldType::Float, "weight", "Poids", None, false, None));

        form.validate();

        assert!(form.errors.is_empty())
    }    

    #[test]
    fn test_required_field_missing() {
        let mut form = TarifForm { 
            fields: HashMap::new(), 
            ..Default::default()
        };

        form.add(InputField::new(FieldType::Float, "weight", "Poids", None, true, None));

        form.validate();

        assert!(!form.errors.is_empty())
    }     
}