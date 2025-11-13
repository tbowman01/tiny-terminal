# Tiny Terminal - Figma View & Design Concept

## Overview

This document outlines the visual design system and UI/UX concepts for tiny-terminal, a hackable terminal toy with Matrix-style effects. The design focuses on creating an immersive, customizable terminal experience with smooth animations and intuitive configuration.

## Design Philosophy

- **Minimalist**: Clean, distraction-free interface
- **Performant**: 60 FPS smooth animations by default
- **Customizable**: Extensive configuration options
- **Nostalgic**: Matrix-inspired aesthetic with modern performance

## Visual Components

### 1. Terminal Canvas

**Frame**: Full terminal window
- **Background**: Pure black (`#000000`)
- **Rendering Area**: 100% of terminal viewport
- **Refresh Rate**: 60 FPS (configurable: 30-144 FPS)

```
┌─────────────────────────────────────────┐
│                                         │
│    [Matrix Rain Effect Canvas]          │
│                                         │
│    ▓░▓░░▒▓░                             │
│    ░▒▓▒▓░▒                              │
│    ▓▒░▓▒▓░                              │
│    ░▓▒░▓▒▓                              │
│                                         │
│                                         │
└─────────────────────────────────────────┘
```

### 2. Character Drops

**Individual Drop Column**:
- **Column Width**: 2 characters minimum (configurable)
- **Drop Length**: Variable (5-20 characters)
- **Head Character**: Bright white/green
- **Body Characters**: Gradient from bright to dim
- **Tail Characters**: Fade to background

**Color Gradient** (Matrix effect):
```
Head:   RGB(255, 255, 255) or RGB(200, 255, 200) - Bright white/pale green
Body:   RGB(0, 255, 0) - Bright green
        ↓ (gradient)
Tail:   RGB(0, 100, 0) - Dim green
        ↓
Gone:   RGB(0, 0, 0) - Black (background)
```

### 3. Character Set Display

**Default Character Pool**:
```
ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉ
0123456789
@#$%&*
```

**Visual Style**:
- Monospace font (terminal default)
- Half-width katakana preferred
- Random selection for authentic Matrix feel

## Layout Specifications

### Terminal Grid

```
┌────────────────────────────────────────────┐
│ Column Distribution (based on density)     │
│                                            │
│ Low Density (0.5):    Sparse drops         │
│ ░  ▓     ░    ▓                            │
│    ░     ▓                                 │
│    ▓     ░                                 │
│                                            │
│ Default Density (1.0): Balanced            │
│ ░▓  ░▓  ▓░  ░▓                             │
│ ▓░  ▓░  ░▓  ▓░                             │
│                                            │
│ High Density (2.0):   Dense matrix         │
│ ░▓▒░▓░▒▓░▒▓░▓▒                             │
│ ▓░▒▓░▒░▓▒░▒▓░▒                             │
└────────────────────────────────────────────┘
```

### Column Spacing

- **Minimum Spacing**: 1 character between columns
- **Dynamic Columns**: Calculated based on terminal width
- **Column Calculation**: `num_columns = term_width / column_width`

## User Interface States

### 1. Launch State

```
┌────────────────────────────────────┐
│                                    │
│  [Clear black screen]              │
│  [0.1s delay]                      │
│  [First drops begin falling]       │
│                                    │
└────────────────────────────────────┘
```

### 2. Running State

```
┌────────────────────────────────────┐
│ ▓░     ░▒▓                         │
│ ░▒     ▓░▒    ░▒                   │
│ ▒▓     ▒▓░    ▓▒                   │
│ ▓░     ░▒▓    ▒▓                   │
│   ░▒     ▓░     ▒░▓                │
│   ▓▒     ░▓     ░▓▒                │
│   ░▓       ▒    ▓░▒                │
│                                    │
│                                    │
│ Controls: q/ESC/Ctrl+C to exit    │
└────────────────────────────────────┘
```

### 3. Exit State

```
┌────────────────────────────────────┐
│                                    │
│  [Immediate clear]                 │
│  [Terminal restore]                │
│  [Cursor restored]                 │
│  [Return to shell]                 │
│                                    │
└────────────────────────────────────┘
```

## Interaction Design

### Input Controls

| Key | Action | Visual Feedback |
|-----|--------|----------------|
| `q` | Quit | Immediate screen clear |
| `Esc` | Quit | Immediate screen clear |
| `Ctrl+C` | Quit | Immediate screen clear |

**Non-blocking Input**: User input doesn't interrupt animation flow

### Configuration Preview

**Before Launch (via CLI)**:
```bash
$ tiny-terminal --density 1.5 --fps 90
# Visual effect: Denser drops, smoother animation
```

## Animation Specifications

### Drop Behavior

```
Frame 0:  ▓         (spawn at top)
Frame 1:  ░▓        (move down, fade previous)
Frame 2:  ░░▓       (continue falling)
Frame 3:   ░░▓      (tail fading)
Frame 4:    ░░▓     (continuous motion)
```

**Physics**:
- **Velocity**: 1 character per frame (consistent)
- **Spawn Rate**: Based on density parameter
  - `0.5 density`: ~0.5 drops per column per second
  - `1.0 density`: ~1.0 drop per column per second
  - `2.0 density`: ~2.0 drops per column per second
- **Lifespan**: Until drop reaches bottom of screen

### Frame Timing

```
Target FPS: 60
Frame Duration: ~16.67ms
Buffer Swap: V-sync preferred
```

## Color Schemes

### 1. Classic Matrix (Default)

```
Background: #000000 (Pure black)
Bright:     #00FF00 (Bright green)
Medium:     #00AA00 (Medium green)
Dim:        #005500 (Dark green)
Head:       #FFFFFF or #C8FFC8 (White/pale green)
```

### 2. Binary Style (Alternative)

```
Background: #000000
Characters: #00FF00
Charset:    "01" only
```

### 3. Custom (User-defined)

- Support for disabling color via `green = false`
- Monochrome mode with grayscale gradient

## Configuration Visualization

### Low Density Example
```toml
fps = 30
density = 0.5
charset = "01"
```
Visual: Sparse, slow-falling binary drops

### High Density Example
```toml
fps = 90
density = 1.75
charset = "ﾐﾅｾﾛｸｹ012345789"
```
Visual: Dense, fast-falling katakana matrix

## Responsive Behavior

### Terminal Resize

**Behavior**:
1. Detect resize event
2. Clear screen
3. Recalculate column layout
4. Resume with new dimensions
5. No state loss

```
Before:  [80 cols × 24 rows]
During:  [Clear and recalculate]
After:   [120 cols × 40 rows]
```

### Performance Adaptation

- **High FPS (90+)**: Smooth, fluid motion
- **Standard FPS (60)**: Balanced performance
- **Low FPS (30)**: Energy-saving mode

## Technical Implementation Notes

### Render Pipeline

```
1. Clear buffer
2. Update drop positions
3. Calculate character brightness
4. Render characters to buffer
5. Swap buffers
6. Sleep until next frame
```

### Optimization Strategies

- **Double Buffering**: Prevent screen tearing
- **Dirty Rectangle**: Update only changed regions
- **Character Pooling**: Reuse character objects
- **GPU Acceleration**: Terminal-dependent

## Accessibility Considerations

### Visual
- High contrast (black background, bright characters)
- Configurable density for reduced motion sensitivity
- Option to disable animation entirely (future)

### Controls
- Multiple exit methods (q, ESC, Ctrl+C)
- Non-modal interface
- No required input during runtime

## Future Enhancements

### Planned Visual Features

1. **Color Themes**
   - Blue matrix
   - Red matrix
   - Rainbow mode
   - Custom RGB values

2. **Interactive Elements**
   - Pause/resume (spacebar)
   - Real-time density adjustment (+/-)
   - Speed controls (arrow keys)
   - HUD overlay with stats

3. **Additional Effects**
   - Starfield
   - Snow
   - Fire
   - Custom user effects

4. **Recording Mode**
   - GIF export
   - Video recording
   - Screenshot capture

## Figma Design System

### Component Library

**Base Components**:
- `Drop/Column` - Individual falling character stream
- `Character/Bright` - Head character
- `Character/Normal` - Body character
- `Character/Dim` - Tail character
- `Canvas/Background` - Terminal background

**Frames**:
- `Terminal/Default` (80×24)
- `Terminal/Wide` (120×40)
- `Terminal/Ultrawide` (200×50)

### Design Tokens

```
Colors:
  --bg-default: #000000
  --fg-bright: #00FF00
  --fg-medium: #00AA00
  --fg-dim: #005500
  --fg-head: #FFFFFF

Spacing:
  --column-width: 2ch
  --row-height: 1em

Animation:
  --fps-slow: 30
  --fps-default: 60
  --fps-fast: 90

Density:
  --density-sparse: 0.5
  --density-default: 1.0
  --density-dense: 2.0
```

## Implementation Checklist

- [x] Black background canvas
- [x] Green character rendering
- [x] Falling animation at 60 FPS
- [x] Configurable FPS
- [x] Configurable density
- [x] Configurable charset
- [x] Multiple exit methods
- [x] Terminal resize handling
- [x] Configuration hierarchy
- [ ] Color theme support
- [ ] Interactive runtime controls
- [ ] Additional effects

## Conclusion

This design system provides a comprehensive framework for the tiny-terminal visual experience. The focus remains on performance, customizability, and nostalgic aesthetics while maintaining modern development practices.

---

**Version**: 1.0.0
**Last Updated**: 2025-11-13
**Status**: Initial design specification
