'use strict';
const MANIFEST = 'flutter-app-manifest';
const TEMP = 'flutter-temp-cache';
const CACHE_NAME = 'flutter-app-cache';

const RESOURCES = {"sqlite3.wasm": "2068781fd3a05f89e76131a98da09b5b",
"icons/Icon-maskable-192.png": "c457ef57daa1d16f64b27b786ec2ea3c",
"icons/Icon-512.png": "96e752610906ba2a93c65f8abe1645f1",
"icons/Icon-192.png": "ac9a721a12bbc803b44f645561ecb1e1",
"icons/Icon-maskable-512.png": "301a7604d45b3e739efc881eb04896ea",
"version.json": "6d04cb54e307c269ac10de742cfd1f67",
"manifest.json": "9fd6ef9f20407e8b95a48e040ff54627",
"canvaskit/canvaskit.wasm": "567c5aeda8f673d1b76256d2682fd3b9",
"canvaskit/skwasm.js": "445e9e400085faead4493be2224d95aa",
"canvaskit/skwasm.wasm": "5a5c6171d2eed658e3b5fff70c4af82c",
"canvaskit/skwasm.js.symbols": "f1b800f6a9fac877f65a545b9f76c703",
"canvaskit/skwasm.worker.js": "bfb704a6c714a75da9ef320991e88b03",
"canvaskit/canvaskit.js.symbols": "d0b3bf92014e698856d505a65b5f0ee9",
"canvaskit/chromium/canvaskit.wasm": "e86670fe2af62cbdd3b75e098f393924",
"canvaskit/chromium/canvaskit.js.symbols": "e3a8db3bea434c929936f69d84e2f2bd",
"canvaskit/chromium/canvaskit.js": "2f82009588e8a72043db753d360d488f",
"canvaskit/canvaskit.js": "7737f5fc722b6a040ac15271ea8d92fb",
"index.html": "53cb28a73d96df41c3d45f219c94e5fc",
"/": "53cb28a73d96df41c3d45f219c94e5fc",
"flutter.js": "4af2b91eb221b73845365e1302528f07",
"favicon.png": "5dcef449791fa27946b3d35ad8803796",
"main.dart.js": "dad0ea8a5b117cbfd7a3bb48805b8486",
"assets/packages/rust_crypto/lib/assets/rust_crypto_wasm.wasm": "14534e63438ba10c961e08b595b6fdf7",
"assets/packages/image_rs/lib/assets/image_rs_wasm.wasm": "d52c060b4a28be14d611714d39910caa",
"assets/packages/wasm_parser/lib/assets/wasm_parser_wasm.wasm": "44713a36ff804554b9d0a9c110cdff3f",
"assets/packages/wasm_parser/lib/assets/wasm_parser_wasm.threads.wasm": "3be3bc21ee096503e103194a3e2146db",
"assets/packages/sql_parser/lib/assets/sql_parser_wasm.wasm": "8878571c71257a703649c640b38264d8",
"assets/packages/sql_parser/assets/sql_parser_wasm.wasm": "8878571c71257a703649c640b38264d8",
"assets/packages/cupertino_icons/assets/CupertinoIcons.ttf": "89ed8f4e49bcdfc0b5bfc9b24591e347",
"assets/packages/compression_rs/lib/assets/compression_rs_wasm.threads.wasm": "6275a3402960142f22089e2637bbeb9f",
"assets/packages/compression_rs/lib/assets/compression_rs_wasm.brotli.threads.wasm": "b299006721a7ff55f5fe740197f11390",
"assets/packages/compression_rs/lib/assets/compression_rs_wasm.wasm": "8b4d76dfb3af94ee9e2cfefae1fe4338",
"assets/packages/wasm_wit_component/lib/dart_wit_component.wasm": "a9a0d7d9e538cdd1bdea5a8c098f0264",
"assets/packages/wasm_run/lib/assets/wasm-feature-detect.js": "fbba857d68dfbf499b84606326f6c942",
"assets/packages/wasm_run/lib/assets/browser_wasi_shim.js": "91ba950eb53119a764eecec75ed5bdb5",
"assets/packages/wasm_run/lib/assets/wasm.worker.js": "100dc1f86c19f6ddcc6ddf64d81388df",
"assets/AssetManifest.bin": "5ec1a59b02f0ccececac55faf9450834",
"assets/shaders/ink_sparkle.frag": "ecc85a2e95f5e9f53123dcaf8cb9b6ce",
"assets/AssetManifest.json": "62d8a7fc2bf1591d5f58891e144b034c",
"assets/FontManifest.json": "dc3d03800ccca4601324923c0b1d6d57",
"assets/fonts/MaterialIcons-Regular.otf": "0367ab89f6b19c37c3289d9e180afedd",
"assets/AssetManifest.bin.json": "57f45a62fb1379cbf075607f0475dc5c",
"assets/NOTICES": "412712204c790793c961d0ae65f0dacd",
"assets/assets/images/flip-h.png": "c8a426b34045f7ba06af6514b3a6b71a",
"assets/assets/images/grayscale.png": "1fe2188f3f642d89a42ec217356c11de",
"assets/assets/images/resize-102x50.png": "bb6b0cdc54d6c2767663ae81b24d58cf",
"assets/assets/images/resize-102x50.jpg": "9d7d9b59f7084e0f722631588cad27b7",
"assets/assets/images/gray-rot90.png": "e75f0fe35ffea8cb4fc55ad6d39c7060",
"assets/assets/images/resize-102x50-exact.png": "8f69839de3138459405b2129b8029e0b",
"assets/assets/images/contrast20.png": "b3e27f84ccee2d8751af01f4cd9b4788",
"assets/assets/images/webp-convert.bmp": "fa31a8507ed15d8fc403be29c56d0752",
"assets/assets/images/code.png": "9e0cb968022c3d1082aa37b68e289def",
"assets/assets/images/crop-100x52.png": "552f1a0c26900da731e2c8ebde4471c2",
"assets/assets/images/convert.gif": "f4d7e51e364afd466232ba3bfc515e52",
"assets/assets/images/webp-example.webp": "d7ada0276744162efb83ff627557df21",
"assets/assets/images/webp-convert-blur2.bmp": "ea3246f73f967e2f3c375ead32b851da",
"assets/assets/google_fonts/Cousine/Cousine-Regular.ttf": "ea28fb808e427a5eaac9639551ac38dc",
"assets/assets/google_fonts/Cousine/LICENSE.txt": "d273d63619c9aeaf15cdaf76422c4f87",
"assets/assets/google_fonts/Cousine/Cousine-BoldItalic.ttf": "410ff03998bfb1355fe65e240438173c",
"assets/assets/google_fonts/Cousine/Cousine-Italic.ttf": "3a48c1f3958d278bb678665eab4fe75d",
"assets/assets/google_fonts/Cousine/Cousine-Bold.ttf": "0cce26797769173fad6c80adbf30e36a"};
// The application shell files that are downloaded before a service worker can
// start.
const CORE = ["main.dart.js",
"index.html",
"assets/AssetManifest.bin.json",
"assets/FontManifest.json"];

// During install, the TEMP cache is populated with the application shell files.
self.addEventListener("install", (event) => {
  self.skipWaiting();
  return event.waitUntil(
    caches.open(TEMP).then((cache) => {
      return cache.addAll(
        CORE.map((value) => new Request(value, {'cache': 'reload'})));
    })
  );
});
// During activate, the cache is populated with the temp files downloaded in
// install. If this service worker is upgrading from one with a saved
// MANIFEST, then use this to retain unchanged resource files.
self.addEventListener("activate", function(event) {
  return event.waitUntil(async function() {
    try {
      var contentCache = await caches.open(CACHE_NAME);
      var tempCache = await caches.open(TEMP);
      var manifestCache = await caches.open(MANIFEST);
      var manifest = await manifestCache.match('manifest');
      // When there is no prior manifest, clear the entire cache.
      if (!manifest) {
        await caches.delete(CACHE_NAME);
        contentCache = await caches.open(CACHE_NAME);
        for (var request of await tempCache.keys()) {
          var response = await tempCache.match(request);
          await contentCache.put(request, response);
        }
        await caches.delete(TEMP);
        // Save the manifest to make future upgrades efficient.
        await manifestCache.put('manifest', new Response(JSON.stringify(RESOURCES)));
        // Claim client to enable caching on first launch
        self.clients.claim();
        return;
      }
      var oldManifest = await manifest.json();
      var origin = self.location.origin;
      for (var request of await contentCache.keys()) {
        var key = request.url.substring(origin.length + 1);
        if (key == "") {
          key = "/";
        }
        // If a resource from the old manifest is not in the new cache, or if
        // the MD5 sum has changed, delete it. Otherwise the resource is left
        // in the cache and can be reused by the new service worker.
        if (!RESOURCES[key] || RESOURCES[key] != oldManifest[key]) {
          await contentCache.delete(request);
        }
      }
      // Populate the cache with the app shell TEMP files, potentially overwriting
      // cache files preserved above.
      for (var request of await tempCache.keys()) {
        var response = await tempCache.match(request);
        await contentCache.put(request, response);
      }
      await caches.delete(TEMP);
      // Save the manifest to make future upgrades efficient.
      await manifestCache.put('manifest', new Response(JSON.stringify(RESOURCES)));
      // Claim client to enable caching on first launch
      self.clients.claim();
      return;
    } catch (err) {
      // On an unhandled exception the state of the cache cannot be guaranteed.
      console.error('Failed to upgrade service worker: ' + err);
      await caches.delete(CACHE_NAME);
      await caches.delete(TEMP);
      await caches.delete(MANIFEST);
    }
  }());
});
// The fetch handler redirects requests for RESOURCE files to the service
// worker cache.
self.addEventListener("fetch", (event) => {
  if (event.request.method !== 'GET') {
    return;
  }
  var origin = self.location.origin;
  var key = event.request.url.substring(origin.length + 1);
  // Redirect URLs to the index.html
  if (key.indexOf('?v=') != -1) {
    key = key.split('?v=')[0];
  }
  if (event.request.url == origin || event.request.url.startsWith(origin + '/#') || key == '') {
    key = '/';
  }
  // If the URL is not the RESOURCE list then return to signal that the
  // browser should take over.
  if (!RESOURCES[key]) {
    return;
  }
  // If the URL is the index.html, perform an online-first request.
  if (key == '/') {
    return onlineFirst(event);
  }
  event.respondWith(caches.open(CACHE_NAME)
    .then((cache) =>  {
      return cache.match(event.request).then((response) => {
        // Either respond with the cached resource, or perform a fetch and
        // lazily populate the cache only if the resource was successfully fetched.
        return response || fetch(event.request).then((response) => {
          if (response && Boolean(response.ok)) {
            cache.put(event.request, response.clone());
          }
          return response;
        });
      })
    })
  );
});
self.addEventListener('message', (event) => {
  // SkipWaiting can be used to immediately activate a waiting service worker.
  // This will also require a page refresh triggered by the main worker.
  if (event.data === 'skipWaiting') {
    self.skipWaiting();
    return;
  }
  if (event.data === 'downloadOffline') {
    downloadOffline();
    return;
  }
});
// Download offline will check the RESOURCES for all files not in the cache
// and populate them.
async function downloadOffline() {
  var resources = [];
  var contentCache = await caches.open(CACHE_NAME);
  var currentContent = {};
  for (var request of await contentCache.keys()) {
    var key = request.url.substring(origin.length + 1);
    if (key == "") {
      key = "/";
    }
    currentContent[key] = true;
  }
  for (var resourceKey of Object.keys(RESOURCES)) {
    if (!currentContent[resourceKey]) {
      resources.push(resourceKey);
    }
  }
  return contentCache.addAll(resources);
}
// Attempt to download the resource online before falling back to
// the offline cache.
function onlineFirst(event) {
  return event.respondWith(
    fetch(event.request).then((response) => {
      return caches.open(CACHE_NAME).then((cache) => {
        cache.put(event.request, response.clone());
        return response;
      });
    }).catch((error) => {
      return caches.open(CACHE_NAME).then((cache) => {
        return cache.match(event.request).then((response) => {
          if (response != null) {
            return response;
          }
          throw error;
        });
      });
    })
  );
}