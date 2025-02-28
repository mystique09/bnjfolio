use leptos::prelude::*;

use crate::components::icons::Peace;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="relative h-full min-h-full w-full px-4 py-20 md:px-0 grid place-content-center">
            <div
                class="flex h-full w-full max-w-[691px] flex-col items-center justify-center gap-8 md:gap-16 lg:gap-24"
            >
                <div class="flex h-full w-full flex-col items-center justify-center gap-8">
                    <div class="flex items-center justify-center gap-4">
                        <h2 class="font-heading text-lg font-medium text-black md:text-xl lg:text-3xl">
                            "Hi, I'm Benj"
                        </h2>
                        <Peace />
                    </div>
                    <p class="font-body text-center text-4xl font-medium text-black md:text-5xl lg:text-[64px]">
                        Your Partner in Building Modern, Responsive Web Solutions.
                    </p>
                    <div
                        class="font-body mx-auto max-w-[461px] text-center text-base leading-normal font-normal text-black md:text-lg lg:text-2xl"
                    >
                        <p>
                            a <span class="text-base font-bold md:text-lg lg:text-2xl">Web Developer</span> in PH.
                        </p>
                        <p>
                            I specialize in Front-End Development and crafting Responsive Web Designs that deliver
                            seamless user experiences across all devices.
                        </p>
                    </div>
                </div>
                <a
                    href="/#projects"
                    class="font-heading rounded-full bg-black px-12 py-6 text-sm font-medium text-white uppercase hover:scale-98 hover:transition-all lg:text-base"
                    >
                    Browse my Projects
                </a>
            </div>
        </section>
    }
}
