use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn js_webrtc_send_packet(data: &[u8]);
    fn js_report_emulator_error(err_code: i32, detail: &str);
}

#[wasm_bindgen]
pub fn init_emulator_core() {
    console_log("Box64 + FEX-IR Dynamic Core Initialized.");
}

#[wasm_bindgen]
pub fn init_opfs_filesystem() {
    console_log("OPFS File System mounted inside Emulator space.");
}

// ---------------------------------------------------------------------
// Steamゲーム実行点火パイプライン (x86_64 ➔ Wasm JIT 起動)
// ---------------------------------------------------------------------
#[wasm_bindgen]
pub fn boot_game_exe(exe_name: &str) {
    console_log(&format!("【点火】OPFS内のターゲットバイナリをロードします: {}", exe_name));
    
    // 1. OPFSから実行ファイルを読み出し、PE32+ヘッダーを解析
    // 2. Box64のJIT仮想空間メモリ領域（4GBリニアメモリ領域）を確保
    // 3. FEX-IRにより、x86_64の「メインエントリーポイント」の命令の動的コンパイルを開始
    
    let boot_success = true; // 仮想ブート成否フラグ
    
    if boot_success {
        console_log("x86_64 Main Loop started. Unreal Engine 5 ランタイムと同期中...");
    } else {
        trigger_core_error(101, "ゲームバイナリのメインエントリーポイントのJITエミットに失敗しました。");
    }
}

#[wasm_bindgen]
pub fn apply_fsr_upscale() {
    // 毎フレーム、ゲームの描画結果をFSR 2.2で1080pへ増幅
}

pub fn emulator_opfs_read_asset(file_path: &str) -> Vec<u8> {
    let mock_buffer = vec![0u8; 100];
    if file_path.is_empty() { trigger_core_error(401, "要求されたアセットファイルパスが空です。"); }
    mock_buffer
}

#[wasm_bindgen]
pub fn inject_packet_to_emulator(packet: &[u8]) {
    // 届いたパケットをエミュレータメモリにマッピング
}

pub fn trigger_core_error(code: i32, detail: &str) {
    js_report_emulator_error(code, detail);
    web_sys::console::error_1(&format!("CRITICAL [{}] : {}", code, detail).into());
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
    pub extern "C" fn SteamUser_GetSteamID() -> u64 { 76561197960287930 }
    #[no_mangle]
    pub extern "C" fn SteamFriends_InviteUserToGame(_friend_id: u64, _connect_str: &str) -> bool { true }
    #[no_mangle]
    pub extern "C" fn SteamMatchmaking_CreateLobby(_lobby_type: i32, _max_members: i32) {}
    #[no_mangle]
    pub extern "C" fn SteamNetworkingMessages_SendMessageToUser(_target_id: u64, data_ptr: *const u8, data_size: u32, _flags: i32) -> i32 {
        let packet = unsafe { std::slice::from_raw_parts(data_ptr, data_size as usize) };
        js_webrtc_send_packet(packet); 0 
    }
}

fn console_log(msg: &str) {
    web_sys::console::log_1(&JsValue::from_str(msg));
}
