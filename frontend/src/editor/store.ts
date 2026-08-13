import { create } from 'zustand';
import type { Slide } from './model';

type EditorState = {
  slides: Slide[];
  currentSlide: number;
  setSlides: (slides: Slide[]) => void;
  selectSlide: (index: number) => void;
};

export const useEditorStore = create<EditorState>((set) => ({
  slides: [],
  currentSlide: 0,
  setSlides: (slides) => set({ slides }),
  selectSlide: (index) => set({ currentSlide: index }),
}));
