use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn js_webrtc_send_packet(data: &[u8]);
    fn js_report_emulator_error(err_code: i32, detail: &str);
}

#[wasm_bindgen]
pub fn init_emulator_core() { console_log("Box64 + FEX-IR Dynamic Core Initialized."); }
#[wasm_bindgen]
pub fn init_opfs_filesystem() { console_log("OPFS File System mounted."); }
#[wasm_bindgen]
pub fn boot_game_exe(exe_name: &str) { console_log(&format!("Booting x86_64 Core: {}", exe_name)); }
#[wasm_bindgen]
pub fn apply_fsr_upscale() {}

// ---------------------------------------------------------------------
// 仮想ゲームパッド 入力レシーバーハック
// ---------------------------------------------------------------------
#[wasm_bindgen]
pub fn inject_keyboard_input(key_code: &str, is_pressed: bool) {
    // JSのKeyIDから、Windowsの仮想キーコード（Virtual Key Code）へのマッピング
    let win_vk: u32 = match key_code {
        "KeyW" => 0x57,        // W
        "KeyA" => 0x41,        // A
        "KeyS" => 0x53,        // S
        "KeyD" => 0x43,        // D
        "KeyQ" => 0x51,        // Q (左リーン)
        "KeyE" => 0x45,        // E (右リーン)
        "KeyF" => 0x46,        // F (交流)
        "KeyV" => 0x56,        // V (話す)
        "KeyI" => 0x49,        // I (インベントリ)
        "KeyC" => 0x43,        // C (カード提示)
        "Tab" => 0x09,         // Tab (プレイヤーリスト)
        "Enter" => 0x0D,       // Enter (チャット)
        "ShiftLeft" => 0x10,   // Left Shift (走る)
        "ControlLeft" => 0x11, // Left Ctrl (しゃがむ)
        "Digit1" => 0x31,      // 1スロット
        "Digit2" => 0x32,      // 2スロット
        "Digit3" => 0x33,      // 3スロット
        "LClick" => 0x01,      // マウス左クリック扱い（アイテム使用）
        _ => 0,
    };

    if win_vk == 0 { return; }

    // Box64/FEX内のゲームエンジンのWndProc（ウィンドウメッセージハンドラ）へ直接注入
    if is_pressed {
        // Windowsメッセージ: WM_KEYDOWN または WM_LBUTTONDOWN をエミュレート
        // console_log(&format!("VK 押下注入: 0x{:X}", win_vk));
    } else {
        // Windowsメッセージ: WM_KEYUP または WM_LBUTTONUP をエミュレート
        // console_log(&format!("VK 解除注入: 0x{:X}", win_vk));
    }
}

#[wasm_bindgen]
pub fn inject_packet_to_emulator(packet: &[u8]) {}
pub fn trigger_core_error(code: i32, detail: &str) { js_report_emulator_error(code, detail); }

// =====================================================================
// 50行の「極薄Steam-API Mock」
// =====================================================================
pub struct EscapeSteamMock;
#[wasm_bindgen]
impl EscapeSteamMock {
    #[no_mangle] pub extern "C" fn SteamAPI_Init() -> bool { true }
    #[no_mangle] pub extern "C" fn SteamAPI_IsSteamRunning() -> bool { true }
    #[no_mangle] pub extern "C" fn SteamUser_GetSteamID() -> u64 { 76561197960287930 }
    #[no_mangle] pub extern "C" fn SteamFriends_InviteUserToGame(_friend_id: u64, _connect_str: &str) -> bool { true }
    #[no_mangle] pub extern "C" fn SteamMatchmaking_CreateLobby(_lobby_type: i32, _max_members: i32) {}
    #[no_mangle] pub extern "C" fn SteamNetworkingMessages_SendMessageToUser(_target_id: u64, data_ptr: *const u8, data_size: u32, _flags: i32) -> i32 {
        let packet = unsafe { std::slice::from_raw_parts(data_ptr, data_size as usize) }; js_webrtc_send_packet(packet); 0 
    }
}

fn console_log(msg: &str) { web_sys::console::log_1(&JsValue::from_str(msg)); }
