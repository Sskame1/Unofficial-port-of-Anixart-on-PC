(() => {
  function renderNotifications() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen notifications-screen">
        <header class="feature-header">
          <button class="settings-back" type="button">← Обзор</button>
          <h1>Уведомления</h1>
        </header>
        <div class="notification-empty">
          <span>🔔</span>
          <strong>Нет новых уведомлений</strong>
          <p>Здесь будут появляться уведомления о новых эпизодах, комментариях и других событиях</p>
        </div>
      </section>`;

    root.querySelector(".settings-back").addEventListener("click", () =>
      window.app.navigate("discover")
    );
  }

  window.appScreens = {
    ...(window.appScreens || {}),
    notifications: renderNotifications
  };
})();
