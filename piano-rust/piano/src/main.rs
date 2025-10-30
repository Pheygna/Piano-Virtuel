use rodio::{OutputStream, Sink};
use rodio::buffer::SamplesBuffer;
use std::io;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

// Structure pour garder l'état de l'application
struct App {
    last_note: Option<(char, f32)>,
    pressed_keys: Vec<(char, Instant)>,
}

impl App {
    fn new() -> Self {
        Self {
            last_note: None,
            pressed_keys: Vec::new(),
        }
    }

    fn press_key(&mut self, key: char, freq: f32) {
        self.last_note = Some((key, freq));
        self.pressed_keys.push((key, Instant::now()));
        // Garder seulement les touches pressées récemment (dernier 500ms)
        self.pressed_keys.retain(|(_, time)| time.elapsed() < Duration::from_millis(500));
    }

    fn is_key_pressed(&self, key: char) -> bool {
        self.pressed_keys.iter().any(|(k, _)| *k == key)
    }
}

// Génère une onde sinusoïdale pour une fréquence donnée
fn generate_tone(frequency: f32, duration_ms: u64) -> SamplesBuffer<f32> {
    let sample_rate = 48000u32;
    let duration_samples = (sample_rate as u64 * duration_ms) / 1000;
    
    let samples: Vec<f32> = (0..duration_samples)
        .map(move |i| {
            let t = i as f32 / sample_rate as f32;
            let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin();
            let envelope = if i < (sample_rate / 10) as u64 {
                i as f32 / (sample_rate as f32 / 10.0)
            } else if i > duration_samples - (sample_rate / 20) as u64 {
                (duration_samples - i) as f32 / (sample_rate as f32 / 20.0)
            } else {
                1.0
            };
            sample * envelope * 0.3
        })
        .collect();
    
    SamplesBuffer::new(1, sample_rate, samples)
}

// Fréquences des notes
fn get_frequency(key: char) -> Option<f32> {
    match key {
        'a' => Some(261.63), 'z' => Some(277.18), 'e' => Some(293.66),
        'r' => Some(311.13), 't' => Some(329.63), 'y' => Some(349.23),
        'u' => Some(369.99), 'i' => Some(392.00), 'o' => Some(415.30),
        'p' => Some(440.00), 'q' => Some(466.16), 's' => Some(493.88),
        'd' => Some(523.25), 'f' => Some(554.37), 'g' => Some(587.33),
        'h' => Some(622.25), 'j' => Some(659.25), 'k' => Some(698.46),
        'l' => Some(739.99), 'm' => Some(783.99),
        _ => None,
    }
}

fn get_note_name(key: char) -> &'static str {
    match key {
        'a' => "C4", 'z' => "C#4", 'e' => "D4", 'r' => "D#4",
        't' => "E4", 'y' => "F4", 'u' => "F#4", 'i' => "G4",
        'o' => "G#4", 'p' => "A4", 'q' => "A#4", 's' => "B4",
        'd' => "C5", 'f' => "C#5", 'g' => "D5", 'h' => "D#5",
        'j' => "E5", 'k' => "F5", 'l' => "F#5", 'm' => "G5",
        _ => "?",
    }
}

fn is_black_key(key: char) -> bool {
    matches!(key, 'z' | 'r' | 'u' | 'o' | 'q' | 'f' | 'h' | 'l')
}

// Dessiner une touche du piano
fn draw_key(key: char, is_pressed: bool, is_black: bool) -> Paragraph<'static> {
    let note_name = get_note_name(key);
    let key_char = key.to_uppercase().to_string();
    
    let (bg_color, fg_color, border_color) = if is_pressed {
        if is_black {
            (Color::Cyan, Color::Black, Color::Yellow)
        } else {
            (Color::Yellow, Color::Black, Color::Cyan)
        }
    } else {
        if is_black {
            (Color::Black, Color::White, Color::DarkGray)
        } else {
            (Color::White, Color::Black, Color::Gray)
        }
    };

    let text = vec![
        Line::from(Span::styled(
            key_char,
            Style::default().fg(fg_color).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            note_name,
            Style::default().fg(if is_pressed { Color::Red } else { Color::DarkGray }),
        )),
    ];

    Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .style(Style::default().bg(bg_color))
        )
        .alignment(Alignment::Center)
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Titre
    let title = Paragraph::new(vec![
        Line::from(Span::styled(
            "🎹 PIANO VIRTUEL EN RUST 🎹",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(title, chunks[0]);

    // Octave 1
    let octave1_keys = ['a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'q', 's'];
    draw_octave(f, chunks[1], &octave1_keys, app, "OCTAVE 1");

    // Octave 2
    let octave2_keys = ['d', 'f', 'g', 'h', 'j', 'k', 'l', 'm'];
    draw_octave(f, chunks[2], &octave2_keys, app, "OCTAVE 2");

    // Info sur la dernière note
    let info_text = if let Some((key, freq)) = app.last_note {
        format!(
            "🎵 Dernière note: {} ({}) - {:.2} Hz",
            key.to_uppercase(),
            get_note_name(key),
            freq
        )
    } else {
        "🎵 Appuie sur les touches pour jouer...".to_string()
    };

    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)));
    f.render_widget(info, chunks[3]);

    // Instructions
    let instructions = Paragraph::new("📝 ESC ou Ctrl+C pour quitter")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[4]);
}

fn draw_octave(f: &mut Frame, area: Rect, keys: &[char], app: &App, title: &str) {
    let octave_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));
    
    let inner = octave_block.inner(area);  // ✅ Calcule inner AVANT
    f.render_widget(octave_block, area);   // ✅ Maintenant on peut déplacer
    
    let key_width = inner.width / keys.len() as u16;
    
    for (i, &key) in keys.iter().enumerate() {
        let key_area = Rect {
            x: inner.x + (i as u16 * key_width),
            y: inner.y,
            width: key_width.saturating_sub(1),
            height: inner.height,
        };
        
        let is_pressed = app.is_key_pressed(key);
        let is_black = is_black_key(key);
        let key_widget = draw_key(key, is_pressed, is_black);
        f.render_widget(key_widget, key_area);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialiser l'audio
    let (_stream, stream_handle) = OutputStream::try_default()?;
    
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        if let Some(freq) = get_frequency(c) {
                            let sink = Sink::try_new(&stream_handle)?;
                            let source = generate_tone(freq, 500);
                            sink.append(source);
                            sink.detach();
                            app.press_key(c, freq);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}