# React Editor Architecture

```text
Toolbar
  |
Canvas <---- Zustand Store ----> Property Panel
  |
Slide JSON Schema
  |
Rust PPT Engine
```

The editor uses the same slide model as the renderer so AI generated slides can be edited directly.

## Data Flow

```text
LLM
 |
Slide JSON
 |
React Editor
 |
User Changes
 |
Rust Renderer
 |
PPTX
```
