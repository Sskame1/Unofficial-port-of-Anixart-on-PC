(() => {
  function renderProfile() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen profile-screen" aria-labelledby="screen-title">
        <header class="profile-hero"><div class="profile-avatar">◉</div><h1 id="screen-title">Гость</h1><p>Войдите, чтобы открыть профиль Anixart</p><button class="profile-login" type="button">Войти в аккаунт</button><button class="profile-more" type="button" aria-label="Дополнительно">⋮</button></header>
        <section class="profile-stats"><div class="profile-stat"><span>●</span><div><strong>Смотрю</strong><b>—</b></div></div><div class="profile-stat"><span>●</span><div><strong>В планах</strong><b>—</b></div></div><div class="profile-stat"><span>●</span><div><strong>Просмотрено</strong><b>—</b></div></div><div class="profile-stat"><span>●</span><div><strong>Отложено</strong><b>—</b></div></div><div class="profile-donut"></div></section>
        <section class="profile-detail"><h2>Статистика профиля</h2><p>Жанры и динамика просмотра будут показаны после входа в аккаунт.</p><button type="button" class="profile-login">Войти, чтобы синхронизировать</button></section>
        <section class="profile-detail"><h2>Просмотрено недавно</h2><div class="history-empty">Войдите, чтобы продолжить просмотр с любого устройства.</div></section>
        <button class="profile-settings" type="button">⚙ &nbsp; Настройки приложения <span>›</span></button>
      </section>`;
    root.querySelectorAll(".profile-login").forEach(button => button.addEventListener("click", () => window.app.auth("signin")));
    root.querySelector(".profile-settings").addEventListener("click", () => window.app.navigate("settings"));
    root.querySelector(".profile-more").addEventListener("click", () => window.app.setStatus("Профиль", "Дополнительные действия появятся после входа."));
  }
  window.appScreens = { ...(window.appScreens || {}), profile: renderProfile };
})();
