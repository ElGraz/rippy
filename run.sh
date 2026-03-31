#!/bin/bash

VENV_DIR=".venv"


if [ ! -d "$VENV_DIR" ]; then
    echo "Creating virtual environment..."
    python3 -m venv $VENV_DIR
    source ./$VENV_DIR/bin/activate
    echo "Installing dependencies..."
    ./$VENV_DIR/bin/pip install --upgrade pip
    ./$VENV_DIR/bin/pip install discid musicbrainzngs
fi

source ./$VENV_DIR/bin/activate

./$VENV_DIR/bin/python -m rippy.cli "$@"
