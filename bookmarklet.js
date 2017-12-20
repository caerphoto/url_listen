(function () {
  'use strict';
  var xhr = new XMLHttpRequest();
  var currentUrl = encodeURIComponent(window.location);

  xhr.open('POST', 'http://192.168.1.64:5000/', true);
  xhr.setRequestHeader('Content-type', 'application/x-www-form-urlencoded');

  xhr.send('url=' + currentUrl);
}());
