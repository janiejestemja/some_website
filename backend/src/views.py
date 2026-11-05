from flask import Blueprint, render_template, request, jsonify
from flask_login import current_user

views = Blueprint("views", __name__)


@views.route("/", defaults={"path": ""}, methods=["GET"])
@views.route("/<path:path>")
def home(path):
    return render_template("index.html", user=current_user)


@views.route("/test", methods=["GET", "POST"])
def test():
    if request.method == "POST":
        # body = request.form.get("body")
        data = request.get_json()
        body = data.get("body")
        print("Body: ", body)
    return jsonify({ "success": True }), 200
