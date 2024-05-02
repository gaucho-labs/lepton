use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/demo.css"/>

        // sets the document title
        <Title text="Welcome to Lepton"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

use lepton::components::{accordion::*, badge::*, button::*, card::*, checkbox::*};

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-start gap-8 p-4">
            <Section title="Button">
                <Button>Default</Button>
                <Button variant=ButtonType::Destructive>Destructive</Button>
                <Button variant=ButtonType::Secondary>Secondary</Button>
                <Button variant=ButtonType::Outline>Outline</Button>
                <Button variant=ButtonType::Ghost>Ghost</Button>
                <Button variant=ButtonType::Link>Link</Button>
            </Section>

            <Section title="Badge">
                <Badge>Default</Badge>
                <Badge variant=BadgeType::Secondary>Secondary</Badge>
                <Badge variant=BadgeType::Destructive>Destructive</Badge>
                <Badge variant=BadgeType::Outline>Outline</Badge>
            </Section>

            <Section title="Accordion">
                <Accordion
                    kind=AccordionKind::Single {
                        value: None,
                        default_value: None,
                        on_value_change: None,
                        collapsible: Some(true.into()),
                    }
                    attr:class="w-full"
                >
                    <AccordionItem value="item-1">
                        <AccordionTrigger>Is it accessible?</AccordionTrigger>
                        <AccordionContent>
                            Yes. It adheres to the WAI-ARIA design pattern.
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="item-2">
                        <AccordionTrigger>Is it styled?</AccordionTrigger>
                        <AccordionContent>
                            "Yes. It comes with default styles that matches the other
                            components' aesthetic."
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="item-3">
                        <AccordionTrigger>Is it animated?</AccordionTrigger>
                        <AccordionContent>
                            {"Yes. It's animated by default, but you can disable it if you prefer."}
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            </Section>

            <Section title="Checkbox">
                    <div></div>
                // <Checkbox
                //     default_checked=CheckedState::Checked(true).into()
                // ></Checkbox>
            </Section>

            <Section title="Card">
                <Card>
                    <CardHeader>
                        <CardTitle>Card Title</CardTitle>
                        <CardDescription>Card Description</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p>Card Content</p>
                    </CardContent>
                    <CardFooter>
                        <p>Card Footer</p>
                    </CardFooter>
                </Card>
            </Section>

        </div>
    }
}

#[component]
fn Section(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col items-start gap-4 p-4">
            <h2 class="text-xl font-semibold">{title}</h2>
            <div class="flex flex-col items-start gap-2">{children()}</div>
        </div>
    }
}
