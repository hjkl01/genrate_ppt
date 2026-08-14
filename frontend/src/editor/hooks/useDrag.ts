import { useState } from 'react';

export function useDrag(initialX = 0, initialY = 0) {
  const [position, setPosition] = useState({ x: initialX, y: initialY });

  function move(dx: number, dy: number) {
    setPosition((prev) => ({
      x: prev.x + dx,
      y: prev.y + dy,
    }));
  }

  return {
    position,
    move,
    setPosition,
  };
}
