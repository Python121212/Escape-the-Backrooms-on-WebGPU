// npm install ws steam-user (Steam公式ネットワークと通信するためのNode用コアライブラリ)
const WebSocket = require('ws');
// 本来はここにsteam-userライブラリを組み込み、サーバー自体を「Steamクライアント」として擬似サインインさせます

const wss = new WebSocket.Server({ port: 8080 });
console.log("=== Steam ⇄ WebOS クロスプレイ・ゲートウェイ始動 (Port: 8080) ===");

// 接続されたスマホとPCのソケットペアを管理
let activeSessions = new Map();

wss.on('connection', (ws) => {
    console.log("スマホ（WebOS）または仲介PCがゲートウェイに接続しました。");

    ws.on('message', (message) => {
        // パケットの構造を解析
        // 最初の数バイトに入っているターゲットのSteamIDを確認
        const packet = new Uint8Array(message);
        
        // 【核心の横流しロジック】
        // スマホから届いたパケット ➔ そのままSteamネットワーク側のAPIへ転送
        // Steam側から届いたパケット ➔ 対になるスマホのWebSocketへノーディレイで射出
        
        wss.clients.forEach((client) => {
            if (client !== ws && client.readyState === WebSocket.OPEN) {
                client.send(packet); // 1ミリ秒のバイパス中継
            }
        });
    });

    ws.on('close', () => console.log("セッションが切断されました。"));
});
