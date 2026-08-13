export interface SlideElement {
  id: string;
  type: 'text' | 'image' | 'chart';
  x: number;
  y: number;
  width: number;
  height: number;
  content?: string;
}

export interface Slide {
  id: string;
  title: string;
  elements: SlideElement[];
}
