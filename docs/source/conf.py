# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'py_serial_rs'
copyright = '2023, Jürgen Hahn'
author = 'Jürgen Hahn'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

templates_path = ['_templates']
exclude_patterns = []

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.autosummary',
    'sphinx.ext.napoleon',
"sphinx.ext.viewcode",
    "sphinx_design",
    "sphinx_pyscript",
]

pygments_style = 'sphinx'

autodoc_member_order = "bysource"
autodoc_default_options = {
    "members": True,
}

autoclass_content = "both"

autosummary_generate = True

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

# html_theme = 'alabaster'
html_theme = "sphinx_book_theme"
html_static_path = ["_static"]

# html_static_path = ['_static']

html_title = ""
html_theme_options = {
    "home_page_in_toc": True,
    # "github_url": "https://github.com/executablebooks/MyST-Parser",
    # "repository_url": "https://github.com/executablebooks/MyST-Parser",
    # "repository_branch": "master",
    "path_to_docs": "docs",
    "use_repository_button": False,
    "use_edit_page_button": False,
    "use_issues_button": False,
    # "announcement": "<b>v1.0.0</b> is now out! See the Changelog for details",
}
html_last_updated_fmt = ""
