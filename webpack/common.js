/**
 * Common styles / scripts included into every page.
 */

import './main.scss';
import './icons/bootstrap-icons.scss';

import 'bootstrap/js/dist/collapse';
import 'bootstrap/js/dist/tab';
import 'bootstrap/js/dist/offcanvas';
import AnchorJS from 'anchor-js';

window.addEventListener('DOMContentLoaded', () => {
  const anchors = new AnchorJS();
  anchors.options = {
    visible: 'touch',
    icon: '§',
    titleText: 'Link to this section',
  };
  anchors.add('main h2, main h3');
});
