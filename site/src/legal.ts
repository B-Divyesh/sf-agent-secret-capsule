import './styles.css';

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => { void navigator.serviceWorker.register('/sw.js'); });
}

window.addEventListener('load', () => {
  const heading = document.querySelector<HTMLElement>('h1');
  heading?.focus();
  const announcement = document.getElementById('route-announcement');
  if (announcement) announcement.textContent = `${document.title} loaded`;
});
