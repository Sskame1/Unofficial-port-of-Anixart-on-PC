(() => {
  let feedKind = "mine";
  let feedPage = 1;
  let feedTotalPages = 1;
  function escape(value) {
    return String(value ?? "").replace(/[&<>"']/g, char => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[char]));
  }
  function value(data, keys) {
    if (!data || typeof data !== "object") return "";
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
    return fallback;
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
  function safeUrl(raw) {
    try { const url = new URL(String(raw || ""), location.href); return ["http:", "https:"].includes(url.protocol) ? url.href : ""; }
    catch (_) { return ""; }
  }
  function profileOf(item) { return item?.author || item?.profile || item?.data?.author || item?.data?.profile || {}; }
  function channelOf(item) { return item?.channel || item?.data?.channel || {}; }
  function renderBlocks(article) {
    const payload = article?.payload && typeof article.payload === "object" ? article.payload : {};
    const blocks = Array.isArray(payload.blocks) ? payload.blocks : [];
    const output = [];
    blocks.forEach(block => {
      const data = block?.data && typeof block.data === "object" ? block.data : block || {};
      const type = String(block?.type || block?.name || "").toLowerCase();
      const text = value(data, ["text", "value", "content", "caption", "title"]);
      if (text) output.push(`<div class="feed-text-block">${safeArticleHtml(text)}</div>`);
      const rawMedia = Array.isArray(data.items) ? data.items : Array.isArray(data.media) ? data.media : [];
      const media = rawMedia.map(item => typeof item === "string" ? { url: item } : item).map(item => safeUrl(value(item, ["url", "image", "src", "file"]))).filter(Boolean);
      if (type.includes("media") || media.length) output.push(`<div class="feed-media-grid">${media.map(url => `<img src="${escape(url)}" alt="" loading="lazy" referrerpolicy="no-referrer">`).join("")}</div>`);
      const release = data.release || data.release_card || data.anime;
      if (release && Number(release.id) > 0) output.push(`<button class="feed-release-link" type="button" data-release-id="${Number(release.id)}">${escape(value(release, ["title_ru", "title_original", "name"]) || "Открыть релиз")}</button>`);
    });
    return output.join("") || safeArticleHtml(value(article, ["description", "body", "content", "message", "text"])) || '<p class="feed-empty-body">Публикация без текста</p>';
  }
  function sendFeed(page = 1) {
    const requestId = (window.appFeedRequestId = (window.appFeedRequestId || 0) + 1);
    feedPage = page;
    window.appIpc.send({ action: "load_feed", kind: feedKind, page, request_id: requestId });
  }
  function renderFeed() {
    const root = document.getElementById("screen-content");
    const requestId = (window.appFeedRequestId = (window.appFeedRequestId || 0) + 1);
    if (feedKind === "mine" && !window.appStorage?.get("anixart_token")) {
      root.innerHTML = `<div class="placeholder account-placeholder"><div class="placeholder-icon">🔐</div><p>Войдите в аккаунт для просмотра ленты</p><button class="btn btn-primary" type="button" id="feed-login">Войти в аккаунт</button></div>`;
      root.querySelector("#feed-login")?.addEventListener("click", () => window.app.navigate("signin"));
      return;
    }
    root.innerHTML = `<section class="screen feed-screen"><header class="feed-toolbar"><label class="feed-search"><span aria-hidden="true">⌕</span><input type="search" placeholder="Поиск записей и блогов" aria-label="Поиск записей и блогов"></label><button type="button" class="feed-toolbar-icon" id="feed-refresh" title="Обновить">↻</button><button type="button" class="feed-toolbar-icon" data-route="notifications" title="Уведомления">♧</button></header><nav class="feed-tabs"><button type="button" data-feed-kind="mine" class="${feedKind === "mine" ? "is-active" : ""}">Моя лента</button><button type="button" data-feed-kind="latest" class="${feedKind === "latest" ? "is-active" : ""}">Свежее</button><button type="button" class="feed-tabs-icon" title="Избранные каналы">☆</button></nav><div class="feed-layout"><main><header class="feed-section-heading"><h1>${feedKind === "latest" ? "Свежее" : "Моя лента"}</h1><button type="button" id="feed-all">Все</button></header><div id="feed-content"><div class="placeholder"><div class="placeholder-icon">⏳</div><p>Загружаем ленту…</p></div></div><nav class="feed-pagination" id="feed-pagination" hidden><button type="button" id="feed-prev">←</button><span id="feed-page-info"></span><button type="button" id="feed-next">→</button></nav></main><aside class="feed-sidebar"><h2>Каналы</h2><p>Подпишитесь на блоги, чтобы видеть их публикации в своей ленте.</p><button type="button" class="btn btn-secondary" id="feed-channel-search">Найти каналы</button></aside></div></section>`;
    root.querySelectorAll("[data-feed-kind]").forEach(button => button.addEventListener("click", () => { feedKind = button.dataset.feedKind; renderFeed(); }));
    root.querySelector("#feed-refresh")?.addEventListener("click", () => sendFeed(1));
    root.querySelector("#feed-prev")?.addEventListener("click", () => sendFeed(Math.max(1, feedPage - 1)));
    root.querySelector("#feed-next")?.addEventListener("click", () => sendFeed(Math.min(feedTotalPages, feedPage + 1)));
    window.appIpc.send({ action: "load_feed", kind: feedKind, page: 1, request_id: requestId });
  }
  function showFeed(response) {
    if (response?.request_id && response.request_id !== window.appFeedRequestId) return;
    response = response?.data || response || {};
    const root = document.getElementById("feed-content");
    if (!root) return;
    const items = response.content || [];
    if (!items.length) { root.innerHTML = `<div class="placeholder"><div class="placeholder-icon">📭</div><p>В ленте пока нет публикаций</p></div>`; return; }
    feedPage = Number(response.current_page) === 0 ? 1 : Number(response.current_page) || feedPage;
    feedTotalPages = Math.max(1, Number(response.total_page_count) || 1);
    root.innerHTML = items.map(item => {
      const data = item?.data && typeof item.data === "object" ? item.data : item;
      const profile = profileOf(item);
      const channelData = channelOf(item);
      const title = value(item, ["title", "name"]) || value(data, ["title", "name"]) || value(item?.channel, ["name", "title"]) || "Публикация Anixart";
      const rawDescription = articleText(item) || value(data, ["description", "body", "content", "message", "text"]);
      const author = typeof profile === "object" ? value(profile, ["login", "name"]) : value(data, ["login", "author"]);
      const channel = typeof channelData === "object" ? value(channelData, ["name", "title"]) : "";
      const stamp = Number(value(item, ["creation_date", "created_date", "timestamp"]) || value(data, ["creation_date", "created_date", "timestamp"])) || 0;
      const date = stamp > 1000000000 ? new Date(stamp * 1000).toLocaleString("ru-RU", { day: "2-digit", month: "short", hour: "2-digit", minute: "2-digit" }) : value(item, ["date", "published_at"]);
      const image = safeUrl(value(profile, ["avatar", "image"]) || value(channelData, ["avatar", "image"]));
      const body = renderBlocks(item);
      const commentCount = Number(item.comment_count) || 0;
      const voteCount = Number(item.vote_count) || 0;
      return `<article class="feed-post" data-article-id="${Number(item.id) || 0}"><header class="feed-post-header">${image ? `<img class="feed-post-avatar" src="${escape(image)}" alt="" loading="lazy" referrerpolicy="no-referrer">` : '<span class="feed-post-avatar feed-post-avatar-empty">◉</span>'}<div class="feed-post-author"><strong>${escape(author || "Пользователь")}</strong><small>${escape(date)}</small></div>${channel ? `<span class="feed-post-channel">${escape(channel)}</span>` : ""}<button class="feed-follow-button" type="button">Подписаться</button><button class="feed-more-button" type="button" aria-label="Дополнительно">⋮</button></header><button class="feed-post-title" type="button">${escape(title)}</button><div class="feed-post-body">${body}</div><button class="feed-expand-button" type="button">Показать ещё</button><footer class="feed-post-actions"><button type="button" data-feed-action="comments">💬 ${commentCount}</button><button type="button" data-feed-action="replies">↩ ${Number(item.repost_count) || 0}</button><button type="button" data-feed-action="vote">♥ ${voteCount}</button></footer></article>`;
    }).join("");
    root.querySelectorAll(".feed-post").forEach(card => {
      card.querySelector(".feed-expand-button")?.addEventListener("click", event => { card.classList.toggle("is-expanded"); event.currentTarget.textContent = card.classList.contains("is-expanded") ? "Свернуть" : "Показать ещё"; });
      card.querySelectorAll("[data-release-id]").forEach(button => button.addEventListener("click", () => window.appIpc.send({ action: "open_release", release_id: Number(button.dataset.releaseId) })));
      card.querySelector("[data-feed-action=comments]")?.addEventListener("click", () => card.querySelector(".feed-post-title")?.click());
      card.querySelector(".feed-post-title")?.addEventListener("click", () => {
        const id = Number(card.dataset.articleId);
        if (!id) return;
        document.getElementById("article-modal")?.remove();
        root.insertAdjacentHTML("beforeend", '<div class="friends-overlay" id="article-modal"><article class="friends-dialog article-dialog"><header><h2>Публикация</h2><button type="button" id="article-close" aria-label="Закрыть">×</button></header><p class="placeholder">Загружаем публикацию…</p></article></div>');
        root.querySelector("#article-close")?.addEventListener("click", () => document.getElementById("article-modal")?.remove());
        window.appIpc.send({ action: "load_article", article_id: id });
      });
    });
    const pagination = document.getElementById("feed-pagination");
    if (pagination) { pagination.hidden = feedTotalPages <= 1; document.getElementById("feed-page-info").textContent = `${feedPage} / ${feedTotalPages}`; document.getElementById("feed-prev").disabled = feedPage <= 1; document.getElementById("feed-next").disabled = feedPage >= feedTotalPages; }
  }
  function showError(message) {
    const root = document.getElementById("feed-content");
    if (root) root.innerHTML = `<div class="placeholder"><div class="placeholder-icon">⚠️</div><p>Не удалось загрузить ленту</p><p class="error-message">${escape(message)}</p></div>`;
  }
  function showArticle(article) {
    const dialog = document.querySelector("#article-modal .article-dialog");
    if (!dialog) return;
    const profile = profileOf(article); const channelData = channelOf(article); const image = safeUrl(value(profile, ["avatar", "image"]) || value(channelData, ["avatar", "image"]));
    const channel = escape(value(channelData, ["name", "title"]));
    dialog.innerHTML = `<header><h2>${channel || "Публикация"}</h2><button type="button" id="article-close" aria-label="Закрыть">×</button></header><div class="article-author">${image ? `<img class="article-avatar" src="${escape(image)}" alt="">` : ""}<strong>${escape(value(profile, ["login", "name"]) || "Пользователь")}</strong></div><div class="article-full-body">${renderBlocks(article)}</div><footer class="article-actions">💬 ${Number(article.comment_count) || 0} · ♥ ${Number(article.vote_count) || 0}</footer>`;
    dialog.querySelector("#article-close")?.addEventListener("click", () => document.getElementById("article-modal")?.remove());
  }
  function showArticleError(message) {
    const dialog = document.querySelector("#article-modal .article-dialog");
    if (dialog) dialog.innerHTML = `<header><h2>Публикация</h2><button type="button" id="article-close">×</button></header><p>Не удалось открыть публикацию: ${escape(message)}</p>`;
    dialog?.querySelector("#article-close")?.addEventListener("click", () => document.getElementById("article-modal")?.remove());
  }
  window.appScreens = { ...(window.appScreens || {}), feed: renderFeed, refreshFeed: () => sendFeed(1), showFeed, showFeedError: showError, showArticle, showArticleError };
})();
