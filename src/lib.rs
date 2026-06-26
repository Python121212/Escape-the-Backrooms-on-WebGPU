use wasm_bindgen::prelude::*;

// JavaScript（フロントエンド側）に用意するWebRTC・IPv6通信関数をインポート
#[wasm_bindgen]
extern "C" {
    fn js_webrtc_send_packet(target_id: u64, data: &[u8]);
    fn js_trigger_steam_callback(callback_id: i32, param: u64);
}

// ゲームが求めるSteamAPIのインターフェースをエミュレートする構造体
#[wasm_bindgen]
pub struct EscapeSteamMock;

#[wasm_bindgen]
impl EscapeSteamMock {
    // 1. 起動時の生存チェックを「問題なし（True）」と即答してパス
    #[no_mangle]
    pub extern "C" fn SteamAPI_Init() -> bool { true }
    #[no_mangle]
    pub extern "C" fn SteamAPI_IsSteamRunning() -> bool { true }

    // 2. ユーザー認証の偽装（PWA側から渡されたあなたの本物のSteamIDを返す）
    #[no_mangle]
    pub extern "C" fn SteamUser_GetSteamID() -> u64 {
        76561197960287930 // 固定のダミー、またはJS側から動的に書き換え可能にする
    }

    // 3. 友達へのゲーム招待（Invite）をフックし、Cloudflare網経由で直接パケット送信
    #[no_mangle]
    pub extern "C" fn SteamFriends_InviteUserToGame(friend_id: u64, connect_str: &str) -> bool {
        // 本来Steamクライアントがやる処理を、直接steam-protoのパケットに変換してJS側へ丸投げ
        unsafe { js_trigger_steam_callback(103, friend_id); } // k_iSteamFriendsCallbacks + 3
        true
    }

    // 4. マッチング機能：ゲームが部屋（Lobby）を作ろうとしたら、WebRTC用の部屋番号を即時発行
    #[no_mangle]
    pub extern "C" fn SteamMatchmaking_CreateLobby(lobby_type: i32, _max_members: i32) {
        let mock_lobby_id = 9999888877776666u64; // あなたと友達を繋ぐ専用の固定/動的ロビーID
        // ゲームに対して「部屋の作成に成功したぞ」というニセのコールバックを1ミリ秒で発火
        unsafe { js_trigger_steam_callback(502, mock_lobby_id); } // LobbyCreated_t
    }

    // 5. 【極限ショートカット】数万行のネットワーク処理を破棄し、WebRTC Data Channelへ直結
    #[no_mangle]
    pub extern "C" fn SteamNetworkingMessages_SendMessageToUser(target_id: u64, data_ptr: *const u8, data_size: u32, _flags: i32) -> i32 {
        // ゲームが放ったx86_64由来の生パケット（位置情報・同期信号）を、メモリから直接スライスとして切り出し
        let packet = unsafe { std::slice::from_raw_parts(data_ptr, data_size as usize) };
        
        // 【核心】Goldbergの重いパケットラッピングを一切せず、生のままIPv6直通WebRTCへ右から左へ流す
        js_webrtc_send_packet(target_id, packet);
        0 // k_EResultOK (成功)
    }
}
