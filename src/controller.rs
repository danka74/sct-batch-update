use std::borrow::Borrow;

use axum::{extract::State, Json};
use hyper::{header::{ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE}, StatusCode};
use reqwest::Client;
use fhir_sdk::r5::resources::ValueSet;

use crate::{model::{BatchChangeRequest, BatchResponse}, Error};

pub async fn process_batch(
    State(client): State<Client>,
    Json(payload): Json<BatchChangeRequest>,
) -> (StatusCode, Json<BatchResponse>) {
    

    let url = format!(
        "http://localhost:8080/fhir/ValueSet/$expand?url=\
        http://snomed.info/sct/45991000052106?fhir_vs=ecl/\
        {}&includeDesignations=true&count=1&filter={}",
        payload.settings.ecl,
        payload.settings.term
    );

    println!("{}", url);

    match client.get(url)
        .header(ACCEPT_LANGUAGE, "sv")
        .send().await {
        Ok(resp) => {
            let vs = resp.json::<ValueSet>().await.unwrap();

            let exp = &vs.expansion.as_ref().unwrap();

            for concept in exp.contains.iter() {
                println!("{concept:#?}");

                
            }
            
        }
        Err(err) => {
            println!("{err:#?}");
            // crate::Error::Reqwest(err);
        }
    };



    let res = BatchResponse {
        status: payload.settings.name,
    };
    (StatusCode::OK, Json(res))
}