import ClipboardJS from 'clipboard';
import './common';

const DESCRIPTIONS_STORAGE_KEY = 'jwt__toggleDescriptions';
const SAVE_DATA_STORAGE_KEY = 'jwt__saveData';

let app = null;

import(/* webpackChunkName: "bundle" */ '../pkg').then((wasm) => {
  const saveDataToggle = document.getElementById('toggle-saving-data');
  app = wasm.runApp(saveDataToggle.checked);

  const randomizeButton = document.getElementById('randomize-token');
  randomizeButton.addEventListener('click', () => {
    app.randomizeToken();
  });
});

window.addEventListener('DOMContentLoaded', () => {
  // eslint-disable-next-line no-new
  new ClipboardJS('.btn.btn-copy');

  // Setting: description toggle.
  const descriptionToggle = document.getElementById('toggle-descriptions');
  const rootContainer = document.getElementById('app-root');
  const onDescriptionToggleChange = () => {
    const showDescriptions = descriptionToggle.checked;
    rootContainer.classList.toggle('toggled-description-hide', !showDescriptions);
    localStorage.setItem(DESCRIPTIONS_STORAGE_KEY, showDescriptions.toString());
  };
  descriptionToggle.addEventListener('change', onDescriptionToggleChange);

  const showDescriptions = localStorage.getItem(DESCRIPTIONS_STORAGE_KEY);
  descriptionToggle.checked = (showDescriptions === null) || (showDescriptions === 'true');
  onDescriptionToggleChange();

  // Setting: saving key / token.
  const saveDataToggle = document.getElementById('toggle-saving-data');
  const onSaveDataToggleChange = () => {
    const saveData = saveDataToggle.checked;
    if (app !== null) {
      app.setSaveFlag(saveData);
    }
    localStorage.setItem(SAVE_DATA_STORAGE_KEY, saveData.toString());
  };
  saveDataToggle.addEventListener('change', onSaveDataToggleChange);

  const saveData = localStorage.getItem(SAVE_DATA_STORAGE_KEY);
  saveDataToggle.checked = (saveData === 'true');
  onSaveDataToggleChange();
});
