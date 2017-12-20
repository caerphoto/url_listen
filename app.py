from flask import Flask, request
app = Flask(__name__)


@app.route("/", methods=["POST"])
def open_url():
    url = request.form.get("url")
    if url is None:
        return ("No 'url' parameter given.", 400, [])

    return ("Opened URL %s" % (url,), 200, [])


if __name__ == "__main__":
    app.run()
