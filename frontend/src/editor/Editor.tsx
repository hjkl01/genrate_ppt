import Canvas from './Canvas';
import PropertyPanel from './PropertyPanel';
import Toolbar from './Toolbar';

export default function Editor() {
  return (
    <div className="editor">
      <Toolbar />
      <div className="editor-body">
        <aside className="slide-sidebar">Slides</aside>
        <main className="canvas-area"><Canvas /></main>
        <aside className="property-area"><PropertyPanel /></aside>
      </div>
    </div>
  );
}
