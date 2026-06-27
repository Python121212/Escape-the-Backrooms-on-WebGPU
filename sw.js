const CACHE_NAME = 'v6-core-cache-v1';
const ASSETS = [
  './',
  './index.html',
  './pkg/escape_the_backrooms_on_webgpu.js',
  './pkg/escape_the_backrooms_on_webgpu_bg.wasm',
  './manifest.json'
];

// インストール時にコアファイルを一発キャッシュ
self.addEventListener('install', (e) => {
  e.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(ASSETS))
  );
});

// ネットワークを介さずキャッシュから爆速で引き出す（オフライン/0秒起動対応）
self.addEventListener('fetch', (e) => {
  e.respondWith(
    caches.match(e.request).then((response) => response || fetch(e.request))
  );
});
