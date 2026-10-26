(() => {
  const tabs = [
    { key: "anime", label: "Аниме" },
    { key: "donghua", label: "Дунхуа" },
    { key: "latest", label: "Последнее" },
    { key: "ongoing", label: "Онгоинги" },
    { key: "announced", label: "Анонсы" },
    { key: "completed", label: "Завершенные" },
    { key: "movies", label: "Фильмы" },
    { key: "ova", label: "OVA" }
  ];

  let currentTab = "anime";
  let page = 1;

  const tabFilters = {
    anime: { country: "Япония" },
    donghua: { country: "Китай" },
    latest: { sort: 0 },
    ongoing: { status_id: 2 },
    announced: { status_id: 3 },
    completed: { status_id: 1 },
    movies: { category_id: 2 },
    ova: { category_id: 3 }
  };

  function renderHome() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <div class="screen-header home-header">
        <div>
          <h1>Главная</h1>
          <p>Каталог аниме и дорам</p>
        </div>
        <button class="header-search-button" id="home-search" type="button" aria-label="Поиск аниме" title="Поиск аниме">
          <svg aria-hidden="true" viewBox="0 0 24 24" focusable="false"><circle cx="11" cy="11" r="6.5"></circle><path d="m16 16 4.5 4.5"></path></svg>
          <span>Поиск</span>
        </button>
      </div>

      <div class="content-tabs">
        ${tabs.map(tab => `
          <button class="tab-item ${tab.key === currentTab ? "active" : ""}" data-tab="${tab.key}">
            ${tab.label}
          </button>
        `).join("")}
      </div>

      <div id="home-results" class="release-grid">
        <div class="placeholder">
          <div class="placeholder-icon">📺</div>
          <p>Загружаем релизы...</p>
        </div>
      </div>
    `;

    root.querySelectorAll(".tab-item").forEach(button => {
      button.addEventListener("click", () => {
        currentTab = button.dataset.tab;
        page = 1;
        renderHome();
      });
    });

    root.querySelector("#home-search")?.addEventListener("click", () => window.app.navigate("search"));

    loadContent();
  }

  function loadContent() {
    const results = document.getElementById("home-results");
    if (results) {
      results.innerHTML = '<div class="placeholder"><div class="placeholder-icon">⏳</div><p>Загружаем релизы...</p></div>';
    }
    window.appIpc.send({
      action: "apply_filter",
      page,
      filter: {
        ...(tabFilters[currentTab] || {}),
        sort: tabFilters[currentTab]?.sort ?? 0,
        genres: [],
        types: tabFilters[currentTab]?.types || [],
        age_ratings: [],
        profile_list_exclusions: [],
        is_genres_exclude_mode_enabled: false
      }
    });
  }

  function showResults(response) {
    const results = document.getElementById("home-results");
    if (!results) return;
    const content = response.content || [];
    if (!content.length) {
      results.innerHTML = '<div class="placeholder"><div class="placeholder-icon">📭</div><p>В этом разделе пока нет релизов</p></div>';
      return;
    }
    results.replaceChildren(...content.map(release => window.releaseCard.render(release)));
  }

  window.appScreens = { ...(window.appScreens || {}), home: renderHome, refreshHome: loadContent, showHomeResults: showResults };
})();
