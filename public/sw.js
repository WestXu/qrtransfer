// Service Worker for qrtransfer PWA
// Caches all static assets (including the WASM module) so the app works
// offline and the WASM binary is served instantly on subsequent visits.

const CACHE_VERSION = 'v1';
const CACHE_NAME = `qrtransfer-${CACHE_VERSION}`;

// Assets we know exist at deploy time and should be pre-cached on install.
const PRECACHE_URLS = [
  '/',
  '/manifest.json',
  '/favicon.svg',
];

// ── Install ──────────────────────────────────────────────────────────────────
// Pre-cache the shell HTML and manifest, then activate immediately.
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(PRECACHE_URLS))
  );
  // Skip the waiting phase so the new SW takes effect right away.
  self.skipWaiting();
});

// ── Activate ─────────────────────────────────────────────────────────────────
// Delete any caches belonging to an older version of this SW.
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((names) =>
      Promise.all(
        names
          .filter((name) => name !== CACHE_NAME)
          .map((name) => caches.delete(name))
      )
    )
  );
  // Take control of all open tabs immediately.
  self.clients.claim();
});

// ── Fetch ─────────────────────────────────────────────────────────────────────
// Strategy:
//   • WASM, JS, CSS, images  → cache-first  (large/stable binaries)
//   • HTML / everything else → network-first (always try to get fresh HTML)
self.addEventListener('fetch', (event) => {
  // Only handle same-origin GET requests.
  if (event.request.method !== 'GET') return;

  const url = new URL(event.request.url);
  if (url.origin !== self.location.origin) return;

  const path = url.pathname;
  const isStaticAsset =
    path.endsWith('.wasm') ||
    // Exclude the service worker itself so it can always be updated.
    (path.endsWith('.js') && path !== '/sw.js') ||
    path.endsWith('.css')  ||
    path.endsWith('.png')  ||
    path.endsWith('.jpg')  ||
    path.endsWith('.svg')  ||
    path.endsWith('.ico');

  if (isStaticAsset) {
    // Cache-first: serve from cache, fall back to network and populate cache.
    event.respondWith(
      caches.match(event.request).then((cached) => {
        if (cached) return cached;
        return fetch(event.request).then((response) => {
          if (response.ok) {
            const clone = response.clone();
            caches.open(CACHE_NAME).then((cache) => cache.put(event.request, clone));
          }
          return response;
        }).catch(() => caches.match(event.request));
      })
    );
  } else {
    // Network-first: try network, fall back to cache when offline.
    event.respondWith(
      fetch(event.request)
        .then((response) => {
          if (response.ok) {
            const clone = response.clone();
            caches.open(CACHE_NAME).then((cache) => cache.put(event.request, clone));
          }
          return response;
        })
        .catch(() => caches.match(event.request))
    );
  }
});
