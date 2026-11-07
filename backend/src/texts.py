import os

from flask import jsonify, Blueprint, request, current_app, send_from_directory
from flask_login import login_required, current_user
from werkzeug.utils import secure_filename

texts = Blueprint("texts", __name__)

ALLOWED_EXTENSIONS = {"txt", "odt", "pdf", "py", "rs", "cpp"}

def allowed_file(filename):
    return "." in filename and filename.rsplit(".", 1)[1].lower() in ALLOWED_EXTENSIONS

@texts.route("/list_textfiles", methods=["GET"])
@login_required
def list_textfiles():
    # Directory for the current user
    user_dir = os.path.join(current_app.root_path, "filesystem", str(current_user.id), "text")
    os.makedirs(user_dir, exist_ok=True)  # Ensure the directory exists

    # List of text files
    files = os.listdir(user_dir)
    return jsonify({ "files": files }), 200

@texts.route("/download_textfile/<filename>", methods=["GET"])
@login_required
def download_textfile(filename):
    user_dir = os.path.join(current_app.root_path, "filesystem", str(current_user.id), "text")
    file_path = os.path.join(user_dir, filename)
    if not os.path.exists(file_path):
        return jsonify({ "success": False }), 200
    return send_from_directory(user_dir, filename, as_attachment=True)

@texts.route("/upload_textfile", methods=["POST"])
@login_required
def upload_textfile():
    user_dir = os.path.join(current_app.root_path, "filesystem", str(current_user.id), "text")
    os.makedirs(user_dir, exist_ok=True)  # Ensure the directory exists
    if "file" not in request.files:
        # flash("No file part", category="error")
        return jsonify({ "success": False }), 200

    file = request.files["file"]
    if file.filename == "":
        # flash("No selected file", category="error")
        return jsonify({ "success": False }), 200

    if file and allowed_file(file.filename):
        filename = secure_filename(file.filename)
        # Calculate current folder size
        folder_size = sum(os.path.getsize(os.path.join(user_dir, f)) for f in os.listdir(user_dir) if os.path.isfile(os.path.join(user_dir, f)))

        # Maximum allowed storage in bytes (5MB)
        max_storage = 5 * 1024 * 1024

        # Get size of uploaded file
        file.seek(0, os.SEEK_END)  # Move cursor to the end of the file
        file_size = file.tell()    # Get current cursor position
        file.seek(0)               # Reset cursor to the beginning of the file

        # print(file_size)

        if folder_size + file_size > max_storage:
            # flash("Storage limit exceeded. Please delete some files.", category="error")
            return jsonify({ "success": False }), 200

        # Save the file
        file.save(os.path.join(user_dir, filename))
        # flash("Textfile uploaded successfully.", category="success")
        return jsonify({ "success": True }), 200

    # flash("Invalid file type.", category="error")
    return jsonify({ "success": False }), 200

@texts.route("/delete_textfile", methods=["POST"])
@login_required
def delete_textfile():
    data = request.get_json()
    filename = data.get("filename")

    if not filename:
        return jsonify({"success": False, "error": "Filename not provided."}), 400

    user_dir = os.path.join(current_app.root_path, "filesystem", str(current_user.id), "text")
    file_path = os.path.join(user_dir, filename)

    # Ensure the file exists and belongs to the user
    if os.path.exists(file_path):
        try:
            os.remove(file_path)
            flash("Textfile deleted.", category="success")
            return jsonify({"success": True}), 200
        except Exception as e:
            return jsonify({"success": False, "error": str(e)}), 500
    else:
        return jsonify({"success": False, "error": "File not found."}), 404
