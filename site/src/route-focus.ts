export function focusAndAnnounceRoute() {
  const heading = document.querySelector<HTMLElement>('h1');
  heading?.focus();
  const announcement = document.getElementById('route-announcement');
  if (announcement) announcement.textContent = `${document.title} loaded`;
}

export function keepSkipLinkFirstAfterRouteFocus() {
  const heading = document.querySelector<HTMLElement>('h1');
  const skipLink = document.querySelector<HTMLAnchorElement>('.skip-link');
  if (!heading || !skipLink) return;

  document.addEventListener('keydown', (event) => {
    if (event.key !== 'Tab' || event.shiftKey || document.activeElement !== heading) return;
    event.preventDefault();
    skipLink.focus();
  });
}
