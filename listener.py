from flask import Flask, request
import logging
from logging import StreamHandler
import re
from subprocess import call

app = Flask(__name__)

# Change this to the IP of the sending machine.
LISTEN_TO = '192.168.1.95'


def get_html():
    bookmarklet_file = open("bookmarklet.js")
    html_file = open("bookmarklet.html", "r")

    html = html_file.read().replace("REPLACE_JS", bookmarklet_file.read())

    bookmarklet_file.close()
    html_file.close()

    return html


@app.after_request
def add_header(response):
    response.headers["Access-Control-Allow-Origin"] = "*"
    return response


@app.route("/", methods=["POST", "GET"])
def open_url():
    if request.method == "GET":
        return (get_html(), 405, [])

    url = request.form.get("url")
    if url is None:
        return ("No 'url' parameter given.", 400, [])

    match = re.search("^https?://.*", url)

    if match is None:
        return ("Invalid URL given.", 400, [])

    if request.remote_addr != LISTEN_TO:
        return ("Invalid client IP.", 403, [])

    return_code = call(["open", url])

    if return_code != 0:
        return ("Failed to open URL on remote machine.", 500, [])

    logger.info(
        "%s %s",
        "Opening URL",
        request.form.get("url")
    )

    return ("Opened URL %s" % (url,), 200, [])


if __name__ == "__main__":
    handler = StreamHandler()
    logger = logging.getLogger(__name__)
    logger.setLevel(logging.INFO)
    logger.addHandler(handler)
    context = ("cert.pem", "key.pem")
    app.run(host="0.0.0.0", port=5000, ssl_context=context)
