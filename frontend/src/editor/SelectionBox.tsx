import React from 'react';

export function SelectionBox({x, y, width, height}: {x:number;y:number;width:number;height:number}) {
  return (
    <div style={{
      position: 'absolute',
      left: x,
      top: y,
      width,
      height,
      border: '1px dashed #666',
      pointerEvents: 'none'
    }}/>
  );
}
