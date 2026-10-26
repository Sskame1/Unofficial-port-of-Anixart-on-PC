(() => {
  const entries = [
    ["◐", "Тёмная тема", "Включена", "appearance"],
    ["♧", "Уведомления", "Настроить", "notifications"],
    ["▷", "Воспроизведение", "Плеер и качество", "playback"],
    ["◉", "Внешний вид", "Цвета и иконка", "appearance"],
    ["☁", "Управление данными", "Импорт и экспорт", "data"],
    ["◖", "Управление рекламой", "Настройки источников APK", "ads"],
    ["•••", "Дополнительно", "Другие параметры", "additional"],
    ["?", "Помощь", "Справка и поддержка", "help"],
    ["◇", "Правила сообщества", "", "rules"],
    ["↻", "Проверить обновления", "", "updates"],
  ];
  function renderSettings() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen settings-screen" aria-labelledby="screen-title">
        <h1 id="screen-title">Настройки</h1><p class="settings-owner">Гость · настройки приложения</p>
        <div class="settings-list">${entries.map(([icon, title, desc, id]) => `<button class="settings-row" type="button" data-setting="${id}"><span class="settings-row-icon">${icon}</span><span class="settings-row-copy"><strong>${title}</strong>${desc ? `<small>${desc}</small>` : ""}</span><span class="settings-chevron">›</span></button>`).join("")}</div>
        <div class="settings-footer">Интерфейс Anixart · перенос по APK</div>
      </section>`;
    root.querySelectorAll("[data-setting]").forEach(button => button.addEventListener("click", () => {
      const key = button.dataset.setting;
      if (key === "notifications") return window.app.navigate("notifications");
      if (key === "ads") return window.app.setStatus("Управление рекламой", "В APK есть раздельные настройки Kodik и веб-плеера. Переносим их после проверки поведения источников.");
      if (key === "playback" || key === "appearance" || key === "data" || key === "additional") return renderSettingDetail(key);
      window.app.setStatus(button.querySelector("strong").textContent, "Этот экран перенесём из APK отдельным срезом.");
    }));
  }
  function renderSettingDetail(key) {
    const titles = { playback: "Воспроизведение", appearance: "Внешний вид", data: "Управление данными", additional: "Дополнительно" };
    const root = document.getElementById("screen-content");
    root.innerHTML = `<section class="screen settings-screen"><button class="settings-back" type="button">← Настройки</button><h1>${titles[key]}</h1><div class="settings-detail-list">${(key === "playback" ? ["Плеер по умолчанию", "Качество видео", "Перемотка назад", "Перемотка вперёд", "Автоматический переход к следующей серии", "Субтитры"] : key === "appearance" ? ["Тёмная тема", "Цветовая схема", "Язык интерфейса", "Иконка приложения"] : key === "data" ? ["Экспорт закладок", "Импорт закладок", "Очистить кэш"] : ["Очистить историю", "Открыть папку загрузок", "О приложении"]).map(label => `<label class="settings-toggle-row"><span>${label}</span><input type="checkbox" ${/тема|автоматический/i.test(label) ? "checked" : ""}><i></i></label>`).join("")}</div><p class="settings-note">Настройки пока отображаются как интерфейс APK. Сохранение будет подключено к Windows-настройкам отдельно.</p></section>`;
    root.querySelector(".settings-back").addEventListener("click", renderSettings);
  }
  function renderNotifications() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `<section class="screen notifications-screen"><button class="settings-back" type="button">← Назад</button><h1>Уведомления</h1><div class="notification-empty"><span>♧</span><strong>Пока нет уведомлений</strong><p>Здесь будут уведомления Anixart</p></div></section>`;
    root.querySelector(".settings-back").addEventListener("click", () => window.app.navigate("home"));
  }
  window.appScreens = { ...(window.appScreens || {}), settings: renderSettings, notifications: renderNotifications };
})();
