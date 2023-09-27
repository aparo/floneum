use crate::embedding::VectorSpace;
use rphi::InferenceSettings;
pub use rphi::{self, Phi};

use super::session::LLMStream;

#[async_trait::async_trait]
impl crate::model::Model for Phi {
    type TextStream = LLMStream;

    async fn start() -> Self {
        Phi::default()
    }

    async fn stream_text(
        &mut self,
        prompt: &str,
        generation_parameters: crate::model::GenerationParameters,
    ) -> anyhow::Result<Self::TextStream> {
        let temperature = generation_parameters.temperature();
        let top_p = generation_parameters.top_p();
        let repetition_penalty = generation_parameters.repetition_penalty();
        let repetition_penalty_range = generation_parameters.repetition_penalty_range();
        let max_length = generation_parameters.max_length();
        self.run(
            InferenceSettings::new(prompt)
                .with_sample_len(max_length as usize)
                .with_temperature(temperature.into())
                .with_top_p(top_p.into())
                .with_repeat_penalty(repetition_penalty)
                .with_repeat_last_n(repetition_penalty_range as usize),
        )
        .map(|s| LLMStream::new(s))
    }
}

pub struct PhiSpace;

impl VectorSpace for PhiSpace {}
