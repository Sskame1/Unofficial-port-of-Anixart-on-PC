(() => {
  function renderHome() {
    const main = document.getElementById("screen-content");
    main.innerHTML = `
      <section class="screen home-screen" aria-labelledby="screen-title">
        <nav class="home-tabs" aria-label="Каталог">
          <button type="button" class="home-tab" data-route="bookmarks">Моя вкладка</button>
          <button type="button" class="home-tab is-active">Аниме</button>
          <button type="button" class="home-tab">Дунхуа</button>
          <button type="button" class="home-tab">Последнее</button>
          <button type="button" class="home-tab" data-route="discover">Популярное</button>
        </nav>
        <h1 class="visually-hidden" id="screen-title">Главная</h1>
        <section class="home-feature" aria-label="Популярные релизы" tabindex="0">
          <div class="feature-placeholder"><span class="loading-orbit"></span></div>
          <div class="feature-caption"><strong>Случайное аниме</strong><span>Нажмите, чтобы открыть карточку</span></div>
          <button class="feature-arrow" id="home-random" type="button" aria-label="Открыть случайный релиз">›</button>
        </section>
        <div class="home-shortcuts">
          <button class="shortcut" data-route="discover"><span class="shortcut-icon">♨</span><span>Популярное</span></button>
          <button class="shortcut" data-route="schedule"><span class="shortcut-icon">▤</span><span>Расписание</span></button>
          <button class="shortcut" data-route="collections"><span class="shortcut-icon">▣</span><span>Коллекции</span><i></i></button>
          <button class="shortcut" data-route="filter"><span class="shortcut-icon">☷</span><span>Фильтр</span></button>
          <button class="shortcut" id="home-random-shortcut"><span class="shortcut-icon">⤨</span><span>Рандом</span></button>
        </div>
        <section class="home-section">
          <div class="section-heading"><div><h2>Из каталога</h2><p>Публичный релиз Anixart</p></div><button type="button" data-route="discover">Обзор</button></div>
          <div class="home-release-grid" id="home-release-grid"><div class="poster-skeleton"></div><div class="poster-skeleton"></div><div class="poster-skeleton"></div></div>
          <div class="home-empty-note" id="home-note">Загрузите случайный релиз, чтобы посмотреть доступную карточку.</div>
        </section>
      </section>`;
    main.querySelectorAll('[data-route]').forEach(button => button.addEventListener('click', () => window.app.navigate(button.dataset.route)));
    main.querySelectorAll('.home-tab:not([data-route])').forEach(button => button.addEventListener('click', () => {
      main.querySelectorAll('.home-tab').forEach(tab => tab.classList.remove('is-active'));
      button.classList.add('is-active');
      window.app.setStatus("Каталог", `Вкладка «${button.textContent.trim()}» ещё подключается к API.`);
    }));
    document.getElementById("home-random")?.addEventListener("click", event => { event.stopPropagation(); loadRandom(); });
    document.getElementById("home-random-shortcut")?.addEventListener("click", loadRandom);
    loadRandom();
  }

  function loadRandom() {
    const note = document.getElementById("home-note");
    if (note) note.textContent = "Загружаем релиз из публичного каталога…";
    window.appIpc.send({ action: "discover_random_release" });
  }

  function showRandomRelease(release) {
    const grid = document.getElementById("home-release-grid");
    if (!grid) return;
    const card = window.releaseCard.render(release, true);
    grid.replaceChildren(card);
    const feature = document.querySelector(".home-feature");
    if (feature && typeof release.image === "string" && release.image.startsWith("https://")) {
      feature.dataset.releaseId = release.id;
      feature.classList.add("has-release");
      feature.setAttribute("role", "button");
      feature.setAttribute("aria-label", `Открыть релиз: ${release.title_ru || release.title_original || "Аниме"}`);
      const image = document.createElement("img");
      image.className = "feature-poster";
      image.alt = "";
      image.src = release.image;
      image.referrerPolicy = "no-referrer";
      image.addEventListener("error", () => { image.remove(); feature.classList.remove("has-release"); }, { once: true });
      feature.prepend(image);
    }
    const title = document.querySelector(".feature-caption strong");
    const subtitle = document.querySelector(".feature-caption span");
    if (title) title.textContent = release.title_ru || release.title_original || "Аниме";
    if (subtitle) subtitle.textContent = [release.year, release.grade ? `★ ${release.grade.toFixed(1)}` : ""].filter(Boolean).join(" · ");
    const note = document.getElementById("home-note");
    if (note) note.textContent = "Откройте карточку, чтобы увидеть подробности и доступные эпизоды.";
  }

  function showRandomError(message) {
    const grid = document.getElementById("home-release-grid");
    if (!grid) return;
    const error = document.createElement("div");
    error.className = "home-data-error";
    error.textContent = `Не удалось загрузить каталог: ${message}`;
    grid.replaceChildren(error);
    const note = document.getElementById("home-note");
    if (note) note.textContent = "Проверьте подключение и нажмите «Рандом», чтобы повторить запрос.";
  }

  window.appScreens = { ...(window.appScreens || {}), home: renderHome, showHomeRelease: showRandomRelease, showHomeError: showRandomError };
})();
