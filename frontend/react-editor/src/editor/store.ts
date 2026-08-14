import { create } from 'zustand'

export type SlideElement = {
  id: string
  type: 'text' | 'image' | 'chart'
  x: number
  y: number
  width: number
  height: number
  content: string
}

export type Slide = {
  id: string
  title: string
  elements: SlideElement[]
}

type EditorState = {
  slides: Slide[]
  currentSlide: string | null
  selectedElement: string | null
  setSlides: (slides: Slide[]) => void
  selectElement: (id: string | null) => void
}

export const useEditorStore = create<EditorState>((set) => ({
  slides: [],
  currentSlide: null,
  selectedElement: null,
  setSlides: (slides) => set({ slides }),
  selectElement: (id) => set({ selectedElement: id }),
}))
