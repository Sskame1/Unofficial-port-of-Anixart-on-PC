(() => {
  const tabs = [
    { key: "recommendations", label: "Рекомендации" },
    { key: "interesting", label: "Интересное" },
    { key: "discussing", label: "Обсуждают" },
    { key: "watching", label: "Смотрят" }
  ];
  const sections = [
    { key: "recommendations", title: "Рекомендации", note: "На основе ваших оценок" },
    { key: "interesting", title: "Интересное", note: "Подборки и события Anixart" },
    { key: "watching", title: "Сейчас смотрят", note: "Релизы, которые смотрят прямо сейчас" },
    { key: "discussing", title: "Обсуждают", note: "Новые обсуждения релизов" }
  ];

  let currentTab = "recommendations";
  let page = 1;
  let totalPages = 1;
  let viewMode = "overview";
  const loaded = new Map();
  const pending = new Set();

  function icon(path) {
    return `<svg aria-hidden="true" viewBox="0 0 24 24"><${path}/></svg>`;
  }

  function renderDiscover() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="discover-screen">
        <header class="discover-heading">
          <div><span class="eyebrow">ANIXART</span><h1>Обзор</h1><p>Популярное, рекомендации и активность сообщества</p></div>
          <div class="discover-heading-actions">${viewMode === "list" ? `<button class="discover-back-overview" id="discover-back" type="button">← Обзор</button>` : ""}<button class="header-search-button" id="discover-search" type="button">${icon('circle cx="11" cy="11" r="6.5"/><path d="m16 16 4.5 4.5"')}<span>Поиск аниме</span></button></div>
        </header>

        <div class="discover-feature-row">
          <article class="discover-feature-card discover-feature-card--accent" role="button" tabindex="0" data-discover-tab="interesting" aria-label="Открыть интересное">
            <div><span class="feature-icon">✦</span><strong>Интересное</strong><p>Новые подборки, обсуждения и релизы в одном месте.</p></div>
            <span class="feature-arrow" aria-hidden="true">→</span>
          </article>
          <article class="discover-feature-card" role="button" tabindex="0" data-route="filter" aria-label="Открыть каталог релизов">
            <div><span class="feature-icon">☷</span><strong>Каталог релизов</strong><p>Найди тайтл по жанрам, стране, типу и статусу.</p></div>
            <span class="feature-arrow" aria-hidden="true">→</span>
          </article>
          <article class="discover-feature-card" role="button" tabindex="0" data-route="collections" aria-label="Открыть коллекции">
            <div><span class="feature-icon">▦</span><strong>Коллекции</strong><p>Готовые тематические списки от пользователей.</p></div>
            <span class="feature-arrow" aria-hidden="true">→</span>
          </article>
          <article class="discover-feature-card" role="button" tabindex="0" data-route="schedule" aria-label="Открыть расписание">
            <div><span class="feature-icon">▣</span><strong>Расписание</strong><p>Новые эпизоды и даты выхода по дням недели.</p></div>
            <span class="feature-arrow" aria-hidden="true">→</span>
          </article>
        </div>

        ${viewMode === "overview" ? `<nav class="discover-tabs" aria-label="Разделы обзора">
          ${tabs.map(tab => `<button type="button" class="tab-item ${tab.key === currentTab ? "active" : ""}" data-tab="${tab.key}">${tab.label}</button>`).join("")}
        </nav>` : `<nav class="discover-tabs" aria-label="Разделы обзора">${tabs.map(tab => `<button type="button" class="tab-item ${tab.key === currentTab ? "active" : ""}" data-tab="${tab.key}">${tab.label}</button>`).join("")}</nav><div class="discover-list-heading"><h2>${sections.find(section => section.key === currentTab)?.title || "Обзор"}</h2><span>Страница ${page} из ${totalPages}</span></div><div id="discover-all-results" class="release-grid"><div class="placeholder"><p>Загружаем...</p></div></div><div class="discover-pagination"><button id="discover-prev" type="button" ${page <= 1 ? "disabled" : ""}>← Назад</button><span>${page} / ${totalPages}</span><button id="discover-next" type="button" ${page >= totalPages ? "disabled" : ""}>Далее →</button></div>`}

        ${viewMode === "overview" ? `<div class="discover-sections">
          ${sections.map(section => `
            <section class="discover-section" data-section="${section.key}">
              <div class="discover-section-heading"><div><h2>${section.title}</h2><p>${section.note}</p></div><button type="button" class="discover-see-all" data-section-tab="${section.key}">Все <span>→</span></button></div>
              <div class="discover-strip" id="discover-${section.key}"><div class="placeholder"><p>Загружаем...</p></div></div>
            </section>`).join("")}
        </div>` : ""}
      </section>`;

    root.querySelector("#discover-search")?.addEventListener("click", () => window.app.navigate("search"));
    root.querySelector("#discover-back")?.addEventListener("click", () => { viewMode = "overview"; renderDiscover(); });
    root.querySelectorAll(".discover-feature-card").forEach(card => {
      const activate = () => {
        if (card.dataset.route) window.app.navigate(card.dataset.route);
        else if (card.dataset.discoverTab) {
          currentTab = card.dataset.discoverTab;
          viewMode = "list";
          page = 1;
          renderDiscover();
        }
      };
      card.addEventListener("click", activate);
      card.addEventListener("keydown", event => {
        if (event.key === "Enter" || event.key === " ") { event.preventDefault(); activate(); }
      });
    });
    root.querySelectorAll("[data-section-tab]").forEach(button => button.addEventListener("click", () => {
      currentTab = button.dataset.discoverTab || button.dataset.sectionTab;
      page = 1;
      viewMode = "list";
      renderDiscover();
      root.querySelectorAll(".tab-item").forEach(tab => tab.classList.toggle("active", tab.dataset.tab === currentTab));
      root.querySelector(`[data-section="${currentTab}"]`)?.scrollIntoView({ behavior: "smooth", block: "start" });
    }));
    root.querySelectorAll(".tab-item").forEach(button => button.addEventListener("click", () => {
      currentTab = button.dataset.tab;
      page = 1;
      if (viewMode === "list") { renderDiscover(); return; }
      root.querySelectorAll(".tab-item").forEach(tab => tab.classList.toggle("active", tab.dataset.tab === currentTab));
      loadSection(currentTab);
      root.querySelector(`[data-section="${currentTab}"]`)?.scrollIntoView({ behavior: "smooth", block: "start" });
    }));

    if (viewMode === "list") {
      root.querySelector("#discover-prev")?.addEventListener("click", () => { if (page > 1) { page--; loadSection(currentTab); } });
      root.querySelector("#discover-next")?.addEventListener("click", () => { if (page < totalPages) { page++; loadSection(currentTab); } });
      loadSection(currentTab);
    } else sections.forEach(section => loadSection(section.key));
  }

  function loadSection(kind) {
    if (pending.has(kind)) return;
    pending.add(kind);
    const target = document.getElementById(`discover-${kind}`);
    if (target && !loaded.has(kind)) target.innerHTML = `<div class="placeholder"><p>Загружаем...</p></div>`;
    window.appIpc.send({ action: "load_discover_page", kind, page: viewMode === "list" ? page : 1 });
  }

  function showPage(response) {
    const payload = response.data || response;
    const kind = response.kind || currentTab;
    pending.delete(kind);
    loaded.set(kind, payload);
    if (viewMode === "list" && kind === currentTab) {
      totalPages = Math.max(1, Number(payload.total_page_count) || 1);
      const results = document.getElementById("discover-all-results");
      if (results) results.replaceChildren(...(payload.content || []).map(item => kind === "interesting" ? renderInteresting(item) : window.releaseCard.render(item)));
      document.querySelector(".discover-list-heading span")?.replaceChildren(document.createTextNode(`Страница ${page} из ${totalPages}`));
      document.querySelector(".discover-pagination span")?.replaceChildren(document.createTextNode(`${page} / ${totalPages}`));
      const prev = document.getElementById("discover-prev"), next = document.getElementById("discover-next");
      if (prev) prev.disabled = page <= 1;
      if (next) next.disabled = page >= totalPages;
      return;
    }
    const target = document.getElementById(`discover-${kind}`);
    if (!target) return;
    const content = payload.content || [];
    if (!content.length) { target.innerHTML = `<div class="placeholder"><p>Пока нет данных</p></div>`; return; }
    target.replaceChildren(...content.map(item => {
      if (kind === "interesting") return renderInteresting(item);
      return window.releaseCard.render(item);
    }));
  }

  function renderInteresting(item) {
    const card = document.createElement("button");
    card.type = "button";
    card.className = "interesting-card";
    const title = item.title_ru || item.title_original || item.title || "Интересное";
    card.innerHTML = `<div class="interesting-card-image">${item.image ? `<img src="${item.image}" alt="" loading="lazy">` : "✦"}</div><div class="interesting-card-copy"><strong>${title}</strong><p>${item.description || "Открыть релиз"}</p></div>`;
    card.addEventListener("click", () => { const id = Number(item.action || item.id); if (id > 0) window.appIpc.send({ action: "open_release", release_id: id }); });
    return card;
  }

  function showError(payload) {
    const message = payload.message || "Ошибка загрузки";
    const target = payload.kind ? document.getElementById(`discover-${payload.kind}`) : null;
    (target ? [target] : [...document.querySelectorAll(".discover-strip")]).forEach(element => { element.innerHTML = `<div class="placeholder"><p>${message}</p></div>`; });
  }

  window.appScreens = { ...(window.appScreens || {}), discover: renderDiscover, refreshDiscover: renderDiscover, showDiscoverPage: showPage, showDiscoverError: showError };
})();
