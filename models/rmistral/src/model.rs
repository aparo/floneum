use crate::raw::{MistralCache, Model};
use anyhow::{Error as E, Result};
use llm_samplers::{
    prelude::Logits,
    types::{HasSamplerResources, SamplerError},
};
use std::fmt::{Debug, Formatter};
use std::{collections::HashMap, sync::Arc};

use candle_core::{DType, Device, Tensor};
use kalosm_language_model::{SyncModel, SyncModelExt};
use tokenizers::Tokenizer;

use crate::InferenceSettings;

/// A Mistral-1.5 session.
pub struct MistralSession {
    cache: MistralCache,
    current_tokens: Vec<u32>,
}

impl MistralSession {
    /// Export the current cache tensor map.
    pub fn get_tensor_map(&self) -> HashMap<String, Tensor> {
        self.cache.get_tensor_map()
    }

    /// Import a cache tensor map.
    pub fn set_tensor_map(&mut self, map: HashMap<String, Tensor>) {
        self.cache = MistralCache::from_tensor_map(map);
    }

    /// Create a cache from a tensor map. This can be used to load a cache from disk.
    pub fn from_tensor_map(map: HashMap<String, Tensor>, current_tokens: Vec<u32>) -> Self {
        Self {
            cache: MistralCache::from_tensor_map(map),
            current_tokens,
        }
    }

    /// Get the current tokens.
    pub fn get_current_tokens(&self) -> &[u32] {
        &self.current_tokens
    }
}

/// The inner, synchronous Mistral model.
pub struct MistralModel {
    model: Model,
    device: Device,
    tokenizer: Tokenizer,
    cache: MistralCache,
}

impl SyncModel for MistralModel {
    type Session = MistralSession;

    fn new_session(&self) -> anyhow::Result<Self::Session> {
        let mut cache = self.cache.clone();
        cache.clear();
        Ok(Self::Session {
            cache,
            current_tokens: Vec::new(),
        })
    }

    fn feed_text(&mut self, session: &mut Self::Session, prompt: &str) -> anyhow::Result<Logits> {
        let encoded = self.tokenizer.encode(prompt, true).map_err(E::msg)?;
        let tokens = encoded.get_ids();
        self.feed_tokens(session, tokens)
    }

    fn feed_tokens(
        &mut self,
        session: &mut Self::Session,
        tokens: &[u32],
    ) -> anyhow::Result<Logits> {
        session.current_tokens.extend(tokens);

        let token_count = tokens.len();
        Self::forward(
            &mut self.model,
            &self.device,
            tokens,
            session.current_tokens.len() - token_count,
            Some(&mut session.cache),
            None,
        )
    }

    fn stop_token(&self) -> anyhow::Result<u32> {
        let eos_token = match self.tokenizer.get_vocab(true).get("</s>") {
            Some(token) => *token,
            None => anyhow::bail!("cannot find the </s> token"),
        };
        Ok(eos_token)
    }

    fn tokenizer(&self) -> std::sync::Arc<dyn kalosm_sample::Tokenizer + Send + Sync> {
        Arc::new(self.tokenizer.clone())
            as std::sync::Arc<dyn kalosm_sample::Tokenizer + Send + Sync>
    }
}

impl MistralModel {
    fn forward(
        model: &mut Model,
        device: &Device,
        tokens: &[u32],
        seqlen_offset: usize,
        cache: Option<&mut MistralCache>,
        top_k: Option<usize>,
    ) -> anyhow::Result<Logits> {
        if tokens.is_empty() {
            return Err(anyhow::anyhow!("Cannot run model on empty input"));
        }

        let input = Tensor::new(tokens, device)?.unsqueeze(0)?;
        let logits = model.forward(&input, seqlen_offset, cache)?;
        let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;
        let logits: Vec<f32> = logits.to_vec1()?;
        match top_k {
            Some(top_k) => Ok(Logits::try_from_iter_top_k(logits, top_k)?),
            None => Ok(Logits::try_from_iter(logits)?),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        model: Model,
        tokenizer: Tokenizer,
        device: Device,
        cache: MistralCache,
    ) -> Self {
        Self {
            cache,
            model,
            device,
            tokenizer,
        }
    }

    pub(crate) fn _infer(
        &mut self,
        settings: InferenceSettings,
        sampler: std::sync::Arc<std::sync::Mutex<dyn llm_samplers::prelude::Sampler>>,
        out: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> Result<()> {
        let InferenceSettings {
            prompt,
            sample_len,
            stop_on,
        } = settings;

        let mut session = self.new_session()?;

        self.stream_text_with_sampler(
            &mut session,
            prompt.as_str(),
            Some(sample_len as u32),
            stop_on.as_deref(),
            sampler,
            |token| {
                out.send(token)
                    .map_err(|_| anyhow::anyhow!("Failed to send token to output channel"))
                    .map(|_| kalosm_language_model::ModelFeedback::Continue)
            },
        )?;

        Ok(())
    }
}

struct SamplerResources<'a, 'b, R: rand::Rng> {
    rng: &'a mut R,
    previous_tokens: &'b [u32],
}

impl<R> Debug for SamplerResources<'_, '_, R>
where
    R: rand::Rng,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SamplerResources")
            .field("previous_tokens", &self.previous_tokens)
            .finish()
    }
}

impl<R> HasSamplerResources for SamplerResources<'_, '_, R>
where
    R: rand::Rng,
{
    fn with_rng_mut(
        &mut self,
        fun: &mut dyn FnMut(&mut dyn rand::RngCore),
    ) -> Result<(), SamplerError> {
        fun(self.rng);
        Ok(())
    }

    fn with_last_tokens(&self, fun: &mut dyn FnMut(&[u32])) -> Result<(), SamplerError> {
        fun(self.previous_tokens);
        Ok(())
    }
}
