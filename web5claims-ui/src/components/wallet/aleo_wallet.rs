use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AleoWalletProps {
    #[prop_or_default]
    pub on_connect: Callback<String>,
    #[prop_or_default]
    pub on_error: Callback<String>,
    #[prop_or_default]
    pub class: String,
}

#[derive(Clone, PartialEq)]
pub struct WalletState {
    pub connected: bool,
    pub address: Option<String>,
    pub network: Option<String>,
}

#[function_component(AleoWallet)]
pub fn aleo_wallet(props: &AleoWalletProps) -> Html {
    let wallet_state = use_state(|| WalletState {
        connected: false,
        address: None,
        network: None,
    });

    let connect_wallet = {
        let wallet_state = wallet_state.clone();
        let on_connect = props.on_connect.clone();
        let on_error = props.on_error.clone();

        Callback::from(move |_| {
            let window = web_sys::window().unwrap();

            // Check if Aleo wallet is available
            let has_wallet =
                js_sys::Reflect::has(&window, &JsValue::from_str("aleo")).unwrap_or(false);

            if !has_wallet {
                on_error
                    .emit("No Aleo wallet found. Please install Leo Wallet extension.".to_string());
                return;
            }

            let promise = js_sys::Promise::new(&mut |resolve, reject| {
                let window = web_sys::window().unwrap();

                match js_sys::Reflect::get(&window, &JsValue::from_str("aleo")) {
                    Ok(aleo) => {
                        // Call the connect method on the wallet
                        match js_sys::Reflect::get(&aleo, &JsValue::from_str("connect")) {
                            Ok(connect_fn) => {
                                if connect_fn.is_function() {
                                    let connect_fn =
                                        connect_fn.dyn_into::<js_sys::Function>().unwrap();

                                    match connect_fn.call0(&aleo) {
                                        Ok(promise) => {
                                            let promise =
                                                promise.dyn_into::<js_sys::Promise>().unwrap();
                                            let _promise_result =
                                                promise.then(&resolve).catch(&reject);
                                        }
                                        Err(_) => {
                                            let _ = reject.call1(
                                                &JsValue::NULL,
                                                &JsValue::from_str("Failed to call connect method"),
                                            );
                                        }
                                    }
                                } else {
                                    let _ = reject.call1(
                                        &JsValue::NULL,
                                        &JsValue::from_str("Connect is not a function"),
                                    );
                                }
                            }
                            Err(_) => {
                                let _ = reject.call1(
                                    &JsValue::NULL,
                                    &JsValue::from_str("Connect method not found"),
                                );
                            }
                        }
                    }
                    Err(_) => {
                        let _ = reject
                            .call1(&JsValue::NULL, &JsValue::from_str("Aleo object not found"));
                    }
                }
            });

            let wallet_state_clone = wallet_state.clone();
            let on_connect_clone = on_connect.clone();
            let on_error_clone = on_error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match wasm_bindgen_futures::JsFuture::from(promise).await {
                    Ok(result) => {
                        // Parse the connection result
                        if let Some(obj) = result.dyn_ref::<js_sys::Object>() {
                            let address = js_sys::Reflect::get(obj, &JsValue::from_str("address"))
                                .ok()
                                .and_then(|v| v.as_string());

                            let network = js_sys::Reflect::get(obj, &JsValue::from_str("network"))
                                .ok()
                                .and_then(|v| v.as_string());

                            if let Some(addr) = &address {
                                wallet_state_clone.set(WalletState {
                                    connected: true,
                                    address: address.clone(),
                                    network,
                                });

                                on_connect_clone.emit(addr.clone());
                                log::info!("Connected to Aleo wallet: {}", addr);
                            }
                        }
                    }
                    Err(e) => {
                        let error_msg = match e.as_string() {
                            Some(msg) => msg,
                            None => "Failed to connect to wallet".to_string(),
                        };
                        on_error_clone.emit(error_msg);
                        log::error!("Wallet connection error: {:?}", e);
                    }
                }
            });
        })
    };

    let disconnect_wallet = {
        let wallet_state = wallet_state.clone();

        Callback::from(move |_| {
            // Reset the wallet state
            wallet_state.set(WalletState {
                connected: false,
                address: None,
                network: None,
            });
        })
    };

    html! {
        <div class={classes!("aleo-wallet-container", props.class.clone())}>
            if !wallet_state.connected {
                <button
                    class="btn btn-primary"
                    onclick={connect_wallet}
                >
                    <span class="mr-2">{"🔑"}</span>
                    {"Connect Aleo Wallet"}
                </button>
            } else {
                <div class="card bg-base-200 p-4">
                    <div class="flex items-center justify-between">
                        <div>
                            <span class="font-semibold">{"Connected: "}</span>
                            <span class="text-xs font-mono bg-base-300 px-2 py-1 rounded">
                                {wallet_state.address.clone().unwrap_or_default()}
                            </span>
                            if let Some(network) = &wallet_state.network {
                                <div class="badge badge-sm badge-accent mt-1">{network}</div>
                            }
                        </div>
                        <button
                            class="btn btn-sm btn-outline"
                            onclick={disconnect_wallet}
                        >
                            {"Disconnect"}
                        </button>
                    </div>
                </div>
            }
        </div>
    }
}
