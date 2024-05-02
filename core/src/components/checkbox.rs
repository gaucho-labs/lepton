use leptix_primitives::{
    components::checkbox,
    components::checkbox::{CheckboxIndicator, CheckboxRoot, CheckboxRootProps},
    Attributes,
};
use leptos::{
    ev::{KeyboardEvent, MouseEvent},
    html::AnyElement,
    *,
};

use crate::utils::merge_class_attribute;

pub use checkbox::CheckedState;

#[component]
pub fn Checkbox(
    #[prop(optional)] as_child: Option<bool>,
    #[prop(optional)] required: Option<MaybeSignal<bool>>,
    #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
    #[prop(optional)] checked: Option<MaybeSignal<CheckedState>>,
    #[prop(optional)] default_checked: Option<MaybeSignal<CheckedState>>,
    #[prop(optional)] on_checked_change: Option<Callback<CheckedState>>,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(optional)] on_key_down: Option<Callback<KeyboardEvent>>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] mut attrs: Attributes,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
    // TODO: Fix panic in browser.
    let indicator = Box::new(move || {
        view! {
            <CheckboxIndicator attr:class="flex items-center justify-center text-current">
                <svg
                    width="15"
                    height="15"
                    viewBox="0 0 15 15"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4"
                >
                    <path
                        d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z"
                        fill="currentColor"
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            </CheckboxIndicator>
        }.into()
    });

    merge_class_attribute(
        &mut attrs,
         "peer h-4 w-4 shrink-0 rounded-sm border border-primary shadow focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground",
        class
    );

    let root_props = CheckboxRootProps {
        as_child,
        required,
        disabled,
        checked,
        default_checked,
        on_checked_change,
        on_click,
        on_key_down,
        attrs,
        node_ref,
        children: indicator,
    };

    CheckboxRoot(root_props)
}
