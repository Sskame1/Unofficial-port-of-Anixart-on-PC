(() => {
  let signInTimeout = null;

  function renderAuth(mode = "signin") {
    const root = document.getElementById("screen-content");

    const titles = {
      signin: { title: "Вход в аккаунт", subtitle: "Войдите для доступа ко всем функциям" },
      signup: { title: "Регистрация", subtitle: "Создайте новый аккаунт" },
      restore: { title: "Восстановление", subtitle: "Введите email для восстановления пароля" }
    };

    const current = titles[mode] || titles.signin;

    root.innerHTML = `
      <div style="max-width: 500px; margin: 0 auto; padding-top: 60px;">
        <div style="text-align: center; margin-bottom: 40px;">
          <div style="width: 80px; height: 80px; margin: 0 auto 20px; background: linear-gradient(135deg, var(--accent), #e91e63); border-radius: 20px; display: flex; align-items: center; justify-content: center; font-size: 40px; color: white; box-shadow: var(--shadow);">
            A
          </div>
          <h1 style="font-size: 28px; margin-bottom: 8px;">${current.title}</h1>
          <p style="color: var(--text-secondary); font-size: 15px;">${current.subtitle}</p>
        </div>

        <div class="profile-section">
          <div id="auth-error" class="auth-error" role="alert" hidden></div>
          ${mode === "signin" ? `
            <div style="display: grid; gap: 16px;">
              <div>
                <label style="display: block; margin-bottom: 8px; font-size: 14px; font-weight: 500;">
                  Логин или Email
                </label>
                <input type="text" id="auth-login" placeholder="Введите логин или email" style="width: 100%; padding: 12px 16px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-primary); color: var(--text-primary); font-size: 15px;">
              </div>
              <div>
                <label style="display: block; margin-bottom: 8px; font-size: 14px; font-weight: 500;">
                  Пароль
                </label>
                <input type="password" id="auth-password" placeholder="Введите пароль" style="width: 100%; padding: 12px 16px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-primary); color: var(--text-primary); font-size: 15px;">
              </div>
              <div style="text-align: right;">
                <button class="btn btn-secondary" style="border: 0; padding: 0; font-size: 14px;" onclick="window.appScreens.auth('restore')">
                  Забыли пароль?
                </button>
              </div>
              <button class="btn btn-primary" id="auth-submit" style="width: 100%; padding: 14px; font-size: 15px;">
                Войти
              </button>
              <button class="btn btn-secondary google-signin" id="google-signin" type="button" style="width: 100%; padding: 14px;">
                <span class="google-mark" aria-hidden="true">G</span>
                Продолжить с Google
              </button>
            </div>
          ` : mode === "signup" ? `
            <div style="display: grid; gap: 16px;">
              <div>
                <label style="display: block; margin-bottom: 8px; font-size: 14px; font-weight: 500;">
                  Логин
                </label>
                <input type="text" id="auth-login" placeholder="Выберите логин" style="width: 100%; padding: 12px 16px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-primary); color: var(--text-primary); font-size: 15px;">
              </div>
              <div>
                <label style="display: block; margin-bottom: 8px; font-size: 14px; font-weight: 500;">
                  Email
                </label>
                <input type="email" id="auth-email" placeholder="Ваш email" style="width: 100%; padding: 12px 16px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-primary); color: var(--text-primary); font-size: 15px;">
              </div>
              <div>
                <label style="display: block; margin-bottom: 8px; font-size: 14px; font-weight: 500;">
                  Пароль
                </label>
                <input type="password" id="auth-password" placeholder="Придумайте пароль" style="width: 100%; padding: 12px 16px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-primary); color: var(--text-primary); font-size: 15px;">
              </div>
              <button class="btn btn-primary" id="auth-submit" style="width: 100%; padding: 14px; font-size: 15px;">
                Зарегистрироваться
              </button>
            </div>
          ` : `
            <div style="display: grid; gap: 16px;">
              <div>
                <label style="display: block; margin-bottom: 8px; font-size: 14px; font-weight: 500;">
                  Email
                </label>
                <input type="email" id="auth-email" placeholder="Ваш email" style="width: 100%; padding: 12px 16px; border: 1px solid var(--border); border-radius: 8px; background: var(--bg-primary); color: var(--text-primary); font-size: 15px;">
              </div>
              <button class="btn btn-primary" id="auth-submit" style="width: 100%; padding: 14px; font-size: 15px;">
                Отправить код
              </button>
            </div>
          `}

          <div style="text-align: center; margin-top: 24px; font-size: 14px; color: var(--text-secondary);">
            ${mode === "signin" ? `
              Нет аккаунта?
              <button class="btn btn-secondary" style="border: 0; padding: 0; color: var(--accent);" onclick="window.appScreens.auth('signup')">
                Зарегистрироваться
              </button>
            ` : `
              Уже есть аккаунт?
              <button class="btn btn-secondary" style="border: 0; padding: 0; color: var(--accent);" onclick="window.appScreens.auth('signin')">
                Войти
              </button>
            `}
          </div>
        </div>

        <div style="text-align: center; margin-top: 24px;">
          <button class="btn btn-secondary" onclick="window.app.navigate('home')">
            Вернуться на главную
          </button>
        </div>
      </div>
    `;

    const submit = root.querySelector("#auth-submit");
    if (submit && mode === "signin") {
      submit.addEventListener("click", () => {
        const login = root.querySelector("#auth-login")?.value.trim();
        const password = root.querySelector("#auth-password")?.value;
        if (!login || !password) {
          window.app.setStatus("Вход", "Заполните логин и пароль.");
          return;
        }
        submit.disabled = true;
        submit.textContent = "Выполняется вход…";
        const error = root.querySelector("#auth-error");
        if (error) error.hidden = true;
        if (signInTimeout) clearTimeout(signInTimeout);
        signInTimeout = setTimeout(() => {
          signInTimeout = null;
          submit.disabled = false;
          submit.textContent = "Войти";
          showAuthError("Сервер Anixart долго не отвечает. Проверьте подключение и попробуйте ещё раз.");
        }, 32000);
        window.appIpc.send({ action: "sign_in", login, password });
      });

      root.querySelector("#google-signin")?.addEventListener("click", () => {
        const googleButton = root.querySelector("#google-signin");
        googleButton.disabled = true;
        googleButton.dataset.label = googleButton.textContent.trim();
        googleButton.textContent = "Открываем Google…";
        window.appIpc.send({ action: "sign_in_google" });
      });
    }
  }

  function showAuthError(message) {
    if (signInTimeout) {
      clearTimeout(signInTimeout);
      signInTimeout = null;
    }
    const submit = document.getElementById("auth-submit");
    if (submit) {
      submit.disabled = false;
      submit.textContent = "Войти";
    }
    const googleButton = document.getElementById("google-signin");
    if (googleButton) {
      googleButton.disabled = false;
      googleButton.innerHTML = '<span class="google-mark" aria-hidden="true">G</span>Продолжить с Google';
    }
    const error = document.getElementById("auth-error");
    if (!error) return;
    error.textContent = message;
    error.hidden = false;
  }

  window.appScreens = { ...(window.appScreens || {}), auth: renderAuth, showAuthError };
})();
