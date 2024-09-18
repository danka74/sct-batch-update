use std::borrow::Borrow;

use axum::{extract::State, Json};
use hyper::{header::{ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE}, StatusCode};
use reqwest::{Client, Response};
use fhir_sdk::r4b::{resources::ValueSet, types::{Coding, ExtensionValue}};

use crate::{model::{BatchChangeRequest, BatchResponse}, Error};

pub async fn process_batch(
    State(client): State<Client>,
    Json(input): Json<BatchChangeRequest>,
) -> (StatusCode, Json<BatchResponse>) {
    

    let url = format!(
        "http://localhost:8080/fhir/ValueSet/$expand?url=\
        http://snomed.info/sct/45991000052106?fhir_vs=ecl/\
        {}&includeDesignations=true&includeDescriptionId=true\
        &count=1&filter={}",
        input.settings.ecl,
        input.settings.term
    );

    println!("{}", url);

    match client.get(url).header(ACCEPT_LANGUAGE, "sv").send().await {
        Ok(resp) => {
            match resp.json::<ValueSet>().await {
                Ok(vs) => {
                    process_valueset(vs);
                }
                Err(err) => {
                    println!("{err:#?}");
                }
            }

            
            
        }
        Err(err) => {
            println!("{err:#?}");
            // crate::Error::Reqwest(err);
        }
    };



    let res = BatchResponse {
        status: input.settings.name,
    };
    (StatusCode::OK, Json(res))
}

fn process_valueset(vs: ValueSet) -> Option<()>{
    if let Some(expansion) = vs.expansion.as_ref() {
        for contains in expansion.contains.iter() {
            if let Some(concept) = contains.as_ref() {
                let code = concept.code.as_ref()?;

                let designations = concept.designation
                    .iter()
                    .filter(|&d| {
                        if let Some(designation) = d.as_ref() {
                            if let Some(r#use) = designation.r#use.as_ref() {
                                designation.language == Some("sv".to_string()) &&
                                    r#use.code.as_ref() == Some(&"900000000000013009".to_string())
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    
                    });
                for designation in designations {
                    // println!("{designation:#?}");
                    let value = designation.as_ref()?.extension.iter().find(|&e| {
                        e.url.eq("http://hl7.org/fhir/StructureDefinition/coding-sctdescid") 
                    })?.value.as_ref()?;
                    if let ExtensionValue::String(desc_id) = &value {
                        println!("{}: {} ({})", code, designation.as_ref()?.value, desc_id);
                    }                   
                }
            }
        }
    }
    Some(())
}