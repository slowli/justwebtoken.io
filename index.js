/* eslint-env jquery */

import(/* webpackChunkName: "bundle" */ "./pkg").then(module => module.run_app());

$(() => {
  const descriptionToggle = $('#toggle-descriptions');
  descriptionToggle.change(() => {
    const isHidden = !descriptionToggle.prop('checked');
    $('#app-root').toggleClass('toggled-description-hide', isHidden);
  });
});
