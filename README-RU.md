# Anixart для Windows

Десктопный клиент Anixart на Rust. Интерфейс собран из локальных HTML-, CSS- и JavaScript-модулей и открывается во встроенном Windows WebView.

## Сборка

Требования: Windows x64, Rust и Microsoft Edge WebView2 Runtime.

```powershell
cargo run --release
```

Чтобы выбрать другой API endpoint Anixart, перед запуском задайте `ANIXART_API_URL`: `https://api-s.anixsekai.com/`, `https://api.anixart.app/` или `https://api.anixart.tv/`.

## ToDo

- Проверить полный просмотр эпизода от открытия до конца.
- Завершить перенос каталога, профиля, коллекций и социальных функций.
- Проверить зеркала API и действия аккаунта на живом сервисе.
- Улучшить упаковку приложения и автоматизацию релизов Windows.

## Лицензия

Код проекта распространяется по MIT. Это не предоставляет прав на Anixart, предоставленный APK, видео и сторонние брендовые материалы. Подробности — в [LICENSE](LICENSE).
