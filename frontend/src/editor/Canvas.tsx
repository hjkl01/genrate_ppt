import type { Slide } from '../model';

export function Canvas({ slide }: { slide?: Slide }) {
  return (
    <div className="slide-canvas">
      {slide?.elements.map((element) => (
        <div key={element.id}>
          {element.content}
        </div>
      ))}
    </div>
  );
}
