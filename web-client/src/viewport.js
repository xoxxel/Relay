// Keep keyboard-aware dialog sizing separate from document scrolling.
export function trackViewport() {
  const viewport = window.visualViewport;
  const update = () => {
    document.documentElement.style.setProperty('--viewport-height', `${viewport?.height ?? window.innerHeight}px`);
    document.documentElement.style.setProperty('--viewport-top', `${viewport?.offsetTop ?? 0}px`);
  };
  update();
  viewport?.addEventListener('resize', update);
  viewport?.addEventListener('scroll', update);
  window.addEventListener('resize', update);
  return () => {
    viewport?.removeEventListener('resize', update);
    viewport?.removeEventListener('scroll', update);
    window.removeEventListener('resize', update);
    document.documentElement.style.removeProperty('--viewport-height');
    document.documentElement.style.removeProperty('--viewport-top');
  };
}

export function lockPageScroll() {
  const body = document.body;
  const previous = Object.fromEntries(['position', 'top', 'left', 'right', 'width'].map(key => [key, body.style[key]]));
  const { scrollX, scrollY } = window;
  Object.assign(body.style, { position: 'fixed', top: `-${scrollY}px`, left: '0', right: '0', width: '100%' });
  return () => {
    Object.assign(body.style, previous);
    window.scrollTo(scrollX, scrollY);
  };
}
