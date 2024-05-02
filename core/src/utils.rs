use leptix_primitives::Attributes;
use leptos::*;
use tailwind_fuse::*;

#[derive(Debug)]
pub enum ClassAttributeEntry<'a> {
    Exists {
        attr: &'a mut Attribute,
        value: MaybeSignal<Oco<'static, str>>,
    },
    NonString {
        attr: &'a mut Attribute,
    },
    Empty,
}

pub fn merge_class_attribute(
    attrs: &mut Attributes,
    base_class: &'static str,
    override_class: MaybeSignal<String>,
) {
    match extract_class_from_attributes(attrs) {
        ClassAttributeEntry::Exists { attr, value } => {
            let class = create_memo(move |_| {
                let attr_class = value.get();
                tw_merge!(base_class, attr_class.as_str(), override_class.get())
            });
            let class = class.into_attribute();
            *attr = class;
        }
        ClassAttributeEntry::NonString { attr } => {
            let class = create_memo(move |_| tw_merge!(base_class, override_class.get()));
            let class = class.into_attribute();
            *attr = class;
        }
        ClassAttributeEntry::Empty => {
            let class = create_memo(move |_| tw_merge!(base_class, override_class.get()));
            let class = class.into_attribute();
            attrs.push(("class", class));
        }
    }
}

pub fn extract_class_from_attributes(attrs: &mut Attributes) -> ClassAttributeEntry {
    let maybe_attr = attrs
        .iter_mut()
        .find(|(k, _)| *k == "class")
        .map(|(_, attr)| attr);

    match maybe_attr {
        Some(attr) => {
            match attr {
                Attribute::String(s) | Attribute::Option(Some(s)) => {
                    let value = MaybeSignal::Static(s.clone());
                    ClassAttributeEntry::Exists { attr, value }
                }
                Attribute::Fn(func) => {
                    let func = func.clone();
                    // TODO: Test that this still maintains reactivity.
                    let derived = Signal::derive(move || {
                        let mut value = func();
                        while let Attribute::Fn(func) = value {
                            value = func();
                        }
                        match value {
                            Attribute::String(s) | Attribute::Option(Some(s)) => s,
                            Attribute::Bool(_) => "".into(),
                            Attribute::Option(None) => "".into(),
                            Attribute::Fn(_) => unreachable!("Fn should be unwrapped above."),
                        }
                    });
                    ClassAttributeEntry::Exists {
                        attr,
                        value: MaybeSignal::Dynamic(derived),
                    }
                }
                Attribute::Option(None) => ClassAttributeEntry::NonString { attr },
                Attribute::Bool(_) => ClassAttributeEntry::NonString { attr },
            }
        }
        None => ClassAttributeEntry::Empty,
    }
}
