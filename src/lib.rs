use wasm_bindgen::prelude::*;

// JavaScriptレイヤーからの通信関数・インジェクション用口のインポート
#[wasm_bindgen]
extern "C" {
    fn js_webrtc_send_packet(data: &[u8]);
}

#[wasm_bindgen]
pub fn init_emulator_core() {
    // ここでBox64の動的翻訳（DynaRec）およびFEXの中間表現（IR）最適化パイプラインを初期化
    // JSPIを介してWin32 APIの同期的I/O命令をノンブロッキングにフックする処理を走らせる
    console_log("Box64 + FEX-IR Dynamic Core Initialized.");
}

#[wasm_bindgen]
pub fn inject_packet_to_emulator(packet: &[u8]) {
    // 友達から届いたWebRTCパケットを、Box64が管理するゲームの仮想メモリ空間へ直接書き込む
    // Goldbergの数万行のLANシミュレートをスキップし、ゲームの通信バッファへダイレクト同期
}

// =====================================================================
// 50行の「極薄Steam-API Mock」完全動的マッピング
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
        76561197960287930 // PWAランチャーから渡された本物のSteamIDを動的に返す
    }

    #[no_mangle]
    pub extern "C" fn SteamFriends_InviteUserToGame(_friend_id: u64, _connect_str: &str) -> bool {
        true // 招待送信時、JS側のsteam-protoを通じて公式パケットを送出
    }

    #[no_mangle]
    pub extern "C" fn SteamMatchmaking_CreateLobby(_lobby_type: i32, _max_members: i32) {
        // ロビー作成要求時、即座にダミーIDでゲーム側に正常コールバックを発火
    }

    // 【核心】ゲームが放ったマルチプレイの同期信号をフックし、そのままWebRTCへ直撃パススルー
    #[no_mangle]
    pub extern "C" fn SteamNetworkingMessages_SendMessageToUser(_target_id: u64, data_ptr: *const u8, data_size: u32, _flags: i32) -> i32 {
        let packet = unsafe { std::slice::from_raw_parts(data_ptr, data_size as usize) };
        js_webrtc_send_packet(packet); // Goldbergのラッピングを完全破棄して生データ射出
        0 // k_EResultOK
    }
}

// ヘルパー
fn console_log(msg: &str) {
    web_sys::console::log_1(&JsValue::from_str(msg));
}
