/**
 * Common styles / scripts included into every page.
 */

// TODO: extract only necessary icons
import 'bootstrap-icons/font/bootstrap-icons.css';
import './main.css';

import 'bootstrap/js/dist/tab';
import AnchorJS from 'anchor-js';

document.addEventListener('DOMContentLoaded', () => {
  const anchors = new AnchorJS();
  anchors.options = {
    visible: 'touch',
    icon: '§',
    titleText: 'Link to this section'
  };
  anchors.add('main h2, main h3');
});
