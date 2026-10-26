(() => {
  function renderFeed() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen feed-screen" aria-labelledby="screen-title">
        <nav class="feed-tabs" role="tablist"><button class="is-active" role="tab">Моя лента</button><button role="tab">Свежее</button><button role="tab" aria-label="Рекомендации">☆</button></nav>
        <div class="feed-heading"><h1 id="screen-title">Последнее⌄</h1><button type="button">Все</button></div>
        <div class="feed-people" aria-label="Активные пользователи"><span>●</span><span>●</span><span>●</span><span>●</span></div>
        <button class="feed-compose" type="button"><span class="avatar-placeholder">◉</span><span>Расскажите о чём-нибудь...</span></button>
        <div class="feed-empty"><div class="feed-empty-icon">▤</div><strong>Лента сообщества</strong><p>Записи и обновления появятся после загрузки публичной ленты.</p><button class="library-login" type="button">Войти в аккаунт</button></div>
      </section>`;
    root.querySelectorAll(".feed-tabs button").forEach(button => button.addEventListener("click", () => {
      root.querySelectorAll(".feed-tabs button").forEach(tab => tab.classList.remove("is-active"));
      button.classList.add("is-active");
      window.app.setStatus("Лента", "Эта вкладка будет подключена к API ленты.");
    }));
    root.querySelector(".feed-compose").addEventListener("click", () => window.app.auth("signin"));
    root.querySelector(".feed-empty .library-login").addEventListener("click", () => window.app.auth("signin"));
  }
  window.appScreens = { ...(window.appScreens || {}), feed: renderFeed };
})();
