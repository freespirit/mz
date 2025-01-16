# mz

Support for Mojo in Zed

# Highlights

Hightlights based on the default for Python, with additional Mojo-specific keywords etc.

# LSP

Very basic support for mojo-lsp-server.

# Zed settings

Zed needs to be configured what LSP to use for Mojo:

```json
"languages": {
        "Mojo": {
            "language_servers": ["mojo-lsp"]
        }
    },
```

And the configuration for the Mojo LSP:

```json
"lsp": {
        "mojo-lsp": {
            "binary": {
                "path": "magic",
                "arguments": ["run", "mojo-lsp-server"]
            }
        }
    },
```
