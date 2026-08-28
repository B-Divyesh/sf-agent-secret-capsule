import './styles.css';
import { focusAndAnnounceRoute, keepSkipLinkFirstAfterRouteFocus } from './route-focus';

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => { void navigator.serviceWorker.register('/sw.js'); });
}

keepSkipLinkFirstAfterRouteFocus();
window.addEventListener('load', focusAndAnnounceRoute);
