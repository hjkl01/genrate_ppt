import { useState } from 'react'
import './styles.css'

type Component =
  | { type: 'text'; text: string; role?: string }
  | { type: 'card'; title: string; body?: string[] }
  | { type: 'image'; prompt: string; alt?: string }
  | { type: 'node'; id: string; label: string }
  | { type: 'connector'; from: string; to: string; arrow?: boolean }
  | { type: 'timeline_item'; label: string; description: string }
  | { type: 'metric'; label: string; value: string; detail?: string }

type Slide = { id: string; kind: string; title: string; subtitle?: string; components: Component[] }
type GenerateResult = { stage: string; topic: string; spec: { title: string; theme: string; slides: Slide[] }; feedback: string[] }

async function generatePpt(payload: { topic: string; audience?: string; style?: string; slide_count: number }): Promise<GenerateResult> {
  const response = await fetch('/api/generate', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) })
  const data = await response.json()
  if (!response.ok || data.error) throw new Error(data.error ?? `Generation failed (${response.status})`)
  return data as GenerateResult
}

function App() {
  const [topic, setTopic] = useState('Rust 微服务架构')
  const [audience, setAudience] = useState('技术团队')
  const [style, setStyle] = useState('现代、简洁、技术感')
  const [slideCount, setSlideCount] = useState(8)
  const [result, setResult] = useState<GenerateResult | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [selectedSlide, setSelectedSlide] = useState(0)

  const submit = async () => {
    if (!topic.trim()) return setError('请输入 PPT 主题')
    setLoading(true); setError('')
    try {
      setResult(await generatePpt({ topic: topic.trim(), audience: audience.trim() || undefined, style: style.trim() || undefined, slide_count: slideCount }))
      setSelectedSlide(0)
    } catch (err) { setError(err instanceof Error ? err.message : '生成失败') }
    finally { setLoading(false) }
  }

  const currentSlide = result?.spec.slides[selectedSlide]
  return <div className="app-shell">
    <header className="topbar"><div className="brand"><div className="brand-mark">G</div><div><strong>Genrate PPT</strong><span>Rust AI Presentation Engine</span></div></div><div className="status">Semantic DSL · LLM Planner</div></header>
    <div className="workspace">
      <aside className="control-panel">
        <div className="panel-title"><h2>生成 PPT</h2><p>输入主题，让 AI 自动规划演示文稿。</p></div>
        <label><span>主题</span><textarea value={topic} onChange={e => setTopic(e.target.value)} rows={4} placeholder="例如：Rust 微服务架构" /></label>
        <label><span>受众</span><input value={audience} onChange={e => setAudience(e.target.value)} placeholder="例如：技术团队" /></label>
        <label><span>风格</span><input value={style} onChange={e => setStyle(e.target.value)} placeholder="例如：现代、简洁" /></label>
        <label><span>页数：{slideCount}</span><input type="range" min={1} max={20} value={slideCount} onChange={e => setSlideCount(Number(e.target.value))} /></label>
        <button className="generate-button" onClick={submit} disabled={loading}>{loading ? 'AI 正在生成…' : '生成 PPT'}</button>
        {error && <div className="error">{error}</div>}
        <div className="pipeline"><div className="pipeline-title">生成流程</div><span>Prompt</span><b>→</b><span>LLM</span><b>→</b><span>DSL</span><div className="pipeline-line" /><span>Layout</span><b>→</b><span>PPTX</span><b>→</b><span>QA</span></div>
      </aside>
      <aside className="slide-list"><div className="list-header">幻灯片 {result ? `(${result.spec.slides.length})` : ''}</div>{result?.spec.slides.map((slide, index) => <button key={slide.id} className={`slide-thumb ${selectedSlide === index ? 'active' : ''}`} onClick={() => setSelectedSlide(index)}><small>{String(index + 1).padStart(2, '0')}</small><strong>{slide.title}</strong><span>{slide.kind}</span></button>)}{!result && <div className="empty-list">生成后将在这里显示大纲</div>}</aside>
      <main className="canvas-area">{currentSlide ? <div className="slide-stage"><div className="slide-card"><div className="slide-number">{String(selectedSlide + 1).padStart(2, '0')}</div><div className="slide-content"><div className="slide-kind">{currentSlide.kind}</div><h1>{currentSlide.title}</h1>{currentSlide.subtitle && <p className="subtitle">{currentSlide.subtitle}</p>}<div className="component-grid">{currentSlide.components.map((component, index) => <div className="component" key={`${currentSlide.id}-${index}`}>
        {'text' in component && <p>{component.text}</p>}
        {'title' in component && 'body' in component && <><strong>{component.title}</strong>{component.body?.map(item => <p key={item}>{item}</p>)}</>}
        {'label' in component && 'value' in component && <><strong>{component.value}</strong><p>{component.label}</p></>}
        {'label' in component && !('value' in component) && <><strong>{component.label}</strong><p>{'description' in component ? component.description : component.label}</p></>}
        {'prompt' in component && <><strong>图片</strong><p>{component.prompt}</p></>}
        {'from' in component && <p>{component.from} → {component.to}</p>}
      </div>)}</div></div></div></div> : <div className="welcome"><div className="welcome-icon">✦</div><h1>AI PPT Generator</h1><p>填写左侧主题，点击「生成 PPT」，开始创建你的演示文稿。</p><button className="welcome-button" onClick={submit} disabled={loading}>{loading ? '正在生成…' : '开始生成'}</button></div>}</main>
      <aside className="ai-panel"><div className="list-header">AI Assistant</div>{result ? <><div className="ai-card"><span className="label">生成状态</span><strong>{result.stage}</strong><p>已生成 {result.spec.slides.length} 页 Semantic Slide DSL。</p></div><div className="ai-card"><span className="label">演示文稿</span><strong>{result.spec.title}</strong><p>主题：{result.topic}</p></div><div className="feedback"><span className="label">Pipeline</span>{result.feedback.map(item => <p key={item}>✓ {item}</p>)}</div><div className="notice">当前版本已经打通 AI → DSL → 前端预览。PPTX Renderer / 下载将在下一阶段接入。</div></> : <div className="ai-empty"><p>还没有生成内容。</p><p>AI 会根据主题、受众和风格自动生成 PPT 大纲，并在这里实时预览。</p></div>}</aside>
    </div>
  </div>
}

export default App
