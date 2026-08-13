# Editor History and API Design

## History

The editor keeps immutable Slide JSON snapshots.

Flow:

User Change

```
Canvas Change
      |
      v
History Stack
      |
 +----+----+
 | undo redo |
 +-----------+
```

## API

Frontend communicates with Rust backend:

```
GET  /api/slides/{id}
POST /api/slides/save
POST /api/render
```

The same Slide JSON model is shared by:

- LLM generation
- React editor
- Rust renderer
