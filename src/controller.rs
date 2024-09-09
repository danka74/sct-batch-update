use std::borrow::Borrow;

use axum::{extract::State, Json};
use hyper::{header::{ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE}, StatusCode};
use reqwest::{Client, Response};
use fhir_sdk::r4b::{resources::ValueSet, types::Coding};

use crate::{model::{BatchChangeRequest, BatchResponse}, Error};

pub async fn process_batch(
    State(client): State<Client>,
    Json(input): Json<BatchChangeRequest>,
) -> (StatusCode, Json<BatchResponse>) {
    

    let url = format!(
        "http://localhost:8080/fhir/ValueSet/$expand?url=\
        http://snomed.info/sct/45991000052106?fhir_vs=ecl/\
        {}&includeDesignations=true&count=1&filter={}",
        input.settings.ecl,
        input.settings.term
    );

    println!("{}", url);

    match client.get(url).header(ACCEPT_LANGUAGE, "sv").send().await {
        Ok(resp) => {
            match resp.json::<ValueSet>().await {
                Ok(vs) => {
                    if let Some(expansion) = vs.expansion.as_ref() {
                        for contains in expansion.contains.iter() {
                            if let Some(concept) = contains.as_ref() {
                                let code = concept.code.as_ref().unwrap();

                                let designations = concept.designation
                                    .iter()
                                    .filter(|&d| {
                                        if let Some(designation) = d.as_ref() {
                                            designation.language == Some("sv".to_string()) &&
                                                designation.r#use.as_ref().unwrap().code.as_ref() == Some(&"900000000000013009".to_string())
                                        } else {
                                            false
                                        }
                                    
                                    });
                                for designation in designations {
                                    // println!("{designation:#?}");
                                    println!("{}: {}", code, designation.as_ref().unwrap().value);
                                }
                            }
                        }
                    }
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