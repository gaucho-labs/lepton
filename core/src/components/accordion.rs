use leptix_primitives::{components::accordion, Attributes};
use leptos::{html::AnyElement, *};
use tailwind_fuse::*;

use crate::utils::merge_class_attribute;

pub use accordion::AccordionKind;
pub use accordion::AccordionRoot as Accordion;

#[component]
pub fn AccordionItem(
    #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(attrs)] mut attrs: Attributes,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    children: Children,
) -> impl IntoView {
    merge_class_attribute(&mut attrs, "border-b", class);

    accordion::AccordionItem(accordion::AccordionItemProps {
        disabled,
        value,
        attrs,
        node_ref,
        children,
    })
}

#[component]
pub fn AccordionTrigger(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(attrs)] mut attrs: Attributes,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    children: ChildrenFn,
) -> impl IntoView {
    use accordion::AccordionHeader as BaseAccordionHeader;

    let children = Box::new(move || {
        view! {
            {children}
            <svg
                width="15"
                height="15"
                viewBox="0 0 15 15"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                class="h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200"
            >
                <path
                    d="M3.13523 6.15803C3.3241 5.95657 3.64052 5.94637 3.84197 6.13523L7.5 9.56464L11.158 6.13523C11.3595 5.94637 11.6759 5.95657 11.8648 6.15803C12.0536 6.35949 12.0434 6.67591 11.842 6.86477L7.84197 10.6148C7.64964 10.7951 7.35036 10.7951 7.15803 10.6148L3.15803 6.86477C2.95657 6.67591 2.94637 6.35949 3.13523 6.15803Z"
                    fill="currentColor"
                    fill-rule="evenodd"
                    clip-rule="evenodd"
                ></path>
            </svg>
        }.into()
    });

    merge_class_attribute(&mut attrs, "flex flex-1 items-center justify-between py-4 text-sm font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180", class);

    let trigger = accordion::AccordionTrigger(accordion::AccordionTriggerProps {
        attrs,
        node_ref,
        children,
    });

    view! { <BaseAccordionHeader attr:class="flex">{trigger}</BaseAccordionHeader> }
}

// TODO: this behavior of applying attrs and class to different components is a little weird.
#[component]
pub fn AccordionContent(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Attributes,
    children: ChildrenFn,
) -> impl IntoView {
    use accordion::AccordionContent as BaseAccordionContent;

    let class = create_memo(move |_| tw_merge!("pb-4 pt-0", class.get()));

    view! {
        <BaseAccordionContent
            attrs=attrs
            attr:class="overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down"
        >
            <div attr:class=class>{children()}</div>
        </BaseAccordionContent>
    }
}
