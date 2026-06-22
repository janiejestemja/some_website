from os import getenv
from flask import Flask


def create_app():
    app = Flask(__name__)
    app.config["SECRET_KEY"] = getenv(
        "FLASK_SECRET_KEY",
        "fallback_secret_key"
    )
    app.config.update(
        SESSION_COOKIE_SECURE=True,
        SESSION_COOKIE_HTTPONLY=True,
        SESSION_COOKIE_SAMESITE="Lax",
    )

    from .views import views
    app.register_blueprint(views, url_prefix="/")

    return app
