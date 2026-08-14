# MVP Editor Plan

## Layout

Toolbar

+ Sidebar
+ Canvas
+ Property Panel

## Data Flow

LLM -> Slide JSON -> React Editor -> Rust Renderer -> PPTX

## API

- GET /api/slides
- POST /api/slides/save
- POST /api/render
- POST /api/vision/check
- POST /api/repair
