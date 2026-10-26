(() => {
  const tabs = [
    { key: "recommendations", label: "Рекомендации" },
    { key: "interesting", label: "Интересное" },
    { key: "discussing", label: "Обсуждают" },
    { key: "watching", label: "Смотрят" }
  ];

  let currentTab = "recommendations";
  let page = 1;
  let totalPages = 1;

  function renderDiscover() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <div class="screen-header">
        <h1>Обзор</h1>
        <p>Популярное и рекомендованное</p>
      </div>

      <div class="content-tabs">
        ${tabs.map(tab => `
          <button class="tab-item ${tab.key === currentTab ? "active" : ""}" data-tab="${tab.key}">
            ${tab.label}
          </button>
        `).join("")}
      </div>

      <div id="discover-results" class="release-grid">
        <div class="placeholder">
          <div class="placeholder-icon">🔍</div>
          <p>Загружаем каталог...</p>
        </div>
      </div>

      <div id="discover-pagination" style="display: none; margin-top: 32px; justify-content: center; gap: 16px;">
        <button class="btn btn-secondary" id="discover-prev">← Назад</button>
        <span style="display: flex; align-items: center; color: var(--text-secondary);">
          Страница <strong style="margin: 0 6px;">${page}</strong> из <strong style="margin: 0 6px;">${totalPages}</strong>
        </span>
        <button class="btn btn-secondary" id="discover-next">Далее →</button>
      </div>
    `;

    root.querySelectorAll(".tab-item").forEach(button => {
      button.addEventListener("click", () => {
        currentTab = button.dataset.tab;
        page = 1;
        renderDiscover();
      });
    });

    const prevBtn = root.querySelector("#discover-prev");
    const nextBtn = root.querySelector("#discover-next");

    if (prevBtn) {
      prevBtn.addEventListener("click", () => {
        if (page > 1) {
          page--;
          loadContent();
        }
      });
    }

    if (nextBtn) {
      nextBtn.addEventListener("click", () => {
        if (page < totalPages) {
          page++;
          loadContent();
        }
      });
    }

    loadContent();
  }

  function loadContent() {
    window.appIpc.send({ action: "load_discover_page", kind: currentTab, page });
  }

  function showPage(response) {
    const results = document.getElementById("discover-results");
    if (!results) return;

    if (response.total_page_count) {
      totalPages = response.total_page_count;
    }

    if (!response.content?.length) {
      results.innerHTML = `
        <div class="placeholder">
          <div class="placeholder-icon">📭</div>
          <p>В этом разделе пока нет релизов</p>
        </div>
      `;
      return;
    }

    if (currentTab === "interesting") {
      results.classList.remove("release-grid");
      results.classList.add("interesting-grid");
      results.replaceChildren(...response.content.map(item => renderInteresting(item)));
    } else {
      results.classList.remove("interesting-grid");
      results.classList.add("release-grid");
      results.replaceChildren(...response.content.map(release => window.releaseCard.render(release)));
    }

    const pagination = document.getElementById("discover-pagination");
    if (pagination && totalPages > 1) {
      pagination.style.display = "flex";
      const prevBtn = document.getElementById("discover-prev");
      const nextBtn = document.getElementById("discover-next");
      if (prevBtn) prevBtn.disabled = page <= 1;
      if (nextBtn) nextBtn.disabled = page >= totalPages;
    }
  }

  function renderInteresting(item) {
    const card = document.createElement("button");
    card.type = "button";
    card.className = "interesting-card";
    const title = item.title || item.title_ru || item.title_original || "Подборка Anixart";
    const image = item.image || "";
    card.innerHTML = `<div class="interesting-card-image">${image ? `<img src="${image}" alt="" loading="lazy">` : `<span>✦</span>`}</div><div class="interesting-card-copy"><strong>${title}</strong><p>${item.description || ""}</p></div>`;
    card.addEventListener("click", () => {
      const id = Number(item.action || item.id);
      if (id > 0) window.appIpc.send({ action: "open_release", release_id: id });
    });
    return card;
  }

  function showError(message) {
    const results = document.getElementById("discover-results");
    if (results) {
      results.classList.remove("interesting-grid");
      results.classList.add("release-grid");
      results.innerHTML = `
        <div class="placeholder">
          <div class="placeholder-icon">⚠️</div>
          <p>Не удалось загрузить каталог</p>
          <p style="font-size: 13px; margin-top: 8px;">${message}</p>
        </div>
      `;
    }
  }

  window.appScreens = {
    ...(window.appScreens || {}),
    discover: renderDiscover,
    refreshDiscover: loadContent,
    showDiscoverPage: showPage,
    showDiscoverError: showError
  };
})();
