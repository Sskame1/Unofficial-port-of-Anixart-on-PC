(() => {
  const tabs = [
    { key: "history", label: "История" },
    { key: "favorites", label: "Избранное" },
    { key: "watching", label: "Смотрю" },
    { key: "planned", label: "В планах" },
    { key: "completed", label: "Просмотрено" },
    { key: "postponed", label: "Отложено" },
    { key: "dropped", label: "Брошено" }
  ];
  let currentTab = "history";
  let page = 1;
  let totalPages = 1;

  function renderBookmarks() {
    const root = document.getElementById("screen-content");
    if (!window.appStorage?.get("anixart_token")) {
      root.innerHTML = `<div class="placeholder account-placeholder"><div class="placeholder-icon">🔐</div><p>Войдите в аккаунт для просмотра закладок</p><button class="btn btn-primary" type="button" id="bookmarks-login">Войти в аккаунт</button></div>`;
      root.querySelector("#bookmarks-login")?.addEventListener("click", () => window.app.navigate("signin"));
      return;
    }
    root.innerHTML = `<div class="screen-header"><h1>Закладки</h1><p>История просмотра и списки релизов</p></div><div class="content-tabs bookmarks-tabs">${tabs.map(tab => `<button class="tab-item ${tab.key === currentTab ? "active" : ""}" data-tab="${tab.key}" type="button">${tab.label}</button>`).join("")}</div><div id="bookmarks-results" class="release-grid"><div class="placeholder"><div class="placeholder-icon">⏳</div><p>Загружаем список…</p></div></div><div id="bookmarks-pagination" class="filter-pagination" style="display:none"><button id="bookmarks-prev" type="button">← Назад</button><span id="bookmarks-page-info"></span><button id="bookmarks-next" type="button">Далее →</button></div>`;
    root.querySelectorAll("[data-tab]").forEach(button => button.addEventListener("click", () => { currentTab = button.dataset.tab; page = 1; renderBookmarks(); }));
    root.querySelector("#bookmarks-prev")?.addEventListener("click", () => { if (page > 1) { page -= 1; requestPage(); } });
    root.querySelector("#bookmarks-next")?.addEventListener("click", () => { if (page < totalPages) { page += 1; requestPage(); } });
    requestPage();
  }

  function requestPage() {
    window.appIpc.send(currentTab === "history" ? { action: "load_history", page } : { action: "load_bookmarks", kind: currentTab, page });
  }

  function showPage(response) {
    const results = document.getElementById("bookmarks-results");
    if (!results) return;
    totalPages = Math.max(1, response.total_page_count || 1);
    const releases = response.content || [];
    if (!releases.length) results.innerHTML = `<div class="placeholder"><div class="placeholder-icon">📭</div><p>В этом списке пока нет релизов</p></div>`;
    else results.replaceChildren(...releases.map(release => window.releaseCard.render(release)));
    const pagination = document.getElementById("bookmarks-pagination");
    if (pagination) {
      pagination.style.display = totalPages > 1 ? "flex" : "none";
      const info = document.getElementById("bookmarks-page-info");
      if (info) info.textContent = `Страница ${page} из ${totalPages}`;
      document.getElementById("bookmarks-prev")?.toggleAttribute("disabled", page <= 1);
      document.getElementById("bookmarks-next")?.toggleAttribute("disabled", page >= totalPages);
    }
  }

  function showError(message) {
    const results = document.getElementById("bookmarks-results");
    if (results) results.innerHTML = `<div class="placeholder"><div class="placeholder-icon">⚠️</div><p>Не удалось загрузить закладки</p><p class="error-message">${String(message || "Ошибка")}</p></div>`;
  }
  window.appScreens = { ...(window.appScreens || {}), bookmarks: renderBookmarks, refreshBookmarks: requestPage, showBookmarks: showPage, showBookmarksError: showError };
})();
