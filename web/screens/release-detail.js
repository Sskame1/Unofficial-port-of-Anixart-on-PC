(() => {
  let selectedRelease = null;
  let selectedType = null;
  let selectedSource = null;

  function renderReleaseDetail(release) {
    selectedRelease = release;
    selectedType = null;
    selectedSource = null;
    const main = document.getElementById("screen-content");
    main.innerHTML = `
      <article class="screen release-page" aria-labelledby="release-title">
        <div class="release-hero" id="release-hero">
          <img class="release-hero-backdrop" id="release-backdrop" alt="" aria-hidden="true">
          <div class="release-hero-scrim"></div>
          <button class="release-back" id="release-back" type="button" aria-label="Назад">‹</button>
          <img class="release-hero-poster" id="release-poster" alt="">
        </div>
        <section class="release-heading">
          <div class="release-title-row"><div><h1 id="release-title"></h1><p id="release-original"></p></div><span class="age-chip" id="release-age"></span></div>
          <div class="release-kicker" id="release-kicker"></div>
        </section>
        <section class="release-actionbar" aria-label="Действия релиза">
          <button id="release-watch" class="watch-button" type="button"><span>▷</span> Смотреть</button>
          <button class="release-action" data-action="status" type="button"><span>＋</span><small>Список</small></button>
          <button class="release-action" data-action="favorite" type="button"><span>▮</span><small>В избранное</small></button>
          <button class="release-action" data-action="comments" type="button"><span>☷</span><small>Отзывы</small></button>
        </section>
        <section class="release-info-block" id="release-info">
          <div class="release-facts" id="release-facts"></div>
          <div class="release-description-block"><p id="release-description"></p><button id="release-expand" type="button">Показать ещё</button></div>
        </section>
        <section class="release-rating-block"><div class="release-section-heading"><h2>Рейтинг</h2><span id="release-rating"></span></div><div class="rating-bars"><i style="--value:92%"></i><i style="--value:77%"></i><i style="--value:54%"></i><i style="--value:31%"></i><i style="--value:16%"></i></div><p id="release-votes"></p><div class="release-vote-actions"><button data-vote="1">★</button><button data-vote="2">★</button><button data-vote="3">★</button><button data-vote="4">★</button><button data-vote="5">★</button></div></section>
        <section class="release-subsection"><div class="release-section-heading"><h2>Похожие релизы</h2><button type="button">Все ›</button></div><div class="detail-poster-row" id="related-releases"></div></section>
        <section class="release-subsection"><div class="release-section-heading"><h2>Скриншоты</h2></div><div class="detail-screenshot-row" id="release-screenshots"></div></section>
        <section class="release-subsection release-comments" id="release-comments"><div class="release-section-heading"><h2>Отзывы</h2></div><button class="btn btn-secondary" type="button" id="release-comments-load">Показать отзывы</button><div id="release-comments-list" class="release-comments-list"></div></section>
      </article>`;
    const setText = (id, value) => { const node = document.getElementById(id); if (node) node.textContent = value || ""; };
    setText("release-title", release.title_ru || release.title_original || "Релиз");
    setText("release-original", release.title_original);
    setText("release-age", release.age_rating ? `${release.age_rating}+` : "");
    setText("release-kicker", [release.year, release.season ? `Сезон ${release.season}` : "", release.grade ? `★ ${release.grade.toFixed(1)}` : ""].filter(Boolean).join(" · "));
    setText("release-description", release.description || "Описание пока недоступно.");
    setText("release-rating", release.grade ? `★ ${release.grade.toFixed(1)}` : "—");
    setText("release-votes", release.vote_count ? `${release.vote_count.toLocaleString("ru-RU")} оценок` : "Голосование сообщества");
    const poster = document.getElementById("release-poster");
    if (release.image) { poster.src = release.image; poster.referrerPolicy = "no-referrer"; poster.alt = `Постер: ${release.title_ru || release.title_original}`; }
    const backdrop = document.getElementById("release-backdrop");
    if (release.image) { backdrop.src = release.image; backdrop.referrerPolicy = "no-referrer"; }
    const facts = [
      ["Тип", release.category?.name], ["Страна", release.country], ["Жанры", release.genres],
      ["Эпизоды", release.episodes_total ? `${release.episodes_released || 0} из ${release.episodes_total}` : release.episodes_released ? `${release.episodes_released} эп.` : ""],
      ["Студия", release.studio], ["Первоисточник", release.source], ["Длительность", release.duration ? `${release.duration} мин.` : ""], ["Статус", release.status?.name],
    ].filter(([, value]) => value);
    document.getElementById("release-facts").replaceChildren(...facts.map(([name, value]) => {
      const row = document.createElement("p"); row.className = "release-fact";
      const label = document.createElement("span"); label.textContent = name;
      const detail = document.createElement("strong"); detail.textContent = value;
      row.append(label, detail); return row;
    }));
    document.getElementById("release-back").addEventListener("click", () => window.app.backFromRelease());
    renderRelated(release.related_releases || release.recommended_releases || []);
    renderScreenshots(release.screenshot_images || []);
    document.getElementById("release-watch").addEventListener("click", () => { window.app.currentRoute = "watch"; window.app.navigate("watch"); });
    document.getElementById("release-comments-load").addEventListener("click", () => { document.getElementById("release-comments-list").innerHTML = '<p class="placeholder">Загружаем отзывы…</p>'; window.appIpc.send({ action: "load_release_comments", release_id: release.id, page: 1 }); });
    document.getElementById("release-expand").addEventListener("click", event => {
      const desc = document.getElementById("release-description");
      desc.classList.toggle("is-expanded"); event.currentTarget.textContent = desc.classList.contains("is-expanded") ? "Свернуть" : "Показать ещё";
    });
    document.querySelectorAll(".release-action").forEach(button => button.addEventListener("click", () => {
      if (button.dataset.action === "comments") {
        const section = document.getElementById("release-comments");
        section?.scrollIntoView({ behavior: "smooth", block: "start" });
        document.getElementById("release-comments-load")?.click();
      } else window.app.auth("signin");
    }));
    document.querySelectorAll(".release-vote-actions button").forEach(button => button.addEventListener("click", () => window.app.auth("signin")));
  }
  function renderRelated(items) {
    const row = document.getElementById("related-releases");
    if (!row) return;
    if (!items.length) { row.innerHTML = '<p class="placeholder">Похожие релизы не указаны.</p>'; return; }
    row.replaceChildren(...items.map(item => window.releaseCard.render(item)));
  }
  function renderScreenshots(images) {
    const row = document.getElementById("release-screenshots");
    if (!row) return;
    if (!images.length) { row.innerHTML = '<p class="placeholder">Скриншоты для этого релиза не добавлены.</p>'; return; }
    row.innerHTML = images.map(url => `<img src="${escapeHtml(url)}" alt="Скриншот" loading="lazy" referrerpolicy="no-referrer">`).join("");
  }
  function escapeHtml(value) { return String(value ?? "").replace(/[&<>"']/g, c => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c])); }
  function renderWatch(release) {
    if (!release) return renderReleaseDetail({});
    const main = document.getElementById("screen-content");
    main.innerHTML = `<section class="screen watch-screen"><button class="btn btn-secondary" id="watch-back" type="button">← К релизу</button><header class="screen-header"><h1>${escapeHtml(release.title_ru || release.title_original || "Просмотр")}</h1><p>Выберите озвучку, источник и серию</p></header><section class="panel episode-picker" id="episode-picker"><label class="field-label" for="episode-type">Озвучка / субтитры<select id="episode-type" disabled><option>Загружаем…</option></select></label><label class="field-label" for="episode-source">Источник<select id="episode-source" disabled><option>Сначала выберите тип</option></select></label><div id="episode-list" class="episode-list"><div class="placeholder">Загружаем доступные варианты…</div></div></section><section class="player-panel hidden" id="player-container" aria-label="Плеер эпизода"></section></section>`;
    selectedRelease = release; selectedType = null; selectedSource = null;
    document.getElementById("watch-back").addEventListener("click", () => window.app.navigate("release"));
    document.getElementById("episode-type").addEventListener("change", event => { selectedType = Number(event.currentTarget.value); selectedSource = null; renderSelect("episode-source", [], "Загружаем источники…"); renderEpisodeState("Загружаем источники…"); window.appIpc.send({ action: "load_episode_sources", release_id: release.id, type_id: selectedType }); });
    document.getElementById("episode-source").addEventListener("change", event => { selectedSource = Number(event.currentTarget.value); renderEpisodeState("Загружаем эпизоды…"); window.appIpc.send({ action: "load_episodes", release_id: release.id, type_id: selectedType, source_id: selectedSource }); });
    window.appIpc.send({ action: "load_episode_types", release_id: release.id });
  }
  function renderSelect(id, items, placeholder) {
    const select = document.getElementById(id); if (!select) return;
    select.replaceChildren(new Option(placeholder, ""));
    for (const item of items) select.add(new Option(item.name || `#${item.id}`, String(item.id)));
    select.disabled = items.length === 0;
  }
  function renderEpisodeState(message) { const list = document.getElementById("episode-list"); if (list) { const node = document.createElement("div"); node.className = "placeholder"; node.textContent = message; list.replaceChildren(node); } }
  function showTypes(items) { renderSelect("episode-type", items, items.length ? "Выберите озвучку" : "Нет доступных типов"); if (!items.length) renderEpisodeState("Для этого релиза пока нет доступных эпизодов."); }
  function showSources(items) { renderSelect("episode-source", items, items.length ? "Выберите источник" : "Нет доступных источников"); if (!items.length) renderEpisodeState("Для выбранного типа источники не найдены."); }
  function showEpisodes(items) {
    const list = document.getElementById("episode-list"); if (!list) return;
    if (!items.length) return renderEpisodeState("Эпизоды не найдены.");
    list.replaceChildren(...items.map(item => {
      const button = document.createElement("button"); button.type = "button"; button.className = "episode-item"; button.textContent = item.name || `Эпизод ${item.position}`;
      button.addEventListener("click", () => window.appIpc.send({ action: "play_episode", release_id: selectedRelease.id, type_id: selectedType, source_id: selectedSource, position: item.position })); return button;
    }));
  }
  function showError(message) { renderEpisodeState(`Не удалось загрузить эпизоды: ${message}`); }
  function showComments(response) {
    const root = document.getElementById("release-comments-list"); if (!root) return;
    const items = response.content || [];
    root.innerHTML = items.length ? items.map(item => `<article class="release-comment"><strong>${escapeHtml(item.profile?.login || "Пользователь")}</strong><p>${escapeHtml(item.message || "")}</p></article>`).join("") : '<p class="placeholder">Отзывов пока нет.</p>';
  }
  function showCommentsError(message) { const root = document.getElementById("release-comments-list"); if (root) root.innerHTML = `<p class="placeholder">Не удалось загрузить отзывы: ${escapeHtml(message)}</p>`; }
  window.releaseDetailScreen = { render: renderReleaseDetail, renderWatch, showTypes, showSources, showEpisodes, showError, showComments, showCommentsError };
})();
