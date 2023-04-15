from app import db, login
from flask_login import UserMixin

from werkzeug.security import generate_password_hash, check_password_hash

class User(UserMixin, db.Model):
    id = db.Column(db.Integer, primary_key=True)
    username = db.Column(db.String(50), index=True, unique=True)
    email = db.Column(db.String(100), index=True, unique=True)
    firstname = db.Column(db.String(50))
    lastname = db.Column(db.String(50))
    password_hash = db.Column(db.String(128))

    def __repr__(self):
        return f'<User {self.firstname} {self.lastname}, {self.username}>'

    def set_password(self, password):
        self.password_hash = generate_password_hash(password)

    def check_password(self, password):
        return check_password_hash(self.password_hash, password)

@login.user_loader
def load_user(id):
    return User.query.get(int(id))

class Project(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    project_name = db.Column(db.String(70), index=True, unique=True)
    project_desc = db.Column(db.Text())
    project_link = db.Column(db.String(100))
