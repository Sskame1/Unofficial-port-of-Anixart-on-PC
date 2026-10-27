(() => {
  let selectedRelease = null;
  let selectedType = null;
  let selectedSource = null;
  let pendingListType = null;
  let episodeTypes = [];
  let episodeSources = [];
  let episodeItems = [];
  let reviewSort = 3;
  let pendingReviewVote = null;
  let reviewPage = 1;

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
          <button class="release-action release-list-action" data-action="status" type="button"><span>＋</span><small id="release-list-label">Список</small></button>
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
      </article>`;
    const setText = (id, value) => { const node = document.getElementById(id); if (node) node.textContent = value || ""; };
    setText("release-title", release.title_ru || release.title_original || "Релиз");
    setText("release-original", release.title_original);
    setText("release-age", release.age_rating ? `${release.age_rating}+` : "");
    setText("release-kicker", [release.year, release.season ? `Сезон ${release.season}` : "", release.grade ? `★ ${release.grade.toFixed(1)}` : ""].filter(Boolean).join(" · "));
    setText("release-description", release.description || "Описание пока недоступно.");
    setText("release-rating", release.grade ? `★ ${release.grade.toFixed(1)}` : "—");
    setText("release-votes", release.vote_count ? `${release.vote_count.toLocaleString("ru-RU")} оценок` : "Голосование сообщества");
    updateListButton(release.profile_list_status);
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
    document.getElementById("release-expand").addEventListener("click", event => {
      const desc = document.getElementById("release-description");
      desc.classList.toggle("is-expanded"); event.currentTarget.textContent = desc.classList.contains("is-expanded") ? "Свернуть" : "Показать ещё";
    });
    document.querySelectorAll(".release-action").forEach(button => button.addEventListener("click", () => {
      if (button.dataset.action === "comments") {
        reviewSort = 3;
        window.app.navigate("reviews");
      } else if (button.dataset.action === "status") showListPicker(release);
      else window.app.auth("signin");
    }));
    document.querySelectorAll(".release-vote-actions button").forEach(button => button.addEventListener("click", () => window.app.auth("signin")));
  }
  function renderReviews(release) {
    const root = document.getElementById("screen-content");
    if (!release) return renderReleaseDetail({});
    root.innerHTML = `<section class="screen reviews-page"><button class="watch-back-button" id="reviews-back" type="button">← <span>К релизу</span></button><header class="reviews-heading"><h1>Отзывы</h1><p>${escapeHtml(release.title_ru || release.title_original || "Релиз")}</p></header><div class="reviews-toolbar"><span id="reviews-count">Отзывы пользователей</span><label for="reviews-sort">Сортировка</label><select id="reviews-sort"><option value="3" ${reviewSort === 3 ? "selected" : ""}>Популярные</option><option value="1" ${reviewSort === 1 ? "selected" : ""}>Сначала новые</option><option value="2" ${reviewSort === 2 ? "selected" : ""}>Сначала старые</option></select></div><div id="release-reviews-list" class="release-reviews-list"><p class="placeholder">Загружаем отзывы…</p></div></section>`;
    document.getElementById("reviews-back").addEventListener("click", () => window.app.navigate("release"));
    document.getElementById("reviews-sort").addEventListener("change", event => {
      reviewSort = Number(event.currentTarget.value);
      loadReviews(1);
    });
    loadReviews(1);
  }
  function loadReviews(page) {
    reviewPage = page;
    const list = document.getElementById("release-reviews-list");
    if (list) list.innerHTML = '<p class="placeholder">Загружаем отзывы…</p>';
    window.appIpc.send({ action: "load_release_comments", release_id: window.app.currentRelease?.id, page, sort: reviewSort });
  }
  function showListPicker(release) {
    const options = [[1,"Смотрю"],[2,"В планах"],[3,"Просмотрено"],[4,"Отложено"],[5,"Брошено"]];
    const overlay = document.createElement("div"); overlay.className = "list-picker-overlay";
    overlay.innerHTML = `<section class="list-picker"><header><strong>Мой список</strong><button type="button" aria-label="Закрыть">×</button></header>${options.map(([id,label])=>`<button type="button" data-list="${id}" class="${Number(release.profile_list_status) === id ? "is-current" : ""}"><span>${label}</span><span class="list-picker-check">${Number(release.profile_list_status) === id ? "✓" : ""}</span></button>`).join("")}</section>`;
    overlay.querySelector("header button").addEventListener("click",()=>overlay.remove());
    overlay.addEventListener("click",event=>{ if(event.target===overlay) overlay.remove(); });
    overlay.querySelectorAll("[data-list]").forEach(button=>button.addEventListener("click",()=>{
      const listType = Number(button.dataset.list);
      if (!window.appStorage?.get("anixart_token")) { overlay.remove(); window.app.auth("signin"); return; }
      pendingListType = listType;
      button.disabled = true; button.textContent = "Сохраняем…";
      window.appIpc.send({ action: "add_release_to_list", release_id: release.id, list_type: listType });
    }));
    document.body.append(overlay);
  }
  function listDone() {
    document.querySelector(".list-picker-overlay")?.remove();
    if (selectedRelease && pendingListType !== null) {
      selectedRelease.profile_list_status = pendingListType || null;
      updateListButton(selectedRelease.profile_list_status);
    }
    pendingListType = null;
    window.app.setStatus("Список", "Список релиза обновлён.");
  }
  function listError(message) {
    const button = document.querySelector(`.list-picker [data-list="${pendingListType}"]`);
    if (button) { button.disabled = false; const label = button.querySelector("span"); if (label) label.textContent = ({1:"Смотрю",2:"В планах",3:"Просмотрено",4:"Отложено",5:"Брошено"})[Number(pendingListType)] || "Список"; }
    pendingListType = null;
    window.app.setStatus("Список", message || "Не удалось сохранить релиз.");
  }
  function updateListButton(status) {
    const button = document.querySelector('.release-action[data-action="status"]');
    const label = document.getElementById("release-list-label");
    if (!button || !label) return;
    const names = { 1: "Смотрю", 2: "В планах", 3: "Просмотрено", 4: "Отложено", 5: "Брошено" };
    label.textContent = names[Number(status)] || "Список";
    button.classList.toggle("is-selected", Boolean(names[Number(status)]));
    button.setAttribute("aria-label", names[Number(status)] ? `Изменить список: ${names[Number(status)]}` : "Добавить в список");
  }
  function renderRelated(items) {
    const row = document.getElementById("related-releases");
    if (!row) return;
    if (!items.length) { row.innerHTML = '<p class="placeholder">Похожие релизы не указаны.</p>'; return; }
    row.replaceChildren(...items.map(item => { const card = window.releaseCard.render(item); card.classList.add("related-release-card"); return card; }));
  }
  function renderScreenshots(images) {
    const row = document.getElementById("release-screenshots");
    if (!row) return;
    if (!images.length) { row.innerHTML = '<p class="placeholder">Скриншоты для этого релиза не добавлены.</p>'; return; }
    row.innerHTML = images.map((url, index) => `<button class="screenshot-thumb" type="button" data-screenshot-index="${index}" aria-label="Открыть скриншот ${index + 1}"><img src="${escapeHtml(url)}" alt="Скриншот ${index + 1}" loading="lazy" referrerpolicy="no-referrer"></button>`).join("");
    row.querySelectorAll("[data-screenshot-index]").forEach(button => button.addEventListener("click", () => openScreenshotViewer(images, Number(button.dataset.screenshotIndex))));
  }
  function openScreenshotViewer(images, index) {
    const current = Math.max(0, Math.min(index, images.length - 1));
    const overlay = document.createElement("div");
    overlay.className = "screenshot-viewer";
    overlay.setAttribute("role", "dialog");
    overlay.setAttribute("aria-modal", "true");
    overlay.innerHTML = `<div class="screenshot-viewer-panel"><button class="screenshot-viewer-close" type="button" aria-label="Закрыть">×</button><img class="screenshot-viewer-image" src="${escapeHtml(images[current])}" alt="Скриншот ${current + 1}" referrerpolicy="no-referrer"><div class="screenshot-viewer-counter">${current + 1} / ${images.length}</div></div>`;
    const close = () => { overlay.remove(); document.removeEventListener("keydown", onKey); };
    const onKey = event => { if (event.key === "Escape") close(); };
    overlay.addEventListener("click", event => { if (event.target === overlay) close(); });
    overlay.querySelector(".screenshot-viewer-close").addEventListener("click", close);
    document.addEventListener("keydown", onKey);
    document.body.append(overlay);
  }
  function escapeHtml(value) { return String(value ?? "").replace(/[&<>"']/g, c => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c])); }
  function renderWatch(release) {
    if (!release) return renderReleaseDetail({});
    const main = document.getElementById("screen-content");
    main.innerHTML = `<section class="screen watch-screen"><button class="watch-back-button" id="watch-back" type="button">← <span>К релизу</span></button><header class="watch-screen-header"><h1>Выберите озвучку</h1><p>${escapeHtml(release.title_ru || release.title_original || "Просмотр")}</p></header><section class="episode-picker" id="episode-picker"><div class="watch-choice-section"><div class="watch-section-title"><h2>Озвучки и субтитры</h2><span id="episode-type-count"></span></div><div id="episode-type-list" class="episode-choice-list"><div class="placeholder">Загружаем доступные варианты…</div></div></div><div class="watch-choice-section hidden" id="episode-source-section"><div class="watch-section-title"><button class="watch-inline-back" id="episode-source-back" type="button">← Озвучки</button><h2 id="episode-source-heading">Выберите источник</h2></div><div id="episode-source-list" class="episode-choice-list"></div></div><div class="watch-choice-section hidden" id="episode-list-section"><div class="watch-section-title"><button class="watch-inline-back" id="episode-list-back" type="button">← Источники</button><h2 id="episode-list-heading">Серии</h2></div><p class="episode-list-note">Нажмите на серию, чтобы открыть плеер</p><div id="episode-list" class="episode-list"><div class="placeholder">Выберите источник</div></div></div></section></section>`;
    selectedRelease = release;
    document.getElementById("watch-back").addEventListener("click", () => window.app.navigate("release"));
    document.getElementById("episode-source-back").addEventListener("click", showTypeChoices);
    document.getElementById("episode-list-back").addEventListener("click", showSourceChoices);
    document.getElementById("episode-type-list").innerHTML = '<div class="placeholder">Загружаем доступные варианты…</div>';
    if (episodeTypes.length) showTypes(episodeTypes);
    if (selectedType && episodeSources.length) showSources(episodeSources);
    if (selectedSource && episodeItems.length) {
      const selected = episodeSources.find(item => Number(item.id) === selectedSource);
      document.getElementById("episode-list-heading").textContent = selected?.name || "Серии";
      showEpisodes(episodeItems);
      showEpisodeChoices();
    } else if (selectedType) {
      showSourceChoices();
    }
    window.appIpc.send({ action: "load_episode_types", release_id: release.id });
  }
  function renderPlayer(release, episode) {
    const main = document.getElementById("screen-content");
    if (!episode) {
      main.innerHTML = '<section class="screen player-page"><p class="placeholder">Серия не выбрана.</p><button class="watch-back-button" id="player-back" type="button">← К сериям</button></section>';
      document.getElementById("player-back").addEventListener("click", () => window.app.navigate("watch"));
      return;
    }
    const url = String(episode.url || "");
    const title = escapeHtml(episode.name || "Эпизод");
    const releaseTitle = escapeHtml(release?.title_ru || release?.title_original || "Просмотр");
    if (!/^https:\/\//i.test(url)) {
      main.innerHTML = `<section class="screen player-page"><button class="watch-back-button" id="player-back" type="button">← <span>К сериям</span></button><p class="placeholder">Ссылка на видеоплеер имеет неподдерживаемый формат.</p></section>`;
    } else {
      main.innerHTML = `<section class="screen player-page"><button class="watch-back-button" id="player-back" type="button">← <span>К сериям</span></button><header class="player-page-heading"><p>${releaseTitle}</p><h1>${title}</h1></header><section class="player-panel" aria-label="Плеер эпизода"><iframe title="${title}" src="${escapeHtml(url)}" allow="autoplay; fullscreen; picture-in-picture" allowfullscreen referrerpolicy="strict-origin-when-cross-origin"></iframe></section></section>`;
    }
    document.getElementById("player-back").addEventListener("click", () => window.app.navigate("watch"));
  }
  function showPlayerError(message) {
    const main = document.getElementById("screen-content");
    if (!main || !document.documentElement.dataset.route.includes("player")) return;
    main.innerHTML = `<section class="screen player-page"><button class="watch-back-button" id="player-back" type="button">← <span>К сериям</span></button><p class="placeholder">Не удалось открыть эпизод: ${escapeHtml(message)}</p></section>`;
    document.getElementById("player-back").addEventListener("click", () => window.app.navigate("watch"));
  }
  function setWatchSection(activeId) { ["episode-type-list", "episode-source-section", "episode-list-section"].forEach(id => { const node = document.getElementById(id); if (!node) return; node.classList.toggle("hidden", id !== activeId); }); }
  function formatCount(value) { const count = Number(value) || 0; return count >= 1_000_000 ? `${(count / 1_000_000).toFixed(1).replace(".0", "")} млн` : count >= 1_000 ? `${Math.round(count / 1_000)} тыс.` : String(count); }
  function renderChoiceIcon(icon, name, isSub) { const accessible = icon && /^https?:\/\//i.test(icon); return `<span class="episode-choice-icon">${accessible ? `<img src="${escapeHtml(icon)}" alt="" loading="lazy" referrerpolicy="no-referrer" onerror="this.remove()">` : `<span aria-hidden="true">${isSub ? "文" : escapeHtml((name || "?").slice(0, 1).toUpperCase())}</span>`}</span>`; }
  function showTypeChoices() { setWatchSection("episode-type-list"); }
  function showSourceChoices() { setWatchSection("episode-source-section"); }
  function showEpisodeChoices() { setWatchSection("episode-list-section"); }
  function renderEpisodeState(message) { const list = document.getElementById("episode-list"); if (list) { const node = document.createElement("div"); node.className = "placeholder"; node.textContent = message; list.replaceChildren(node); } }
  function showTypes(items) {
    episodeTypes = items;
    const list = document.getElementById("episode-type-list"); if (!list) return;
    document.getElementById("episode-type-count").textContent = `${items.length} вариантов`;
    if (!items.length) { list.innerHTML = '<p class="placeholder">Для этого релиза нет доступных озвучек или субтитров.</p>'; return; }
    list.replaceChildren(...items.map(item => {
      const button = document.createElement("button"); button.type = "button"; button.className = "episode-choice-row";
      button.innerHTML = `${renderChoiceIcon(item.icon, item.name, item.is_sub)}<span class="episode-choice-copy"><strong>${escapeHtml(item.name || "Без названия")}${item.is_sub ? '<em>СУБТИТРЫ</em>' : ""}</strong><span>${Number(item.episodes_count) || 0} эпизодов${item.workers ? ` · ${escapeHtml(item.workers)}` : ""}</span></span><span class="episode-choice-views">${formatCount(item.view_count)} <span aria-hidden="true">◉</span></span>`;
      button.addEventListener("click", () => { selectedType = Number(item.id); selectedSource = null; document.getElementById("episode-source-heading").textContent = item.name || "Выберите источник"; document.getElementById("episode-source-list").innerHTML = '<p class="placeholder">Загружаем источники…</p>'; showSourceChoices(); window.appIpc.send({ action: "load_episode_sources", release_id: selectedRelease.id, type_id: selectedType }); }); return button;
    }));
  }
  function showSources(items) {
    episodeSources = items;
    const list = document.getElementById("episode-source-list"); if (!list) return;
    if (!items.length) { list.innerHTML = '<p class="placeholder">Для выбранной озвучки источники не найдены.</p>'; return; }
    if (items.length === 1) { selectSource(items[0]); return; }
    list.replaceChildren(...items.map(item => {
      const button = document.createElement("button"); button.type = "button"; button.className = "episode-choice-row episode-source-row";
      const sourceType = item.source_type;
      button.innerHTML = `${renderChoiceIcon(sourceType?.icon, item.name, sourceType?.is_sub)}<span class="episode-choice-copy"><strong>${escapeHtml(item.name || "Источник")}</strong><span>${Number(item.episodes_count) || 0} эпизодов</span></span><span class="episode-choice-arrow" aria-hidden="true">›</span>`;
      button.addEventListener("click", () => selectSource(item)); return button;
    }));
  }
  function selectSource(item) {
    selectedSource = Number(item.id);
    episodeItems = [];
    document.getElementById("episode-list-heading").textContent = item.name || "Серии";
    renderEpisodeState("Загружаем эпизоды…"); showEpisodeChoices();
    window.appIpc.send({ action: "load_episodes", release_id: selectedRelease.id, type_id: selectedType, source_id: selectedSource });
  }
  function showEpisodes(items) {
    episodeItems = items;
    const list = document.getElementById("episode-list"); if (!list) return;
    if (!items.length) return renderEpisodeState("Эпизоды не найдены.");
    list.replaceChildren(...items.map(item => {
      const button = document.createElement("button"); button.type = "button"; button.className = `episode-item${item.is_watched ? " is-watched" : ""}`; button.innerHTML = `<span class="episode-item-title">${escapeHtml(item.name || `${item.position} серия`)}</span>${item.is_filler ? '<span class="episode-filler-tag">Филлер</span>' : ""}<span class="episode-item-status" aria-label="${item.is_watched ? "Просмотрено" : "Не просмотрено"}">${item.is_watched ? "✓" : "›"}</span>`;
      button.addEventListener("click", () => window.appIpc.send({ action: "play_episode", release_id: selectedRelease.id, type_id: selectedType, source_id: selectedSource, position: item.position })); return button;
    }));
  }
  function showEpisode(episode) {
    const url = String(episode?.url || "");
    if (!/^https:\/\//i.test(url)) return showError("Ссылка на видеоплеер имеет неподдерживаемый формат.");
    window.app.currentEpisode = episode;
    window.app.navigate("player");
  }
  function showError(message) {
    const targets = ["episode-type-list", "episode-source-section", "episode-list-section"];
    const active = targets.map(id => document.getElementById(id)).find(node => node && !node.classList.contains("hidden"));
    const list = active?.id === "episode-type-list" ? active : active?.id === "episode-source-section" ? document.getElementById("episode-source-list") : document.getElementById("episode-list");
    if (list) { const node = document.createElement("p"); node.className = "placeholder"; node.textContent = `Не удалось загрузить данные: ${message}`; list.replaceChildren(node); }
  }
  function showReviews(response) {
    const root = document.getElementById("release-reviews-list");
    if (!root) return;
    const items = response.content || [];
    const total = Number(response.total_count) || 0;
    const totalPages = Number(response.total_page_count) || 1;
    const count = document.getElementById("reviews-count");
    if (count) count.textContent = `${total.toLocaleString("ru-RU")} отзывов`;
    if (!items.length) {
      root.innerHTML = '<p class="placeholder">Отзывов пока нет.</p>';
      return;
    }
    const cards = items.map(item => {
      const profile = item.profile || {};
      const avatar = profile.avatar && /^https?:\/\//i.test(profile.avatar)
        ? `<img class="review-avatar" src="${escapeHtml(profile.avatar)}" alt="" loading="lazy" referrerpolicy="no-referrer" onerror="this.remove()">`
        : '<span class="review-avatar-fallback" aria-hidden="true">◉</span>';
      const date = Number(item.timestamp) > 0 ? new Date(Number(item.timestamp) * 1000).toLocaleDateString("ru-RU") : "";
      const likes = Number(item.likes_count ?? item.vote_count) || 0;
      const vote = Number(item.vote) || 0;
      const deleted = Boolean(item.is_deleted);
      return `<article class="release-review" data-review-id="${Number(item.id) || 0}"><header class="release-review-author">${avatar}<span><strong>${escapeHtml(profile.login || "Пользователь")}</strong><small>${date}${item.is_edited ? " · изменён" : ""}</small></span></header><p class="release-review-message">${deleted ? "Отзыв удалён" : escapeHtml(item.message || "")}</p>${item.is_spoiler ? '<span class="review-spoiler">Спойлер</span>' : ""}<footer class="release-review-actions"><span class="review-reply-count">↩ ${Number(item.reply_count) || 0}</span><span class="review-votes"><button type="button" data-vote="-1" aria-label="Не нравится" class="${vote < 0 ? "is-voted" : ""}" ${item.can_like === false ? "disabled" : ""}>−</button><strong>${likes}</strong><button type="button" data-vote="1" aria-label="Нравится" class="${vote > 0 ? "is-voted" : ""}" ${item.can_like === false ? "disabled" : ""}>＋</button></span></footer></article>`;
    }).join("");
    const hasPrevious = reviewPage > 1;
    const hasNext = totalPages > reviewPage;
    root.innerHTML = `${cards}<nav class="reviews-pagination" aria-label="Страницы отзывов"><button type="button" data-page="${reviewPage - 1}" ${hasPrevious ? "" : "disabled"}>← Назад</button><span>${reviewPage} / ${totalPages}</span><button type="button" data-page="${reviewPage + 1}" ${hasNext ? "" : "disabled"}>Далее →</button></nav>`;
    root.querySelectorAll("[data-vote]").forEach(button => button.addEventListener("click", () => {
      const card = button.closest("[data-review-id]");
      const commentId = Number(card?.dataset.reviewId);
      if (!commentId) return;
      if (!window.appStorage?.get("anixart_token")) return window.app.auth("signin");
      pendingReviewVote = button;
      button.disabled = true;
      window.appIpc.send({ action: "vote_release_comment", comment_id: commentId, vote: Number(button.dataset.vote) });
    }));
    root.querySelectorAll("[data-page]").forEach(button => button.addEventListener("click", () => loadReviews(Number(button.dataset.page))));
  }
  function showReviewsError(message) {
    const root = document.getElementById("release-reviews-list");
    if (root) root.innerHTML = `<p class="placeholder">Не удалось загрузить отзывы: ${escapeHtml(message)}</p>`;
  }
  function finishReviewVote(success, message) {
    if (pendingReviewVote) pendingReviewVote.disabled = false;
    pendingReviewVote = null;
    if (success) loadReviews(reviewPage);
    else window.app.setStatus("Оценка отзыва", message || "Не удалось отправить оценку");
  }
  window.releaseDetailScreen = { render: renderReleaseDetail, renderWatch, renderPlayer, renderReviews, showTypes, showSources, showEpisodes, showEpisode, showError, showPlayerError, showReviews, showReviewsError, finishReviewVote, listDone, listError };
})();
