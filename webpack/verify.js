import './common';
import(/* webpackChunkName: "bundle" */ "../pkg").then(module => module.run_app());

window.addEventListener('DOMContentLoaded', () => {
  const descriptionToggle = document.getElementById('toggle-descriptions');
  const rootContainer = document.getElementById('app-root');

  descriptionToggle.addEventListener('change', () => {
    const isHidden = !descriptionToggle.checked;
    rootContainer.classList.toggle('toggled-description-hide', isHidden);
  });
});
