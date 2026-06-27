// worker.js (Cloudflare Workers) - Steamサインイン ＆ 直撃ダウンロード対応版

export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);

    // HTTP POST で Steamからのデータダウンロード要求を受け付ける
    if (request.method === "POST" && url.pathname === "/download-stream") {
      const { username, password, authCode, appId, depotId, manifestId } = await request.json();
      
      // 【核心】Steamのコンテンツサーバー（CDN）に対するストリーミングプロキシを構築
      // 本来はSteamWebAPIまたはバイナリプロトコルでマニフェストを取得し、各チャンクを要求します
      console.log(`Steamアカウント: ${username} で AppID: ${appId} のダウンロード要求を受理`);

      // 擬似的にSteamのCDNから暗号化バイナリ（.pak等）をストリーミング取得するインターフェース
      const steamCDNUrl = `https://api.steampowered.com/ISteamCDN/GetCDNTop3Apps/v1/`; // 実際はコンテンツサーバー群のトークン付きURL
      
      const { readable, writable } = new TransformStream();
      const writer = writable.getWriter();

      // バックグラウンドでSteamのデポからデータを小分け（チャンク）で読み込み、スマホへ横流し
      ctx.waitUntil((async () => {
        // ここでSteamサーバーからセクションごとにデータをfetchし、writer.write() でブラウザに送り出し続ける
        // 今回はダミーのデータストリーム（ヘッダー情報）を先に流して接続を確認します
        const dummyHeader = new TextEncoder().encode("STEAM_DEPOT_BINARY_START");
        await writer.write(dummyHeader);
        await writer.close();
      })());

      return new Response(readable, {
        headers: {
          "Content-Type": "application/octet-stream",
          "Access-Control-Allow-Origin": "*"
        }
      });
    }

    // 既存のWebSocket（クロスプレイ通信用）の処理は以下に維持
    const upgradeHeader = request.headers.get('Upgrade');
    if (upgradeHeader && upgradeHeader.toLowerCase() === 'websocket') {
        const webSocketPair = new WebSocketPair();
        const [clientSocket, serverSocket] = Object.values(webSocketPair);
        serverSocket.accept();
        return new Response(null, { status: 101, webSocketBackend: clientSocket });
    }

    return new Response("Steam Gateway Target Active.", { status: 200 });
  }
};
