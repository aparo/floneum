mod host;
mod plugin;
pub use plugin::*;
mod embedding_db;
mod llm;
mod node;
mod page;
mod proxies;

pub struct Both {
    interface0: exports::plugins::main::definitions::Definitions,
}
const _: () = {
    use wasmtime::component::__internal::anyhow;
    impl Both {
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wasmtime::Result<()>
        where
            U: plugins::main::types::Host + plugins::main::imports::Host + Send,
            T: Send,
        {
            plugins::main::types::add_to_linker(linker, get)?;
            plugins::main::imports::add_to_linker(linker, get)?;
            Ok(())
        }
        #[doc = " Instantiates the provided `module` using the specified"]
        #[doc = " parameters, wrapping up the result in a structure that"]
        #[doc = " translates between wasm and the host."]
        pub async fn instantiate_async<T: Send>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = linker.instantiate_async(&mut store, component).await?;
            Ok((Self::new(store, &instance)?, instance))
        }
        #[doc = " Instantiates a pre-instantiated module using the specified"]
        #[doc = " parameters, wrapping up the result in a structure that"]
        #[doc = " translates between wasm and the host."]
        pub async fn instantiate_pre<T: Send>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = instance_pre.instantiate_async(&mut store).await?;
            Ok((Self::new(store, &instance)?, instance))
        }
        #[doc = " Low-level creation wrapper for wrapping up the exports"]
        #[doc = " of the `instance` provided in this structure of wasm"]
        #[doc = " exports."]
        #[doc = ""]
        #[doc = " This function will extract exports from the `instance`"]
        #[doc = " defined within `store` and wrap them all up in the"]
        #[doc = " returned structure which can be used to interact with"]
        #[doc = " the wasm module."]
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();
            let interface0 = exports::plugins::main::definitions::Definitions::new(
                &mut __exports
                    .instance("plugins:main/definitions")
                    .ok_or_else(|| {
                        anyhow::anyhow!("exported instance `plugins:main/definitions` not present")
                    })?,
            )?;
            Ok(Both { interface0 })
        }
        pub fn plugins_main_definitions(
            &self,
        ) -> &exports::plugins::main::definitions::Definitions {
            &self.interface0
        }
    }
};
pub mod plugins {
    pub mod main {
        #[allow(clippy::all)]
        pub mod types {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(enum)]
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub enum BrowserMode {
                #[component(name = "headless")]
                Headless,
                #[component(name = "headfull")]
                Headfull,
            }
            impl core::fmt::Debug for BrowserMode {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        BrowserMode::Headless => f.debug_tuple("BrowserMode::Headless").finish(),
                        BrowserMode::Headfull => f.debug_tuple("BrowserMode::Headfull").finish(),
                    }
                }
            }
            const _: () = {
                assert!(1 =  =  <BrowserMode as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 =  =  <BrowserMode as wasmtime::component::ComponentType>::ALIGN32);
            };
            pub enum Page {}

            #[wasmtime::component::__internal::async_trait]
            pub trait HostPage {
                async fn new(
                    &mut self,
                    mode: BrowserMode,
                    url: String,
                ) -> wasmtime::Result<wasmtime::component::Resource<Page>>;

                async fn find_in_current_page(
                    &mut self,
                    self_: wasmtime::component::Resource<Page>,
                    selector: String,
                ) -> wasmtime::Result<wasmtime::component::Resource<Node>>;

                async fn screenshot_browser(
                    &mut self,
                    self_: wasmtime::component::Resource<Page>,
                ) -> wasmtime::Result<Vec<u8>>;

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Page>,
                ) -> wasmtime::Result<()>;
            }
            pub enum Node {}

            #[wasmtime::component::__internal::async_trait]
            pub trait HostNode {
                async fn get_element_text(
                    &mut self,
                    self_: wasmtime::component::Resource<Node>,
                ) -> wasmtime::Result<String>;

                async fn click_element(
                    &mut self,
                    self_: wasmtime::component::Resource<Node>,
                ) -> wasmtime::Result<()>;

                async fn type_into_element(
                    &mut self,
                    self_: wasmtime::component::Resource<Node>,
                    keys: String,
                ) -> wasmtime::Result<()>;

                async fn get_element_outer_html(
                    &mut self,
                    self_: wasmtime::component::Resource<Node>,
                ) -> wasmtime::Result<String>;

                async fn screenshot_element(
                    &mut self,
                    self_: wasmtime::component::Resource<Node>,
                ) -> wasmtime::Result<Vec<u8>>;

                async fn find_child_of_element(
                    &mut self,
                    self_: wasmtime::component::Resource<Node>,
                    selector: String,
                ) -> wasmtime::Result<wasmtime::component::Resource<Node>>;

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Node>,
                ) -> wasmtime::Result<()>;
            }
            pub enum EmbeddingDb {}

            #[wasmtime::component::__internal::async_trait]
            pub trait HostEmbeddingDb {
                async fn new(
                    &mut self,
                    embeddings: Vec<Embedding>,
                    documents: Vec<String>,
                ) -> wasmtime::Result<wasmtime::component::Resource<EmbeddingDb>>;

                async fn add_embedding(
                    &mut self,
                    self_: wasmtime::component::Resource<EmbeddingDb>,
                    embedding: Embedding,
                    documents: String,
                ) -> wasmtime::Result<()>;

                async fn find_closest_documents(
                    &mut self,
                    self_: wasmtime::component::Resource<EmbeddingDb>,
                    search: Embedding,
                    count: u32,
                ) -> wasmtime::Result<Vec<String>>;

                async fn find_documents_within(
                    &mut self,
                    self_: wasmtime::component::Resource<EmbeddingDb>,
                    search: Embedding,
                    within: f32,
                ) -> wasmtime::Result<Vec<String>>;

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<EmbeddingDb>,
                ) -> wasmtime::Result<()>;
            }
            pub enum Model {}

            #[wasmtime::component::__internal::async_trait]
            pub trait HostModel {
                async fn new(
                    &mut self,
                    ty: ModelType,
                ) -> wasmtime::Result<wasmtime::component::Resource<Model>>;

                async fn model_downloaded(&mut self, ty: ModelType) -> wasmtime::Result<bool>;

                async fn infer(
                    &mut self,
                    self_: wasmtime::component::Resource<Model>,
                    input: String,
                    max_tokens: Option<u32>,
                    stop_on: Option<String>,
                ) -> wasmtime::Result<String>;

                async fn infer_structured(
                    &mut self,
                    self_: wasmtime::component::Resource<Model>,
                    input: String,
                    max_tokens: Option<u32>,
                    structure: wasmtime::component::Resource<Structure>,
                ) -> wasmtime::Result<String>;

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Model>,
                ) -> wasmtime::Result<()>;
            }
            pub enum EmbeddingModel {}

            #[wasmtime::component::__internal::async_trait]
            pub trait HostEmbeddingModel {
                async fn new(
                    &mut self,
                    ty: EmbeddingModelType,
                ) -> wasmtime::Result<wasmtime::component::Resource<EmbeddingModel>>;

                async fn model_downloaded(
                    &mut self,
                    ty: EmbeddingModelType,
                ) -> wasmtime::Result<bool>;

                async fn get_embedding(
                    &mut self,
                    self_: wasmtime::component::Resource<EmbeddingModel>,
                    document: String,
                ) -> wasmtime::Result<Embedding>;

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<EmbeddingModel>,
                ) -> wasmtime::Result<()>;
            }
            pub enum Structure {}

            #[wasmtime::component::__internal::async_trait]
            pub trait HostStructure {
                async fn num(
                    &mut self,
                    num: NumberParameters,
                ) -> wasmtime::Result<wasmtime::component::Resource<Structure>>;

                async fn literal(
                    &mut self,
                    literal: String,
                ) -> wasmtime::Result<wasmtime::component::Resource<Structure>>;

                async fn or(
                    &mut self,
                    or: EitherStructure,
                ) -> wasmtime::Result<wasmtime::component::Resource<Structure>>;

                async fn then(
                    &mut self,
                    then: ThenStructure,
                ) -> wasmtime::Result<wasmtime::component::Resource<Structure>>;

                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Structure>,
                ) -> wasmtime::Result<()>;
            }
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Clone)]
            pub struct Embedding {
                #[component(name = "vector")]
                pub vector: Vec<f32>,
            }
            impl core::fmt::Debug for Embedding {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Embedding")
                        .field("vector", &self.vector)
                        .finish()
                }
            }
            const _: () = {
                assert!(8 =  =  <Embedding as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 =  =  <Embedding as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            pub struct ThenStructure {
                #[component(name = "first")]
                pub first: wasmtime::component::Resource<Structure>,
                #[component(name = "second")]
                pub second: wasmtime::component::Resource<Structure>,
            }
            impl core::fmt::Debug for ThenStructure {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("ThenStructure")
                        .field("first", &self.first)
                        .field("second", &self.second)
                        .finish()
                }
            }
            const _: () = {
                assert!(8 =  =  <ThenStructure as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 =  =  <ThenStructure as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            pub struct EitherStructure {
                #[component(name = "first")]
                pub first: wasmtime::component::Resource<Structure>,
                #[component(name = "second")]
                pub second: wasmtime::component::Resource<Structure>,
            }
            impl core::fmt::Debug for EitherStructure {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("EitherStructure")
                        .field("first", &self.first)
                        .field("second", &self.second)
                        .finish()
                }
            }
            const _: () = {
                assert!(8 =  =  <EitherStructure as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 =  =  <EitherStructure as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Copy, Clone)]
            pub struct NumberParameters {
                #[component(name = "min")]
                pub min: f64,
                #[component(name = "max")]
                pub max: f64,
                #[component(name = "integer")]
                pub integer: bool,
            }
            impl core::fmt::Debug for NumberParameters {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("NumberParameters")
                        .field("min", &self.min)
                        .field("max", &self.max)
                        .field("integer", &self.integer)
                        .finish()
                }
            }
            const _: () = {
                assert!(24 =  =  <NumberParameters as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 =  =  <NumberParameters as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(enum)]
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub enum PrimitiveValueType {
                #[component(name = "number")]
                Number,
                #[component(name = "text")]
                Text,
                #[component(name = "file")]
                File,
                #[component(name = "folder")]
                Folder,
                #[component(name = "embedding")]
                Embedding,
                #[component(name = "database")]
                Database,
                #[component(name = "model")]
                Model,
                #[component(name = "model-type")]
                ModelType,
                #[component(name = "boolean")]
                Boolean,
                #[component(name = "page")]
                Page,
                #[component(name = "node")]
                Node,
                #[component(name = "any")]
                Any,
            }
            impl core::fmt::Debug for PrimitiveValueType {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        PrimitiveValueType::Number => {
                            f.debug_tuple("PrimitiveValueType::Number").finish()
                        }
                        PrimitiveValueType::Text => {
                            f.debug_tuple("PrimitiveValueType::Text").finish()
                        }
                        PrimitiveValueType::File => {
                            f.debug_tuple("PrimitiveValueType::File").finish()
                        }
                        PrimitiveValueType::Folder => {
                            f.debug_tuple("PrimitiveValueType::Folder").finish()
                        }
                        PrimitiveValueType::Embedding => {
                            f.debug_tuple("PrimitiveValueType::Embedding").finish()
                        }
                        PrimitiveValueType::Database => {
                            f.debug_tuple("PrimitiveValueType::Database").finish()
                        }
                        PrimitiveValueType::Model => {
                            f.debug_tuple("PrimitiveValueType::Model").finish()
                        }
                        PrimitiveValueType::ModelType => {
                            f.debug_tuple("PrimitiveValueType::ModelType").finish()
                        }
                        PrimitiveValueType::Boolean => {
                            f.debug_tuple("PrimitiveValueType::Boolean").finish()
                        }
                        PrimitiveValueType::Page => {
                            f.debug_tuple("PrimitiveValueType::Page").finish()
                        }
                        PrimitiveValueType::Node => {
                            f.debug_tuple("PrimitiveValueType::Node").finish()
                        }
                        PrimitiveValueType::Any => {
                            f.debug_tuple("PrimitiveValueType::Any").finish()
                        }
                    }
                }
            }
            const _: () = {
                assert!(1 =  =  <PrimitiveValueType as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 =  =  <PrimitiveValueType as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            #[derive(Copy, Clone)]
            pub enum ValueType {
                #[component(name = "single")]
                Single(PrimitiveValueType),
                #[component(name = "many")]
                Many(PrimitiveValueType),
            }
            impl core::fmt::Debug for ValueType {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        ValueType::Single(e) => {
                            f.debug_tuple("ValueType::Single").field(e).finish()
                        }
                        ValueType::Many(e) => f.debug_tuple("ValueType::Many").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(2 =  =  <ValueType as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 =  =  <ValueType as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Clone)]
            pub struct IoDefinition {
                #[component(name = "name")]
                pub name: String,
                #[component(name = "ty")]
                pub ty: ValueType,
            }
            impl core::fmt::Debug for IoDefinition {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("IoDefinition")
                        .field("name", &self.name)
                        .field("ty", &self.ty)
                        .finish()
                }
            }
            const _: () = {
                assert!(12 =  =  <IoDefinition as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 =  =  <IoDefinition as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(enum)]
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub enum LlamaType {
                #[component(name = "vicuna")]
                Vicuna,
                #[component(name = "guanaco")]
                Guanaco,
                #[component(name = "wizardlm")]
                Wizardlm,
                #[component(name = "orca")]
                Orca,
                #[component(name = "llama-seven-chat")]
                LlamaSevenChat,
                #[component(name = "llama-thirteen-chat")]
                LlamaThirteenChat,
            }
            impl core::fmt::Debug for LlamaType {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        LlamaType::Vicuna => f.debug_tuple("LlamaType::Vicuna").finish(),
                        LlamaType::Guanaco => f.debug_tuple("LlamaType::Guanaco").finish(),
                        LlamaType::Wizardlm => f.debug_tuple("LlamaType::Wizardlm").finish(),
                        LlamaType::Orca => f.debug_tuple("LlamaType::Orca").finish(),
                        LlamaType::LlamaSevenChat => {
                            f.debug_tuple("LlamaType::LlamaSevenChat").finish()
                        }
                        LlamaType::LlamaThirteenChat => {
                            f.debug_tuple("LlamaType::LlamaThirteenChat").finish()
                        }
                    }
                }
            }
            const _: () = {
                assert!(1 =  =  <LlamaType as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 =  =  <LlamaType as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(enum)]
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub enum MptType {
                #[component(name = "base")]
                Base,
                #[component(name = "story")]
                Story,
                #[component(name = "instruct")]
                Instruct,
                #[component(name = "chat")]
                Chat,
            }
            impl core::fmt::Debug for MptType {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        MptType::Base => f.debug_tuple("MptType::Base").finish(),
                        MptType::Story => f.debug_tuple("MptType::Story").finish(),
                        MptType::Instruct => f.debug_tuple("MptType::Instruct").finish(),
                        MptType::Chat => f.debug_tuple("MptType::Chat").finish(),
                    }
                }
            }
            const _: () = {
                assert!(1 =  =  <MptType as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 =  =  <MptType as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(enum)]
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub enum GptNeoXType {
                #[component(name = "large-pythia")]
                LargePythia,
                #[component(name = "tiny-pythia")]
                TinyPythia,
                #[component(name = "dolly-seven-b")]
                DollySevenB,
                #[component(name = "stablelm")]
                Stablelm,
            }
            impl core::fmt::Debug for GptNeoXType {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        GptNeoXType::LargePythia => {
                            f.debug_tuple("GptNeoXType::LargePythia").finish()
                        }
                        GptNeoXType::TinyPythia => {
                            f.debug_tuple("GptNeoXType::TinyPythia").finish()
                        }
                        GptNeoXType::DollySevenB => {
                            f.debug_tuple("GptNeoXType::DollySevenB").finish()
                        }
                        GptNeoXType::Stablelm => f.debug_tuple("GptNeoXType::Stablelm").finish(),
                    }
                }
            }
            const _: () = {
                assert!(1 =  =  <GptNeoXType as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 =  =  <GptNeoXType as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            #[derive(Copy, Clone)]
            pub enum ModelType {
                #[component(name = "MPT")]
                Mpt(MptType),
                #[component(name = "gpt-neo-x")]
                GptNeoX(GptNeoXType),
                #[component(name = "llama")]
                Llama(LlamaType),
                #[component(name = "phi")]
                Phi,
                #[component(name = "mistral")]
                Mistral,
            }
            impl core::fmt::Debug for ModelType {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        ModelType::Mpt(e) => f.debug_tuple("ModelType::Mpt").field(e).finish(),
                        ModelType::GptNeoX(e) => {
                            f.debug_tuple("ModelType::GptNeoX").field(e).finish()
                        }
                        ModelType::Llama(e) => f.debug_tuple("ModelType::Llama").field(e).finish(),
                        ModelType::Phi => f.debug_tuple("ModelType::Phi").finish(),
                        ModelType::Mistral => f.debug_tuple("ModelType::Mistral").finish(),
                    }
                }
            }
            const _: () = {
                assert!(2 =  =  <ModelType as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 =  =  <ModelType as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            pub enum PrimitiveValue {
                #[component(name = "model")]
                Model(wasmtime::component::Resource<Model>),
                #[component(name = "model-type")]
                ModelType(ModelType),
                #[component(name = "database")]
                Database(wasmtime::component::Resource<EmbeddingDb>),
                #[component(name = "number")]
                Number(i64),
                #[component(name = "text")]
                Text(String),
                #[component(name = "file")]
                File(String),
                #[component(name = "folder")]
                Folder(String),
                #[component(name = "embedding")]
                Embedding(Embedding),
                #[component(name = "boolean")]
                Boolean(bool),
                #[component(name = "page")]
                Page(wasmtime::component::Resource<Page>),
                #[component(name = "node")]
                Node(wasmtime::component::Resource<Node>),
            }
            impl core::fmt::Debug for PrimitiveValue {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        PrimitiveValue::Model(e) => {
                            f.debug_tuple("PrimitiveValue::Model").field(e).finish()
                        }
                        PrimitiveValue::ModelType(e) => {
                            f.debug_tuple("PrimitiveValue::ModelType").field(e).finish()
                        }
                        PrimitiveValue::Database(e) => {
                            f.debug_tuple("PrimitiveValue::Database").field(e).finish()
                        }
                        PrimitiveValue::Number(e) => {
                            f.debug_tuple("PrimitiveValue::Number").field(e).finish()
                        }
                        PrimitiveValue::Text(e) => {
                            f.debug_tuple("PrimitiveValue::Text").field(e).finish()
                        }
                        PrimitiveValue::File(e) => {
                            f.debug_tuple("PrimitiveValue::File").field(e).finish()
                        }
                        PrimitiveValue::Folder(e) => {
                            f.debug_tuple("PrimitiveValue::Folder").field(e).finish()
                        }
                        PrimitiveValue::Embedding(e) => {
                            f.debug_tuple("PrimitiveValue::Embedding").field(e).finish()
                        }
                        PrimitiveValue::Boolean(e) => {
                            f.debug_tuple("PrimitiveValue::Boolean").field(e).finish()
                        }
                        PrimitiveValue::Page(e) => {
                            f.debug_tuple("PrimitiveValue::Page").field(e).finish()
                        }
                        PrimitiveValue::Node(e) => {
                            f.debug_tuple("PrimitiveValue::Node").field(e).finish()
                        }
                    }
                }
            }
            const _: () = {
                assert!(16 =  =  <PrimitiveValue as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 =  =  <PrimitiveValue as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            pub enum Input {
                #[component(name = "single")]
                Single(PrimitiveValue),
                #[component(name = "many")]
                Many(Vec<PrimitiveValue>),
            }
            impl core::fmt::Debug for Input {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Input::Single(e) => f.debug_tuple("Input::Single").field(e).finish(),
                        Input::Many(e) => f.debug_tuple("Input::Many").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(24 =  =  <Input as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 =  =  <Input as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            pub enum Output {
                #[component(name = "single")]
                Single(PrimitiveValue),
                #[component(name = "many")]
                Many(Vec<PrimitiveValue>),
                #[component(name = "halt")]
                Halt,
            }
            impl core::fmt::Debug for Output {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Output::Single(e) => f.debug_tuple("Output::Single").field(e).finish(),
                        Output::Many(e) => f.debug_tuple("Output::Many").field(e).finish(),
                        Output::Halt => f.debug_tuple("Output::Halt").finish(),
                    }
                }
            }
            const _: () = {
                assert!(24 =  =  <Output as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 =  =  <Output as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            pub struct Example {
                #[component(name = "name")]
                pub name: String,
                #[component(name = "inputs")]
                pub inputs: Vec<Input>,
                #[component(name = "outputs")]
                pub outputs: Vec<Output>,
            }
            impl core::fmt::Debug for Example {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Example")
                        .field("name", &self.name)
                        .field("inputs", &self.inputs)
                        .field("outputs", &self.outputs)
                        .finish()
                }
            }
            const _: () = {
                assert!(24 =  =  <Example as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 =  =  <Example as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            pub struct Definition {
                #[component(name = "name")]
                pub name: String,
                #[component(name = "description")]
                pub description: String,
                #[component(name = "inputs")]
                pub inputs: Vec<IoDefinition>,
                #[component(name = "outputs")]
                pub outputs: Vec<IoDefinition>,
                #[component(name = "examples")]
                pub examples: Vec<Example>,
            }
            impl core::fmt::Debug for Definition {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Definition")
                        .field("name", &self.name)
                        .field("description", &self.description)
                        .field("inputs", &self.inputs)
                        .field("outputs", &self.outputs)
                        .field("examples", &self.examples)
                        .finish()
                }
            }
            const _: () = {
                assert!(40 =  =  <Definition as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 =  =  <Definition as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            #[derive(Copy, Clone)]
            pub enum EmbeddingModelType {
                #[component(name = "MPT")]
                Mpt(MptType),
                #[component(name = "gpt-neo-x")]
                GptNeoX(GptNeoXType),
                #[component(name = "llama")]
                Llama(LlamaType),
                #[component(name = "bert")]
                Bert,
            }
            impl core::fmt::Debug for EmbeddingModelType {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        EmbeddingModelType::Mpt(e) => {
                            f.debug_tuple("EmbeddingModelType::Mpt").field(e).finish()
                        }
                        EmbeddingModelType::GptNeoX(e) => f
                            .debug_tuple("EmbeddingModelType::GptNeoX")
                            .field(e)
                            .finish(),
                        EmbeddingModelType::Llama(e) => {
                            f.debug_tuple("EmbeddingModelType::Llama").field(e).finish()
                        }
                        EmbeddingModelType::Bert => {
                            f.debug_tuple("EmbeddingModelType::Bert").finish()
                        }
                    }
                }
            }
            const _: () = {
                assert!(2 =  =  <EmbeddingModelType as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 =  =  <EmbeddingModelType as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[wasmtime::component::__internal::async_trait]
            pub trait Host:
                HostPage
                + HostNode
                + HostEmbeddingDb
                + HostModel
                + HostEmbeddingModel
                + HostStructure
            {
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                T: Send,
                U: Host + Send,
            {
                let mut inst = linker.instance("plugins:main/types")?;
                inst.resource::<Page>("page", move |mut store, rep| -> wasmtime::Result<()> {
                    HostPage::drop(
                        get(store.data_mut()),
                        wasmtime::component::Resource::new_own(rep),
                    )
                })?;
                inst.resource::<Node>("node", move |mut store, rep| -> wasmtime::Result<()> {
                    HostNode::drop(
                        get(store.data_mut()),
                        wasmtime::component::Resource::new_own(rep),
                    )
                })?;
                inst.resource::<EmbeddingDb>(
                    "embedding-db",
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostEmbeddingDb::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource::<Model>("model", move |mut store, rep| -> wasmtime::Result<()> {
                    HostModel::drop(
                        get(store.data_mut()),
                        wasmtime::component::Resource::new_own(rep),
                    )
                })?;
                inst.resource::<EmbeddingModel>(
                    "embedding-model",
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostEmbeddingModel::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource::<Structure>(
                    "structure",
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostStructure::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "[constructor]page",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (BrowserMode, String)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostPage::new(host, arg0, arg1).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async("[method]page.find-in-current-page",move|mut caller:wasmtime::StoreContextMut<'_,T>,(arg0,arg1,):(wasmtime::component::Resource<Page>,String,)|Box::new(async move {
          let host = get(caller.data_mut());
          let r = HostPage::find_in_current_page(host,arg0,arg1,).await;
          Ok((r?,))
        }))?;
                inst.func_wrap_async(
                    "[method]page.screenshot-browser",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Page>,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostPage::screenshot_browser(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[method]node.get-element-text",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Node>,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostNode::get_element_text(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[method]node.click-element",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Node>,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostNode::click_element(host, arg0).await;
                            r
                        })
                    },
                )?;
                inst.func_wrap_async("[method]node.type-into-element",move|mut caller:wasmtime::StoreContextMut<'_,T>,(arg0,arg1,):(wasmtime::component::Resource<Node>,String,)|Box::new(async move {
          let host = get(caller.data_mut());
          let r = HostNode::type_into_element(host,arg0,arg1,).await;
          r
        }))?;
                inst.func_wrap_async(
                    "[method]node.get-element-outer-html",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Node>,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostNode::get_element_outer_html(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[method]node.screenshot-element",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Node>,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostNode::screenshot_element(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async("[method]node.find-child-of-element",move|mut caller:wasmtime::StoreContextMut<'_,T>,(arg0,arg1,):(wasmtime::component::Resource<Node>,String,)|Box::new(async move {
          let host = get(caller.data_mut());
          let r = HostNode::find_child_of_element(host,arg0,arg1,).await;
          Ok((r?,))
        }))?;
                inst.func_wrap_async(
                    "[constructor]embedding-db",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (Vec<Embedding>, Vec<String>)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostEmbeddingDb::new(host, arg0, arg1).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[method]embedding-db.add-embedding",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<EmbeddingDb>,
                        Embedding,
                        String,
                    )| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostEmbeddingDb::add_embedding(host, arg0, arg1, arg2).await;
                            r
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[method]embedding-db.find-closest-documents",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<EmbeddingDb>,
                        Embedding,
                        u32,
                    )| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostEmbeddingDb::find_closest_documents(host, arg0, arg1, arg2)
                                .await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[method]embedding-db.find-documents-within",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<EmbeddingDb>,
                        Embedding,
                        f32,
                    )| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostEmbeddingDb::find_documents_within(host, arg0, arg1, arg2)
                                .await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[constructor]model",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (ModelType,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostModel::new(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[static]model.model-downloaded",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (ModelType,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostModel::model_downloaded(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[method]model.infer",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2, arg3): (
                        wasmtime::component::Resource<Model>,
                        String,
                        Option<u32>,
                        Option<String>,
                    )| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostModel::infer(host, arg0, arg1, arg2, arg3).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[method]model.infer-structured",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2, arg3): (
                        wasmtime::component::Resource<Model>,
                        String,
                        Option<u32>,
                        wasmtime::component::Resource<Structure>,
                    )| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostModel::infer_structured(host, arg0, arg1, arg2, arg3).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[constructor]embedding-model",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (EmbeddingModelType,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostEmbeddingModel::new(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[static]embedding-model.model-downloaded",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (EmbeddingModelType,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostEmbeddingModel::model_downloaded(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async("[method]embedding-model.get-embedding",move|mut caller:wasmtime::StoreContextMut<'_,T>,(arg0,arg1,):(wasmtime::component::Resource<EmbeddingModel>,String,)|Box::new(async move {
          let host = get(caller.data_mut());
          let r = HostEmbeddingModel::get_embedding(host,arg0,arg1,).await;
          Ok((r?,))
        }))?;
                inst.func_wrap_async(
                    "[static]structure.num",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (NumberParameters,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostStructure::num(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[static]structure.literal",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (String,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostStructure::literal(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[static]structure.or",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (EitherStructure,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostStructure::or(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "[static]structure.then",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (ThenStructure,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = HostStructure::then(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod imports {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type Embedding = super::super::super::plugins::main::types::Embedding;
            const _: () = {
                assert!(8 =  =  <Embedding as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 =  =  <Embedding as wasmtime::component::ComponentType>::ALIGN32);
            };
            pub type Structure = super::super::super::plugins::main::types::Structure;
            pub type Model = super::super::super::plugins::main::types::Model;
            pub type ModelType = super::super::super::plugins::main::types::ModelType;
            const _: () = {
                assert!(2 =  =  <ModelType as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 =  =  <ModelType as wasmtime::component::ComponentType>::ALIGN32);
            };
            pub type EmbeddingDb = super::super::super::plugins::main::types::EmbeddingDb;
            pub type Node = super::super::super::plugins::main::types::Node;
            pub type Page = super::super::super::plugins::main::types::Page;
            #[wasmtime::component::__internal::async_trait]
            pub trait Host {
                async fn store(&mut self, key: Vec<u8>, value: Vec<u8>) -> wasmtime::Result<()>;

                async fn load(&mut self, key: Vec<u8>) -> wasmtime::Result<Vec<u8>>;

                async fn unload(&mut self, key: Vec<u8>) -> wasmtime::Result<()>;

                async fn log_to_user(&mut self, information: String) -> wasmtime::Result<()>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                T: Send,
                U: Host + Send,
            {
                let mut inst = linker.instance("plugins:main/imports")?;
                inst.func_wrap_async(
                    "store",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (Vec<u8>, Vec<u8>)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = Host::store(host, arg0, arg1).await;
                            r
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "load",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (Vec<u8>,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = Host::load(host, arg0).await;
                            Ok((r?,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "unload",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (Vec<u8>,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = Host::unload(host, arg0).await;
                            r
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "log-to-user",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (String,)| {
                        Box::new(async move {
                            let host = get(caller.data_mut());
                            let r = Host::log_to_user(host, arg0).await;
                            r
                        })
                    },
                )?;
                Ok(())
            }
        }
    }
}
pub mod exports {
    pub mod plugins {
        pub mod main {
            #[allow(clippy::all)]
            pub mod definitions {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type Definition = super::super::super::super::plugins::main::types::Definition;
                const _: () = {
                    assert!(40 =  =  <Definition as wasmtime::component::ComponentType>::SIZE32);
                    assert!(4 =  =  <Definition as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub type Input = super::super::super::super::plugins::main::types::Input;
                const _: () = {
                    assert!(24 =  =  <Input as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 =  =  <Input as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub type Output = super::super::super::super::plugins::main::types::Output;
                const _: () = {
                    assert!(24 =  =  <Output as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 =  =  <Output as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub struct Definitions {
                    structure: wasmtime::component::Func,
                    run: wasmtime::component::Func,
                }
                impl Definitions {
                    pub fn new(
                        __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                    ) -> wasmtime::Result<Definitions> {
                        let structure = *__exports
                            .typed_func::<(), (Definition,)>("structure")?
                            .func();
                        let run = *__exports
                            .typed_func::<(&[Input],), (Vec<Output>,)>("run")?
                            .func();
                        Ok(Definitions { structure, run })
                    }
                    pub async fn call_structure<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<Definition>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<(), (Definition,)>::new_unchecked(
                                self.structure,
                            )
                        };
                        let (ret0,) = callee.call_async(store.as_context_mut(), ()).await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_run<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: &[Input],
                    ) -> wasmtime::Result<Vec<Output>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<(&[Input],),(Vec<Output>,)>::new_unchecked(self.run)
                        };
                        let (ret0,) = callee.call_async(store.as_context_mut(), (arg0,)).await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}
const _: &str = include_str!(r#"/home/alberto/sd/sidrai/floneum/plugin/../wit/plugin.wit"#);
