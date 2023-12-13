use kalosm_language::*;

#[tokio::main]
async fn main() {
    let llm = Phi::start().await;
    let prompt = "Realistic mock user names for a chat application: ";

    let validator = <[Word<1, 10>; 10] as HasParser>::new_parser();
    let words = llm.stream_structured_text(prompt, validator).await.unwrap();

    println!("\n{:#?}", words.result().await);
}
