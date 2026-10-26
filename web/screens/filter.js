(() => {
  let currentPage = 1;
  let lastFilter = null;

  function renderFilter() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen filter-screen">
        <header class="filter-header">
          <button class="settings-back" type="button">← Обзор</button>
          <h1>Фильтр релизов</h1>
        </header>
        
        <div class="filter-fields">
          <label class="filter-field">
            <span><small>Год от</small></span>
            <input id="filter-start" type="number" min="1900" max="2100" placeholder="Любой">
          </label>
          
          <label class="filter-field">
            <span><small>Год до</small></span>
            <input id="filter-end" type="number" min="1900" max="2100" placeholder="Любой">
          </label>
          
          <label class="filter-field">
            <span><small>Категория</small></span>
            <select id="filter-category">
              <option value="">Любая</option>
              <option value="1">Аниме</option>
              <option value="2">Дорама</option>
            </select>
          </label>
          
          <label class="filter-field">
            <span><small>Статус</small></span>
            <select id="filter-status">
              <option value="">Любой</option>
              <option value="1">Вышел</option>
              <option value="2">Выходит</option>
              <option value="3">Анонсирован</option>
            </select>
          </label>
          
          <label class="filter-field">
            <span><small>Студия</small></span>
            <input id="filter-studio" type="text" placeholder="Любая">
          </label>
          
          <label class="filter-field">
            <span><small>Источник</small></span>
            <input id="filter-source" type="text" placeholder="Любой">
          </label>
          
          <label class="filter-field">
            <span><small>Эпизодов от</small></span>
            <input id="filter-episodes-from" type="number" min="0" placeholder="Любое">
          </label>
          
          <label class="filter-field">
            <span><small>Эпизодов до</small></span>
            <input id="filter-episodes-to" type="number" min="0" placeholder="Любое">
          </label>
          
          <label class="filter-field">
            <span><small>Страна</small></span>
            <input id="filter-country" type="text" placeholder="Любая">
          </label>
          
          <label class="filter-field">
            <span><small>Сезон</small></span>
            <select id="filter-season">
              <option value="">Любой</option>
              <option value="1">Зима</option>
              <option value="2">Весна</option>
              <option value="3">Лето</option>
              <option value="4">Осень</option>
            </select>
          </label>
          
          <label class="filter-field">
            <span><small>Длительность эп. от (мин)</small></span>
            <input id="filter-duration-from" type="number" min="0" placeholder="Любая">
          </label>
          
          <label class="filter-field">
            <span><small>Длительность эп. до (мин)</small></span>
            <input id="filter-duration-to" type="number" min="0" placeholder="Любая">
          </label>
          
          <label class="filter-field">
            <span><small>Жанры (через запятую)</small></span>
            <input id="filter-genres" type="text" placeholder="Например: боевик, комедия">
          </label>
          
          <label class="filter-field">
            <span><small>Возрастной рейтинг</small></span>
            <select id="filter-age-rating" multiple size="3">
              <option value="1">G - Все возрасты</option>
              <option value="2">PG - Дети</option>
              <option value="3">PG-13 - Подростки</option>
              <option value="4">R - 17+</option>
              <option value="5">R+ - Насилие</option>
              <option value="6">Rx - Хентай</option>
            </select>
          </label>
          
          <label class="filter-field">
            <span><small>Сортировка</small></span>
            <select id="filter-sort">
              <option value="0">По обновлению ↓</option>
              <option value="1">По оценке ↓</option>
              <option value="2">По году ↓</option>
              <option value="3">По популярности ↓</option>
              <option value="4">По обновлению ↑</option>
              <option value="5">По оценке ↑</option>
              <option value="6">По году ↑</option>
              <option value="7">По популярности ↑</option>
            </select>
          </label>
        </div>
        
        <div class="filter-actions">
          <button id="filter-reset" type="button">Сбросить</button>
          <button id="filter-apply" type="button">Применить фильтр</button>
        </div>
        
        <div id="filter-results" class="release-grid" aria-live="polite"></div>
        
        <div class="filter-pagination" id="filter-pagination" style="display: none;">
          <button id="filter-prev" type="button">← Назад</button>
          <span id="filter-page-info"></span>
          <button id="filter-next" type="button">Далее →</button>
        </div>
      </section>`;

    root.querySelector(".settings-back").addEventListener("click", () =>
      window.app.navigate("discover")
    );
    root.querySelector("#filter-reset").addEventListener("click", resetFilter);
    root.querySelector("#filter-apply").addEventListener("click", applyFilter);
    root.querySelector("#filter-prev")?.addEventListener("click", () => {
      if (currentPage > 1) {
        currentPage--;
        submitFilter();
      }
    });
    root.querySelector("#filter-next")?.addEventListener("click", () => {
      currentPage++;
      submitFilter();
    });
  }

  function resetFilter() {
    document.getElementById("filter-start").value = "";
    document.getElementById("filter-end").value = "";
    document.getElementById("filter-category").value = "";
    document.getElementById("filter-status").value = "";
    document.getElementById("filter-studio").value = "";
    document.getElementById("filter-source").value = "";
    document.getElementById("filter-episodes-from").value = "";
    document.getElementById("filter-episodes-to").value = "";
    document.getElementById("filter-country").value = "";
    document.getElementById("filter-season").value = "";
    document.getElementById("filter-duration-from").value = "";
    document.getElementById("filter-duration-to").value = "";
    document.getElementById("filter-genres").value = "";
    document.getElementById("filter-age-rating").selectedIndex = -1;
    document.getElementById("filter-sort").value = "0";
    document.getElementById("filter-results").replaceChildren();
    document.getElementById("filter-pagination").style.display = "none";
    currentPage = 1;
    lastFilter = null;
  }

  function applyFilter() {
    currentPage = 1;
    submitFilter();
  }

  function submitFilter() {
    const start = Number(document.getElementById("filter-start").value) || null;
    const end = Number(document.getElementById("filter-end").value) || null;
    
    if (start && end && start > end) {
      window.app.setStatus("Фильтр", "Год «от» не может быть больше года «до».");
      return;
    }

    const genresInput = document.getElementById("filter-genres").value.trim();
    const genres = genresInput ? genresInput.split(",").map(g => g.trim()).filter(g => g) : [];

    const ageRatingSelect = document.getElementById("filter-age-rating");
    const ageRatings = Array.from(ageRatingSelect.selectedOptions).map(opt =>
      Number(opt.value)
    );

    const category = document.getElementById("filter-category").value;
    const status = document.getElementById("filter-status").value;
    const studio = document.getElementById("filter-studio").value.trim();
    const source = document.getElementById("filter-source").value.trim();
    const episodesFrom = Number(document.getElementById("filter-episodes-from").value) || null;
    const episodesTo = Number(document.getElementById("filter-episodes-to").value) || null;
    const country = document.getElementById("filter-country").value.trim();
    const season = document.getElementById("filter-season").value;
    const durationFrom = Number(document.getElementById("filter-duration-from").value) || null;
    const durationTo = Number(document.getElementById("filter-duration-to").value) || null;

    lastFilter = {
      start_year: start,
      end_year: end,
      category_id: category ? Number(category) : null,
      status_id: status ? Number(status) : null,
      studio: studio || null,
      source: source || null,
      episodes_from: episodesFrom,
      episodes_to: episodesTo,
      country: country || null,
      season: season ? Number(season) : null,
      episode_duration_from: durationFrom,
      episode_duration_to: durationTo,
      genres,
      age_ratings: ageRatings,
      sort: Number(document.getElementById("filter-sort").value),
      types: [],
      profile_list_exclusions: [],
      is_genres_exclude_mode_enabled: false,
      genres_mode: null
    };

    const results = document.getElementById("filter-results");
    results.innerHTML = '<p class="placeholder">Применяем фильтр…</p>';
    
    window.appIpc.send({
      action: "apply_filter",
      page: currentPage,
      filter: lastFilter
    });
  }

  function showResults(response) {
    const root = document.getElementById("filter-results");
    if (!root) return;

    if (!response.content?.length) {
      root.innerHTML = '<p class="placeholder">По заданным критериям ничего не найдено.</p>';
      document.getElementById("filter-pagination").style.display = "none";
      return;
    }

    root.replaceChildren(...response.content.map(release => window.releaseCard.render(release)));

    const pagination = document.getElementById("filter-pagination");
    const pageInfo = document.getElementById("filter-page-info");
    
    if (response.total_page_count > 1) {
      pagination.style.display = "flex";
      pageInfo.textContent = `Страница ${response.current_page} из ${response.total_page_count}`;
      
      document.getElementById("filter-prev").disabled = response.current_page <= 1;
      document.getElementById("filter-next").disabled = response.current_page >= response.total_page_count;
    } else {
      pagination.style.display = "none";
    }
  }

  function showError(message) {
    const root = document.getElementById("filter-results");
    if (root) {
      root.innerHTML = `<p class="placeholder">Не удалось применить фильтр: ${message}</p>`;
    }
  }

  window.appScreens = {
    ...(window.appScreens || {}),
    filter: renderFilter,
    showFilterResults: showResults,
    showFilterError: showError
  };
})();
