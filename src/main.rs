#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus::document::eval;

fn main() {
    launch(App);
}

#[derive(PartialEq, Clone, Copy)]
enum Tab { Vvx, Varmvatten, Rum }

#[derive(Clone, PartialEq)]
struct Punkt {
    namn: String,
    temp: f64,
}

pub fn App() -> Element {
    let mut current_tab = use_signal(|| Tab::Vvx);
    
    // --- STATE ---
    let mut ute = use_signal(|| 0.0f64);
    let mut fran = use_signal(|| 21.0f64);
    let mut vvx_mät = use_signal(|| 16.0f64);
    let mut vvx_mode = use_signal(|| true); 
    
    let mut vv_lista = use_signal(|| vec![
        Punkt { namn: "Kök".into(), temp: 55.0 },
        Punkt { namn: "Bad 1".into(), temp: 53.0 },
        Punkt { namn: "Bad 2".into(), temp: 51.0 }
    ]);

    let mut rums_lista = use_signal(|| vec![
        Punkt { namn: "Vardagsrum".into(), temp: 21.0 },
        Punkt { namn: "Kök".into(), temp: 20.5 }
    ]);

    // --- LOGIK ---
    let n_procent = {
        let n = fran() - ute();
        if n != 0.0 {
            let res = if vvx_mode() { (vvx_mät() - ute()) / n } else { (fran() - vvx_mät()) / n };
            (res * 100.0).max(0.0)
        } else { 0.0 }
    };

    let vv_text = {
        let data = vv_lista.read();
        let detaljer = data.iter()
            .map(|p| format!("{}: {}°C", p.namn, p.temp))
            .collect::<Vec<_>>().join(", ");
        format!("Varmvattenkontroll: {}.", detaljer)
    };

    rsx! {
        style { "
            body {{ background: #000000; color: #eee; font-family: sans-serif; display: flex; justify-content: center; padding: 10px; }}
            .card {{ background: #1a1a1a; width: 100%; max-width: 420px; padding: 25px; border-radius: 12px; border: 2px solid #333; box-shadow: 0 4px 20px rgba(211, 47, 47, 0.4); }}
            .tabs {{ display: flex; gap: 5px; margin-bottom: 25px; }}
            .tab-btn {{ flex: 1; padding: 12px; background: #333; border: none; color: #ffb300; cursor: pointer; border-radius: 6px; font-weight: bold; border-bottom: 3px solid transparent; transition: 0.2s; }}
            .tab-btn.active {{ background: #d32f2f; color: #ffffff; border-bottom: 3px solid #ffb300; }}
            label {{ font-size: 11px; color: #ffb300; font-weight: bold; text-transform: uppercase; letter-spacing: 1px; margin-bottom: 4px; display: block; }}
            input {{ width: 100%; padding: 12px; margin-bottom: 12px; border-radius: 6px; border: 1px solid #444; background: #000; color: white; box-sizing: border-box; font-size: 16px; }}
            .copy-box {{ background: #000; padding: 15px; border-radius: 6px; border-left: 4px solid #d32f2f; margin-top: 20px; position: relative; border-right: 1px solid #222; }}
            .copy-btn {{ background: #ffb300; color: #000; border: none; padding: 6px 12px; border-radius: 4px; font-size: 11px; cursor: pointer; float: right; font-weight: bold; text-transform: uppercase; }}
            .add-btn {{ background: #d32f2f; color: white; padding: 12px; border: none; border-radius: 6px; width: 100%; cursor: pointer; margin-top: 10px; font-weight: bold; border: 1px solid #ffb300; text-transform: uppercase; }}
            .status-dot {{ width: 12px; height: 12px; border-radius: 50%; flex-shrink: 0; }}
            .row {{ display: flex; align-items: center; gap: 10px; margin-bottom: 10px; background: #262626; padding: 12px; border-radius: 8px; border: 1px solid #333; }}
            .footer {{ margin-top: 40px; padding-top: 20px; border-top: 1px dashed #444; text-align: center; font-size: 10px; color: #666; font-family: monospace; line-height: 1.5; }}
        " }

        div { class: "card",
            h1 { style: "text-align: center; color: #d32f2f; margin-bottom: 5px; letter-spacing: 2px; text-shadow: 1px 1px #000;", "REALSTATE MULTITOOL" }
            p { style: "text-align: center; font-size: 10px; color: #ffb300; margin-bottom: 25px; font-weight: bold;", "LULEÅ HF EDITION" }
            
            div { class: "tabs",
                button { class: if current_tab() == Tab::Vvx { "tab-btn active" } else { "tab-btn" }, onclick: move |_| current_tab.set(Tab::Vvx), "VVX" }
                button { class: if current_tab() == Tab::Varmvatten { "tab-btn active" } else { "tab-btn" }, onclick: move |_| current_tab.set(Tab::Varmvatten), "V-VATTEN" }
                button { class: if current_tab() == Tab::Rum { "tab-btn active" } else { "tab-btn" }, onclick: move |_| current_tab.set(Tab::Rum), "RUM" }
            }

            match current_tab() {
                Tab::Vvx => rsx! {
                    div {
                        button { class: "add-btn", style: "margin-bottom: 20px; background: #222;", onclick: move |_| vvx_mode.toggle(), if vvx_mode() { "MODE: SUPPLY AIR EFFICIENCY" } else { "MODE: EXHAUST AIR EFFICIENCY" } }
                        label { "Outdoor Temp (°C)" }
                        input { r#type: "number", value: "{ute}", oninput: move |e| ute.set(e.value().parse().unwrap_or(0.0)) }
                        label { "Exhaust Temp (°C)" }
                        input { r#type: "number", value: "{fran}", oninput: move |e| fran.set(e.value().parse().unwrap_or(0.0)) }
                        label { if vvx_mode() { "Supply Air Temp (°C)" } else { "Waste Air Temp (°C)" } }
                        input { r#type: "number", value: "{vvx_mät}", oninput: move |e| vvx_mät.set(e.value().parse().unwrap_or(0.0)) }
                        div { style: "text-align: center; padding: 25px; background: #000; border-radius: 8px; border: 2px solid #d32f2f;",
                            h2 { style: "font-size: 48px; margin: 0; color: #ffb300;", "{n_procent:.1}%" }
                            span { style: "color: #888; font-size: 12px; font-weight: bold; text-transform: uppercase;", "Thermal Efficiency" }
                        }
                    }
                },
                Tab::Varmvatten => rsx! {
                    div {
                        for (i, p) in vv_lista.read().iter().enumerate() {
                            {
                                let dot_color = if p.temp < 50.0 { "#f44336" } else { "#4caf50" };
                                rsx! {
                                    div { class: "row",
                                        div { class: "status-dot", style: "background: {dot_color}" }
                                        input { 
                                            style: "flex: 2; border: none; background: transparent; color: white; font-weight: bold;",
                                            value: "{p.namn}",
                                            oninput: move |e| vv_lista.with_mut(|l| l[i].namn = e.value())
                                        }
                                        input { 
                                            style: "flex: 1; text-align: center; margin: 0; color: #ffb300; font-weight: bold;",
                                            r#type: "number",
                                            value: "{p.temp}",
                                            oninput: move |e| vv_lista.with_mut(|l| l[i].temp = e.value().parse().unwrap_or(0.0))
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "copy-box",
                            button { 
                                class: "copy-btn", 
                                onclick: move |_| { eval(&format!("navigator.clipboard.writeText('{}')", vv_text.clone())); }, 
                                "COPY" 
                            }
                            div { style: "color: #ffb300; font-size: 10px; font-weight: bold; margin-bottom: 5px;", "CHECKLIST COMMENT" }
                            p { style: "font-size: 14px; margin: 0; padding-right: 65px; color: #eee; line-height: 1.4;", "{vv_text}" }
                        }
                    }
                },
                Tab::Rum => rsx! {
                    div {
                        for (i, rum) in rums_lista.read().iter().enumerate() {
                            {
                                let dot_color = if rum.temp < 20.0 { "#2196f3" } else if rum.temp > 23.0 { "#ff9800" } else { "#4caf50" };
                                rsx! {
                                    div { class: "row",
                                        div { class: "status-dot", style: "background: {dot_color}" }
                                        input { 
                                            style: "flex: 2; border: none; background: transparent; color: white; font-weight: bold;",
                                            value: "{rum.namn}",
                                            oninput: move |e| rums_lista.with_mut(|l| l[i].namn = e.value())
                                        }
                                        input { 
                                            style: "flex: 1; text-align: center; margin: 0; color: #ffb300; font-weight: bold;",
                                            r#type: "number",
                                            value: "{rum.temp}",
                                            oninput: move |e| rums_lista.with_mut(|l| l[i].temp = e.value().parse().unwrap_or(0.0))
                                        }
                                    }
                                }
                            }
                        }
                        button { 
                            class: "add-btn", 
                            onclick: move |_| rums_lista.with_mut(|l| l.push(Punkt { namn: "".into(), temp: 21.0 })), 
                            "+ ADD ROOM" 
                        }
                        
                        {
                            let data = rums_lista.read();
                            let sum: f64 = data.iter().map(|r| r.temp).sum();
                            let avg = if !data.is_empty() { sum / data.len() as f64 } else { 0.0 };
                            let detaljer = data.iter().filter(|r| !r.namn.is_empty())
                                .map(|r| format!("{}: {}°C", r.namn, r.temp))
                                .collect::<Vec<_>>().join(", ");
                            let r_text = format!("Temp check: {}. Average: {:.1}°C.", detaljer, avg);
                            
                            rsx! {
                                div { class: "copy-box",
                                    button { 
                                        class: "copy-btn", 
                                        onclick: move |_| { eval(&format!("navigator.clipboard.writeText('{}')", r_text.clone())); }, 
                                        "COPY" 
                                    }
                                    div { style: "color: #ffb300; font-size: 10px; font-weight: bold; margin-bottom: 5px;", "CHECKLIST COMMENT" }
                                    p { style: "font-size: 14px; margin: 0; padding-right: 65px; color: #eee; line-height: 1.4;", "{r_text}" }
                                }
                            }
                        }
                    }
                }
            }

            // --- FOOTER ---
            div { class: "footer",
                p { style: "font-weight: bold; color: #d32f2f; margin-bottom: 4px;", "REALSTATE MULTITOOL v0.1.0" }
                p { "MIT License • © 2026 Magnus" }
                p { "Written in Rust by a professional Facility Technician" }
            }
        }
    }
}
