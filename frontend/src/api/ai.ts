export async function chatWithAI(prompt: string) {
  const response = await fetch('/api/ai/chat', {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
    },
    body: JSON.stringify({ prompt }),
  })

  if (!response.ok) {
    throw new Error('AI request failed')
  }

  return response.json() as Promise<{ answer: string }>
}
