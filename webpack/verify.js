import ClipboardJS from 'clipboard';
import './common';

import(/* webpackChunkName: "bundle" */ '../pkg').then((module) => {
  const app = module.runApp();

  const randomizeButton = document.getElementById('randomize-token');
  randomizeButton.addEventListener('click', () => {
    app.randomizeToken();
  });
});

const TOGGLE_DESCRIPTIONS_NAME = 'jwt__toggleDescriptions';

window.addEventListener('DOMContentLoaded', () => {
  const descriptionToggle = document.getElementById('toggle-descriptions');
  const rootContainer = document.getElementById('app-root');

  // eslint-disable-next-line no-new
  new ClipboardJS('.btn.btn-copy');

  const onDescriptionToggleChange = () => {
    const showDescriptions = descriptionToggle.checked;
    rootContainer.classList.toggle('toggled-description-hide', !showDescriptions);
    localStorage.setItem(TOGGLE_DESCRIPTIONS_NAME, showDescriptions.toString());
  };
  descriptionToggle.addEventListener('change', onDescriptionToggleChange);

  const showDescriptions = localStorage.getItem(TOGGLE_DESCRIPTIONS_NAME);
  descriptionToggle.checked = (showDescriptions === null) || (showDescriptions === 'true');
  onDescriptionToggleChange();
});
