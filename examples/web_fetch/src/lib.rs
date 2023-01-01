use web::*;

#[web::main]
pub async fn main() {
    let body = query_selector("body");

    let result = fetch(FetchOptions {
        url: "https://pokeapi.co/api/v2/pokemon/1/",
        response_type: FetchResponseType::ArrayBuffer,
        ..Default::default()
    })
    .await;
    if let FetchResponse::ArrayBuffer(_, ab) = result {
        element_set_inner_html(&body, &format!("Got response: {:?}", ab));
    }
}
