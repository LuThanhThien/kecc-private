# Setting up

## Pre-commit

For development, install `uv` by

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

Create environment by

```bash
uv sync
```

Run install pre-commit by:

```bash
uv run pre-commit install
```

Now, pre-commit is wired with git under `.git/hooks/pre-commit`
