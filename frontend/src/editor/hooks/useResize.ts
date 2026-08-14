import { useState } from 'react';

export function useResize(width = 100, height = 100) {
  const [size, setSize] = useState({ width, height });

  function resize(dw: number, dh: number) {
    setSize((prev) => ({
      width: Math.max(10, prev.width + dw),
      height: Math.max(10, prev.height + dh),
    }));
  }

  return {
    size,
    resize,
    setSize,
  };
}
