use fluxor::prelude::*;

pub fn hello_world(_req: Req, _params: Params) -> Reply {
    boxed(async move {
       let json_response = do_json!(
        r#"
            {"message": "{{waving_hand_emoji}} Hello, World!"}
        "#, 
        waving_hand_emoji = "👋"
    );
        
        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}