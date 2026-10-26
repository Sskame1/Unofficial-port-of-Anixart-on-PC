(() => {
  const tabs = ["Коллекции", "История", "Избранное", "Смотрю", "В планах", "Просмотрено", "Отложено", "Брошено"];
  function renderBookmarks() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen library-screen" aria-labelledby="screen-title">
        <div class="library-tabs" role="tablist">${tabs.map((tab, index) => `<button class="library-tab${index === 1 ? " is-active" : ""}" data-tab="${index}" role="tab" aria-selected="${index === 1}">${tab}</button>`).join("")}</div>
        <div class="library-tools"><strong id="library-count">История</strong><button class="sort-pill" type="button">☰ &nbsp;По добавлению⌄</button><button class="shuffle-button" type="button" aria-label="Случайный релиз">⤨</button></div>
        <div class="library-grid" id="library-grid"></div>
      </section>`;
    root.querySelectorAll(".library-tab").forEach(button => button.addEventListener("click", () => {
      root.querySelectorAll(".library-tab").forEach(tab => { tab.classList.remove("is-active"); tab.setAttribute("aria-selected", "false"); });
      button.classList.add("is-active"); button.setAttribute("aria-selected", "true");
      document.getElementById("library-count").textContent = button.textContent;
      showEmpty(button.textContent);
    }));
    root.querySelector(".sort-pill").addEventListener("click", () => window.app.setStatus("Сортировка", "Списки аккаунта появятся после переноса синхронизации."));
    root.querySelector(".shuffle-button").addEventListener("click", () => window.app.navigate("discover"));
    showEmpty("История");
  }
  function showEmpty(tab) {
    const grid = document.getElementById("library-grid");
    if (!grid) return;
    grid.innerHTML = `<div class="library-empty"><div class="library-empty-mark">▣</div><h2>${tab}</h2><p>Список Anixart хранится в вашем аккаунте.</p><button class="library-login" type="button">Войти в аккаунт</button></div>`;
    grid.querySelector(".library-login").addEventListener("click", () => window.app.auth("signin"));
  }
  window.appScreens = { ...(window.appScreens || {}), bookmarks: renderBookmarks };
})();
