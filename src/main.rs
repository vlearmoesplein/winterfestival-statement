use std::borrow::Cow;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Paragraph {
    text: Cow<'static, str>,
}

#[function_component(App)]
fn app() -> Html {
    let paragraphs = [
        Paragraph {
            text: Cow::Borrowed(
                "Met een dubbel gevoel delen wij dit bericht. Het Winterfestival op het Vlearmoesplein is voor ons altijd een hoogtepunt van het jaar geweest. Drie jaar lang hebben we met veel plezier en toewijding dit evenement in Neede mogen organiseren.",
            ),
        },
        Paragraph {
            text: Cow::Borrowed(
                "Helaas hebben wij moeten besluiten te stoppen met de organisatie van het Winterfestival.",
            ),
        },
        Paragraph {
            text: Cow::Borrowed(
                "Dit is geen overhaaste beslissing geweest. Het tekort aan vrijwilligers en de stijgende kosten maken het steeds moeilijker om een evenement van deze omvang te realiseren. Daardoor is het voor ons niet langer haalbaar om het festival voort te zetten zoals jullie dat van ons gewend zijn.",
            ),
        },
        Paragraph {
            text: Cow::Borrowed(
                "We willen deze gelegenheid aangrijpen om iedereen die ons de afgelopen jaren heeft geholpen, enorm te bedanken.",
            ),
        },
        Paragraph {
            text: Cow::Borrowed(
                "Stichting Vlearmoesplein blijft bestaan. Ook dit jaar zijn we weer te vinden op de kerstmarkt en blijven we, in samenwerking met Livio, de knutselavonden organiseren. Op deze manier blijven we ons als groep jongeren inzetten voor Neede.",
            ),
        },
        Paragraph {
            text: Cow::Borrowed("Nogmaals dank voor alles."),
        },
    ];

    html! {
        <main
            class="min-h-screen bg-base-200 bg-cover bg-center bg-no-repeat p-4 md:p-8"
            style="background-image: url('assets/background-winter.png');"
        >
            <div class="mx-auto flex min-h-[calc(100vh-2rem)] w-full max-w-4xl items-center md:min-h-[calc(100vh-4rem)]">
                <section class="card w-full border border-base-300/70 bg-base-100/92 shadow-2xl">
                    <div class="card-body gap-5 p-6 md:p-10">
                        <div class="space-y-6">
                            <figure class="mx-auto w-full max-w-md">
                                <img
                                    class="h-auto w-full mix-blend-screen"
                                    src="assets/logo-winterfestival.png"
                                    alt="Winterfestival Neede logo"
                                />
                            </figure>
                            <div class="w-full border-t border-base-300/80"></div>
                            <h1 class="text-3xl font-bold leading-tight text-base-content md:text-4xl">
                                { "Verklaring over het Winterfestival" }
                            </h1>
                        </div>

                        <p class="text-lg font-semibold text-base-content">
                            { "Beste vrijwilligers, bezoekers, sponsoren en betrokkenen," }
                        </p>

                        <div class="space-y-4 text-base leading-relaxed text-base-content/90">
                            {
                                paragraphs
                                    .iter()
                                    .map(|paragraph| html! { <p>{ paragraph.text.as_ref() }</p> })
                                    .collect::<Html>()
                            }
                        </div>

                        <div class="mt-3 border-t border-base-300/80 pt-5 text-base-content">
                            <p class="font-medium">{ "Met vriendelijke groet," }</p>
                            <p class="mt-1 font-bold">{ "Het bestuur van Stichting Vlearmoesplein" }</p>
                        </div>
                    </div>
                </section>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
