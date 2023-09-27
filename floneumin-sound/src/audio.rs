use futures_util::Stream;
use std::{
    io::Cursor,
    pin::Pin,
    sync::{Arc, RwLock},
    task::{Context, Poll},
    time::Duration,
};

use cpal::{FromSample, Sample};
use ringbuffer::{GrowableAllocRingBuffer, RingBuffer};

trait AudioData {
    fn read(&mut self) -> anyhow::Result<u8>;
}

#[derive(Clone)]
pub struct AudioSpec {
    sample_rate: u32,
    bits_per_sample: u16,
    float: bool,
    channels: u16,
}

impl From<&cpal::SupportedStreamConfig> for AudioSpec {
    fn from(config: &cpal::SupportedStreamConfig) -> Self {
        Self {
            sample_rate: config.sample_rate().0,
            bits_per_sample: config.sample_format().sample_size() as u16 * 8,
            float: config.sample_format().is_float(),
            channels: config.channels() as u16,
        }
    }
}

impl AudioSpec {
    pub fn sample_size_bytes(&self) -> u16 {
        self.bits_per_sample / 8
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn float(&self) -> bool {
        self.float
    }

    pub fn channels(&self) -> u16 {
        self.channels
    }
}

struct StreamSubscriber {
    time_since_last_sample: u64,
    sample_duration: u64,
    senders: tokio::sync::mpsc::UnboundedSender<rodio::buffer::SamplesBuffer<f32>>,
}

/// A single channel stream of audio data.
#[derive(Clone)]
pub struct AudioStream<S: Sample> {
    buffer: Arc<RwLock<GrowableAllocRingBuffer<S>>>,
    subscribers: Arc<RwLock<Vec<StreamSubscriber>>>,
    spec: AudioSpec,
}

impl<S: Sample> AudioStream<S>
where
    <S as cpal::Sample>::Float: Into<f32>,
{
    pub fn new(seconds: f32, spec: impl Into<AudioSpec>) -> Self {
        let spec = spec.into();
        Self {
            buffer: Arc::new(RwLock::new(GrowableAllocRingBuffer::with_capacity(
                seconds as usize * spec.sample_size_bytes() as usize * spec.sample_rate() as usize,
            ))),
            subscribers: Arc::new(RwLock::new(Vec::new())),
            spec,
        }
    }

    pub fn spec(&self) -> &AudioSpec {
        &self.spec
    }

    fn send_sample(&self, buffer: &GrowableAllocRingBuffer<S>, subscriber: &mut StreamSubscriber) {
        let sample_size_bytes = subscriber.sample_duration as usize;
        let samples: Vec<_> = buffer
            .iter()
            .rev()
            .take(sample_size_bytes)
            .map(|s| s.to_float_sample().into())
            .collect();
        let buffer = rodio::buffer::SamplesBuffer::new(
            self.spec.channels as u16,
            self.spec.sample_rate,
            samples,
        );

        subscriber.senders.send(buffer).unwrap();

        subscriber.time_since_last_sample = 0;
    }

    pub fn subscribe_stream(&self, chunk_duration: Duration) -> AudioChunkStream {
        let chunk_duration_seconds = chunk_duration.as_secs_f32();
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let samples_per_duration = (chunk_duration_seconds * self.spec.sample_rate as f32) as u64;
        let mut subscriber = StreamSubscriber {
            time_since_last_sample: 0,
            sample_duration: samples_per_duration,
            senders: sender,
        };
        self.send_sample(&*self.buffer.read().unwrap(), &mut subscriber);
        self.subscribers.write().unwrap().push(subscriber);

        AudioChunkStream { receiver }
    }

    pub(crate) fn write<U: cpal::Sample>(&self, data: &[U])
    where
        S: FromSample<U>,
    {
        let mut buffer = self.buffer.write().unwrap();
        let mut subscribers = self.subscribers.write().unwrap();
        for sample in data {
            for subscriber in subscribers.iter_mut() {
                subscriber.time_since_last_sample += 1;
                if subscriber.time_since_last_sample >= subscriber.sample_duration {
                    self.send_sample(&*buffer, subscriber);
                }
            }
            buffer.push(sample.to_sample());
        }
    }

    pub fn reader(&self) -> anyhow::Result<rodio::buffer::SamplesBuffer<f32>> {
        let samples: Vec<_> = self
            .buffer
            .read()
            .unwrap()
            .iter()
            .map(|s| s.to_float_sample().into())
            .collect();
        Ok(rodio::buffer::SamplesBuffer::new(
            self.spec.channels as u16,
            self.spec.sample_rate,
            samples,
        ))
    }
}

pub struct AudioChunkStream {
    receiver: tokio::sync::mpsc::UnboundedReceiver<rodio::buffer::SamplesBuffer<f32>>,
}

impl Stream for AudioChunkStream {
    type Item = rodio::buffer::SamplesBuffer<f32>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

pub struct AudioBuffer {
    data: Vec<u8>,
}

impl From<Vec<u8>> for AudioBuffer {
    fn from(data: Vec<u8>) -> Self {
        Self::new(data)
    }
}

impl AudioBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self, anyhow::Error> {
        Ok(Self::new(std::fs::read(path)?))
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn into_data(self) -> Vec<u8> {
        self.data
    }

    pub fn into_reader(self) -> Result<hound::WavReader<Cursor<Vec<u8>>>, anyhow::Error> {
        Ok(hound::WavReader::new(Cursor::new(self.data))?)
    }
}
