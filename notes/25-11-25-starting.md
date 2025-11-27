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
└── pyproject.toml
```

I ran the hello-world with `uv run main.py` which initialized a new virtual environment at `.venv` in the repo. I had to add `.venv` to my .gitignore manually since I'd already created a pretty empty one.

It is suggested to use `uv run <command>` because it will always use the right environment (I guess that is, the one present)... I see that there is a `.python-version` file now in the root of my repo. Yes, `uv` created this, and uses it when making a new virtual environment. It has a single line in it, originall `3.14`, and I changed it to `3.12` and ran `uv run python --version`. After a tweak to the `pyproject.toml` to allow older pythons, this actually rebuilt my virtual environment for me.

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

## Github CLI

Ok I'll try it! `brew install gh`. I needed to start with `gh auth login`; I chose SSH and it found my SSH key and I authenticated in the browser and it was done. I had to then go add `gh auth refresh -s project,read:project` to get some more powers.

Next project: somehow push this new repo to my github.

Discovered: `gh project create` made a "Project" which is more than I needed. Deleted it. Instead, ran

```
gh repo create
```

and used `.` as the repo to push. Then `gh repo view -w` pops it open in the browser, huzzah.

## Try `uv add --dev`

Decided to try something:

```
% uv add --dev ruff maturin pre-commit tbump
```

This added all of these dependencies to the pyproject.toml under `[dependency-groups]` but not as my familiar `[project.optional-dependencies]`, known as "extras" in the python documentation. What was up with those? Among other issues, asking a tool to install "extras" will also install all the main project dependencies. [PEP 735](https://peps.python.org/pep-0735/) goes into some detail about this.

Anyway, `[dependency-groups]` seems to be the new way, and I have `maturin` now.

## Maturin

```
% uv run maturin init
💥 maturin failed
  Caused by: `maturin init` cannot be run on existing projects
```

Ok I already started the project. Let's just try `maturin init` in a subdirectory.

```
% uv run maturin init petunia
✔ 🤷 Which kind of bindings to use?
  📖 Documentation: https://maturin.rs/bindings.html · pyo3
  ✨ Done! Initialized project petunia
```

This actually made a ton of stuff.

```
% tree petunia -La 3
petunia
├── .github
│   └── workflows
│       └── CI.yml
├── .gitignore
├── Cargo.toml
├── pyproject.toml
└── src
    └── lib.rs
```

The `CI.yml` is nice. I'll just try to move the `Cargo.toml` down and copy some `pyproject.toml` to my main file.

Well, I got some boilerplate code. Let's try this:

```
% uv run maturin develop
```

That installed and compiled a bunch of stuff and built my module.

```
% uv run python -c "import petunia; print(petunia.sum_as_string(5, 20))"
25
```

Holy moly it worked.

### [Using Rust from Python](https://pyo3.rs/v0.27.1/rust-from-python.html)

It says I can make modules, functions, and classes with methods. Neato. Their `sum_as_string` example works...

#### `.pyi` file

Since I have `src/lib.rs` I made `src/petunia.pyi` containing

```
from typing import Any

def sum_as_string(a: int, b: int) -> str:
    """
    Add two integers and return their sum as a string.

    Args:
        a: an integer
        b: an integer
    Returns:
        a+b in str format
    """
    ...
```

Note that the docstring is between the `-> str:` and the `...`. I did this and re-ran `maturin develop --bindings pyo3` and then mousing over `sum_as_string` in a Python script showed the docstring. Win!


### [Python object types](https://pyo3.rs/v0.27.1/python-from-rust.html)

The other half of the coin. How do we refer to Python objects?
 - All Python objects are wrapped in smart pointers, `Py<T>`, `Bound<'py, T>`, `Borrowed<'a, 'py, T>`.
 - The generic parameter `T` of the smart pointers can be filled by
   - `PyAny` (resembling `typing.Any`)
   - `PyList`, `PyDict`, `PyTuple`
   - User-defined `#[pyclass]` types



