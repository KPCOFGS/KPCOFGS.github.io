use dioxus::prelude::*;

// Assets
const FAVICON: &str = "/assets/favicon.ico";
const HEADER_SVG: &str = "/assets/header.svg";
static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Document head links
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: CSS }

        // Hero section
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    let skills = [
        "Rust",
        "Python",
        "Linux",
        "Cybersecurity",
        "React",
        "AI",
        "TensorFlow",
        "PyTorch",
        "C/C++",
    ];
    // Define technologies for each project
    let yolo_skills = ["Python", "OpenCV", "YOLOv5", "TensorFlow", "PyTorch"];
    let labins_skills = [
        "JavaScript",
        "ArcGIS Experience Builder",
        "HTML/CSS",
        "ESRI APIs",
    ];
    let portfolio_skills = ["Rust", "Dioxus", "WebAssembly", "HTML/CSS", "JS (optional)"];
    rsx! {
        div { class: "page",
            header { class: "hero",
                h1 { "Shixian Sheng" }
                p { "Software Engineer | AI Developer | Full-Stack Developer | Researcher" }
                div { class: "button-group",
                    a { class: "btn", href: "https://github.com/KPCOFGS", "GitHub" }
                    a { class: "btn", href: "https://linkedin.com/in/shixian-sheng", "LinkedIn" }
                    a { class: "btn", href: "https://dev.to/kpcofgs", "Dev.to" }
                }
            }

            main { class: "container",
                section { class: "card",
                    h2 { class: "section-title", "About Me" }
                    p {
                        "Computer Science graduate at Florida State University with experience in environmental data research, web development, and machine learning systems. Passionate about Linux, Rust, cybersecurity, and AI-driven automation."
                    }
                }

                section { class: "card",
                    h2 { class: "section-title", "Education" }
                    strong { "Florida State University" }
                    p { "Bachelor of Science in Computer Science (2022 - 2025)" }
                    p { "GPA: 3.5 | Honors Graduate" }
                }

                section { class: "card",
                    h2 { class: "section-title", "Experience" }

                    // Researcher
                    div { class: "experience-item",
                        strong { "Researcher" }
                        br {}
                        span { class: "company", "Florida Resources and Environmental Analysis Center" }
                        br {}
                        span { class: "duration-location", "Dec 2023 – Dec 2024 | Tallahassee, Florida, United States" }
                        ul {
                            li { "Conducted detailed quantitative analysis of environmental data, ensuring accuracy in data research, collection, and report preparation for multiple projects." }
                            li { "Collaborated with a multidisciplinary team to identify user needs and implemented features to improve the website’s functionality, increasing accessibility to critical environmental data." }
                            li { "Designed and developed a dynamic website for hydrogeologists, using ArcGIS/ESRI tools to process, map, and integrate spatial data, enabling efficient search, visualization, and analysis for enhanced decision-making." }
                        }
                    }
                    // Computer Lab Assistant
                    div { class: "experience-item",
                        strong { "Computer Lab Assistant" }
                        br {}
                        span { class: "company", "Florida State University" }
                        br {}
                        span { class: "duration-location", "Feb 2025 – May 2025 | Tallahassee, Florida, United States" }
                        ul {
                            li { "Monitored and maintained daily operations of the computer lab to ensure smooth functionality and minimal downtime." }
                            li { "Assisted students and faculty with computer-related issues, including troubleshooting software and printing." }
                        }
                    }
                }




                section { class: "card",
                    h2 { class: "section-title", "Projects" }

                    // YOLO Object Detection
                    div { class: "project-card",
                        a { href: "https://github.com/KPCOFGS/Object_Detection", class: "project-title link", "YOLO Object Detection" }
                        p { "Engineered a real-time object detection pipeline using the YOLO model to analyze live video streams instantly." }
                        p { "Implemented bounding box overlays, confidence-based labeling, and frame-by-frame tracking for clear situational awareness." }
                        div { class: "skills",
                            { yolo_skills.iter().map(|skill| rsx! { span { class: "badge", "{skill}" } }) }
                        }
                        hr {}
                    }
                    br {}
                    br {}
                    // LABINS Mapping Platform
                    div { class: "project-card",
                        a { href: "https://www.labins.org/MAP_EXB/", class: "project-title link", "LABINS Mapping Platform" }
                        p { "Developed an interactive web platform for hydrogeologists to search, visualize, and download environmental data efficiently." }
                        p { "Integrated spatial data using ArcGIS Experience Builder and ESRI tools, allowing dynamic querying and mapping." }
                        p { "Enhanced user experience by implementing search filters, map overlays, and responsive design for field and office use." }
                        div { class: "skills",
                            { labins_skills.iter().map(|skill| rsx! { span { class: "badge", "{skill}" } }) }
                        }
                        hr {}
                    }
                    br {}
                    br {}
                    // Personal Portfolio Website
                    div { class: "project-card",
                        a { href: "https://github.com/KPCOFGS/KPCOFGS.github.io", class: "project-title link", "Personal Portfolio Website" }
                        p { "Designed and implemented this personal portfolio website using Rust and Dioxus for WebAssembly." }
                        p { "Integrated responsive layout, dark theme, custom CSS styling, and self-hosted JetBrains Mono Nerd Font." }
                        p { "Highlights experience in full-stack development, WASM web deployment, and modern UI design principles." }
                        div { class: "skills",
                            { portfolio_skills.iter().map(|skill| rsx! { span { class: "badge", "{skill}" } }) }
                        }
                    }
                }



                section { class: "card",
                    h2 { class: "section-title", "Skills" }
                    div { class: "skills",
                        { skills.iter().map(|skill| rsx! { span { class: "badge", "{skill}" } }) }
                    }
                }

                section { class: "card contact-card",
                    h2 { class: "section-title", "Contact" }
                    div { class: "contact-item",
                        span { class: "contact-label", "📧 Email:" }
                        a { class: "contact-link", href: "mailto:shixian_sheng-2@protonmail.com", "shixian_sheng-2@protonmail.com" }
                    }
                    div { class: "contact-item",
                        span { class: "contact-label", "📞 Phone:" }
                        a { class: "contact-link", href: "tel:+18137230980", "813-723-0980" }
                    }
                    div { class: "contact-item",
                        span { class: "contact-label", "📍 Location:" }
                        a { class: "contact-link", href: "https://www.google.com/maps/place/Miami,+FL", target: "_blank", "Miami, Florida" }
                    }
                }
            }

            footer { class: "footer",
                "© 2025 Shixian Sheng | Built with Dioxus"
            }
        }
    }
}
