import React, { useState } from 'react';

export default function AIAssistantPanel() {
  const [prompt, setPrompt] = useState('');
  const [answer, setAnswer] = useState('');

  async function send() {
    const res = await fetch('/api/ai/chat', {
      method: 'POST',
      headers: {'content-type': 'application/json'},
      body: JSON.stringify({prompt})
    });
    const data = await res.json();
    setAnswer(data.answer || '');
  }

  return (
    <aside className="ai-panel">
      <h3>AI PPT Assistant</h3>
      <textarea value={prompt} onChange={e => setPrompt(e.target.value)} />
      <button onClick={send}>Generate</button>
      <pre>{answer}</pre>
    </aside>
  );
}
