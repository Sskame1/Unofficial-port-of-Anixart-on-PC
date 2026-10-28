(() => {
  function renderSettings() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <div class="screen-header">
        <h1>Настройки</h1>
        <p>Настройки приложения и аккаунта</p>
      </div>

      <div class="profile-section">
        <h2>Производительность</h2>
        <div class="settings-row">
          <div>
            <strong>Кеш изображений</strong>
            <p style="font-size: 13px; color: var(--text-secondary); margin-top: 4px;">Сохранять постеры, аватары, скриншоты и баннеры на диске для быстрой загрузки.</p>
          </div>
          <label class="settings-switch"><input type="checkbox" id="settings-image-cache" checked><span>Включено</span></label>
        </div>
      </div>

      <div class="profile-section">
        <h2>Внешний вид</h2>
        <div class="settings-row">
          <div>
            <strong>Тема оформления</strong>
            <p style="font-size: 13px; color: var(--text-secondary); margin-top: 4px;">
              Переключение между светлой и темной темой
            </p>
          </div>
          <button class="btn btn-secondary" id="settings-theme-toggle">
            Изменить тему
          </button>
        </div>
      </div>

      <div class="profile-section">
        <h2>Аккаунт</h2>
        <div class="settings-row">
          <div>
            <strong>Профиль</strong>
            <p style="font-size: 13px; color: var(--text-secondary); margin-top: 4px;">
              Управление данными профиля
            </p>
          </div>
          <button class="btn btn-secondary" onclick="window.app.navigate('profile')">
            Перейти
          </button>
        </div>
        <div class="settings-row" style="margin-top: 16px;">
          <div>
            <strong>Выйти из аккаунта</strong>
            <p style="font-size: 13px; color: var(--text-secondary); margin-top: 4px;">
              Завершить текущую сессию
            </p>
          </div>
          <button class="btn btn-secondary" id="settings-signout" style="color: var(--accent);">
            Выйти
          </button>
        </div>
      </div>

      <div class="profile-section">
        <h2>Воспроизведение</h2>
        <div class="settings-row">
          <div>
            <strong>Автовоспроизведение</strong>
            <p style="font-size: 13px; color: var(--text-secondary); margin-top: 4px;">
              Автоматически запускать следующий эпизод
            </p>
          </div>
          <label style="display: flex; align-items: center; gap: 8px;">
            <input type="checkbox" checked style="width: 20px; height: 20px;">
            <span style="font-size: 14px;">Включено</span>
          </label>
        </div>
      </div>

      <div class="profile-section">
        <h2>О приложении</h2>
        <div class="settings-row">
          <div>
            <strong>Версия</strong>
            <p style="font-size: 13px; color: var(--text-secondary); margin-top: 4px;">
              Anixart Desktop v${window.appVersion || "0.3.0"}
            </p>
          </div>
        </div>
      </div>
    `;

    const themeToggle = root.querySelector("#settings-theme-toggle");
    if (themeToggle) {
      themeToggle.addEventListener("click", () => {
        window.app.toggleTheme();
      });
    }
    const cacheToggle = root.querySelector("#settings-image-cache");
    if (cacheToggle) {
      cacheToggle.checked = window.appImageCache?.isEnabled?.() !== false;
      cacheToggle.addEventListener("change", () => {
        window.appIpc.send({ action: "set_image_cache_enabled", enabled: cacheToggle.checked });
      });
    }
    root.querySelector("#settings-signout")?.addEventListener("click", () => {
      window.appIpc.send({ action: "sign_out" });
    });
  }

  window.appScreens = { ...(window.appScreens || {}), settings: renderSettings };
})();
