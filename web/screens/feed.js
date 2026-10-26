(() => {
  function escape(value) {
    return String(value ?? "").replace(/[&<>"']/g, char => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[char]));
  }
  function value(data, keys) {
    for (const key of keys) if (data?.[key] !== undefined && data[key] !== null && data[key] !== "") return data[key];
    return "";
  }
  function textFromBlock(block) {
    if (!block || typeof block !== "object") return "";
    const data = block.data && typeof block.data === "object" ? block.data : block;
    return value(data, ["text", "value", "content", "caption", "title"]);
  }
  function articleText(item) {
    const payload = item?.payload && typeof item.payload === "object" ? item.payload : null;
    const blocks = Array.isArray(payload?.blocks) ? payload.blocks : [];
    const fallback = blocks.map(textFromBlock).filter(Boolean).join("\n\n");
    return fallback || (payload?.blocks ? JSON.stringify(payload.blocks) : "");
  }
  function safeArticleHtml(raw) {
    const decoder = document.createElement("textarea");
    decoder.innerHTML = String(raw ?? "");
    const template = document.createElement("template");
    template.innerHTML = decoder.value;
    const allowed = new Set(["B", "STRONG", "I", "EM", "U", "S", "BR", "P", "UL", "OL", "LI", "BLOCKQUOTE", "A"]);
    const walk = node => {
      [...node.childNodes].forEach(child => {
        if (child.nodeType === Node.ELEMENT_NODE) {
          if (!allowed.has(child.tagName)) {
            child.replaceWith(...child.childNodes);
            return;
          }
          const originalHref = child.tagName === "A" ? child.getAttribute("href") : null;
          [...child.attributes].forEach(attr => child.removeAttribute(attr.name));
          if (child.tagName === "A") {
            const candidate = originalHref || "";
            try {
              const url = new URL(candidate, location.href);
              if (["http:", "https:"].includes(url.protocol)) { child.href = url.href; child.target = "_blank"; child.rel = "noopener noreferrer"; }
            } catch { child.replaceWith(...child.childNodes); return; }
          }
          walk(child);
        }
      });
    };
    walk(template.content);
    return template.innerHTML;
  }
  function renderFeed() {
    const root = document.getElementById("screen-content");
    const requestId = (window.appFeedRequestId = (window.appFeedRequestId || 0) + 1);
    if (!window.appStorage?.get("anixart_token")) {
      root.innerHTML = `<div class="placeholder account-placeholder"><div class="placeholder-icon">🔐</div><p>Войдите в аккаунт для просмотра ленты</p><button class="btn btn-primary" type="button" id="feed-login">Войти в аккаунт</button></div>`;
      root.querySelector("#feed-login")?.addEventListener("click", () => window.app.navigate("signin"));
      return;
    }
    root.innerHTML = `<div class="screen-header"><h1>Лента</h1><p>Обновления и публикации сообщества</p></div><div id="feed-content"><div class="placeholder"><div class="placeholder-icon">⏳</div><p>Загружаем ленту…</p></div></div>`;
    window.appIpc.send({ action: "load_feed", page: 1, request_id: requestId });
  }
  function showFeed(response) {
    if (response?.request_id && response.request_id !== window.appFeedRequestId) return;
    response = response?.data || response || {};
    const root = document.getElementById("feed-content");
    if (!root) return;
    const items = response.content || [];
    if (!items.length) { root.innerHTML = `<div class="placeholder"><div class="placeholder-icon">📭</div><p>В ленте пока нет публикаций</p></div>`; return; }
    root.innerHTML = items.map(item => {
      const data = item?.data && typeof item.data === "object" ? item.data : item;
      const profile = item?.profile || data?.profile || item?.author || data?.author;
      const title = value(item, ["title", "name"]) || value(data, ["title", "name"]) || value(item?.channel, ["name", "title"]) || "Публикация Anixart";
      const rawDescription = articleText(item) || value(data, ["description", "body", "content", "message", "text"]);
      const author = typeof profile === "object" ? value(profile, ["login", "name"]) : value(data, ["login", "author"]);
      const channel = typeof item?.channel === "object" ? value(item.channel, ["name", "title"]) : "";
      const date = value(item, ["created_date", "date", "published_at"]) || value(data, ["created_date", "date", "published_at"]);
      const image = value(item, ["image", "avatar", "poster"]) || value(profile, ["avatar", "image"]) || value(data, ["image", "avatar", "poster"]);
      const description = safeArticleHtml(rawDescription);
      return `<article class="feed-card" tabindex="0" role="button" data-article-id="${Number(item.id) || 0}"><div class="feed-card-body"><div class="feed-card-top">${image ? `<img class="feed-card-avatar" src="${escape(image)}" alt="" loading="lazy" referrerpolicy="no-referrer">` : ""}<div><div class="feed-card-title">${escape(title)}</div>${author || channel ? `<div class="feed-card-author">${escape([author, channel].filter(Boolean).join(" · "))}</div>` : ""}</div></div><div class="feed-card-copy">${description}</div>${date ? `<time>${escape(date)}</time>` : ""}</div></article>`;
    }).join("");
    root.querySelectorAll(".feed-card").forEach(card => card.addEventListener("click", () => {
      const id = Number(card.dataset.articleId);
      if (id > 0) {
        document.getElementById("article-modal")?.remove();
        root.insertAdjacentHTML("beforeend", '<div class="friends-overlay" id="article-modal"><article class="friends-dialog article-dialog"><header><h2>Публикация</h2><button type="button" id="article-close" aria-label="Закрыть">×</button></header><p class="placeholder">Загружаем публикацию…</p></article></div>');
        root.querySelector("#article-close")?.addEventListener("click", () => root.querySelector("#article-modal")?.remove());
        window.appIpc.send({ action: "load_article", article_id: id });
      } else card.classList.toggle("is-expanded");
    }));
  }
  function showError(message) {
    const root = document.getElementById("feed-content");
    if (root) root.innerHTML = `<div class="placeholder"><div class="placeholder-icon">⚠️</div><p>Не удалось загрузить ленту</p><p class="error-message">${escape(message)}</p></div>`;
  }
  function showArticle(article) {
    const dialog = document.querySelector("#article-modal .article-dialog");
    if (!dialog) return;
    const payload = article.payload || {};
    const body = safeArticleHtml(articleText(article) || "Пустая публикация");
    const author = escape(article.author?.login || article.profile?.login || "Пользователь");
    const channel = escape(article.channel?.name || "");
    dialog.innerHTML = `<header><h2>${channel || "Публикация"}</h2><button type="button" id="article-close" aria-label="Закрыть">×</button></header><div class="article-author">${author}</div><div class="article-full-body">${body}</div>`;
    dialog.querySelector("#article-close")?.addEventListener("click", () => document.getElementById("article-modal")?.remove());
  }
  function showArticleError(message) {
    const dialog = document.querySelector("#article-modal .article-dialog");
    if (dialog) dialog.innerHTML = `<header><h2>Публикация</h2><button type="button" id="article-close">×</button></header><p>Не удалось открыть публикацию: ${escape(message)}</p>`;
    dialog?.querySelector("#article-close")?.addEventListener("click", () => document.getElementById("article-modal")?.remove());
  }
  window.appScreens = { ...(window.appScreens || {}), feed: renderFeed, refreshFeed: () => {
    const requestId = (window.appFeedRequestId = (window.appFeedRequestId || 0) + 1);
    window.appIpc.send({ action: "load_feed", page: 1, request_id: requestId });
  }, showFeed, showFeedError: showError, showArticle, showArticleError };
})();
