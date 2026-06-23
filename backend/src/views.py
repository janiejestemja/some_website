from flask import Blueprint, render_template
from flask_login import current_user

views = Blueprint("views", __name__)


@views.route("/", defaults={"path": ""}, methods=["GET"])
@views.route("/<path:path>")
def home(path):
    return render_template("index.html", user=current_user)
