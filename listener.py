from flask import Flask, request
import logging
import re
from subprocess import call

app = Flask(__name__)

# Change this to the IP of the sending machine.
LISTEN_TO = '192.168.1.95'


@app.after_request
def add_header(response):
    response.headers["Access-Control-Allow-Origin"] = "*"
    return response


@app.after_request
def log_request(_):
    logging.info(
        "%s %s",
        "Opening URL",
        request.form.get("url")
    )


@app.route("/", methods=["POST"])
def open_url():
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

    return ("Opened URL %s" % (url,), 200, [])


if __name__ == "__main__":
    context = ("cert.pem", "key.pem")
    app.run(host="0.0.0.0", port=5000, ssl_context=context)
