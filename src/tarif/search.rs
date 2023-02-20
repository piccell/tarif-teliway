
use axum::body::Bytes;
use maud::{Markup, html};

use crate::{page, form::extract_key_values};

use super::Form;
use super::{TarifForm, mk_html};

pub async fn loader(form_data: Bytes) -> Markup {
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