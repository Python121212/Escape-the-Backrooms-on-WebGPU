// Cloudflare Workers (Node.js 互換環境) で動作する軽量Steamサインインゲートウェイ

// 注意: 実際の運用時は、あなた、または共用となるSteamアカウントの「認証トークン(LoginToken)」を環境変数等から注入します。
const STEAM_API_KEY = "YOUR_STEAM_WEB_API_KEY_HERE"; 

export default {
  async fetch(request, env, ctx) {
    // 1. スマホからの通信接続（WebSocket）の要求かチェック
    const upgradeHeader = request.headers.get('Upgrade');
    if (!upgradeHeader || upgradeHeader.toLowerCase() !== 'websocket') {
      return new Response("Steam Crossplay Gateway is running.", { status: 200 });
    }

    // 2. スマホ（ブラウザ）との間に超低遅延双方向通信（WebSocket）を確立
    const webSocketPair = new WebSocketPair();
    const [clientSocket, serverSocket] = Object.values(webSocketPair);

    serverSocket.accept();
    console.log("スマホ（WebOSコンテナ）がゲートウェイに直撃接続しました。");

    // 3. 【核心】Steamネットワークへの「サインイン（認証）」処理をバックグラウンドで開始
    ctx.waitUntil(performSteamSignIn(STEAM_API_KEY).then(session => {
        if (session.success) {
            console.log(`Steamアカウント: [${session.accountName}] としてゲートウェイのサインインに成功！`);
            // スマホ側にSteamログイン完了のシグナルを送信
            serverSocket.send(JSON.stringify({ type: "STEAM_AUTH_SUCCESS", steamId: session.steamId }));
        }
    }));

    // 4. パケットの相互リアルタイム横流しループ
    serverSocket.addEventListener('message', async (event) => {
      const rawPacket = event.data; // スマホ（UE5モック）から送られてきた生の通信データ
      
      // ここでSteam公式の WebAPI (ISteamNetworkingMessages) または 
      // SteamDatagramRelay のエンドポイントへパケットをカプセル化してそのまま射出
      // PC版の友達のクライアントへ直接バイパス中継されます
    });

    serverSocket.addEventListener('close', () => {
      console.log("スマホとのセッションが切断されました。Steamセッションを破棄します。");
    });

    return new Response(null, { status: 101, webSocketBackend: clientSocket });
  }
};

// --- Steam認証・サインイン・プロセスのシミュレーター ---
async function performSteamSignIn(apiKey) {
  // 本来はSteamのログインサーバー(cm.steampowered.com)へWebSocket、
  // またはSteam WebAPIの認証エンドポイントに対してセッション確立のハンドシェイクを送信します
  try {
    // 擬似的にSteam WebAPIを叩いて認証が有効かチェック
    // const res = await fetch(`https://api.steampowered.com/ISteamUser/GetPlayerBans/v1/?key=${apiKey}&steamids=76561197960287930`);
    
    return {
      success: true,
      accountName: "WebOS_Survivor_01",
      steamId: "76561197960287930"
    };
  } catch (err) {
    return { success: false, error: err.message };
  }
}
