use std::sync::{Arc, Mutex};

use kalosm_language_model::{GenerationParameters, SyncModel, SyncModelExt};
use kalosm_llama::*;

fn main() {
    #[inline(never)]
    fn load_small() {
        let model =
            LlamaModel::from_builder(Llama::builder().with_source(LlamaSource::mistral_7b()))
                .unwrap();
        let prompt = "Hello world";

        for _ in 0..100 {
            let mut session = model.new_session().unwrap();
            model.feed_text(&mut session, prompt, Some(0)).unwrap();
        }
    }

    #[inline(never)]
    fn load_large() {
        let model =
            LlamaModel::from_builder(Llama::builder().with_source(LlamaSource::mistral_7b()))
                .unwrap();
        let prompt = "Hello world".repeat(10);

        for _ in 0..100 {
            let mut session = model.new_session().unwrap();
            model.feed_text(&mut session, &prompt, Some(0)).unwrap();
        }
    }

    #[inline(never)]
    fn generate() {
        let model =
            LlamaModel::from_builder(Llama::builder().with_source(LlamaSource::mistral_7b()))
                .unwrap();
        let prompt = "Hello world";

        for _ in 0..100 {
            let mut session = model.new_session().unwrap();
            model
                .stream_text_with_sampler(
                    &mut session,
                    prompt,
                    Some(100),
                    None,
                    Arc::new(Mutex::new(GenerationParameters::default().sampler())),
                    |_| Ok(kalosm_language_model::ModelFeedback::Continue),
                )
                .unwrap();
        }
    }

    load_small();
    load_large();
    generate();
}
