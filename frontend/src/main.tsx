import React from 'react'
import { createRoot } from 'react-dom/client'

function App() {
  return (
    <main style={{ padding: 32, fontFamily: 'system-ui, sans-serif' }}>
      <h1>Genrate PPT</h1>
      <p>AI PPT editor is ready.</p>
    </main>
  )
}

createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
