/**
 * Entry point for the claims dictionary page.
 */

import AnchorJS from 'anchor-js';
import './common';

document.addEventListener('DOMContentLoaded', () => {
  const anchors = new AnchorJS();
  anchors.options = {
    visible: 'always',
    placement: 'left',
    icon: '¶',
    class: 'link-secondary',
    titleText: 'Link to this claim',
  };
  anchors.add('div[id^=claim-]');
});
