use leptix_primitives::Attributes;
use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Badge(
    #[prop(into, optional)] variant: MaybeSignal<BadgeType>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Attributes,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let badge = BadgeClass {
            variant: variant.get(),
        };

        class.with(|class| badge.with_class(class))
    });

    view! {
        <span {..attrs} class=class>
            {children()}
        </span>
    }
}

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
)]
pub struct BadgeClass {
    pub variant: BadgeType,
}

#[derive(TwVariant)]
pub enum BadgeType {
    #[tw(
        default,
        class = "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80"
    )]
    Default,
    #[tw(
        class = "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80"
    )]
    Secondary,
    #[tw(
        class = "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80"
    )]
    Destructive,
    #[tw(class = "text-foreground")]
    Outline,
}
