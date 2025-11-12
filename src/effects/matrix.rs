use crossterm::{cursor, queue, style::{Color, Print, ResetColor, SetForegroundColor}, terminal::{Clear, ClearType}};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crate::config::Config;

#[derive(Clone)]
struct Drop {
    x: u16,
    y: i16, // allow negative to spawn above
    speed: u16,
}

pub fn run(cfg: &Config, cancel_key: Option<char>) -> anyhow::Result<()> {
    use crossterm::terminal;
    let mut stdout = stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut rng = StdRng::from_entropy();
    let mut last_resize = (0, 0);
    let mut drops: Vec<Drop> = Vec::new();

    let frame = Duration::from_millis((1000 / cfg.fps.max(1)) as u64);
    let mut last = Instant::now();

    loop {
        // Resize-aware
        let (w, h) = terminal::size()?;
        if (w as usize, h as usize) != (last_resize.0 as usize, last_resize.1 as usize) {
            last_resize = (w, h);
            drops.clear();
            queue!(stdout, Clear(ClearType::All))?;
        }

        // Seed new drops based on density
        let columns = (w / cfg.column_width.max(1)).max(1);
        let add = ((columns as f32) * cfg.density).ceil() as usize;
        for _ in 0..add {
            let col = rng.gen_range(0..columns) * cfg.column_width.max(1);
            drops.push(Drop { x: col, y: -(rng.gen_range(0..(h as u16)) as i16), speed: rng.gen_range(1..=3) });
        }

        // Draw frame
        queue!(stdout, cursor::Hide)?;
        queue!(stdout, Clear(ClearType::All))?;
        for d in drops.iter_mut() {
            d.y += d.speed as i16;
            if d.y >= 0 && (d.y as u16) < h {
                let ch = pick_char(&cfg, &mut rng);
                if cfg.green { queue!(stdout, SetForegroundColor(Color::Green))?; }
                queue!(stdout, cursor::MoveTo(d.x, d.y as u16), Print(ch))?;
                if cfg.green { queue!(stdout, ResetColor)?; }
            }
        }
        stdout.flush()?;

        // Cull off-screen
        drops.retain(|d| (d.y as u16) < h + 5);

        // Key handling (non-blocking)
        if crossterm::event::poll(Duration::from_millis(0))? {
            if let crossterm::event::Event::Key(k) = crossterm::event::read()? {
                use crossterm::event::{KeyCode, KeyModifiers};
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if k.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Char(c) => {
                        if let Some(ck) = cancel_key { if c == ck { break; } }
                    }
                    _ => {}
                }
            }
        }

        // Frame timing
        let now = Instant::now();
        let elapsed = now - last;
        if elapsed < frame { std::thread::sleep(frame - elapsed); }
        last = now;
    }

    // Cleanup
    crossterm::execute!(stdout, cursor::Show)?;
    crossterm::execute!(stdout, terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn pick_char(cfg: &Config, rng: &mut StdRng) -> char {
    let bytes = cfg.charset.as_bytes();
    if bytes.is_empty() { return '.'; }
    let i = rng.gen_range(0..bytes.len());
    bytes[i] as char
}
