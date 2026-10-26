(() => {
  let searchQuery = "";
  let searchResults = [];

  function renderSearch() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <div class="screen-header">
        <h1>Поиск</h1>
        <p>Найдите аниме по названию</p>
      </div>

      <div class="profile-section">
        <div style="display: flex; gap: 12px;">
          <input
            type="text"
            id="search-input"
            placeholder="Введите название аниме..."
            style="flex: 1; padding: 12px 16px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-primary); color: var(--text-primary); font-size: 15px;"
          >
          <button class="btn btn-primary" id="search-btn" style="padding: 12px 24px;">
            Искать
          </button>
        </div>
      </div>

      <div id="search-results" class="release-grid"></div>
    `;

    const input = root.querySelector("#search-input");
    const btn = root.querySelector("#search-btn");
    input.value = searchQuery;

    const performSearch = () => {
      searchQuery = input.value.trim();
      if (searchQuery) {
        const results = document.getElementById("search-results");
        results.innerHTML = `
          <div class="placeholder">
            <div class="placeholder-icon">🔍</div>
            <p>Ищем "${searchQuery}"...</p>
          </div>
        `;
        window.appIpc.send({ action: "search_releases", query: searchQuery });
      }
    };

    btn.addEventListener("click", performSearch);
    input.addEventListener("keypress", (e) => {
      if (e.key === "Enter") performSearch();
    });

    input.focus();
  }

  function showResults(releases) {
    const results = document.getElementById("search-results");
    if (!results) return;

    if (!releases || releases.length === 0) {
      results.innerHTML = `
        <div class="placeholder">
          <div class="placeholder-icon">📭</div>
          <p>По запросу "${searchQuery}" ничего не найдено</p>
        </div>
      `;
      return;
    }

    searchResults = releases;
    results.replaceChildren(...releases.map(release => window.releaseCard.render(release)));
  }

  function showError(message) {
    const results = document.getElementById("search-results");
    if (results) {
      results.innerHTML = `
        <div class="placeholder">
          <div class="placeholder-icon">⚠️</div>
          <p>Ошибка поиска: ${message}</p>
        </div>
      `;
    }
  }

  window.appScreens = {
    ...(window.appScreens || {}),
    search: renderSearch,
    refreshSearch: () => {
      if (!searchQuery) return;
      const results = document.getElementById("search-results");
      if (results) results.innerHTML = '<div class="placeholder"><div class="placeholder-icon">🔍</div><p>Обновляем поиск…</p></div>';
      window.appIpc.send({ action: "search_releases", query: searchQuery });
    },
    showSearchResults: showResults,
    showSearchError: showError
  };
})();
