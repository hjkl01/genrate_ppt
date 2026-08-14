import { useState } from 'react';

export function useEditor() {
  const [selectedId, setSelectedId] = useState<string | null>(null);

  return {
    selectedId,
    select: setSelectedId,
  };
}
