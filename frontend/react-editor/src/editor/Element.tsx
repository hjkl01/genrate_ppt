import React from 'react';

export type ElementType = 'text' | 'image' | 'chart';

export interface SlideElement {
  id: string;
  type: ElementType;
  x: number;
  y: number;
  width: number;
  height: number;
  content: string;
}

export function Element({ element }: { element: SlideElement }) {
  return (
    <div
      style={{
        position: 'absolute',
        left: element.x,
        top: element.y,
        width: element.width,
        height: element.height,
      }}
    >
      {element.content}
    </div>
  );
}
