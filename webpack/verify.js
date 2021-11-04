import ClipboardJS from 'clipboard';
import './common';

import(/* webpackChunkName: "bundle" */ '../pkg').then((module) => {
  const app = module.runApp();

  const randomizeButton = document.getElementById('randomize-token');
  randomizeButton.addEventListener('click', () => {
    app.randomizeToken();
  });
});

window.addEventListener('DOMContentLoaded', () => {
  const descriptionToggle = document.getElementById('toggle-descriptions');
  const rootContainer = document.getElementById('app-root');

  // eslint-disable-next-line no-new
  new ClipboardJS('.btn.btn-copy');

  descriptionToggle.addEventListener('change', () => {
    const isHidden = !descriptionToggle.checked;
    rootContainer.classList.toggle('toggled-description-hide', isHidden);
  });
});
