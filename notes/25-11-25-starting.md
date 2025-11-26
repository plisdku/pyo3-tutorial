# Learning PyO3

November 25, 2025

Going to run through the basics from pyo3.rs. Using maturin because they suggest it. I'll be interested to see what the directory structure ends up looking like.

I also want to try to use `uv` correctly, instead of just using `uv pip` which is I guess a compatibility mode.

# Basic `uv` commands

 - `uv init <project>` to make a new project. I guess it probably makes a git repo.
 - `uv add <package>` will initialize the virtual environment AND add things to the lockfile?
 - `uv run <command>` 
 - `uv lock`  

## Declarative vs. imperative

Dr. GPT says that `uv add` and `uv lock` are declarative because you say _what you want_ but you don't say _how_ to do it. But `uv pip` is imperative because you tell it _how_ to install things. Well ok.

# Today's steps

I had the repo set up already, but I ran `uv init .` and got this directory structure:

```
% tree 
.
├── README.md
├── main.py
├── notes
│   └── 25-11-25-starting.md
└── pyproject.tomlin
```

I ran the hello-world with `uv run main.py` which initialized a new virtual environment at `.venv` in the repo. I had to add `.venv` to my .gitignore manually since I'd already created a pretty empty one.

It is suggested to use `uv run <command>` because it will always use the right environment (I guess that is, the one present)... I see that there is a `.python-version` file now in the root of my repo. It has a single line in it, reading `3.14`, and I changed it to `3.12` and ran `uv run python --version`. After a tweak to the `pyproject.toml` to allow older pythons, this actually rebuilt my virtual environment for me.

Anyway I'm up to here now:

```
% tree -a -L 1     
.
├── .git
├── .gitignore
├── .python-version
├── .venv
├── README.md
├── main.py
├── notes
├── pyproject.toml
└── uv.lock
```

# Github CLI

Ok I'll try it! `brew install gh`. I needed to start with `gh auth login`; I chose SSH and it found my SSH key and I authenticated in the browser and it was done. I had to then go add `gh auth refresh -s project,read:project` to get some more powers.

Next project: somehow push this new repo to my github.

Discovered: `gh project create` made a "Project" which is more than I needed. Deleted it. Instead, ran

```
gh repo create
```

and used `.` as the repo to push. Then `gh repo view -w` pops it open in the browser, huzzah.
