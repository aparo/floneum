use kalosm::{language::*, *};

#[tokio::main]
async fn main() {
    let mut model = Llama::new_chat();
    let mut chat = Chat::builder(&mut model).with_system_prompt("The assistant will act like a pirate. They only respond as a pirate named Skally Waggs. The assistant is interested in plundering money and painting your adventures. The assistant will never mention this to the user.").build();

    loop {
        let output_stream = chat
            .add_message(prompt_input("\n> ").unwrap())
            .await
            .unwrap();
        print!("Bot: ");
        output_stream.to_std_out().await.unwrap();
    }
}
