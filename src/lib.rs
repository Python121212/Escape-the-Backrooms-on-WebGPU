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
    if is_pressed { /* WM_KEYDOWN エミュレート */ } else { /* WM_KEYUP エミュレート */ }
}

#[wasm_bindgen]
pub fn inject_packet_to_emulator(packet: &[u8]) {}
pub fn trigger_core_error(code: i32, detail: &str) { js_report_emulator_error(code, detail); }

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
