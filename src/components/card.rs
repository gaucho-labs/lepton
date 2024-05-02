use leptix_primitives::Attributes;
use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Card(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Attributes,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        class.with(|class| {
            tw_merge!(
                "rounded-xl border bg-card text-card-foreground shadow",
                class
            )
        })
    });

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Attributes,
    children: Children,
) -> impl IntoView {
    let class =
        create_memo(move |_| class.with(|class| tw_merge!("flex flex-col space-y-1.5 p-6", class)));

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Attributes,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        class.with(|class| tw_merge!("font-semibold leading-none tracking-tight", class))
    });

    view! {
        <h3 {..attrs} class=class>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Attributes,
    children: Children,
) -> impl IntoView {
    let class =
        create_memo(move |_| class.with(|class| tw_merge!("text-sm text-muted-foreground", class)));

    view! {
        <p {..attrs} class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Attributes,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| class.with(|class| tw_merge!("p-6 pt-0", class)));

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Attributes,
    children: Children,
) -> impl IntoView {
    let class =
        create_memo(move |_| class.with(|class| tw_merge!("flex items-center p-6 pt-0", class)));

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}
