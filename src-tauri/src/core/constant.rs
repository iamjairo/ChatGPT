pub static TITLEBAR_HEIGHT: f64 = 28.0;
pub static ASK_HEIGHT: f64 = 120.0;

pub static WINDOW_SETTINGS: &str = "settings";

pub static INIT_SCRIPT: &str = r#"
window.addEventListener('DOMContentLoaded', function() {
    function handleUrlChange() {
        const url = window.location.href;
        if (url !== 'about:blank') {
            console.log('URL changed:', url);
            window.__TAURI__.webviewWindow.WebviewWindow.getByLabel('titlebar').emit('navigation:change', { url });
        }
    }

    function handleLinkClick(event) {
        const target = event.target;
        if (target.tagName === 'A' && target.target && target.target !== '_blank') {
            target.target = '_blank';
        }
    }

    document.addEventListener('click', handleLinkClick, true);
    window.addEventListener('popstate', handleUrlChange);
    window.addEventListener('pushState', handleUrlChange);
    window.addEventListener('replaceState', handleUrlChange);

    const originalPushState = history.pushState;
    const originalReplaceState = history.replaceState;

    history.pushState = function() {
        originalPushState.apply(this, arguments);
        console.log('pushState called');
        handleUrlChange();
    };

    history.replaceState = function() {
        originalReplaceState.apply(this, arguments);
        console.log('replaceState called');
        handleUrlChange();
    };

    handleUrlChange();
});
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titlebar_height_is_positive() {
        assert!(TITLEBAR_HEIGHT > 0.0, "TITLEBAR_HEIGHT must be positive");
    }

    #[test]
    fn test_ask_height_is_positive() {
        assert!(ASK_HEIGHT > 0.0, "ASK_HEIGHT must be positive");
    }

    #[test]
    fn test_ask_height_greater_than_titlebar() {
        assert!(
            ASK_HEIGHT > TITLEBAR_HEIGHT,
            "ASK_HEIGHT should be larger than TITLEBAR_HEIGHT"
        );
    }

    #[test]
    fn test_window_settings_is_not_empty() {
        assert!(!WINDOW_SETTINGS.is_empty(), "WINDOW_SETTINGS label must not be empty");
    }

    #[test]
    fn test_init_script_contains_dom_content_loaded() {
        assert!(
            INIT_SCRIPT.contains("DOMContentLoaded"),
            "INIT_SCRIPT should listen for DOMContentLoaded"
        );
    }

    #[test]
    fn test_init_script_contains_navigation_change_event() {
        assert!(
            INIT_SCRIPT.contains("navigation:change"),
            "INIT_SCRIPT should emit navigation:change"
        );
    }

    #[test]
    fn test_init_script_patches_history_push_state() {
        assert!(
            INIT_SCRIPT.contains("history.pushState"),
            "INIT_SCRIPT should patch history.pushState"
        );
    }

    #[test]
    fn test_init_script_patches_history_replace_state() {
        assert!(
            INIT_SCRIPT.contains("history.replaceState"),
            "INIT_SCRIPT should patch history.replaceState"
        );
    }

    #[test]
    fn test_init_script_opens_links_in_new_tab() {
        assert!(
            INIT_SCRIPT.contains("_blank"),
            "INIT_SCRIPT should redirect links to _blank"
        );
    }
}
