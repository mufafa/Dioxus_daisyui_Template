use dioxus::prelude::*;

#[component]
pub fn Preview() -> Element {
    rsx! {

        div { class: "rounded-box bg-base-100 border-base-content/5 text-base-content not-prose grid gap-3 border p-6 max-w-2xl mx-auto my-6",
        div { class: "grid grid-cols-2 gap-2 md:grid-cols-4",
            button { class: "btn", "Default" }
            button { class: "btn btn-primary", "Primary" }
            button { class: "btn btn-secondary", "Secondary" }
            button { class: "btn btn-accent", "Accent" }
            button { class: "btn btn-info", "Info" }
            button { class: "btn btn-success", "Success" }
            button { class: "btn btn-warning", "Warning" }
            button { class: "btn btn-error", "Error" }
        }
        div { class: "grid grid-cols-2 place-items-center gap-2 md:grid-cols-4",
            span { class: "badge", "Default" }
            span { class: "badge badge-primary", "Primary" }
            span { class: "badge badge-secondary", "Secondary" }
            span { class: "badge badge-accent", "Accent" }
            span { class: "badge badge-info", "Info" }
            span { class: "badge badge-success", "Success" }
            span { class: "badge badge-warning", "Warning" }
            span { class: "badge badge-error", "Error" }
        }
        div { class: "flex flex-col gap-3",
            div { class: "flex flex-col gap-3 md:flex-row",
                div { class: "md:w-1/2",
                    div { class: "tabs tabs-lifted",
                        button { class: "tab", "Tab" }
                        button { class: "tab tab-active", "Tab" }
                        button { class: "tab", "Tab" }
                    }
                    div { class: "flex flex-col",
                        span { class: "link", "I'm a simple link" }
                        span { class: "link link-primary", "I'm a simple link" }
                        span { class: "link link-secondary", "I'm a simple link" }
                        span { class: "link link-accent", "I'm a simple link" }
                    }
                }
                div { class: "flex flex-col gap-3 md:w-1/2",
                    progress { max: "100", value: "20", class: "progress", "Default" }
                    progress {
                        max: "100",
                        value: "25",
                        class: "progress progress-primary",
                        "Primary"
                    }
                    progress {
                        max: "100",
                        value: "30",
                        class: "progress progress-secondary",
                        "Secondary"
                    }
                    progress {
                        max: "100",
                        value: "40",
                        class: "progress progress-accent",
                        "Accent"
                    }
                    progress {
                        max: "100",
                        value: "45",
                        class: "progress progress-info",
                        "Info"
                    }
                    progress {
                        value: "55",
                        max: "100",
                        class: "progress progress-success",
                        "Success"
                    }
                    progress {
                        max: "100",
                        value: "70",
                        class: "progress progress-warning",
                        "Warning"
                    }
                    progress {
                        max: "100",
                        value: "90",
                        class: "progress progress-error",
                        "Error"
                    }
                }
            }
            div { class: "flex flex-col gap-3 md:flex-row",
                div { class: "stats bg-base-300 border-base-300 border md:w-1/2",
                    div { class: "stat",
                        div { class: "stat-title", "Total Page Views" }
                        div { class: "stat-value", "89,400" }
                        div { class: "stat-desc", "21% more than last month" }
                    }
                }
                div { class: "flex flex-wrap items-center justify-center gap-3 md:w-1/2",
                    div {
                        style: "--value:60; --size:3.5rem;",
                        class: "radial-progress",
                        "60%"
                    }
                    div {
                        style: "--value:75; --size:3.5rem;",
                        class: "radial-progress",
                        "75%"
                    }
                    div {
                        style: "--value:90; --size:3.5rem;",
                        class: "radial-progress",
                        "90%"
                    }
                }
            }
        }
        div { class: "flex flex-col gap-3",
            div { class: "flex flex-col gap-3 md:flex-row",
                div { class: "md:w-1/2",
                    div {
                        input { r#type: "checkbox", class: "toggle" }
                        input {
                            r#type: "checkbox",
                            class: "toggle toggle-primary",
                        }
                        input {
                            r#type: "checkbox",
                            class: "toggle toggle-secondary",
                        }
                        input { r#type: "checkbox", class: "toggle toggle-accent" }
                    }
                    div {
                        input { r#type: "checkbox", class: "checkbox" }
                        input {
                            r#type: "checkbox",
                            class: "checkbox checkbox-primary",
                        }
                        input {
                            r#type: "checkbox",
                            class: "checkbox checkbox-secondary",
                        }
                        input {
                            r#type: "checkbox",
                            class: "checkbox checkbox-accent",
                        }
                    }
                    div {
                        input {
                            r#type: "radio",
                            name: "radio-1",
                            class: "radio",
                        }
                        input {
                            r#type: "radio",
                            name: "radio-1",
                            class: "radio radio-primary",
                        }
                        input {
                            r#type: "radio",
                            name: "radio-1",
                            class: "radio radio-secondary",
                        }
                        input {
                            name: "radio-1",
                            r#type: "radio",
                            class: "radio radio-accent",
                        }
                    }
                }
                div { class: "md:w-1/2",
                    input {
                        min: "0",
                        r#type: "range",
                        max: "100",
                        class: "range range-xs",
                    }
                    input {
                        r#type: "range",
                        max: "100",
                        min: "0",
                        class: "range range-xs range-primary",
                    }
                    input {
                        max: "100",
                        r#type: "range",
                        min: "0",
                        class: "range range-xs range-secondary",
                    }
                    input {
                        min: "0",
                        r#type: "range",
                        max: "100",
                        class: "range range-xs range-accent",
                    }
                }
            }
            div { class: "flex flex-col gap-3 md:flex-row",
                div { class: "flex flex-col gap-3 md:w-1/2",
                    input {
                        placeholder: "Default",
                        r#type: "text",
                        class: "input input-bordered w-full",
                    }
                    input {
                        r#type: "text",
                        placeholder: "Primary",
                        class: "input input-primary input-bordered w-full",
                    }
                    input {
                        placeholder: "Secondary",
                        r#type: "text",
                        class: "input input-secondary input-bordered w-full",
                    }
                    input {
                        placeholder: "Accent",
                        r#type: "text",
                        class: "input input-accent input-bordered w-full",
                    }
                }
                div { class: "flex flex-col gap-3 md:w-1/2",
                    input {
                        r#type: "text",
                        placeholder: "Info",
                        class: "input input-info input-bordered w-full",
                    }
                    input {
                        r#type: "text",
                        placeholder: "Success",
                        class: "input input-success input-bordered w-full",
                    }
                    input {
                        placeholder: "Warning",
                        r#type: "text",
                        class: "input input-warning input-bordered w-full",
                    }
                    input {
                        placeholder: "Error",
                        r#type: "text",
                        class: "input input-error input-bordered w-full",
                    }
                }
            }
            div { class: "navbar bg-neutral text-neutral-content rounded-box",
                div { class: "flex-none",
                    button { class: "btn btn-square btn-ghost",
                        svg {
                            "viewBox": "0 0 24 24",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "inline-block h-5 w-5 stroke-current",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M4 6h16M4 12h16M4 18h16",
                                "stroke-width": "2",
                            }
                        }
                    }
                }
                div { class: "flex-1",
                    button { class: "btn btn-ghost text-xl normal-case", "daisyUI" }
                }
            }
            div { class: "flex gap-3",
                div { class: "flex flex-grow flex-col gap-3",
                    div { class: "text-4xl font-bold", "Text Size 1" }
                    div { class: "text-3xl font-bold", "Text Size 2" }
                    div { class: "text-2xl font-bold", "Text Size 3" }
                    div { class: "text-xl font-bold", "Text Size 4" }
                    div { class: "text-lg font-bold", "Text Size 5" }
                    div { class: "text-sm font-bold", "Text Size 6" }
                    div { class: "text-xs font-bold", "Text Size 7" }
                }
                ul { class: "steps steps-vertical",
                    li { class: "step step-primary", "Step 1" }
                    li { class: "step step-primary", "Step 2" }
                    li { class: "step", "Step 3" }
                    li { class: "step", "Step 4" }
                }
            }
        }
        div { class: "flex flex-col gap-3",
            div { class: "alert",
                div {
                    svg {
                        "viewBox": "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        class: "stroke-info h-6 w-6 flex-shrink-0",
                        path {
                            "stroke-linejoin": "round",
                            "stroke-linecap": "round",
                            "stroke-width": "2",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                        }
                    }
                    span { "12 unread messages. Tap to see." }
                }
            }
            div { class: "alert alert-info",
                div {
                    svg {
                        "viewBox": "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        class: "h-6 w-6 flex-shrink-0 stroke-current",
                        path {
                            "stroke-linejoin": "round",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                            "stroke-linecap": "round",
                            "stroke-width": "2",
                        }
                    }
                    span { "New software update available." }
                }
            }
            div { class: "alert alert-success",
                div {
                    svg {
                        "viewBox": "0 0 24 24",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-6 w-6 flex-shrink-0 stroke-current",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
                        }
                    }
                    span { "Your purchase has been confirmed!" }
                }
            }
            div { class: "alert alert-warning",
                div {
                    svg {
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-6 w-6 flex-shrink-0 stroke-current",
                        path {
                            "stroke-linecap": "round",
                            d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
                            "stroke-width": "2",
                            "stroke-linejoin": "round",
                        }
                    }
                    span { "Warning: Invalid email address!" }
                }
            }
            div { class: "alert alert-error",
                div {
                    svg {
                        "viewBox": "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        class: "h-6 w-6 flex-shrink-0 stroke-current",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                            "stroke-width": "2",
                        }
                    }
                    span { "Error! Task failed successfully." }
                }
            }
        }
    }

    }
}