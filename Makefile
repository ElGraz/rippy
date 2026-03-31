VENV = .venv
PYTHON = $(VENV)/bin/python
PIP = $(VENV)/bin/pip
PYTEST = $(VENV)/bin/pytest

.PHONY: all setup test clean

all: test

# 1. Create venv and install only external dependencies
setup: $(VENV)/bin/activate

$(VENV)/bin/activate: pyproject.toml
	@echo "Creating virtual environment and installing external deps..."
	python3 -m venv $(VENV)
	$(PIP) install --upgrade pip
	$(PIP) install pytest setuptools
	$(PIP) install discid musicbrainzngs
	@touch $(VENV)/bin/activate

# 2. Run tests by manually injecting the library path
# We set PYTHONPATH to the current directory so 'import lib' works.
test: setup
	$(PIP) install pytest
	@echo "Running tests with PYTHONPATH injection..."
	PYTHONPATH=. $(PYTEST) tests/

# 3. Clean up
clean:
	rm -rf $(VENV)
	find . -type d -name "__pycache__" -exec rm -rf {} +
	find . -type d -name ".pytest_cache" -exec rm -rf {} +
	find . -type d -name "*.egg-info" -exec rm -rf {} +
