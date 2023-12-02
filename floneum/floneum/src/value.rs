use crate::Color;
use dioxus::prelude::*;
use floneum_plugin::{
    exports::plugins::main::definitions::{Input, Output},
    plugins::main::types::*,
};
use std::path::PathBuf;

use crate::{node_value::NodeInput, Signal};

#[derive(Props, PartialEq)]
pub struct ShowOutputProps {
    name: String,
    value: Output,
}

pub fn ShowOutput(cx: Scope<ShowOutputProps>) -> Element {
    let ShowOutputProps { name, value } = &cx.props;
    match value {
        Output::Single(value) => {
            render! {
                div {
                    class: "flex flex-col whitespace-pre-line",
                    "{name}:\n"
                    show_primitive_value(cx, value)
                }
            }
        }
        Output::Many(value) => {
            render! {
                div {
                    class: "flex flex-col",
                    "{name}:"
                    for value in &value {
                        div {
                            class: "whitespace-pre-line",
                            show_primitive_value(cx, value)
                        }
                    }
                }
            }
        }
        _ => {
            render! {
                div {
                    class: "flex flex-col",
                    "{name}: Unset"
                }
            }
        }
    }
}

fn show_primitive_value<'a>(cx: &'a ScopeState, value: &PrimitiveValue) -> Element<'a> {
    match value {
        PrimitiveValue::Text(value)
        | PrimitiveValue::File(value)
        | PrimitiveValue::Folder(value) => {
            render! {"{value}"}
        }
        PrimitiveValue::Embedding(value) => {
            let first_five = value.vector.iter().take(5).collect::<Vec<_>>();
            render! {"{first_five:?}"}
        }
        PrimitiveValue::Model(id) => {
            render! {"Model: {id:?}"}
        }
        PrimitiveValue::EmbeddingModel(id) => {
            render! {"Embedding Model: {id:?}"}
        }
        PrimitiveValue::Database(id) => {
            render! {"Database: {id:?}"}
        }
        PrimitiveValue::Number(value) => {
            render! {"{value}"}
        }
        PrimitiveValue::ModelType(ty) => {
            render! {"{ty.name()}"}
        }
        PrimitiveValue::EmbeddingModelType(ty) => {
            render! {"{ty.name()}"}
        }
        PrimitiveValue::Boolean(val) => {
            render! {"{val:?}"}
        }
        PrimitiveValue::Page(id) => {
            render! {"Page: {id:?}"}
        }
        PrimitiveValue::Node(id) => {
            render! {"Node: {id:?}"}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct ShowInputProps {
    label: String,
    value: Input,
}

pub fn ShowInput(cx: Scope<ShowInputProps>) -> Element {
    let ShowInputProps { label, value } = &cx.props;
    match value {
        Input::Single(value) => {
            render! {
                div {
                    class: "flex flex-col whitespace-pre-line",
                    "{label}:\n"
                    show_primitive_value(cx, value)
                }
            }
        }
        Input::Many(value) => {
            render! {
                div {
                    class: "flex flex-col",
                    "{label}:"
                    for value in &value {
                        div {
                            class: "whitespace-pre-line",
                            show_primitive_value(cx, value)
                        }
                    }
                }
            }
        }
    }
}

#[inline_props]
pub fn ModifyInput(cx: &ScopeState, value: Signal<NodeInput>) -> Element {
    let node = value;
    let current_value = node.read();
    let name = &current_value.definition.name;
    match current_value.value() {
        Input::Single(current_primitive) => match current_primitive {
            PrimitiveValue::Text(value) => {
                render! {
                    div {
                        class: "flex flex-col",
                        "{name}: "
                        textarea {
                            class: "border {Color::outline_color()} {Color::foreground_color()} {Color::foreground_hover()} rounded focus:outline-none focus:border-blue-500",
                            value: "{value}",
                            oninput: |e| {
                                node.write().value = vec![Input::Single(PrimitiveValue::Text(e.value.to_string()))];
                            }
                        }
                    }
                }
            }
            PrimitiveValue::File(file) => {
                render! {
                    button {
                        class: "border {Color::outline_color()} {Color::foreground_hover()} rounded focus:outline-none focus:border-blue-500",
                        onclick: |_| {
                            node.write().value = rfd::FileDialog::new()
                                .set_directory("./sandbox")
                                .set_file_name("Floneum")
                                .set_title("Select File")
                                .save_file()
                                .map(|path| vec![Input::Single(PrimitiveValue::File(path.strip_prefix(PathBuf::from("./sandbox").canonicalize().unwrap()).unwrap_or(&path).to_string_lossy().to_string()))])
                                .unwrap_or_else(|| vec![Input::Single(PrimitiveValue::File("".to_string()))])
                        },
                        "Select File"
                    }
                    "{file}"
                }
            }
            PrimitiveValue::Folder(folder) => {
                render! {
                    button {
                        class: "border {Color::outline_color()} rounded {Color::foreground_hover()} focus:outline-none focus:border-blue-500",
                        onclick: |_| {
                            node.write().value = rfd::FileDialog::new()
                                .set_directory("./sandbox")
                                .set_file_name("Floneum")
                                .set_title("Select Folder")
                                .pick_folder()
                                .map(|path| vec![Input::Single(PrimitiveValue::File(path.strip_prefix(PathBuf::from("./sandbox").canonicalize().unwrap()).unwrap_or(&path).to_string_lossy().to_string()))])
                                .unwrap_or_else(|| vec![Input::Single(PrimitiveValue::File("".to_string()))]);
                        },
                        "Select Folder"
                    }
                    "{folder}"
                }
            }
            PrimitiveValue::Embedding(_)
            | PrimitiveValue::Model(_)
            | PrimitiveValue::EmbeddingModel(_)
            | PrimitiveValue::Database(_)
            | PrimitiveValue::Page(_)
            | PrimitiveValue::Node(_) => show_primitive_value(cx, &current_primitive),
            PrimitiveValue::Number(value) => {
                render! {
                    div {
                        class: "flex flex-col",
                        "{name}: "
                        input {
                            class: "border {Color::outline_color()} {Color::foreground_color()} rounded {Color::foreground_hover()} focus:outline-none focus:border-blue-500",
                            r#type: "number",
                            value: "{value}",
                            oninput: |e| {
                                node
                                    .write().value = vec![Input::Single(PrimitiveValue::Number(e.value.parse().unwrap_or(0)))];
                            }
                        }
                    }
                }
            }
            PrimitiveValue::ModelType(ty) => {
                render! {
                    div {
                        class: "flex flex-col",
                        "{name}: "
                        select {
                            class: "border {Color::outline_color()} {Color::foreground_color()} rounded {Color::foreground_hover()} focus:outline-none focus:border-blue-500",
                            style: "-webkit-appearance:none; -moz-appearance:none; -ms-appearance:none; appearance: none;",
                            onchange: |e| {
                                node
                                    .write().value = vec![Input::Single(
                                    PrimitiveValue::ModelType(
                                        model_type_from_str(&e.value)
                                            .unwrap_or(ModelType::Llama(LlamaType::LlamaThirteenChat)),
                                    ),
                                )];
                            },
                            for variant in ModelType::VARIANTS {
                                option {
                                    value: "{variant.name()}",
                                    selected: "{variant.name() == ty.name()}",
                                    "{variant.name()}"
                                }
                            }
                        }
                    }
                }
            }
            PrimitiveValue::EmbeddingModelType(ty) => {
                render! {
                    div {
                        class: "flex flex-col",
                        "{name}: "
                        select {
                            class: "border {Color::outline_color()} {Color::foreground_color()} rounded {Color::foreground_hover()} focus:outline-none focus:border-blue-500",
                            style: "-webkit-appearance:none; -moz-appearance:none; -ms-appearance:none; appearance: none;",
                            onchange: |e| {
                                node
                                    .write().value = vec![Input::Single(
                                    PrimitiveValue::EmbeddingModelType(
                                        embedding_model_type_from_str(&e.value)
                                            .unwrap_or(EmbeddingModelType::Llama(LlamaType::LlamaThirteenChat)),
                                    ),
                                )];
                            },
                            for variant in EmbeddingModelType::VARIANTS {
                                option {
                                    value: "{variant.name()}",
                                    selected: "{variant.name() == ty.name()}",
                                    "{variant.name()}"
                                }
                            }
                        }
                    }
                }
            }
            PrimitiveValue::Boolean(val) => {
                render! {
                    div {
                        class: "flex flex-col",
                        "{name}: "
                        input {
                            class: "border {Color::outline_color()} {Color::foreground_color()} rounded {Color::foreground_hover()} focus:outline-none focus:border-blue-500",
                            r#type: "checkbox",
                            checked: "{val}",
                            onchange: |e| {
                                node.write().value = vec![Input::Single(PrimitiveValue::Boolean(e.value == "on"))];
                            }
                        }
                    }
                }
            }
        },
        Input::Many(values) => {
            render! {
                div {
                    div {
                        class: "flex flex-col",
                        "{name}: "
                        for value in values.iter() {
                            div {
                                class: "whitespace-pre-line",
                                show_primitive_value(cx, value)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub trait Variants: Sized + 'static {
    const VARIANTS: &'static [Self];
}

impl Variants for ModelType {
    const VARIANTS: &'static [Self] = &[
        ModelType::Llama(LlamaType::Guanaco),
        ModelType::Llama(LlamaType::Orca),
        ModelType::Llama(LlamaType::Vicuna),
        ModelType::Llama(LlamaType::Wizardlm),
        ModelType::Llama(LlamaType::LlamaSevenChat),
        ModelType::Llama(LlamaType::LlamaThirteenChat),
        ModelType::GptNeoX(GptNeoXType::TinyPythia),
        ModelType::GptNeoX(GptNeoXType::LargePythia),
        ModelType::GptNeoX(GptNeoXType::Stablelm),
        ModelType::GptNeoX(GptNeoXType::DollySevenB),
        ModelType::Mpt(MptType::Base),
        ModelType::Mpt(MptType::Chat),
        ModelType::Mpt(MptType::Story),
        ModelType::Mpt(MptType::Instruct),
        ModelType::Phi,
        ModelType::Mistral,
    ];
}

impl Variants for EmbeddingModelType {
    const VARIANTS: &'static [Self] = &[
        EmbeddingModelType::Llama(LlamaType::Guanaco),
        EmbeddingModelType::Llama(LlamaType::Orca),
        EmbeddingModelType::Llama(LlamaType::Vicuna),
        EmbeddingModelType::Llama(LlamaType::Wizardlm),
        EmbeddingModelType::Llama(LlamaType::LlamaSevenChat),
        EmbeddingModelType::Llama(LlamaType::LlamaThirteenChat),
        EmbeddingModelType::GptNeoX(GptNeoXType::TinyPythia),
        EmbeddingModelType::GptNeoX(GptNeoXType::LargePythia),
        EmbeddingModelType::GptNeoX(GptNeoXType::Stablelm),
        EmbeddingModelType::GptNeoX(GptNeoXType::DollySevenB),
        EmbeddingModelType::Mpt(MptType::Base),
        EmbeddingModelType::Mpt(MptType::Chat),
        EmbeddingModelType::Mpt(MptType::Story),
        EmbeddingModelType::Mpt(MptType::Instruct),
        EmbeddingModelType::Bert,
    ];
}

impl Variants for PrimitiveValueType {
    const VARIANTS: &'static [Self] = &[
        PrimitiveValueType::Text,
        PrimitiveValueType::File,
        PrimitiveValueType::Folder,
        PrimitiveValueType::Number,
        PrimitiveValueType::Boolean,
        PrimitiveValueType::Embedding,
        PrimitiveValueType::Model,
        PrimitiveValueType::ModelType,
        PrimitiveValueType::Database,
        PrimitiveValueType::Page,
        PrimitiveValueType::Node,
        PrimitiveValueType::Any,
    ];
}

impl Variants for ValueType {
    const VARIANTS: &'static [Self] = &[
        ValueType::Single(PrimitiveValueType::Text),
        ValueType::Single(PrimitiveValueType::File),
        ValueType::Single(PrimitiveValueType::Folder),
        ValueType::Single(PrimitiveValueType::Number),
        ValueType::Single(PrimitiveValueType::Boolean),
        ValueType::Single(PrimitiveValueType::Embedding),
        ValueType::Single(PrimitiveValueType::Model),
        ValueType::Single(PrimitiveValueType::ModelType),
        ValueType::Single(PrimitiveValueType::Database),
        ValueType::Single(PrimitiveValueType::Page),
        ValueType::Single(PrimitiveValueType::Node),
        ValueType::Single(PrimitiveValueType::Any),
        ValueType::Many(PrimitiveValueType::Text),
        ValueType::Many(PrimitiveValueType::File),
        ValueType::Many(PrimitiveValueType::Folder),
        ValueType::Many(PrimitiveValueType::Number),
        ValueType::Many(PrimitiveValueType::Boolean),
        ValueType::Many(PrimitiveValueType::Embedding),
        ValueType::Many(PrimitiveValueType::Model),
        ValueType::Many(PrimitiveValueType::ModelType),
        ValueType::Many(PrimitiveValueType::Database),
        ValueType::Many(PrimitiveValueType::Page),
        ValueType::Many(PrimitiveValueType::Node),
        ValueType::Many(PrimitiveValueType::Any),
    ];
}

pub trait Named {
    fn name(&self) -> &'static str;
}

impl Named for ModelType {
    fn name(&self) -> &'static str {
        match self {
            ModelType::Llama(LlamaType::Guanaco) => "Guanaco",
            ModelType::Llama(LlamaType::Orca) => "Orca",
            ModelType::Llama(LlamaType::Vicuna) => "Vicuna",
            ModelType::Llama(LlamaType::Wizardlm) => "Wizardlm",
            ModelType::Llama(LlamaType::LlamaSevenChat) => "Llama Seven Chat",
            ModelType::Llama(LlamaType::LlamaThirteenChat) => "Llama Thirteen Chat",
            ModelType::GptNeoX(GptNeoXType::TinyPythia) => "Tiny Pythia",
            ModelType::GptNeoX(GptNeoXType::LargePythia) => "Large Pythia",
            ModelType::GptNeoX(GptNeoXType::Stablelm) => "Stablelm",
            ModelType::GptNeoX(GptNeoXType::DollySevenB) => "Dolly",
            ModelType::Mpt(MptType::Base) => "Mpt base",
            ModelType::Mpt(MptType::Chat) => "Mpt chat",
            ModelType::Mpt(MptType::Story) => "Mpt story",
            ModelType::Mpt(MptType::Instruct) => "Mpt instruct",
            ModelType::Phi => "Phi",
            ModelType::Mistral => "Mistral",
        }
    }
}

impl Named for EmbeddingModelType {
    fn name(&self) -> &'static str {
        match self {
            EmbeddingModelType::Llama(LlamaType::Guanaco) => "Guanaco",
            EmbeddingModelType::Llama(LlamaType::Orca) => "Orca",
            EmbeddingModelType::Llama(LlamaType::Vicuna) => "Vicuna",
            EmbeddingModelType::Llama(LlamaType::Wizardlm) => "Wizardlm",
            EmbeddingModelType::Llama(LlamaType::LlamaSevenChat) => "Llama Seven Chat",
            EmbeddingModelType::Llama(LlamaType::LlamaThirteenChat) => "Llama Thirteen Chat",
            EmbeddingModelType::GptNeoX(GptNeoXType::TinyPythia) => "Tiny Pythia",
            EmbeddingModelType::GptNeoX(GptNeoXType::LargePythia) => "Large Pythia",
            EmbeddingModelType::GptNeoX(GptNeoXType::Stablelm) => "Stablelm",
            EmbeddingModelType::GptNeoX(GptNeoXType::DollySevenB) => "Dolly",
            EmbeddingModelType::Mpt(MptType::Base) => "Mpt base",
            EmbeddingModelType::Mpt(MptType::Chat) => "Mpt chat",
            EmbeddingModelType::Mpt(MptType::Story) => "Mpt story",
            EmbeddingModelType::Mpt(MptType::Instruct) => "Mpt instruct",
            EmbeddingModelType::Bert => "Bert",
        }
    }
}

fn model_type_from_str(s: &str) -> Option<ModelType> {
    match &*s.to_lowercase() {
        "guanaco" => Some(ModelType::Llama(LlamaType::Guanaco)),
        "orca" => Some(ModelType::Llama(LlamaType::Orca)),
        "vicuna" => Some(ModelType::Llama(LlamaType::Vicuna)),
        "wizardlm" => Some(ModelType::Llama(LlamaType::Wizardlm)),
        "llama seven chat" => Some(ModelType::Llama(LlamaType::LlamaSevenChat)),
        "llama thirteen chat" => Some(ModelType::Llama(LlamaType::LlamaThirteenChat)),
        "tiny pythia" => Some(ModelType::GptNeoX(GptNeoXType::TinyPythia)),
        "large pythia" => Some(ModelType::GptNeoX(GptNeoXType::LargePythia)),
        "stablelm" => Some(ModelType::GptNeoX(GptNeoXType::Stablelm)),
        "dolly" => Some(ModelType::GptNeoX(GptNeoXType::DollySevenB)),
        "mpt base" => Some(ModelType::Mpt(MptType::Base)),
        "mpt chat" => Some(ModelType::Mpt(MptType::Chat)),
        "mpt story" => Some(ModelType::Mpt(MptType::Story)),
        "mpt instruct" => Some(ModelType::Mpt(MptType::Instruct)),
        "phi" => Some(ModelType::Phi),
        "mistral" => Some(ModelType::Mistral),
        _ => None,
    }
}

fn embedding_model_type_from_str(s: &str) -> Option<EmbeddingModelType> {
    match &*s.to_lowercase() {
        "guanaco" => Some(EmbeddingModelType::Llama(LlamaType::Guanaco)),
        "orca" => Some(EmbeddingModelType::Llama(LlamaType::Orca)),
        "vicuna" => Some(EmbeddingModelType::Llama(LlamaType::Vicuna)),
        "wizardlm" => Some(EmbeddingModelType::Llama(LlamaType::Wizardlm)),
        "llama seven chat" => Some(EmbeddingModelType::Llama(LlamaType::LlamaSevenChat)),
        "llama thirteen chat" => Some(EmbeddingModelType::Llama(LlamaType::LlamaThirteenChat)),
        "tiny pythia" => Some(EmbeddingModelType::GptNeoX(GptNeoXType::TinyPythia)),
        "large pythia" => Some(EmbeddingModelType::GptNeoX(GptNeoXType::LargePythia)),
        "stablelm" => Some(EmbeddingModelType::GptNeoX(GptNeoXType::Stablelm)),
        "dolly" => Some(EmbeddingModelType::GptNeoX(GptNeoXType::DollySevenB)),
        "mpt base" => Some(EmbeddingModelType::Mpt(MptType::Base)),
        "mpt chat" => Some(EmbeddingModelType::Mpt(MptType::Chat)),
        "mpt story" => Some(EmbeddingModelType::Mpt(MptType::Story)),
        "mpt instruct" => Some(EmbeddingModelType::Mpt(MptType::Instruct)),
        "bert" => Some(EmbeddingModelType::Bert),
        _ => None,
    }
}

pub trait Colored {
    fn color(&self) -> String;
}

impl Colored for ValueType {
    fn color(&self) -> String {
        match self {
            ValueType::Single(ty) => ty.color(),
            ValueType::Many(ty) => ty.color(),
        }
    }
}

impl Colored for PrimitiveValueType {
    fn color(&self) -> String {
        let index = Self::VARIANTS.iter().position(|v| v == self).unwrap();
        let hue = index * 360 / Self::VARIANTS.len();
        format!("hsl({hue}, 100%, 50%)")
    }
}
