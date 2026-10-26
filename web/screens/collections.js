(() => {
  function renderCollections() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen collections-screen">
        <header class="feature-header"><button class="settings-back" type="button">← Обзор</button><h1>Коллекции</h1><p>Подборки релизов сообщества</p></header>
        <nav class="collection-tabs"><button class="is-active">Популярные</button><button>Новые</button><button>Мои</button></nav>
        <div class="collection-skeletons"><div></div><div></div><div></div><div></div></div>
        <p class="collection-note">Публичные коллекции появятся после подключения Collection API.</p>
      </section>`;
    root.querySelector(".settings-back").addEventListener("click", () => window.app.navigate("discover"));
    root.querySelectorAll(".collection-tabs button").forEach(button => button.addEventListener("click", () => {
      root.querySelectorAll(".collection-tabs button").forEach(tab => tab.classList.remove("is-active")); button.classList.add("is-active");
      root.querySelector(".collection-note").textContent = `Раздел «${button.textContent}» будет подключён к Collection API.`;
    }));
  }
  window.appScreens = { ...(window.appScreens || {}), collections: renderCollections };
})();
