<p align="center">
  <img src="web/assets/logo.png" alt="Логотип Anixart" width="96">
</p>

# Anixart для Windows

## Описание

Десктопный клиент Anixart на Rust. Интерфейс использует локальные HTML-, CSS- и JavaScript-модули, собранные во встроенный Windows WebView.

## Сборка

Требования: Windows x64, Rust и Microsoft Edge WebView2 Runtime.

```powershell
cargo run --release
```

Приложение использует настроенный API Anixart и хранит локальный кеш изображений в `%LOCALAPPDATA%\AnixartWindowsClient\Memory`.

## Лицензия

Код проекта распространяется по лицензии MIT. Лицензия не предоставляет прав на Anixart, предоставленный APK, медиаматериалы и сторонние брендовые ресурсы. Подробности — в [LICENSE](LICENSE).
