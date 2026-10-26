(() => {
  const tabs = [
    ["recommendations", "Рекомендации"],
    ["interesting", "Интересное"],
    ["discussing", "Обсуждают"],
    ["watching", "Смотрят"]
  ];
  let kind = "recommendations";
  let page = 1;
  let totalPages = 1;

  function renderDiscover() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen">
        <h1 class="screen-title">Обзор</h1>
        <p class="screen-subtitle">Подборки каталога Anixart</p>
        <nav class="collection-tabs">
          ${tabs.map(([key, label]) => 
            `<button type="button" data-kind="${key}" class="${key === kind ? "is-active" : ""}">${label}</button>`
          ).join("")}
          <button type="button" data-route="filter">Фильтр</button>
        </nav>
        <div id="discover-result" class="release-grid" aria-live="polite">
          <p class="placeholder">Загружаем каталог…</p>
        </div>
        <div class="filter-actions">
          <button id="discover-prev" type="button" disabled>← Назад</button>
          <span class="page-info">Страница <span id="current-page">1</span> из <span id="total-pages">1</span></span>
          <button id="discover-next" type="button">Ещё →</button>
        </div>
      </section>`;

    root.querySelectorAll("[data-kind]").forEach(button =>
      button.addEventListener("click", () => {
        kind = button.dataset.kind;
        page = 1;
        renderDiscover();
      })
    );

    root.querySelector("[data-route]").addEventListener("click", () =>
      window.app.navigate("filter")
    );

    const prevBtn = root.querySelector("#discover-prev");
    const nextBtn = root.querySelector("#discover-next");

    prevBtn.addEventListener("click", () => {
      if (page > 1) {
        page--;
        updateButtons();
        load();
      }
    });

    nextBtn.addEventListener("click", () => {
      if (page < totalPages) {
        page++;
        updateButtons();
        load();
      }
    });

    load();
  }

  function updateButtons() {
    const prevBtn = document.getElementById("discover-prev");
    const nextBtn = document.getElementById("discover-next");
    const currentPageEl = document.getElementById("current-page");
    const totalPagesEl = document.getElementById("total-pages");

    if (prevBtn) prevBtn.disabled = page <= 1;
    if (nextBtn) nextBtn.disabled = page >= totalPages;
    if (currentPageEl) currentPageEl.textContent = page;
    if (totalPagesEl) totalPagesEl.textContent = totalPages;
  }

  function load() {
    const result = document.getElementById("discover-result");
    if (result) result.innerHTML = '<p class="placeholder">Загружаем страницу каталога…</p>';
    window.appIpc.send({ action: "load_discover_page", kind, page });
  }

  function showPage(response) {
    const root = document.getElementById("discover-result");
    if (!root) return;

    if (response.total_page_count) {
      totalPages = response.total_page_count;
    }

    updateButtons();

    if (!response.content?.length) {
      root.innerHTML = '<p class="placeholder">В этом разделе пока нет релизов.</p>';
      return;
    }

    root.replaceChildren(...response.content.map(release => window.releaseCard.render(release)));
  }

  function showError(message) {
    const root = document.getElementById("discover-result");
    if (root) {
      root.innerHTML = `
        <div class="error-state">
          <p>Не удалось загрузить каталог</p>
          <p class="error-message">${message}</p>
          <button type="button" id="retry-discover">Повторить</button>
        </div>`;
      
      root.querySelector("#retry-discover")?.addEventListener("click", load);
    }
  }

  window.appScreens = {
    ...(window.appScreens || {}),
    discover: renderDiscover,
    showDiscoverPage: showPage,
    showDiscoverError: showError
  };
})();
