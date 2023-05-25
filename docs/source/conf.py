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
    'sphinx_rtd_theme',
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
html_theme = "sphinx_rtd_theme"
html_static_path = ["_static"]

# html_static_path = ['_static']

