import os
from flask import Flask, render_template,render_template_string, request
import jinja2
from waitress import serve
import logging
import sys
from os import remove, path
from sys import argv

app = Flask(__name__)


# Blacklist of disallowed characters and substrings
BLACKLIST = ["{%", "%}", "import", "open", "sys"]

# Flag stored in environment variable

def sanitize_input(content):
    for item in BLACKLIST:
        if item in content:
            raise ValueError("Blacklisted characters detected")
    return content

notes = []

@app.route('/', methods=['GET', 'POST'])
def index():
    if request.method == 'POST':
        content = request.form.get('content')
        try:
            sanitized_content = sanitize_input(content)
            return render_template_string(sanitized_content)
        except ValueError as e:
            return str(e), 500
    return render_template('index.html', notes=notes)

