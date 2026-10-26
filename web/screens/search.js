(() => {
  let submittedQuery = "";

  function renderSearch() {
    const main = document.getElementById("screen-content");
    main.innerHTML = `
      <section class="screen" aria-labelledby="screen-title">
        <h1 class="screen-title" id="screen-title">Поиск</h1>
        <form id="release-search-form" class="search-form">
          <label class="field-label" for="release-query">Название релиза</label>
          <div class="search-row">
            <input id="release-query" name="query" type="search" maxlength="120" autocomplete="off" placeholder="Введите название" required>
            <button class="primary-button" id="release-search-submit" type="submit">Найти</button>
          </div>
        </form>
        <div id="search-results" class="search-results" aria-live="polite">
          <div class="placeholder">Введите название и нажмите «Найти».</div>
        </div>
      </section>`;

    const form = main.querySelector("#release-search-form");
    const input = main.querySelector("#release-query");
    input.value = submittedQuery;
    form.addEventListener("submit", event => {
      event.preventDefault();
      submittedQuery = input.value.trim();
      if (!submittedQuery) return;
      main.querySelector("#release-search-submit").disabled = true;
      window.appIpc.send({ action: "search_releases", query: submittedQuery });
    });
  }

  function showSearchPending(query) {
    const results = document.getElementById("search-results");
    if (results) results.innerHTML = `<div class="placeholder"></div>`;
    const pending = results?.querySelector(".placeholder");
    if (pending) pending.textContent = `Ищем «${query}»…`;
  }

  function showSearchResults(releases) {
    const results = document.getElementById("search-results");
    const button = document.getElementById("release-search-submit");
    if (!results) return;
    if (button) button.disabled = false;
    if (!releases.length) {
      results.innerHTML = '<div class="placeholder">Ничего не найдено.</div>';
      return;
    }
    results.replaceChildren(...releases.map(release => window.releaseCard.render(release, true)));
  }

  function showSearchError(message) {
    const results = document.getElementById("search-results");
    const button = document.getElementById("release-search-submit");
    if (button) button.disabled = false;
    if (!results) return;
    const error = document.createElement("div");
    error.className = "placeholder";
    error.textContent = `Поиск не выполнен: ${message}`;
    results.replaceChildren(error);
  }

  window.appScreens = {
    ...(window.appScreens || {}),
    search: renderSearch,
    showSearchPending,
    showSearchResults,
    showSearchError,
  };
})();
