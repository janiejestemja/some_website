from flask import jsonify, Blueprint, request, current_app
from flask_login import current_user, login_required, login_user, logout_user
from werkzeug.security import check_password_hash, generate_password_hash
import os
import shutil

from . import db
from .db_models import User

auth = Blueprint("auth", __name__)


@auth.route("/login", methods=["POST"])
def login():
    req_data = request.get_json()
    email = req_data.get("email")
    passwordA = req_data.get("password")

    # Query Database
    user = User.query.filter_by(email=email).first()
    if user:
        if check_password_hash(user.password, passwordA):

            if current_user.is_authenticated:
                print("user already logged in.")
                return jsonify({ "success": False }), 200

            # Create a session
            login_user(user, remember=True)

            # Flashing login message
            return jsonify({ "success": True }), 200

        else:
            print("incorrect credentials")
            return jsonify({ "success": False }), 200
    else:
        print("account not found")
        return jsonify({ "success": False }), 200



@auth.route("/signup", methods=["POST"])
def signup():
    req_data = request.get_json()
    user_name = req_data.get("user_name")
    email = req_data.get("email")
    passwordA = req_data.get("passwordA")
    passwordB = req_data.get("passwordB")
    user = User.query.filter_by(email=email).first()

    if user:
        print("Email is already signed up.")

    # Placeholder validation
    elif len(email) <= 0:
        print("Email must be greater than 0 characters.")
    elif passwordA != passwordB:
        print("Passwords do not match.")
    elif len(passwordA) < 8:
        print("Password must be at least 8 characters.")

    # Create new user
    else:
        new_user = User(
            email=email,
            user_name=user_name,
            password=generate_password_hash(passwordA)
        )

        # Add User to Database
        db.session.add(new_user)
        db.session.commit()

        # Create the user-specific directories
        user_dir = os.path.join(
            current_app.root_path,
            "filesystem",
            str(new_user.id)
        )
        img_dir = os.path.join(user_dir, "img")
        text_dir = os.path.join(user_dir, "text")

        # Create the directories if they don't exist
        os.makedirs(user_dir, exist_ok=True)
        os.makedirs(img_dir, exist_ok=True)
        os.makedirs(text_dir, exist_ok=True)

        # Create a session right after signing up
        login_user(new_user, remember=True)

        return jsonify({ "success": True }), 200
    return jsonify({ "success": False }), 200

@auth.route("/logout", methods=["POST"])
@login_required
def logout():
    # Ending users session
    if current_user.is_authenticated:
        logout_user()
        return jsonify({ "success": True }), 200
    else:
        return jsonify({ "success": False }), 200
