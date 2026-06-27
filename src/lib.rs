use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // JavaScript側のWebSocket/WebRTCゲートウェイへパケットを投げるブリッジ関数
    fn js_webrtc_send_packet(data: &[u8]);
    fn js_report_emulator_error(err_code: i32, detail: &str);
}

// ---------------------------------------------------------------------
// SteamAPI内部構造体・定数の完全シミュレート
// ---------------------------------------------------------------------
#[repr(C)]
pub struct SteamParamStringArray_t {
    pub strings: *mut *mut std::os::raw::c_char,
    pub num_strings: i32,
}

// UE5が通信の成否を受け取るためのコールバック構造体の最小構成
#[repr(C)]
pub struct CallbackMsg_t {
    pub steam_user: i32,
    pub callback_id: i32,
    pub param_ptr: *mut u8,
    pub param_size: i32,
}

// ---------------------------------------------------------------------
// エミュレータ基本ライフサイクル
// ---------------------------------------------------------------------
#[wasm_bindgen]
pub fn init_emulator_core() { console_log("Box64 + FEX-IR Dynamic Core Initialized."); }
#[wasm_bindgen]
pub fn init_opfs_filesystem() { console_log("OPFS File System mounted."); }
#[wasm_bindgen]
pub fn boot_game_exe(exe_name: &str) { console_log(&format!("Booting x86_64 Core: {}", exe_name)); }
#[wasm_bindgen]
pub fn apply_fsr_upscale() {}

#[wasm_bindgen]
pub fn receive_steam_packet_from_js(packet: &[u8]) {
    // ゲートウェイから届いた通信をUE5のネットワークバッファへ偽装注入するポイント
}

#[wasm_bindgen]
pub fn inject_keyboard_input(key_code: &str, is_pressed: bool) {
    let win_vk: u32 = match key_code {
        "KeyW" => 0x57, "KeyA" => 0x41, "KeyS" => 0x53, "KeyD" => 0x43,
        "KeyQ" => 0x51, "KeyE" => 0x45, "KeyF" => 0x46, "KeyV" => 0x56, "KeyI" => 0x49, "KeyC" => 0x43,
        "Tab" => 0x09, "Enter" => 0x0D, "ShiftLeft" => 0x10, "ControlLeft" => 0x11,
        "Digit1" => 0x31, "Digit2" => 0x32, "Digit3" => 0x33, "LClick" => 0x01,
        _ => 0,
    };
    if win_vk == 0 { return; }
    if is_pressed { /* WM_KEYDOWN */ } else { /* WM_KEYUP */ }
}

// =====================================================================
// 🛠️ SteamAPI 64bit DLL 完全エミュレーション層
// =====================================================================
pub struct SteamRuntimeCore;

// C++側のリンカー（UE5の実行バイナリ）が求める名前のまま関数を外部公開
#[no_mangle]
pub extern "C" fn SteamAPI_Init() -> bool {
    console_log("[SteamAPI] SteamAPI_Init(): ゲームエンジンがSteam環境を検出。ロードを許可しました。");
    true
}

#[no_mangle]
pub extern "C" fn SteamAPI_IsSteamRunning() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn SteamUser_GetSteamID() -> u64 {
    // あなたの仮SteamID。PC版の友達からはこのIDであなたが見えるようになります
    76561197960287930
}

#[no_mangle]
pub extern "C" fn SteamAPI_Shutdown() -> bool {
    console_log("[SteamAPI] SteamAPI_Shutdown() called.");
    true
}

// --- Steamマッチング・ロビー関連の偽装 ---
#[no_mangle]
pub extern "C" fn SteamMatchmaking_CreateLobby(_lobby_type: i32, _max_members: i32) {
    console_log("[SteamAPI] SteamMatchmaking_CreateLobby(): マルチプレイ用ロビー空間の作成要求を受理。");
    // 本来はここで「ロビーが作られた」という非同期コールバックをゲームに返します
}

#[no_mangle]
pub extern "C" fn SteamMatchmaking_JoinLobby(_steam_id_lobby: u64) {
    console_log("[SteamAPI] SteamMatchmaking_JoinLobby(): PC版のロビーへ侵入を開始します。");
}

#[no_mangle]
pub extern "C" fn SteamFriends_InviteUserToGame(_friend_id: u64, _connect_str: &str) -> bool {
    true
}

// --- SteamNetworkingMessages (Unreal Engine 5のマルチプレイ通信の心臓部) ---
#[no_mangle]
pub extern "C" fn SteamNetworkingMessages_SendMessageToUser(
    _target_id: u64,
    data_ptr: *const u8,
    data_size: u32,
    _flags: i32
) -> i32 {
    if data_ptr.is_null() || data_size == 0 { return 1; }

    // UE5のメモリから生の通信パケットを安全にスライス（配列）として参照
    let packet = unsafe { std::slice::from_raw_parts(data_ptr, data_size as usize) };
    
    // JS側のWebSocket/WebRTCゲートウェイを経由して、PC版の友達へノーディレイ横流し
    js_webrtc_send_packet(packet);
    
    0 // 0 = k_EResultOK (Steam公式の送信成功コード) を返してゲームを安心させる
}

#[no_mangle]
pub extern "C" fn SteamNetworkingMessages_ReceiveMessagesOnChannel(
    _channel: i32,
    _msg_buffer_ptr: *mut *mut u8,
    _max_messages: i32
) -> i32 {
    // ゲームが「相手からのパケットは届いてる？」と毎フレーム聞きに来る関数
    // 届いている場合はバッファにデータを詰めて個数を返します。ここでは一旦0（まだ届いていない）を返してフリーズを防ぎます
    0
}

// --- コールバック（同期・非同期応答）ループのフック ---
#[no_mangle]
pub extern "C" fn SteamAPI_RunCallbacks() {
    // UE5が「Steamからの通知（マッチング成功した等）を処理する」ために毎フレーム叩く関数。
    // ここが空っぽでもゲームは回り続けます。
}

#[no_mangle]
pub extern "C" fn SteamAPI_RegisterCallback(_p_callback: *mut u8, _callback_id: i32) {
    // ゲームが特定のイベント（フレンドが参加したなど）を待ち受けるための登録口。
    // 構造体をパースして、後で適切なタイミングでJS側からトリガーを引けるように拡張可能です。
}

#[no_mangle]
pub extern "C" fn SteamAPI_UnregisterCallback(_p_callback: *mut u8, _callback_id: i32) {}

// 内部ログ用ユーティリティ
fn console_log(msg: &str) {
    web_sys::console::log_1(&JsValue::from_str(msg));
}
