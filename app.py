from flask import Flask, request
import re
from subprocess import call

app = Flask(__name__)


@app.route("/", methods=["POST"])
def open_url():
    url = request.form.get("url")
    if url is None:
        return ("No 'url' parameter given.", 400, [])

    match = re.search("^https?://.*", url)

    if match is None:
        return ("Invalid URL given.", 400, [])

    return_code = call(["open", url])

    if return_code != 0:
        return ("Failed to open URL on remote machine.", 500, [])

    return ("Opened URL %s" % (url,), 200, [])


if __name__ == "__main__":
    app.run()
