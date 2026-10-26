(() => {
  const copy = {
    signin: { title: "Вход", subtitle: "Войдите, чтобы синхронизировать свои списки", action: "Войти", switch: "Нет аккаунта?", switchLabel: "Создать профиль" },
    signup: { title: "Создать профиль", subtitle: "Присоединяйтесь к сообществу Anixart", action: "Продолжить", switch: "Уже есть аккаунт?", switchLabel: "Войти" },
    restore: { title: "Восстановление пароля", subtitle: "Укажите почту, привязанную к аккаунту", action: "Отправить код", switch: "Вспомнили пароль?", switchLabel: "Войти" },
    verify: { title: "Подтверждение почты", subtitle: "Введите код из письма", action: "Подтвердить", switch: "Письмо не пришло?", switchLabel: "Отправить повторно" },
  };
  function renderAuth(route) {
    const state = copy[route] || copy.signin;
    const main = document.getElementById("screen-content");
    main.innerHTML = `
      <section class="auth-screen" aria-labelledby="auth-title">
        <button class="auth-back" id="auth-back" type="button" aria-label="Назад">←</button>
        <div class="auth-brand-mark" aria-hidden="true">A</div>
        <div class="auth-heading"><h1 id="auth-title"></h1><p id="auth-subtitle"></p></div>
        ${route === "signin" ? `
          <div class="auth-providers"><p class="auth-caption">Войти с помощью</p>
            <button class="provider-button" data-provider="yandex"><span class="provider-icon yandex">Я</span>Яндекс</button>
            <button class="provider-button" data-provider="vk"><span class="provider-icon vk">ᴠκ</span>ВКонтакте</button>
            <button class="provider-button" data-provider="telegram"><span class="provider-icon telegram">➤</span>Telegram</button>
            <button class="provider-button" data-provider="google"><span class="provider-icon google">G</span>Google</button>
          </div><div class="auth-divider"><span>или по электронной почте</span></div>` : ""}
        <form class="auth-form" id="auth-form" novalidate></form>
        <div class="auth-error" id="auth-error" role="alert" hidden></div>
        <div class="auth-switch"><span id="auth-switch-text"></span><button id="auth-switch" type="button"></button></div>
        ${route === "signin" || route === "signup" ? `<button class="auth-skip" id="auth-skip" type="button">Продолжить как гость</button>` : ""}
        <p class="auth-terms">Продолжая, вы принимаете <button type="button">условия использования</button> и <button type="button">политику конфиденциальности</button>.</p>
      </section>`;
    document.getElementById("auth-title").textContent = state.title;
    document.getElementById("auth-subtitle").textContent = state.subtitle;
    document.getElementById("auth-switch-text").textContent = state.switch;
    document.getElementById("auth-switch").textContent = state.switchLabel;
    const form = document.getElementById("auth-form");
    if (route === "signin") form.innerHTML = `
      <label class="auth-field">Почта или логин<input name="login" autocomplete="username" required placeholder="Введите почту или логин"></label>
      <label class="auth-field">Пароль<span class="password-field"><input name="password" type="password" autocomplete="current-password" required placeholder="Введите пароль"><button type="button" class="password-toggle" aria-label="Показать пароль">◉</button></span></label>
      <button class="auth-forgot" type="button" id="auth-forgot">Забыли пароль?</button><button class="auth-submit" type="submit">${state.action}</button>`;
    else if (route === "signup") form.innerHTML = `
      <label class="auth-field">Логин<input name="login" autocomplete="username" required minlength="3" placeholder="Придумайте логин"></label>
      <label class="auth-field">Электронная почта<input name="email" type="email" autocomplete="email" required placeholder="name@example.com"></label>
      <label class="auth-field">Пароль<input name="password" type="password" autocomplete="new-password" required placeholder="Не менее 8 символов"></label>
      <label class="auth-consent"><input type="checkbox" required><span>Я принимаю условия регистрации и обработки данных</span></label><button class="auth-submit" type="submit">${state.action}</button>`;
    else form.innerHTML = `<label class="auth-field">${route === "verify" ? "Код подтверждения" : "Электронная почта"}<input name="value" ${route === "verify" ? "inputmode=\"numeric\"" : "type=\"email\""} required placeholder="${route === "verify" ? "Введите код" : "name@example.com"}"></label><button class="auth-submit" type="submit">${state.action}</button>`;
    document.getElementById("auth-back").addEventListener("click", () => window.app.navigate("profile"));
    document.getElementById("auth-switch").addEventListener("click", () => window.app.auth(route === "signin" ? "signup" : "signin"));
    document.getElementById("auth-forgot")?.addEventListener("click", () => window.app.auth("restore"));
    document.getElementById("auth-skip")?.addEventListener("click", () => window.app.navigate("home"));
    form.addEventListener("submit", event => { event.preventDefault(); if (form.reportValidity()) showAuthError("Сетевая авторизация будет подключена следующим этапом."); });
    main.querySelectorAll("[data-provider]").forEach(button => button.addEventListener("click", () => showAuthError(`${button.textContent.trim()}: требуется официальная настройка OAuth для Windows.`)));
    main.querySelector(".password-toggle")?.addEventListener("click", () => { const input = form.querySelector('[name="password"]'); input.type = input.type === "password" ? "text" : "password"; });
  }
  function showAuthError(message) { const error = document.getElementById("auth-error"); if (error) { error.hidden = false; error.textContent = message; } }
  window.appScreens = { ...(window.appScreens || {}), auth: renderAuth, showAuthError };
})();
