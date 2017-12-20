# URL Listen

Super simple app to listen for POST requests and open the URL specified by the
'url' parameter in the request body on the current machine.

I made this so I can open a page on one machine then, via the bookmarklet, carry
on reading it on another.

## Warning

It's not even slightly secure, so make sure you are careful about what IP you
listen to. Set the `LISTEN_TO` value appropriately in `listener.py`.

## SSL Certificate/key

You will need to specify an SSL certificate and key, or generate one yourself.
See [this
guide](https://blog.miguelgrinberg.com/post/running-your-flask-application-over-https)
for more details about generating an SSL certificate.  This is needed so the
listener can use HTTPS, which is needed to avoide mixed content errors when
using the bookmarklet on HTTPS-secured pages.

## Requirements

The only requirement is Flask. I use the app with Python 2.7 but it might run on
3.x.
