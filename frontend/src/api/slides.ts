export interface SlideDocument {
  id: string;
  slides: unknown[];
}

const API_BASE = import.meta.env.VITE_API_BASE || '';

export async function loadSlides(id: string): Promise<SlideDocument> {
  const response = await fetch(`${API_BASE}/api/slides/${id}`);
  if (!response.ok) throw new Error('failed to load slides');
  return response.json();
}

export async function saveSlides(document: SlideDocument) {
  const response = await fetch(`${API_BASE}/api/slides/save`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(document),
  });

  if (!response.ok) throw new Error('failed to save slides');
  return response.json();
}
