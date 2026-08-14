import { useState } from 'react'
import { chatWithAI } from '../api/ai'

export default function SlideAIBar() {
  const [input, setInput] = useState('')
  const [result, setResult] = useState('')

  async function run() {
    const data = await chatWithAI(input)
    setResult(data.answer)
  }

  return (
    <section>
      <input
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder="例如：优化当前页面布局"
      />
      <button onClick={run}>AI</button>
      <div>{result}</div>
    </section>
  )
}
