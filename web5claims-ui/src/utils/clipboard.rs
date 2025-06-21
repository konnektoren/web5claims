use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};
use yew::prelude::*;

/// Simplified clipboard function for immediate use with UseStateHandle
pub fn copy_to_clipboard_simple(text: &str, copy_status: UseStateHandle<Option<String>>) {
    let text_owned = text.to_string();

    if let Some(window) = web_sys::window() {
        let clipboard = window.navigator().clipboard();
        let promise = clipboard.write_text(&text_owned);
        let copy_status = copy_status.clone();

        wasm_bindgen_futures::spawn_local(async move {
            match wasm_bindgen_futures::JsFuture::from(promise).await {
                Ok(_) => {
                    copy_status.set(Some("✅ Copied to clipboard!".to_string()));
                    // Clear message after 3 seconds
                    let copy_status_clear = copy_status.clone();
                    gloo::timers::callback::Timeout::new(3000, move || {
                        copy_status_clear.set(None);
                    })
                    .forget();
                }
                Err(_) => {
                    // Fall back to legacy method
                    fallback_copy(&text_owned, copy_status);
                }
            }
        });
    } else {
        copy_status.set(Some("❌ Cannot access clipboard".to_string()));
    }
}

/// Fallback copy function using legacy execCommand
fn fallback_copy(text: &str, copy_status: UseStateHandle<Option<String>>) {
    let document = document();

    // Create temporary textarea
    if let Ok(textarea) = document.create_element("textarea") {
        if let Ok(textarea) = textarea.dyn_into::<HtmlTextAreaElement>() {
            textarea.set_value(text);

            // Style the textarea to be invisible
            let style = textarea.style();
            let _ = style.set_property("position", "fixed");
            let _ = style.set_property("left", "-9999px");
            let _ = style.set_property("opacity", "0");

            // Add to DOM, select, copy, and remove
            if let Some(body) = document.body() {
                if body.append_child(&textarea).is_ok() {
                    textarea.select();

                    // Try to copy
                    if let Ok(html_doc) = document.dyn_into::<HtmlDocument>() {
                        if html_doc.exec_command("copy").unwrap_or(false) {
                            copy_status.set(Some("✅ Copied to clipboard!".to_string()));
                        } else {
                            copy_status.set(Some("❌ Copy failed".to_string()));
                        }
                    } else {
                        copy_status.set(Some("❌ Copy not supported".to_string()));
                    }

                    let _ = body.remove_child(&textarea);

                    // Clear message after 3 seconds
                    let copy_status_clear = copy_status.clone();
                    gloo::timers::callback::Timeout::new(3000, move || {
                        copy_status_clear.set(None);
                    })
                    .forget();
                }
            }
        }
    }
}

/// Async clipboard copy with Result return type
pub async fn copy_to_clipboard(text: &str) -> Result<(), String> {
    if let Some(window) = web_sys::window() {
        // Try modern clipboard API first
        let clipboard = window.navigator().clipboard();
        let promise = clipboard.write_text(text);

        match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(_) => return Ok(()),
            Err(_) => {
                // Fall through to legacy method
            }
        }
    }

    // Legacy fallback method
    copy_to_clipboard_legacy(text)
}

/// Legacy clipboard copy using execCommand with Result return type
pub fn copy_to_clipboard_legacy(text: &str) -> Result<(), String> {
    let document = document();

    // Create temporary textarea
    let textarea = document
        .create_element("textarea")
        .map_err(|_| "Failed to create textarea element")?;

    let textarea: HtmlTextAreaElement = textarea
        .dyn_into()
        .map_err(|_| "Failed to cast to textarea")?;

    // Set up textarea
    textarea.set_value(text);
    let style = textarea.style();
    let _ = style.set_property("position", "fixed");
    let _ = style.set_property("left", "-9999px");
    let _ = style.set_property("top", "-9999px");
    let _ = style.set_property("opacity", "0");

    // Add to DOM
    if let Some(body) = document.body() {
        body.append_child(&textarea)
            .map_err(|_| "Failed to append textarea to body")?;

        // Select and copy
        textarea.select();

        // Set selection range
        if textarea.set_selection_range(0, text.len() as u32).is_ok() {
            // Try to get HtmlDocument for execCommand
            if let Ok(html_doc) = document.dyn_into::<HtmlDocument>() {
                let success = html_doc.exec_command("copy").unwrap_or(false);

                // Clean up
                let _ = body.remove_child(&textarea);

                if success {
                    Ok(())
                } else {
                    Err("Copy command returned false".to_string())
                }
            } else {
                let _ = body.remove_child(&textarea);
                Err("Could not access document.execCommand".to_string())
            }
        } else {
            let _ = body.remove_child(&textarea);
            Err("Could not select text".to_string())
        }
    } else {
        Err("No document body found".to_string())
    }
}
