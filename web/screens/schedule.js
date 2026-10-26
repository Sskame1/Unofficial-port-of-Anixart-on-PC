(() => {
  const days = [
    ["monday", "Понедельник"],
    ["tuesday", "Вторник"],
    ["wednesday", "Среда"],
    ["thursday", "Четверг"],
    ["friday", "Пятница"],
    ["saturday", "Суббота"],
    ["sunday", "Воскресенье"]
  ];
  let selected = "monday";
  let schedule = null;

  function renderSchedule() {
    const root = document.getElementById("screen-content");
    root.innerHTML = `
      <section class="screen schedule-screen">
        <header class="feature-header">
          <button class="settings-back" type="button">← Обзор</button>
          <h1>Расписание</h1>
          <p>Новые эпизоды по дням недели</p>
        </header>
        <nav class="schedule-days" role="tablist">
          ${days.map(([key, label]) =>
            `<button class="schedule-day${key === selected ? " is-active" : ""}"
                     role="tab"
                     aria-selected="${key === selected}"
                     data-day="${key}">
              <span>${label}</span>
            </button>`
          ).join("")}
        </nav>
        <div class="schedule-state" id="schedule-results">
          <p class="placeholder">Загружаем расписание…</p>
        </div>
      </section>`;

    root.querySelector(".settings-back").addEventListener("click", () =>
      window.app.navigate("discover")
    );

    root.querySelectorAll(".schedule-day").forEach(button =>
      button.addEventListener("click", () => {
        selected = button.dataset.day;
        root.querySelectorAll(".schedule-day").forEach(day => {
          day.classList.toggle("is-active", day === button);
          day.setAttribute("aria-selected", day === button);
        });
        renderItems();
      })
    );

    window.appIpc.send({ action: "load_schedule" });
  }

  function renderItems() {
    const root = document.getElementById("schedule-results");
    if (!root || !schedule) return;

    const releases = schedule[selected] || [];
    if (!releases.length) {
      root.innerHTML = '<p class="placeholder">На этот день релизы не запланированы.</p>';
      return;
    }

    root.className = "release-grid schedule-grid";
    root.replaceChildren(...releases.map(release => window.releaseCard.render(release)));
  }

  function showSchedule(data) {
    schedule = data;
    renderItems();
  }

  function showError(message) {
    const root = document.getElementById("schedule-results");
    if (root) {
      root.innerHTML = `
        <div class="error-state">
          <p>Не удалось загрузить расписание</p>
          <p class="error-message">${message}</p>
          <button type="button" id="retry-schedule">Повторить</button>
        </div>`;

      root.querySelector("#retry-schedule")?.addEventListener("click", () => {
        root.innerHTML = '<p class="placeholder">Загружаем расписание…</p>';
        window.appIpc.send({ action: "load_schedule" });
      });
    }
  }

  window.appScreens = {
    ...(window.appScreens || {}),
    schedule: renderSchedule,
    showSchedule,
    showScheduleError: showError
  };
})();
