(() => {
  let kind = "popular";
  let page = 1;
  let totalPages = 1;

  const tabs = [
    ["popular", "Популярные"],
    ["new", "Новые"],
    ["mine", "Мои коллекции"]
  ];

  function escape(value) {
    return String(value ?? "").replace(/[&<>"']/g, c => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]));
  }

  function renderCollections() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen collections-screen">
        <header class="feature-header collection-header">
          <button class="settings-back" type="button">← Обзор</button>
          <div><h1>Коллекции</h1><p>Подборки релизов сообщества</p></div>
        </header>
        <nav class="collection-tabs" role="tablist">
          ${tabs.map(([key, label]) => `<button type="button" role="tab" data-kind="${key}" class="${key === kind ? "is-active" : ""}">${label}</button>`).join("")}
        </nav>
        <div id="collections-results" class="collection-list"><div class="placeholder"><div class="placeholder-icon">⏳</div><p>Загружаем коллекции…</p></div></div>
        <div id="collections-pagination" class="collection-pagination" hidden>
          <button class="btn btn-secondary" id="collections-prev" type="button">← Назад</button>
          <span id="collections-page-info"></span>
          <button class="btn btn-secondary" id="collections-next" type="button">Далее →</button>
        </div>
      </section>`;
    root.querySelector(".settings-back").addEventListener("click", () => window.app.navigate("discover"));
    root.querySelectorAll(".collection-tabs button").forEach(button => button.addEventListener("click", () => {
      kind = button.dataset.kind;
      page = 1;
      root.querySelectorAll(".collection-tabs button").forEach(tab => tab.classList.toggle("is-active", tab === button));
      loadCollections();
    }));
    root.querySelector("#collections-prev").addEventListener("click", () => { if (page > 1) { page--; loadCollections(); } });
    root.querySelector("#collections-next").addEventListener("click", () => { if (page < totalPages) { page++; loadCollections(); } });
    loadCollections();
  }

  function loadCollections() {
    const root = document.getElementById("collections-results");
    if (root) root.innerHTML = '<div class="placeholder"><div class="placeholder-icon">⏳</div><p>Загружаем коллекции…</p></div>';
    const requestId = (window.appCollectionRequestId = (window.appCollectionRequestId || 0) + 1);
    window.appIpc.send({ action: "load_collections", kind, page, request_id: requestId });
  }

  function showCollections(response) {
    if (response?.request_id && response.request_id !== window.appCollectionRequestId) return;
    response = response?.data || response || {};
    const root = document.getElementById("collections-results");
    if (!root) return;
    totalPages = Number(response.total_page_count) || 1;
    const items = response.content || [];
    if (!items.length) {
      root.innerHTML = `<div class="placeholder"><div class="placeholder-icon">📚</div><p>${kind === "mine" ? "У вас пока нет коллекций" : "Коллекции не найдены"}</p></div>`;
    } else {
      root.innerHTML = items.map(collection => {
        const title = escape(collection.title || "Коллекция без названия");
        const description = escape(collection.description || "Нет описания");
        const creator = escape(collection.creator?.login || "Пользователь Anixart");
        const image = collection.image ? `<img src="${escape(collection.image)}" alt="" loading="lazy" referrerpolicy="no-referrer">` : `<span>▦</span>`;
        return `<button type="button" class="collection-card" data-id="${Number(collection.id) || 0}"><div class="collection-card-image">${image}</div><div class="collection-card-body"><strong>${title}</strong><span class="collection-card-author">${creator}</span><p>${description}</p><small>♥ ${Number(collection.favorites_count) || 0} · 💬 ${Number(collection.comment_count) || 0}</small></div></button>`;
      }).join("");
      root.querySelectorAll(".collection-card").forEach(card => card.addEventListener("click", () => {
        const id = Number(card.dataset.id);
        if (id > 0) {
          const requestId = (window.appCollectionRequestId = (window.appCollectionRequestId || 0) + 1);
          window.appIpc.send({ action: "open_collection", collection_id: id, request_id: requestId });
        }
      }));
    }
    const pagination = document.getElementById("collections-pagination");
    if (pagination) {
      pagination.hidden = totalPages <= 1;
      document.getElementById("collections-page-info").textContent = `Страница ${page} из ${totalPages}`;
      document.getElementById("collections-prev").disabled = page <= 1;
      document.getElementById("collections-next").disabled = page >= totalPages;
    }
  }

  function showError(message) {
    const root = document.getElementById("collections-results");
    if (root) root.innerHTML = `<div class="placeholder"><div class="placeholder-icon">⚠️</div><p>Не удалось загрузить коллекции</p><p class="error-message">${escape(message)}</p><button class="btn btn-secondary" id="retry-collections" type="button">Повторить</button></div>`;
    root?.querySelector("#retry-collections")?.addEventListener("click", loadCollections);
  }

  function showCollection(response) {
    if (response?.request_id && response.request_id !== window.appCollectionRequestId) return;
    response = response?.data || response || {};
    const collection = response.collection;
    const root = document.getElementById("collections-results");
    if (!root || !collection) return showError("Коллекция не найдена");
    const created = Number(collection.creation_date) > 0 ? new Date(Number(collection.creation_date) * 1000).toLocaleDateString("ru-RU") : "";
    root.innerHTML = `<article class="collection-detail"><button class="btn btn-secondary" id="collection-back" type="button">← Все коллекции</button><div class="collection-detail-hero">${collection.image ? `<img src="${escape(collection.image)}" alt="" loading="lazy">` : ""}<div><h2>${escape(collection.title)}</h2><p>${escape(collection.description || "Нет описания")}</p><small>♥ <span id="collection-likes">${Number(collection.favorites_count) || 0}</span> · 💬 ${Number(collection.comment_count) || 0}${created ? ` · Создана ${created}` : ""}</small><button class="btn btn-secondary collection-like" id="collection-like" type="button">${collection.is_favorite ? "♥ В закладках" : "♡ Добавить в закладки"}</button></div></div><h3 class="collection-releases-title">Релизы в коллекции</h3><div id="collection-releases" class="release-grid"></div><section class="collection-comments"><h3>Комментарии</h3><div id="collection-comments-list"></div></section></article>`;
    const releases = response.releases || [];
    const releasesRoot = root.querySelector("#collection-releases");
    if (!releases.length) releasesRoot.innerHTML = '<p class="placeholder">В коллекции пока нет релизов.</p>';
    else releasesRoot.replaceChildren(...releases.map(release => window.releaseCard.render(release)));
    root.querySelector("#collection-back")?.addEventListener("click", loadCollections);
    const likeButton = root.querySelector("#collection-like");
    likeButton?.addEventListener("click", () => {
      const enabled = likeButton.dataset.enabled !== "true";
      likeButton.disabled = true;
      window.appIpc.send({ action: "toggle_collection_favorite", collection_id: collection.id, enabled });
      likeButton.dataset.pending = String(enabled);
    });
    const comments = response.comments || [];
    const commentsRoot = root.querySelector("#collection-comments-list");
    commentsRoot.innerHTML = comments.length ? comments.map(comment => `<article class="collection-comment"><strong>${escape(comment.profile?.login || "Пользователь")}</strong><p>${escape(comment.message)}</p>${Number(comment.creation_date) > 0 ? `<small>${new Date(Number(comment.creation_date) * 1000).toLocaleString("ru-RU")}</small>` : ""}</article>`).join("") : '<p class="placeholder">Комментариев пока нет.</p>';
    likeButton.dataset.enabled = String(Boolean(collection.is_favorite));
  }

  function favoriteDone(payload) {
    const button = document.getElementById("collection-like");
    if (!button) return;
    button.disabled = false;
    const enabled = Boolean(payload.enabled);
    button.dataset.enabled = String(enabled);
    button.textContent = enabled ? "♥ В закладках" : "♡ Добавить в закладки";
  }
  function favoriteError(message) {
    const button = document.getElementById("collection-like");
    if (button) button.disabled = false;
    window.app.setStatus("Коллекции", message || "Не удалось изменить закладку");
  }

  window.appScreens = { ...(window.appScreens || {}), collections: renderCollections, showCollections, showCollectionsError: showError, showCollection, collectionFavoriteDone: favoriteDone, collectionFavoriteError: favoriteError };
})();
