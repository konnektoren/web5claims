use js_sys::{Object, Promise, Reflect};
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AleoWalletProps {
    #[prop_or_default]
    pub on_connect: Callback<WalletInfo>,
    #[prop_or_default]
    pub on_disconnect: Callback<()>,
    #[prop_or_default]
    pub on_error: Callback<String>,
    #[prop_or_default]
    pub class: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct WalletInfo {
    pub address: String,
    pub network: String,
    pub balance: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct WalletState {
    pub connected: bool,
    pub wallet_info: Option<WalletInfo>,
    pub connecting: bool,
    pub error: Option<String>,
}

impl Default for WalletState {
    fn default() -> Self {
        Self {
            connected: false,
            wallet_info: None,
            connecting: false,
            error: None,
        }
    }
}

#[function_component(AleoWallet)]
pub fn aleo_wallet(props: &AleoWalletProps) -> Html {
    let wallet_state = use_state(WalletState::default);

    // Check if wallet is already connected on component mount
    {
        let wallet_state = wallet_state.clone();
        let on_connect = props.on_connect.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Checking for existing Leo Wallet connection...");
                if is_leo_wallet_available() {
                    match get_wallet_status().await {
                        Ok(Some(info)) => {
                            log::info!("Found existing wallet connection: {}", info.address);
                            wallet_state.set(WalletState {
                                connected: true,
                                wallet_info: Some(info.clone()),
                                connecting: false,
                                error: None,
                            });
                            on_connect.emit(info);
                        }
                        Ok(None) => {
                            log::info!("No existing wallet connection found");
                        }
                        Err(e) => {
                            log::error!("Error checking wallet status: {}", e);
                        }
                    }
                } else {
                    log::info!("Leo Wallet not available");
                }
            });
            || ()
        });
    }

    let connect_wallet = {
        let wallet_state = wallet_state.clone();
        let on_connect = props.on_connect.clone();
        let on_error = props.on_error.clone();

        Callback::from(move |_| {
            log::info!("Connect wallet button clicked");

            if !is_leo_wallet_available() {
                let error_msg =
                    "Leo Wallet not found. Please install the Leo Wallet browser extension.";
                log::error!("{}", error_msg);
                on_error.emit(error_msg.to_string());
                return;
            }

            let mut new_state = (*wallet_state).clone();
            new_state.connecting = true;
            new_state.error = None;
            wallet_state.set(new_state);

            let wallet_state_clone = wallet_state.clone();
            let on_connect_clone = on_connect.clone();
            let on_error_clone = on_error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Attempting to connect to Leo Wallet...");
                match connect_to_leo_wallet().await {
                    Ok(wallet_info) => {
                        log::info!(
                            "Successfully connected to Leo Wallet: {}",
                            wallet_info.address
                        );
                        wallet_state_clone.set(WalletState {
                            connected: true,
                            wallet_info: Some(wallet_info.clone()),
                            connecting: false,
                            error: None,
                        });
                        on_connect_clone.emit(wallet_info);
                    }
                    Err(error) => {
                        log::error!("Failed to connect to Leo Wallet: {}", error);
                        wallet_state_clone.set(WalletState {
                            connected: false,
                            wallet_info: None,
                            connecting: false,
                            error: Some(error.clone()),
                        });
                        on_error_clone.emit(error);
                    }
                }
            });
        })
    };

    let disconnect_wallet = {
        let wallet_state = wallet_state.clone();
        let on_disconnect = props.on_disconnect.clone();

        Callback::from(move |_| {
            wallet_state.set(WalletState::default());
            on_disconnect.emit(());
            log::info!("Disconnected from Leo Wallet");
        })
    };

    let refresh_balance = {
        let wallet_state = wallet_state.clone();

        Callback::from(move |_| {
            if let Some(mut wallet_info) = wallet_state.wallet_info.clone() {
                let wallet_state_clone = wallet_state.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    log::info!("Refreshing wallet balance...");
                    match get_wallet_balance(&wallet_info.address).await {
                        Ok(balance) => {
                            log::info!("Balance updated: {}", balance);
                            wallet_info.balance = Some(balance);
                            wallet_state_clone.set(WalletState {
                                connected: true,
                                wallet_info: Some(wallet_info),
                                connecting: false,
                                error: None,
                            });
                        }
                        Err(e) => {
                            log::error!("Failed to get balance: {}", e);
                        }
                    }
                });
            }
        })
    };

    html! {
        <div class={classes!("aleo-wallet-container", props.class.clone())}>
            if wallet_state.connecting {
                <div class="card bg-base-200 p-4">
                    <div class="flex items-center justify-center space-x-2">
                        <span class="loading loading-spinner loading-sm"></span>
                        <span>{"Connecting to Leo Wallet..."}</span>
                    </div>
                </div>
            } else if !wallet_state.connected {
                <div class="space-y-4">
                    <button
                        class="btn btn-primary btn-lg w-full"
                        onclick={connect_wallet}
                    >
                        <span class="text-2xl mr-2">{"🦁"}</span>
                        {"Connect Leo Wallet"}
                    </button>

                    if let Some(error) = &wallet_state.error {
                        <div class="alert alert-error">
                            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <span>{error}</span>
                        </div>
                    }

                    <div class="text-sm text-base-content/70 text-center">
                        {"Install Leo Wallet: "}
                        <a
                            href="https://leo.app"
                            target="_blank"
                            class="link link-primary"
                        >
                            {"https://leo.app"}
                        </a>
                    </div>

                    // Debug info
                    <details class="collapse bg-base-200">
                        <summary class="collapse-title text-sm">{"🔧 Debug Info"}</summary>
                        <div class="collapse-content text-xs">
                            <p>{"Leo Wallet Available: "}{if is_leo_wallet_available() { "✅ Yes" } else { "❌ No" }}</p>
                            <p>{"Window Object: "}{if web_sys::window().is_some() { "✅ Available" } else { "❌ Missing" }}</p>
                            <DebugWalletObject />
                        </div>
                    </details>
                </div>
            } else if let Some(wallet_info) = &wallet_state.wallet_info {
                <div class="card bg-base-100 shadow-lg">
                    <div class="card-body">
                        <div class="flex items-center justify-between mb-4">
                            <div class="flex items-center space-x-2">
                                <span class="text-2xl">{"🦁"}</span>
                                <div>
                                    <h3 class="font-bold">{"Leo Wallet Connected"}</h3>
                                    <div class="badge badge-success badge-sm">{&wallet_info.network}</div>
                                </div>
                            </div>
                            <button
                                class="btn btn-sm btn-outline btn-error"
                                onclick={disconnect_wallet}
                            >
                                {"Disconnect"}
                            </button>
                        </div>

                        <div class="space-y-3">
                            <div>
                                <label class="text-xs font-medium opacity-70">{"Address:"}</label>
                                <div class="flex items-center space-x-2">
                                    <code class="text-xs bg-base-200 px-2 py-1 rounded flex-1 truncate">
                                        {format_address(&wallet_info.address)}
                                    </code>
                                    <button
                                        class="btn btn-xs btn-ghost"
                                        onclick={copy_address_to_clipboard(wallet_info.address.clone())}
                                        title="Copy address"
                                    >
                                        {"📋"}
                                    </button>
                                </div>
                            </div>

                            if let Some(balance) = &wallet_info.balance {
                                <div>
                                    <label class="text-xs font-medium opacity-70">{"Balance:"}</label>
                                    <div class="flex items-center justify-between">
                                        <span class="font-mono text-sm">{format!("{} ALEO", balance)}</span>
                                        <button
                                            class="btn btn-xs btn-ghost"
                                            onclick={refresh_balance}
                                            title="Refresh balance"
                                        >
                                            {"🔄"}
                                        </button>
                                    </div>
                                </div>
                            } else {
                                <button
                                    class="btn btn-sm btn-outline w-full"
                                    onclick={refresh_balance}
                                >
                                    {"Load Balance"}
                                </button>
                            }
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}

#[function_component(DebugWalletObject)]
fn debug_wallet_object() -> Html {
    let debug_info = use_state(|| "Loading...".to_string());

    {
        let debug_info = debug_info.clone();
        use_effect_with((), move |_| {
            if let Some(window) = web_sys::window() {
                let mut info = String::new();

                if let Ok(leo_wallet) = Reflect::get(&window, &JsValue::from_str("leoWallet")) {
                    info.push_str("leoWallet object found\n");

                    // Check available methods
                    if let Some(obj) = leo_wallet.dyn_ref::<Object>() {
                        let keys = Object::get_own_property_names(obj);
                        for i in 0..keys.length() {
                            if let Some(key) = keys.get(i).as_string() {
                                info.push_str(&format!("- {}\n", key));
                            }
                        }
                    }
                } else {
                    info.push_str("leoWallet object not found\n");
                }

                debug_info.set(info);
            }
            || ()
        });
    }

    html! {
        <div class="mt-2">
            <p class="font-mono text-xs whitespace-pre-line">{&*debug_info}</p>
        </div>
    }
}

// Helper functions for Leo Wallet interaction

fn is_leo_wallet_available() -> bool {
    if let Some(window) = web_sys::window() {
        let has_leo_wallet =
            Reflect::has(&window, &JsValue::from_str("leoWallet")).unwrap_or(false);
        let has_leo = Reflect::has(&window, &JsValue::from_str("leo")).unwrap_or(false);
        let has_aleo = Reflect::has(&window, &JsValue::from_str("aleo")).unwrap_or(false);

        log::info!(
            "Wallet availability check - leoWallet: {}, leo: {}, aleo: {}",
            has_leo_wallet,
            has_leo,
            has_aleo
        );

        has_leo_wallet || has_leo || has_aleo
    } else {
        false
    }
}

async fn connect_to_leo_wallet() -> Result<WalletInfo, String> {
    let window = web_sys::window().ok_or("No window available")?;
    log::info!("Window available, attempting wallet connection");

    // Get the Leo Wallet object
    let leo_wallet = Reflect::get(&window, &JsValue::from_str("leoWallet"))
        .map_err(|_| "Leo Wallet object not found")?;

    log::info!("Leo Wallet object found");

    // Try different approaches to connect

    // Approach 1: Try requestPermissions first (common pattern)
    if let Ok(request_permissions) =
        Reflect::get(&leo_wallet, &JsValue::from_str("requestPermissions"))
    {
        if request_permissions.is_function() {
            log::info!("Trying requestPermissions method...");
            let request_permissions_fn =
                request_permissions.dyn_into::<js_sys::Function>().unwrap();

            match request_permissions_fn.call0(&leo_wallet) {
                Ok(result) => {
                    if let Ok(promise) = result.clone().dyn_into::<Promise>() {
                        match JsFuture::from(promise).await {
                            Ok(_permissions) => {
                                log::info!("Permissions granted, now getting account info...");
                                return get_account_info(&leo_wallet).await;
                            }
                            Err(e) => {
                                log::warn!("requestPermissions failed: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to call requestPermissions: {:?}", e);
                }
            }
        }
    }

    // Approach 2: Try connect method with empty object parameter
    if let Ok(connect_fn) = Reflect::get(&leo_wallet, &JsValue::from_str("connect")) {
        if connect_fn.is_function() {
            log::info!("Trying connect method with empty params...");
            let connect_fn = connect_fn.dyn_into::<js_sys::Function>().unwrap();

            // Try with empty object parameter
            let empty_obj = Object::new();
            match connect_fn.call1(&leo_wallet, &empty_obj) {
                Ok(result) => {
                    if let Ok(promise) = result.clone().dyn_into::<Promise>() {
                        match JsFuture::from(promise).await {
                            Ok(connect_result) => {
                                log::info!("Connect with params succeeded");
                                return parse_wallet_info(connect_result);
                            }
                            Err(e) => {
                                log::warn!("Connect with params failed: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to call connect with params: {:?}", e);
                }
            }
        }
    }

    // Approach 3: Try getting current account if already connected
    log::info!("Trying to get current account info...");
    get_account_info(&leo_wallet).await
}

async fn get_account_info(leo_wallet: &JsValue) -> Result<WalletInfo, String> {
    // Try different ways to get account information

    // Method 1: accounts property
    if let Ok(accounts) = Reflect::get(leo_wallet, &JsValue::from_str("accounts")) {
        if !accounts.is_null() && !accounts.is_undefined() {
            if let Some(array) = accounts.dyn_ref::<js_sys::Array>() {
                if array.length() > 0 {
                    let first_account = array.get(0);
                    log::info!("Found account in accounts array");
                    return parse_wallet_info(first_account);
                }
            }
        }
    }

    // Method 2: account property
    if let Ok(account) = Reflect::get(leo_wallet, &JsValue::from_str("account")) {
        if !account.is_null() && !account.is_undefined() {
            log::info!("Found account property");
            return parse_wallet_info(account);
        }
    }

    // Method 3: Try getAccount method
    if let Ok(get_account) = Reflect::get(leo_wallet, &JsValue::from_str("getAccount")) {
        if get_account.is_function() {
            let get_account_fn = get_account.dyn_into::<js_sys::Function>().unwrap();
            match get_account_fn.call0(leo_wallet) {
                Ok(result) => {
                    if let Ok(promise) = result.clone().dyn_into::<Promise>() {
                        match JsFuture::from(promise).await {
                            Ok(account_result) => {
                                log::info!("getAccount method succeeded");
                                return parse_wallet_info(account_result);
                            }
                            Err(e) => {
                                log::warn!("getAccount method failed: {:?}", e);
                            }
                        }
                    } else {
                        log::info!("getAccount returned direct result");
                        return parse_wallet_info(result);
                    }
                }
                Err(e) => {
                    log::warn!("Failed to call getAccount: {:?}", e);
                }
            }
        }
    }

    Err("No account information available. Wallet might not be connected.".to_string())
}

async fn get_wallet_status() -> Result<Option<WalletInfo>, String> {
    if !is_leo_wallet_available() {
        return Ok(None);
    }

    let window = web_sys::window().ok_or("No window available")?;
    let leo_wallet = Reflect::get(&window, &JsValue::from_str("leoWallet"))
        .map_err(|_| "Leo Wallet not found")?;

    match get_account_info(&leo_wallet).await {
        Ok(info) => Ok(Some(info)),
        Err(_) => Ok(None), // No error, just not connected
    }
}

async fn get_wallet_balance(_address: &str) -> Result<String, String> {
    // For now, return a placeholder since balance fetching requires
    // network calls that might not be available in the wallet API
    Ok("0.0".to_string())
}

fn parse_wallet_info(js_value: JsValue) -> Result<WalletInfo, String> {
    log::info!("Parsing wallet info from: {:?}", js_value);

    if let Some(obj) = js_value.dyn_ref::<Object>() {
        // Try different property names for address
        let address = Reflect::get(obj, &JsValue::from_str("address"))
            .or_else(|_| Reflect::get(obj, &JsValue::from_str("account")))
            .or_else(|_| Reflect::get(obj, &JsValue::from_str("publicKey")))
            .or_else(|_| Reflect::get(obj, &JsValue::from_str("viewKey")))
            .ok()
            .and_then(|v| v.as_string())
            .ok_or("No address found in wallet response")?;

        let network = Reflect::get(obj, &JsValue::from_str("network"))
            .or_else(|_| Reflect::get(obj, &JsValue::from_str("chainId")))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| "testnet3".to_string());

        let balance = Reflect::get(obj, &JsValue::from_str("balance"))
            .ok()
            .and_then(|v| v.as_string());

        log::info!("Parsed wallet - Address: {}, Network: {}", address, network);

        Ok(WalletInfo {
            address,
            network,
            balance,
        })
    } else if js_value.is_string() {
        let address = js_value
            .as_string()
            .ok_or("Failed to parse address string")?;
        log::info!("Parsed wallet address from string: {}", address);

        Ok(WalletInfo {
            address,
            network: "testnet3".to_string(),
            balance: None,
        })
    } else {
        Err(format!("Invalid wallet info format: {:?}", js_value))
    }
}

fn format_address(address: &str) -> String {
    if address.len() > 16 {
        format!("{}...{}", &address[..8], &address[address.len() - 8..])
    } else {
        address.to_string()
    }
}

fn copy_address_to_clipboard(address: String) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let address = address.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(clipboard) =
                    Reflect::get(&window.navigator(), &JsValue::from_str("clipboard"))
                {
                    if let Ok(write_text) =
                        Reflect::get(&clipboard, &JsValue::from_str("writeText"))
                    {
                        if let Ok(function) = write_text.dyn_into::<js_sys::Function>() {
                            let _ = function.call1(&clipboard, &JsValue::from_str(&address));
                        }
                    }
                }
            }
        });
    })
}
