const WebSocket = require('ws');
const SteamUser = require('steam-user');

// ---------------------------------------------------------------------
// 1. Steamネットワークへのサインイン設定
// ---------------------------------------------------------------------
const client = new SteamUser();

// ※安全のため、環境変数から認証情報を読み込む設計にしています。
const STEAM_ACCOUNT = {
    accountName: process.env.STEAM_USERNAME || 'あなたのSteamアカウント名',
    password: process.env.STEAM_PASSWORD || 'あなたのSteamパスワード',
    // 2段階認証（Steam Guard）が有効な場合は、初回起動時にコンソールからコード入力を求められます
};

console.log("[Steam] 公式ネットワークへサインインを要求中...");
client.logOn(STEAM_ACCOUNT);

client.on('loggedOn', (details) => {
    console.log(`[Steam] サインイン成功！ サーバーSteamID: ${client.steamID.getSteamID64()}`);
    // ここでSteam上のステータスを「オンライン」かつ「Escape the Backroomsプレイ中」に偽装
    client.setPersona(SteamUser.EPersonaState.Online);
    client.gamesPlayed([1111310]); // Escape the BackroomsのSteam AppID
    console.log("[Steam] ステータス: 『Escape the Backrooms』を起動中として偽装完了");
});

client.on('error', (err) => {
    console.error("[Steam] サインインエラー:", err.message);
});

// ---------------------------------------------------------------------
// 2. スマホ（WebOSコンテナ）側を待ち受けるWebSocketサーバー
// ---------------------------------------------------------------------
const wss = new WebSocket.Server({ port: 8080 });
console.log("=== スマホ側接続用 WebSocketサーバー始動 (Port: 8080) ===");

// 接続されているスマホのソケットを管理
let mobileSocket = null;

wss.on('connection', (ws) => {
    console.log("[Gateway] スマホ（WebOSコンテナ）がゲートウェイに接続されました。");
    mobileSocket = ws;

    // スマホからパケット（バイナリ）が届いたときの処理
    ws.on('message', (message) => {
        const packet = Buffer.from(message);
        
        // 【核心：スマホ ➔ Steam網への射出】
        // スマホのWasm（UE5）が作ったパケットを、Steamの友達のSteamIDへ直接送信
        // 本来はパケットの先頭にあるターゲットSteamIDを解析して送信先を決めます
        // client.sendNetworkingMessage(targetSteamID, channel, packet);
        
        console.log(`[Bridge] スマホから ${packet.length} バイトのデータを取得 ➔ Steam網へ転送`);
    });

    ws.on('close', () => {
        console.log("[Gateway] スマホとの接続が切断されました。");
        mobileSocket = null;
    });
});

// ---------------------------------------------------------------------
// 3. 【核心：Steam網 ➔ スマホへの逆流流し込み】
// ---------------------------------------------------------------------
// PC版の友達のSteamから、このゲートウェイのアカウント宛てにパケットが届いた瞬間をキャッチ
client.on('networkingMessage', (steamID, channel, message) => {
    console.log(`[Bridge] PC版フレンド[${steamID.getSteamID64()}]からパケット受信`);

    // スマホ（WebOS）が現在接続中であれば、ノーディレイでWebSocketへそのまま横流し
    if (mobileSocket && mobileSocket.readyState === WebSocket.OPEN) {
        mobileSocket.send(message); 
    }
});
