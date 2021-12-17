use crate::{base_component::BaseComponent, component::Component};

#[derive(Clone, Debug)]
pub enum StringOrComponent {
    String(String),
    Component(Component),
}

#[derive(Clone, Debug)]
pub struct TranslatableComponent {
    pub base: BaseComponent,
    pub key: String,
    pub args: Vec<StringOrComponent>,
}

impl TranslatableComponent {
    pub fn new(key: String, args: Vec<StringOrComponent>) -> Self {
        Self {
            base: BaseComponent::new(),
            key,
            args,
        }
    }
}