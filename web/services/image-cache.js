(() => {
  let enabled = true;
  let nextId = 1;
  const imagesById = new Map();
  const dataByUrl = new Map();
  const pendingByUrl = new Map();
  const failedUrls = new Set();
  const originals = new WeakMap();
  const categoryOf = element => {
    const classes = `${element.className || ""} ${element.closest?.("[class]")?.className || ""}`.toLowerCase();
    if (classes.includes("screenshot")) return "screenshots";
    if (classes.includes("avatar")) return "avatars";
    if (classes.includes("feed")) return "feed";
    if (classes.includes("discover")) return "discover";
    if (classes.includes("banner") || classes.includes("hero")) return "banners";
    if (classes.includes("collection")) return "collections";
    return "posters";
  };
  const remote = url => /^https:\/\//i.test(String(url || ""));
  function loadImage(image) {
    const url = originals.get(image) || image.dataset.cacheUrl || image.getAttribute("src");
    if (!remote(url) || !enabled || image.dataset.cacheRequested === "true" || failedUrls.has(url)) return;
    if (dataByUrl.has(url)) {
      image.dataset.cacheUrl = url;
      image.src = dataByUrl.get(url);
      return;
    }
    image.dataset.cacheUrl = url;
    image.dataset.cacheRequested = "true";
    const waiter = { image, url };
    const pending = pendingByUrl.get(url) || [];
    pending.push(waiter);
    pendingByUrl.set(url, pending);
    if (pending.length > 1) return;
    const requestId = nextId++;
    imagesById.set(requestId, { url, waiters: pending });
    window.appIpc.send({ action: "cache_image", request_id: requestId, url, category: categoryOf(image) });
  }
  function apply(root = document) {
    const images = [...(root.matches?.("img[src]") ? [root] : []), ...(root.querySelectorAll?.("img[src]") || [])];
    images.forEach(image => {
      const src = image.getAttribute("src") || "";
      if (remote(src) && !originals.has(image)) originals.set(image, src);
      loadImage(image);
    });
  }
  function receive(message) {
    const id = Number(message?.payload?.request_id);
    const request = imagesById.get(id);
    if (!request) return;
    imagesById.delete(id);
    pendingByUrl.delete(request.url);
    const dataUrl = message.event === "cached_image_data" ? message.payload?.data_url : null;
    if (dataUrl) dataByUrl.set(request.url, dataUrl);
    else failedUrls.add(request.url);
    request.waiters.forEach(({ image, url }) => {
      if (!image.isConnected || image.dataset.cacheUrl !== url) return;
      delete image.dataset.cacheRequested;
      if (dataUrl) image.src = dataUrl;
      else image.src = url;
    });
  }
  function setEnabled(value) {
    enabled = Boolean(value);
    if (!enabled) {
      document.querySelectorAll("img[data-cache-url]").forEach(image => {
        image.src = originals.get(image) || image.dataset.cacheUrl;
        delete image.dataset.cacheRequested;
      });
      return;
    }
    document.querySelectorAll("img[src]").forEach(image => {
      const src = image.getAttribute("src") || "";
      if (remote(src)) originals.set(image, src);
      loadImage(image);
    });
  }
  window.appImageCache = { setEnabled, apply, receive, isEnabled: () => enabled };
  new MutationObserver(records => records.forEach(record => {
    if (record.type === "attributes" && record.target.nodeType === 1 && record.target.matches("img[src]")) {
      const image = record.target;
      const src = image.getAttribute("src") || "";
      if (remote(src)) {
        originals.set(image, src);
        delete image.dataset.cacheRequested;
        delete image.dataset.cacheUrl;
        loadImage(image);
      }
    }
    record.addedNodes.forEach(node => { if (node.nodeType === 1) apply(node); });
  })).observe(document.documentElement, { childList: true, subtree: true, attributes: true, attributeFilter: ["src"] });
})();
