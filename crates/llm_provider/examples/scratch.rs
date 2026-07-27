use domain::llm_provider::Provider as _;

#[tokio::main]
async fn main() {
    let provider = llm_provider::provider::Provider::new(
        "http://127.0.0.1:1234".into(),
        "google/gemma-4-e4b".into(),
    );
    // Provider и mod должны быть pub

    let token = tokio_util::sync::CancellationToken::new();
    match provider
        .completion(&[], "напиши эссе на 100 слов", token)
        .await
    {
        Ok(text) => println!("OK: {text}"),
        Err(e) => eprintln!("ERR: {e}"),
    }
}
