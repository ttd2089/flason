set shell := ["pwsh", "-c"]

test:
    {{justfile_directory()}}/tests/cli-output/run-tests.ps1
