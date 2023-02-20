use axum::body::Bytes;
use maud::Markup;
use crate::{page, form::extract_key_values, html};

use super::{TarifForm, mk_html, Form};

pub async fn loader<'a>() -> Markup {
    let default_values = vec![("country_from".to_string(), "FR".to_string()),("country_to".to_string(), "FR".to_string())];
    let form = TarifForm::init(default_values);

    let content = mk_html(form);

    page(content)
}

pub async fn action(form_data: Bytes) -> Markup {
    let key_values = extract_key_values(form_data);    
    let mut tarif_form = TarifForm::init(key_values);
    tarif_form.validate();
    
    let content = 
        if tarif_form.errors.is_empty() {
            html! {
                p { "SEARCH" }
            }    
        }
        else {
            mk_html(tarif_form)            
        };

    page(content)
}

