use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn js_webrtc_send_packet(data: &[u8]);
    // エラー報告用のJS関数をインポート
    fn js_report_emulator_error(err_code: i32, detail: &str);
}

#[wasm_bindgen]
pub fn init_emulator_core() {
    console_log("Box64 + FEX-IR Dynamic Core Initialized.");
}

#[wasm_bindgen]
pub fn inject_packet_to_emulator(packet: &[u8]) {
    // 届いたパケットをエミュレータメモリにマッピング
}

// エミュレータ内部で異常を検知した際の共通エラー発火トリガー
pub fn trigger_core_error(code: i32, detail: &str) {
    js_report_emulator_error(code, detail);
    web_sys::console::error_1(&format!("CRITICAL [{}] : {}", code, detail).into());
}

// 【デバッグ用フック例】命令解釈時に未対応コードが来たらHUDへ即座に身元を通知
pub fn execute_x86_instruction(opcode: u8) {
    if opcode == 0x0F { 
        trigger_core_error(
            101, 
            &format!("未対応のx86拡張オペコード [0x{:X}] を検知。フォールバックを試みます。", opcode)
        );
    }
}

// =====================================================================
// 50行の「極薄Steam-API Mock」
// =====================================================================
pub struct EscapeSteamMock;

#[wasm_bindgen]
impl EscapeSteamMock {
    #[no_mangle]
    pub extern "C" fn SteamAPI_Init() -> bool { true }
    
    #[no_mangle]
    pub extern "C" fn SteamAPI_IsSteamRunning() -> bool { true }

    #[no_mangle]
    pub extern "C" fn SteamUser_GetSteamID() -> u64 {
        76561197960287930 
    }

    #[no_mangle]
    pub extern " => " fn SteamFriends_InviteUserToGame(_friend_id: u64, _connect_str: &str) -> bool {
        true 
    }

    #[no_mangle]
    pub extern "C" fn SteamMatchmaking_CreateLobby(_lobby_type: i32, _max_members: i32) {}

    #[no_mangle]
    pub extern "C" fn SteamNetworkingMessages_SendMessageToUser(_target_id: u64, data_ptr: *const u8, data_size: u32, _flags: i32) -> i32 {
        let packet = unsafe { std::slice::from_raw_parts(data_ptr, data_size as usize) };
        js_webrtc_send_packet(packet); 
        0 
    }
}

fn console_log(msg: &str) {
    web_sys::console::log_1(&JsValue::from_str(msg));
}
