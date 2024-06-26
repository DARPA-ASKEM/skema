
# Configuring your dev environment

We recommend configuring your local development environment using [`conda`](https://docs.conda.io/en/latest/miniconda.html):

```bash
conda create -n skema python=3.8 -c conda-forge rust=1.70.0 openjdk=11 sbt=1.9.0 nodejs=18.15.0
conda activate skema
# Install tree-sitter parsers
python skema/program_analysis/tree_sitter_parsers/build_parsers.py --all
# download the checkpoint for the img2mml service
python scripts/retrieve_model.py
# mathjax deps for img2mml
(cd skema/img2mml/data_generation && npm install)
```

## Installing the Python library in development mode

```bash
pip install -e ".[core]"
```

The command above installs the minimum set packages required for the Code2FN pipeline. 

To additionally install dev dependencies:

```bash
pip install -e ".[core,dev]"
```

To install **all** components (including dev dependencies for documentation generation):
```bash
pip install ".[all]"
```
