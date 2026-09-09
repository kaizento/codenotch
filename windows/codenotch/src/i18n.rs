//! Rust-side strings (tray menu, window labels, provider notes). The page has its own dictionary;
//! keys are kept identical on both sides.

use std::sync::RwLock;

/// The language the app currently runs in, already resolved ("auto" expanded).
/// Provider threads have no AppHandle, so the resolved value is mirrored here.
static CURRENT: RwLock<String> = RwLock::new(String::new());

pub fn set_current(lang: &str) {
    let resolved = if lang == "auto" { resolve_auto().to_string() } else { lang.to_string() };
    if let Ok(mut g) = CURRENT.write() {
        *g = resolved;
    }
}

pub fn current() -> String {
    match CURRENT.read() {
        Ok(g) if !g.is_empty() => g.clone(),
        _ => resolve_auto().to_string(),
    }
}

/// Translate with the app's current language — for code that has no `lang` at hand.
pub fn t(key: &str) -> &'static str {
    tr(&current(), key)
}

pub fn resolve_auto() -> &'static str {
    #[cfg(windows)]
    unsafe {
        use windows::Win32::Globalization::GetUserDefaultLocaleName;
        let mut buf = [0u16; 85];
        let n = GetUserDefaultLocaleName(&mut buf);
        if n > 0 {
            let name = String::from_utf16_lossy(&buf[..(n as usize - 1)]).to_lowercase();
            if name.starts_with("zh") {
                return "zh";
            }
            if name.starts_with("ja") {
                return "ja";
            }
            if name.starts_with("ko") {
                return "ko";
            }
            if name.starts_with("ru") {
                return "ru";
            }
        }
    }
    "en"
}

pub fn tr(lang: &str, key: &str) -> &'static str {
    let l = if lang == "auto" { resolve_auto() } else { lang };
    match (l, key) {
        // ---------- tray ----------
        ("zh", "install") => "安装 Claude Code 钩子",
        ("zh", "uninstall") => "卸载钩子",
        ("zh", "language") => "语言",
        ("zh", "lang_auto") => "跟随系统",
        ("zh", "reset_pos") => "重置悬浮条位置",
        ("zh", "quit") => "退出",
        ("zh", "hooks_missing") => "钩子未安装：右键托盘图标 → 安装 Claude Code 钩子（桌面版无需，已自动兜底）",
        ("zh", "autostart") => "开机自启（静默待命）",
        ("ja", "autostart") => "Windows起動時に自動開始",
        ("ko", "autostart") => "Windows 시작 시 자동 실행",
        ("zh", "refresh") => "立即刷新用量",
        ("zh", "open_data") => "打开数据文件夹（日志 / 图标）",
        ("ja", "open_data") => "データフォルダを開く（ログ / アイコン）",
        ("ko", "open_data") => "데이터 폴더 열기 (로그 / 아이콘)",
        ("ru", "open_data") => "Открыть папку данных (логи / иконки)",
        (_, "open_data") => "Open data folder (logs / icons)",
        ("ja", "refresh") => "使用量を今すぐ更新",
        ("ko", "refresh") => "사용량 지금 새로고침",
        ("ja", "install") => "Claude Code フックを導入",
        ("ja", "uninstall") => "フックを削除",
        ("ja", "language") => "言語",
        ("ja", "lang_auto") => "システムに従う",
        ("ja", "reset_pos") => "バー位置をリセット",
        ("ja", "quit") => "終了",
        ("ja", "hooks_missing") => "フック未導入：トレイ右クリック → フックを導入（デスクトップ版は自動フォールバック済み）",
        ("ko", "install") => "Claude Code 후크 설치",
        ("ko", "uninstall") => "후크 제거",
        ("ko", "language") => "언어",
        ("ko", "lang_auto") => "시스템 따르기",
        ("ko", "reset_pos") => "바 위치 초기화",
        ("ko", "quit") => "종료",
        ("ko", "hooks_missing") => "후크 미설치: 트레이 우클릭 → 후크 설치 (데스크톱판은 자동 폴백)",
        ("ru", "install") => "Установить хуки Claude Code",
        ("ru", "uninstall") => "Удалить хуки",
        ("ru", "language") => "Язык",
        ("ru", "lang_auto") => "Как в системе",
        ("ru", "reset_pos") => "Сбросить положение панели",
        ("ru", "quit") => "Выход",
        ("ru", "hooks_missing") => "Хуки не установлены: правый клик по значку в трее → Установить хуки Claude Code (для десктопного приложения работает запасной вариант)",
        ("ru", "autostart") => "Запускать с Windows (тихо)",
        ("ru", "refresh") => "Обновить показания сейчас",
        (_, "install") => "Install Claude Code hooks",
        (_, "uninstall") => "Uninstall hooks",
        (_, "language") => "Language",
        (_, "lang_auto") => "Follow system",
        (_, "reset_pos") => "Reset bar position",
        (_, "quit") => "Quit",
        (_, "hooks_missing") => "Hooks not installed: tray right-click → Install Claude Code hooks (desktop app auto-fallback active)",
        (_, "autostart") => "Start with Windows (silent)",
        (_, "refresh") => "Refresh usage now",

        // ---------- window labels ----------
        ("ru", "win_session") => "Текущая сессия",
        ("ru", "win_weekly_all") => "Неделя (все модели)",
        ("ru", "win_weekly_opus") => "Неделя (Opus)",
        ("ru", "win_weekly_scoped") => "Неделя (по модели)",
        ("ru", "win_weekly_limit") => "Недельный лимит",
        ("ru", "win_monthly_limit") => "Месячный лимит",
        ("ru", "win_min_limit") => "Лимит на {n} мин",
        ("ru", "win_hour_limit") => "Лимит на {n} ч",
        ("ru", "win_day_limit") => "Лимит на {n} дн",
        ("ru", "win_longer") => "Длинное окно",
        ("ru", "win_requests_today") => "Запросов сегодня · лимит не публикуется",
        (_, "win_session") => "Current session",
        (_, "win_weekly_all") => "Weekly (all models)",
        (_, "win_weekly_opus") => "Weekly (Opus)",
        (_, "win_weekly_scoped") => "Weekly (model-scoped)",
        (_, "win_weekly_limit") => "Weekly limit",
        (_, "win_monthly_limit") => "Monthly limit",
        (_, "win_min_limit") => "{n}m limit",
        (_, "win_hour_limit") => "{n}h limit",
        (_, "win_day_limit") => "{n}d limit",
        (_, "win_longer") => "Longer window",
        (_, "win_requests_today") => "Requests today · no limit published",

        // ---------- provider notes ----------
        ("ru", "note_no_credential") => "Учётные данные Claude Code не найдены",
        ("ru", "note_cred_expired") => "Срок учётных данных истёк — выполните любую команду claude (или напишите Claude), чтобы обновить их",
        ("ru", "note_cred_rejected") => "Учётные данные отклонены (сменили аккаунт?)",
        ("ru", "note_rate_limited") => "Лимит запросов, повтор через {n} с",
        ("ru", "note_rate_limited_dash") => "Лимит запросов — повтор через {n} с",
        ("ru", "note_codex_no_windows") => "Codex не вернул окон лимитов",
        ("ru", "note_live_failed") => "Не удалось прочитать данные ({e})",
        ("ru", "note_via_codex") => "{plan} · через Codex",
        ("ru", "note_via_antigravity") => "через Antigravity",
        ("ru", "note_ag_closed") => "Antigravity закрыт — показано последнее значение",
        ("ru", "note_ag_rejected") => "Google-сессия Antigravity отклонена — войдите заново в Antigravity",
        ("ru", "note_via_google") => "{tier} · через Google",
        ("ru", "note_ag_no_quota") => "{tier} · Google не публикует квоту для этого аккаунта",
        ("ru", "note_ag_open") => "Откройте Antigravity, чтобы прочитать квоту",
        (_, "note_no_credential") => "No Claude Code credential found",
        (_, "note_cred_expired") => "Credential expired — run any claude command (or chat with Claude) to refresh it",
        (_, "note_cred_rejected") => "Credential rejected (switched accounts?)",
        (_, "note_rate_limited") => "Rate limited, retrying in {n}s",
        (_, "note_rate_limited_dash") => "Rate limited — retrying in {n}s",
        (_, "note_codex_no_windows") => "Codex reported no usage windows",
        (_, "note_live_failed") => "Live read failed ({e})",
        (_, "note_via_codex") => "{plan} · via Codex",
        (_, "note_via_antigravity") => "via Antigravity",
        (_, "note_ag_closed") => "Antigravity is closed — last reading kept",
        (_, "note_ag_rejected") => "The Antigravity Google session was rejected — sign in again in Antigravity",
        (_, "note_via_google") => "{tier} · via Google",
        (_, "note_ag_no_quota") => "{tier} · Google publishes no quota for this account",
        (_, "note_ag_open") => "Open Antigravity to read its quota",

        // ---------- tray action results ----------
        ("ru", "msg_error") => "Ошибка: {e}",
        ("ru", "msg_missing_exe") => "не найден {path}",
        ("ru", "msg_hooks_written") => "хуки записаны: {path} (событий: {n})",
        ("ru", "msg_no_settings") => "settings.json не существует — удалять нечего",
        ("ru", "msg_no_hooks") => "конфигурация хуков не найдена",
        ("ru", "msg_hooks_removed") => "удалено хуков Codenotch: {n}",
        ("ru", "msg_autostart_on") => "автозапуск при входе включён (молчит, пока нет сессий)",
        ("ru", "msg_autostart_off") => "автозапуск при входе выключен",
        ("ru", "msg_autostart_absent") => "автозапуск при входе и не был включён",
        ("ru", "msg_reg_failed") => "не удалось запустить reg.exe",
        ("ru", "msg_already_running") => "Codenotch уже запущен ({build}) — закройте его из трея перед запуском новой сборки",
        (_, "msg_error") => "Error: {e}",
        (_, "msg_missing_exe") => "missing {path}",
        (_, "msg_hooks_written") => "wrote {path} ({n} events)",
        (_, "msg_no_settings") => "settings.json does not exist, nothing to uninstall",
        (_, "msg_no_hooks") => "no hooks configuration found",
        (_, "msg_hooks_removed") => "removed {n} Codenotch hook(s)",
        (_, "msg_autostart_on") => "start at sign-in enabled (silent until a session appears)",
        (_, "msg_autostart_off") => "start at sign-in disabled",
        (_, "msg_autostart_absent") => "start at sign-in was not enabled",
        (_, "msg_reg_failed") => "reg.exe failed to run",
        (_, "msg_already_running") => "Codenotch is already running ({build}) — quit it from the tray before starting a new build",

        _ => "?",
    }
}
