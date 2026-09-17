// =============================================================================
//  src/tools/ffmedia.rs — 24-in-1 FFmpeg Multimedia Suite (Modern fkill UI)
// =============================================================================

use std::fs;
use std::io::{self, stdout, Write};
use std::path::PathBuf;
use std::process::Command;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

// ── Colour Palette (matching fkill / dark modern violet theme) ───────────────
const C_BG: Color          = Color::Rgb(10, 10, 18);
const C_BORDER: Color      = Color::Rgb(180, 100, 255); // deep violet accent
const C_ACCENT: Color      = Color::Rgb(200, 140, 255); // bright violet
const C_SELECTED_BG: Color = Color::Rgb(45, 20, 70);  // selected row bg
const C_SELECTED_FG: Color = Color::Rgb(240, 210, 255); // selected text
const C_DIM: Color         = Color::Rgb(90, 90, 120);
const C_TEXT: Color        = Color::Rgb(220, 220, 235);
const C_GREEN: Color       = Color::Rgb(80, 220, 140);
const C_YELLOW: Color      = Color::Rgb(255, 200, 80);
const C_WHITE: Color       = Color::Rgb(255, 255, 255);
const C_CYAN: Color        = Color::Rgb(80, 210, 240);
const C_PINK: Color        = Color::Rgb(255, 110, 180);

#[derive(Debug, Clone)]
pub struct MediaActionInfo {
    pub id: u8,
    pub title: &'static str,
    pub emoji: &'static str,
    pub summary: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub features: &'static [&'static str],
    pub inputs: &'static [&'static str],
    pub sample_cmd: &'static str,
}

pub const ALL_ACTIONS: &[MediaActionInfo] = &[
    MediaActionInfo {
        id: 1,
        title: "Compress Video",
        emoji: "📦",
        summary: "50%-80% size reduction",
        category: "Optimization",
        description: "Compresses video using H.264 CRF encoding. Reduces file size dramatically while preserving high visual fidelity.",
        features: &[
            "Balanced Quality (CRF 23 - Recommended)",
            "High Compression (CRF 28 - ~50-70% size cut)",
            "Extreme Compression (CRF 32 - ~70-80% size cut)",
        ],
        inputs: &["Input video file", "Compression level preset"],
        sample_cmd: "ffmpeg -i input.mp4 -vcodec libx264 -crf 23 compressed.mp4",
    },
    MediaActionInfo {
        id: 2,
        title: "Fast Lossless Trim",
        emoji: "✂️",
        summary: "Instant clip cutting without re-encoding",
        category: "Editing",
        description: "Trims video using stream copy (-c copy). Cuts clips instantly with zero quality loss and no video re-encoding.",
        features: &[
            "Instant execution (no CPU rendering)",
            "100% loss-free original video stream",
            "Custom start timestamp & clip duration",
        ],
        inputs: &["Input video file", "Start time (HH:MM:SS or seconds)", "Duration in seconds"],
        sample_cmd: "ffmpeg -ss 00:01:30 -i input.mp4 -t 30 -c copy trimmed.mp4",
    },
    MediaActionInfo {
        id: 3,
        title: "Concat Videos",
        emoji: "🔗",
        summary: "Merge multiple video clips together",
        category: "Editing",
        description: "Merges multiple video files into a single continuous video using the FFmpeg concat engine.",
        features: &[
            "Merge unlimited video clips",
            "Automatic playlist generator",
            "Seamless stream stitching",
        ],
        inputs: &["List of video files to concatenate"],
        sample_cmd: "ffmpeg -f concat -safe 0 -i list.txt -c copy merged.mp4",
    },
    MediaActionInfo {
        id: 4,
        title: "Resolution & Aspect Ratio",
        emoji: "📐",
        summary: "1080p / 720p / 480p / 9:16 Reels",
        category: "Formatting",
        description: "Rescale video dimensions to popular presets or convert video aspect ratio for Shorts, TikTok, or Instagram Reels.",
        features: &[
            "Full HD 1080p (1920x1080)",
            "HD 720p (1280x720)",
            "SD 480p (854x480)",
            "Vertical 9:16 (1080x1920 Shorts/Reels)",
        ],
        inputs: &["Input video file", "Target resolution preset"],
        sample_cmd: "ffmpeg -i input.mp4 -vf scale=1280:720 rescaled.mp4",
    },
    MediaActionInfo {
        id: 5,
        title: "Speed Control",
        emoji: "⏩",
        summary: "Slow motion & Time-lapse clips",
        category: "Effects",
        description: "Adjust video playback speed to create dramatic slow-motion clips or fast time-lapse sequences with synced audio.",
        features: &[
            "0.25x / 0.5x Slow Motion",
            "1.5x / 2.0x Fast Forward / Time-lapse",
            "Synchronized audio pitch adjustment",
        ],
        inputs: &["Input video file", "Playback speed factor"],
        sample_cmd: "ffmpeg -i input.mp4 -filter_complex \"[0:v]setpts=0.5*PTS[v];[0:a]atempo=2.0[a]\" speed.mp4",
    },
    MediaActionInfo {
        id: 6,
        title: "Rotate & Flip Video",
        emoji: "🔄",
        summary: "90° / 180° / Horizontal & Vertical Flip",
        category: "Effects",
        description: "Fix sideways smartphone videos by rotating orientation or flipping video horizontally/vertically.",
        features: &[
            "Rotate 90° Clockwise / Counter-Clockwise",
            "Rotate 180° Upside Down",
            "Horizontal Mirror Flip / Vertical Flip",
        ],
        inputs: &["Input video file", "Rotation or flip option"],
        sample_cmd: "ffmpeg -i input.mp4 -vf transpose=1 rotated.mp4",
    },
    MediaActionInfo {
        id: 7,
        title: "Extract Audio",
        emoji: "🎵",
        summary: "MP3 / AAC / WAV / FLAC extraction",
        category: "Audio",
        description: "Extract soundtrack from video files and export to popular uncompressed, compressed, or lossless audio formats.",
        features: &[
            "MP3 (Universal compact audio)",
            "AAC (High efficiency audio)",
            "WAV (Uncompressed studio PCM)",
            "FLAC (Lossless high-res audio)",
        ],
        inputs: &["Input media file", "Target audio format"],
        sample_cmd: "ffmpeg -i input.mp4 -vn audio.mp3",
    },
    MediaActionInfo {
        id: 8,
        title: "Mute Video",
        emoji: "🔇",
        summary: "Remove audio stream from video",
        category: "Audio",
        description: "Strip all audio tracks from video file. Video stream is copied without re-encoding for instant processing.",
        features: &[
            "Instant execution (video stream copy)",
            "Zero video quality degradation",
            "Produces clean silent video file",
        ],
        inputs: &["Input video file"],
        sample_cmd: "ffmpeg -i input.mp4 -an -vcodec copy muted.mp4",
    },
    MediaActionInfo {
        id: 9,
        title: "Snapshot Capture",
        emoji: "📸",
        summary: "Ultra HD JPG / PNG photo frame",
        category: "Images",
        description: "Capture a crisp full-resolution image frame from video at any given timestamp.",
        features: &[
            "Exact timestamp extraction",
            "High quality JPG or PNG frame export",
            "Full native video resolution",
        ],
        inputs: &["Input video file", "Target timestamp (HH:MM:SS)"],
        sample_cmd: "ffmpeg -ss 00:00:05 -i input.mp4 -vframes 1 snapshot.jpg",
    },
    MediaActionInfo {
        id: 10,
        title: "Pro Quality GIF Creation",
        emoji: "🎨",
        summary: "Smooth high-FPS animated GIF",
        category: "Images",
        description: "Convert video segment into a smooth animated GIF using high-quality color palette generation and Lanczos scaling.",
        features: &[
            "Smooth 15 FPS animation",
            "Lanczos high-clarity image scaler",
            "Compact web-ready output size",
        ],
        inputs: &["Input video file"],
        sample_cmd: "ffmpeg -i input.mp4 -vf \"fps=15,scale=480:-1:flags=lanczos\" output.gif",
    },
    MediaActionInfo {
        id: 11,
        title: "Privacy Clean",
        emoji: "🔒",
        summary: "Strip EXIF & GPS location metadata",
        category: "Privacy",
        description: "Remove embedded metadata, device serials, author tags, and GPS location coordinates from media files.",
        features: &[
            "Strips EXIF, XMP & GPS metadata tags",
            "Instant stream copy processing",
            "Protects location & camera privacy",
        ],
        inputs: &["Input media file"],
        sample_cmd: "ffmpeg -i input.mp4 -map_metadata -1 -c copy clean.mp4",
    },
    MediaActionInfo {
        id: 12,
        title: "Format Conversion",
        emoji: "🔄",
        summary: "MP4 / MKV / WEBM / MOV / AVI",
        category: "Formatting",
        description: "Convert video containers between popular standard formats for maximum media player compatibility.",
        features: &[
            "MP4 (Universal standard)",
            "MKV (Multi-track container)",
            "WEBM (Web optimized video)",
            "MOV & AVI legacy formats",
        ],
        inputs: &["Input media file", "Target format extension"],
        sample_cmd: "ffmpeg -i input.mkv converted.mp4",
    },
    MediaActionInfo {
        id: 13,
        title: "Terminal Screen Recorder",
        emoji: "🎥",
        summary: "Capture desktop screen & audio stream",
        category: "Recording",
        description: "Record desktop screen with optional microphone / system audio input directly from the terminal.",
        features: &[
            "Linux x11grab + PulseAudio support",
            "macOS AVFoundation screen & mic capture",
            "Windows gdigrab desktop recorder",
        ],
        inputs: &["Audio option (Yes/No)"],
        sample_cmd: "ffmpeg -f x11grab -video_size 1920x1080 -i :0.0 screen_record.mp4",
    },
    MediaActionInfo {
        id: 14,
        title: "Subtitle Burn-in",
        emoji: "✍️",
        summary: "Hardcode SRT/ASS subtitles into video",
        category: "Subtitles",
        description: "Burn external SRT or ASS subtitle files directly into the video stream for universal playback.",
        features: &[
            "Supports SRT and ASS subtitle formats",
            "Hardcoded video subtitles",
            "Preserves audio stream without re-encoding",
        ],
        inputs: &["Input video file", "Subtitle file (.srt/.ass)"],
        sample_cmd: "ffmpeg -i input.mp4 -vf subtitles=sub.srt -c:a copy subtitled.mp4",
    },
    MediaActionInfo {
        id: 15,
        title: "Subtitle Extraction",
        emoji: "📄",
        summary: "Extract subtitle track to .srt",
        category: "Subtitles",
        description: "Extract embedded subtitle stream from MKV/MP4 files into a separate standalone .srt file.",
        features: &[
            "Extracts internal subtitle tracks",
            "Exports directly to .srt text format",
        ],
        inputs: &["Input video file"],
        sample_cmd: "ffmpeg -i input.mkv -map 0:s:0 subtitle.srt",
    },
    MediaActionInfo {
        id: 16,
        title: "Stop Screen Recording",
        emoji: "🛑",
        summary: "Stop active background screen recording",
        category: "Recording",
        description: "Sends SIGINT to the active ffmpeg screen recorder process so the MP4 video is finalized and saved cleanly.",
        features: &[
            "Reads active PID from /tmp/ffrecord.pid",
            "Graceful SIGINT shutdown for clean MP4 muxing",
            "Displays saved video file path",
        ],
        inputs: &["No parameters needed"],
        sample_cmd: "kill -SIGINT $(cat /tmp/ffrecord.pid)",
    },
];

struct App {
    actions: &'static [MediaActionInfo],
    filtered: Vec<usize>,
    list_state: ListState,
    query: String,
}

impl App {
    fn new() -> Self {
        let filtered: Vec<usize> = (0..ALL_ACTIONS.len()).collect();
        let mut list_state = ListState::default();
        if !filtered.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            actions: ALL_ACTIONS,
            filtered,
            list_state,
            query: String::new(),
        }
    }

    fn refilter(&mut self) {
        let q = self.query.to_lowercase();
        self.filtered = self
            .actions
            .iter()
            .enumerate()
            .filter(|(_, a)| {
                q.is_empty()
                    || a.title.to_lowercase().contains(&q)
                    || a.summary.to_lowercase().contains(&q)
                    || a.category.to_lowercase().contains(&q)
                    || a.description.to_lowercase().contains(&q)
                    || a.id.to_string().contains(&q)
            })
            .map(|(i, _)| i)
            .collect();

        let sel = self.list_state.selected().unwrap_or(0);
        if self.filtered.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(sel.min(self.filtered.len() - 1)));
        }
    }

    fn selected_action(&self) -> Option<&'static MediaActionInfo> {
        let idx = self.list_state.selected()?;
        let action_idx = *self.filtered.get(idx)?;
        self.actions.get(action_idx)
    }

    fn move_up(&mut self) {
        if self.filtered.is_empty() {
            return;
        }
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state
            .select(Some(if i == 0 { self.filtered.len() - 1 } else { i - 1 }));
    }

    fn move_down(&mut self) {
        if self.filtered.is_empty() {
            return;
        }
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state
            .select(Some((i + 1) % self.filtered.len()));
    }
}

/// Runs the interactive FFmpeg multimedia suite.
pub fn run(action_opt: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    if !is_ffmpeg_installed() {
        return Err("ffmpeg command not found. Please install ffmpeg to use ffmedia.".into());
    }

    let action_id = match action_opt {
        Some(a) => parse_action_arg(a),
        None => match run_ffmedia_tui()? {
            Some(act) => Some(act.id),
            None => return Ok(()),
        },
    };

    let id = match action_id {
        Some(i) => i,
        None => return Ok(()),
    };

    println!();
    match id {
        1 => compress_video()?,
        2 => trim_video()?,
        3 => concat_videos()?,
        4 => resolution_video()?,
        5 => speed_video()?,
        6 => rotate_video()?,
        7 => extract_audio()?,
        8 => mute_video()?,
        9 => snapshot_video()?,
        10 => create_gif()?,
        11 => clean_metadata()?,
        12 => convert_format()?,
        13 => record_screen()?,
        14 => subtitle_burn()?,
        15 => subtitle_extract()?,
        16 => stop_recording()?,
        _ => println!("Unknown action ID: {}", id),
    }

    Ok(())
}

fn parse_action_arg(arg: &str) -> Option<u8> {
    let s = arg.to_lowercase();
    if s.contains("compress") || s == "1" { Some(1) }
    else if s.contains("trim") || s == "2" { Some(2) }
    else if s.contains("concat") || s == "3" { Some(3) }
    else if s.contains("res") || s.contains("resolution") || s == "4" { Some(4) }
    else if s.contains("speed") || s == "5" { Some(5) }
    else if s.contains("rotate") || s.contains("flip") || s == "6" { Some(6) }
    else if s.contains("extract") || s.contains("audio") || s == "7" { Some(7) }
    else if s.contains("mute") || s == "8" { Some(8) }
    else if s.contains("snapshot") || s.contains("snap") || s == "9" { Some(9) }
    else if s.contains("gif") || s == "10" { Some(10) }
    else if s.contains("clean") || s.contains("privacy") || s == "11" { Some(11) }
    else if s.contains("convert") || s.contains("format") || s == "12" { Some(12) }
    else if s.contains("record") || s.contains("screen") || s == "13" { Some(13) }
    else if s.contains("burn") || s == "14" { Some(14) }
    else if s.contains("sub") || s == "15" { Some(15) }
    else if s.contains("stop") || s == "16" { Some(16) }
    else { None }
}

fn run_ffmedia_tui() -> Result<Option<&'static MediaActionInfo>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout_handle = stdout();
    execute!(stdout_handle, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout_handle);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = loop {
        terminal.draw(|f| draw_ffmedia(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
            match (key.modifiers, key.code) {
                (_, KeyCode::Esc) | (KeyModifiers::CONTROL, KeyCode::Char('c')) => break None,
                (_, KeyCode::Char('q')) if app.query.is_empty() => break None,
                (_, KeyCode::Up) | (KeyModifiers::CONTROL, KeyCode::Char('p')) => app.move_up(),
                (_, KeyCode::Down) | (KeyModifiers::CONTROL, KeyCode::Char('n')) => app.move_down(),
                (_, KeyCode::Char('k')) if !is_ctrl => app.move_up(),
                (_, KeyCode::Char('j')) if !is_ctrl => app.move_down(),
                (_, KeyCode::Enter) => {
                    if let Some(act) = app.selected_action() {
                        break Some(act);
                    }
                }
                (_, KeyCode::Backspace) => {
                    app.query.pop();
                    app.refilter();
                }
                (_, KeyCode::Char(c)) => {
                    app.query.push(c);
                    app.refilter();
                }
                _ => {}
            }
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(res)
}

fn draw_ffmedia(f: &mut Frame, app: &mut App) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Banner
            Constraint::Length(3), // Search bar
            Constraint::Min(8),    // Dual pane
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    // ── Banner ──────────────────────────────────────────────────────────────
    let banner_text = Line::from(vec![
        Span::styled("🎬  ", Style::default().fg(C_ACCENT)),
        Span::styled("FFMEDIA", Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)),
        Span::styled(" — All-in-One FFmpeg Multimedia Suite", Style::default().fg(C_TEXT)),
        Span::styled(
            format!("  ({} Action Modules | FFmpeg Engine)", app.actions.len()),
            Style::default().fg(C_DIM),
        ),
    ]);
    let banner = Paragraph::new(banner_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_BG)),
        );
    f.render_widget(banner, outer[0]);

    // ── Search Bar ──────────────────────────────────────────────────────────
    let search_text = Line::from(vec![
        Span::styled(" 🔍 ", Style::default().fg(C_ACCENT)),
        Span::styled(&app.query, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
        Span::styled("█", Style::default().fg(C_BORDER)),
        Span::styled(
            format!("  ({} matches)", app.filtered.len()),
            Style::default().fg(C_DIM),
        ),
    ]);
    let search_bar = Paragraph::new(search_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_ACCENT))
            .title(Span::styled(" Search Action / Query ", Style::default().fg(C_ACCENT)))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(search_bar, outer[1]);

    // ── Dual Pane (List + Details) ──────────────────────────────────────────
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(outer[2]);

    // ── Left: Action List ───────────────────────────────────────────────────
    let items: Vec<ListItem> = app
        .filtered
        .iter()
        .enumerate()
        .map(|(display_idx, &action_idx)| {
            let act = &app.actions[action_idx];
            let is_sel = app.list_state.selected() == Some(display_idx);

            if is_sel {
                let line = Line::from(vec![
                    Span::styled(" ▶ ", Style::default().fg(C_SELECTED_FG).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("{:<2}. {} ", act.id, act.emoji),
                        Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        act.title,
                        Style::default().fg(C_SELECTED_FG).add_modifier(Modifier::BOLD),
                    ),
                ]);
                ListItem::new(line).style(Style::default().bg(C_SELECTED_BG))
            } else {
                let line = Line::from(vec![
                    Span::styled("   ", Style::default()),
                    Span::styled(
                        format!("{:<2}. {} ", act.id, act.emoji),
                        Style::default().fg(C_DIM),
                    ),
                    Span::styled(act.title, Style::default().fg(C_TEXT)),
                ]);
                ListItem::new(line)
            }
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .title(Span::styled(
                " Action Modules ",
                Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(C_BG)),
    );
    f.render_stateful_widget(list, panes[0], &mut app.list_state);

    // ── Right: Action Detail Preview ────────────────────────────────────────
    let detail_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_BORDER))
        .title(Span::styled(
            " Action Details & Command Syntax ",
            Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(C_BG));

    if let Some(act) = app.selected_action() {
        let mut detail_lines = vec![
            Line::from(vec![
                Span::styled(format!(" {} ", act.emoji), Style::default().fg(C_YELLOW)),
                Span::styled(act.title, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
                Span::styled(format!("  [{}]", act.category), Style::default().fg(C_CYAN)),
            ]),
            Line::from(Span::styled(" ─────────────────────────────────────────────────────────────", Style::default().fg(C_DIM))),
            Line::from(vec![
                Span::styled(" Summary: ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
                Span::styled(act.summary, Style::default().fg(C_TEXT)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" Description:", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(format!("  {}", act.description), Style::default().fg(C_TEXT)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" Key Features & Options:", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
            ]),
        ];

        for feature in act.features {
            detail_lines.push(Line::from(vec![
                Span::styled("  • ", Style::default().fg(C_GREEN)),
                Span::styled(*feature, Style::default().fg(C_TEXT)),
            ]));
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::from(vec![
            Span::styled(" Inputs Required:", Style::default().fg(C_PINK).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" {}", act.inputs.join(", ")), Style::default().fg(C_TEXT)),
        ]));

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::from(vec![
            Span::styled(" Sample FFmpeg Command:", Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD)),
        ]));
        detail_lines.push(Line::from(vec![
            Span::styled(format!("  $ {}", act.sample_cmd), Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
        ]));

        let detail_paragraph = Paragraph::new(detail_lines)
            .wrap(Wrap { trim: true })
            .block(detail_block);
        f.render_widget(detail_paragraph, panes[1]);
    } else {
        let empty_msg = Paragraph::new("No action matching current filter.")
            .alignment(Alignment::Center)
            .block(detail_block);
        f.render_widget(empty_msg, panes[1]);
    }

    // ── Status Bar ──────────────────────────────────────────────────────────
    let status_text = Line::from(vec![
        Span::styled(" ↑↓ Navigate", Style::default().fg(C_DIM)),
        Span::styled("  |  ", Style::default().fg(C_BORDER)),
        Span::styled("Type to filter", Style::default().fg(C_DIM)),
        Span::styled("  |  ", Style::default().fg(C_BORDER)),
        Span::styled("Enter Select Action", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("  |  ", Style::default().fg(C_BORDER)),
        Span::styled("Esc Quit", Style::default().fg(C_DIM)),
    ]);
    let status_bar = Paragraph::new(status_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_DIM))
                .style(Style::default().bg(C_BG)),
        );
    f.render_widget(status_bar, outer[3]);
}

// ── Action Handlers ──────────────────────────────────────────────────────────

fn compress_video() -> Result<(), Box<dyn std::error::Error>> {
    print_header("📦 Compress Video (H.264 CRF)");
    let file = prompt_file("Select video to compress")?;
    let preset = prompt_select(
        "Select Compression Preset:",
        &[
            "1) Balanced Quality (CRF 23 - Recommended)",
            "2) High Compression (CRF 28 - ~50-70% size cut)",
            "3) Extreme Compression (CRF 32 - ~70-80% size cut)",
        ],
    )?;

    let crf = if preset.contains("2)") {
        "28"
    } else if preset.contains("3)") {
        "32"
    } else {
        "23"
    };

    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    let out = file.with_file_name(format!("{}_compressed.{}", stem, ext));

    println!("\n⚡ Compressing '{}' (CRF {})...", file.display(), crf);
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file)
        .arg("-vcodec")
        .arg("libx264")
        .arg("-crf")
        .arg(crf)
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Compression complete: {}", out.display());
    } else {
        println!("❌ Compression failed.");
    }
    Ok(())
}

fn trim_video() -> Result<(), Box<dyn std::error::Error>> {
    print_header("✂️ Fast Lossless Trim");
    let file = prompt_file("Select video to trim")?;
    let start_time = prompt_text("Enter start time (e.g. 00:01:30 or 90)")?;
    let duration = prompt_text("Enter duration in seconds (e.g. 30)")?;

    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    let out = file.with_file_name(format!("{}_trimmed.{}", stem, ext));

    println!("\n⚡ Trimming clip from timestamp {} for {} seconds...", start_time, duration);
    let status = Command::new("ffmpeg")
        .arg("-ss")
        .arg(&start_time)
        .arg("-i")
        .arg(&file)
        .arg("-t")
        .arg(&duration)
        .arg("-c")
        .arg("copy")
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Trim complete: {}", out.display());
    } else {
        println!("❌ Trim failed.");
    }
    Ok(())
}

fn concat_videos() -> Result<(), Box<dyn std::error::Error>> {
    print_header("🔗 Concat Videos (Merge Clips)");
    println!("Enter file paths to concatenate (separated by space or comma):");
    let input_str = prompt_text("Video files")?;
    let paths: Vec<PathBuf> = input_str
        .split(|c| c == ',' || c == ' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect();

    if paths.len() < 2 {
        return Err("At least 2 video files are required for concatenation.".into());
    }

    for p in &paths {
        if !p.exists() {
            return Err(format!("File not found: {}", p.display()).into());
        }
    }

    let first = &paths[0];
    let parent = first.parent().unwrap_or_else(|| std::path::Path::new("."));
    let list_file_path = parent.join("ffmedia_concat_list.txt");
    let mut list_content = String::new();
    for p in &paths {
        let abs = fs::canonicalize(p).unwrap_or_else(|_| p.clone());
        list_content.push_str(&format!("file '{}'\n", abs.display()));
    }

    fs::write(&list_file_path, list_content)?;

    let out = parent.join("merged_output.mp4");
    println!("\n⚡ Merging {} video clips...", paths.len());
    let status = Command::new("ffmpeg")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(&list_file_path)
        .arg("-c")
        .arg("copy")
        .arg(&out)
        .status();

    let _ = fs::remove_file(&list_file_path);

    if let Ok(st) = status {
        if st.success() {
            println!("✅ Concat complete: {}", out.display());
            return Ok(());
        }
    }
    println!("❌ Concat failed.");
    Ok(())
}

fn resolution_video() -> Result<(), Box<dyn std::error::Error>> {
    print_header("📐 Resolution & Aspect Ratio");
    let file = prompt_file("Select video file")?;
    let res = prompt_select(
        "Target resolution:",
        &[
            "1080p Full HD (1920x1080)",
            "720p HD (1280x720)",
            "480p SD (854x480)",
            "9:16 Vertical Reels/Shorts (1080x1920)",
        ],
    )?;

    let scale = if res.contains("720p") {
        "scale=1280:720"
    } else if res.contains("480p") {
        "scale=854:480"
    } else if res.contains("9:16") {
        "scale=1080:1920"
    } else {
        "scale=1920:1080"
    };

    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    let out = file.with_file_name(format!("{}_rescaled.{}", stem, ext));

    println!("\n⚡ Rescaling to {}...", res);
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file)
        .arg("-vf")
        .arg(scale)
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Rescaling complete: {}", out.display());
    } else {
        println!("❌ Rescaling failed.");
    }
    Ok(())
}

fn speed_video() -> Result<(), Box<dyn std::error::Error>> {
    print_header("⏩ Speed Control (Slow Motion / Time-lapse)");
    let file = prompt_file("Select video file")?;
    let speed_choice = prompt_select(
        "Select playback speed:",
        &[
            "1) 0.5x Slow Motion",
            "2) 0.25x Ultra Slow Motion",
            "3) 1.5x Fast Forward",
            "4) 2.0x Time-Lapse",
        ],
    )?;

    let (pts_scale, tempo) = if speed_choice.contains("0.25x") {
        ("4.0", "0.5,atempo=0.5")
    } else if speed_choice.contains("0.5x") {
        ("2.0", "0.5")
    } else if speed_choice.contains("1.5x") {
        ("0.666", "1.5")
    } else {
        ("0.5", "2.0")
    };

    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    let out = file.with_file_name(format!("{}_speed.{}", stem, ext));

    let vf = format!("setpts={}*PTS", pts_scale);
    println!("\n⚡ Applying speed modification...");
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file)
        .arg("-filter_complex")
        .arg(format!("[0:v]{}[v];[0:a]atempo={}[a]", vf, tempo))
        .arg("-map")
        .arg("[v]")
        .arg("-map")
        .arg("[a]")
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Speed adjustment complete: {}", out.display());
    } else {
        println!("❌ Speed adjustment failed.");
    }
    Ok(())
}

fn rotate_video() -> Result<(), Box<dyn std::error::Error>> {
    print_header("🔄 Rotate & Flip Video");
    let file = prompt_file("Select video file")?;
    let option = prompt_select(
        "Select orientation adjustment:",
        &[
            "1) Rotate 90° Clockwise",
            "2) Rotate 90° Counter-Clockwise",
            "3) Rotate 180° Upside Down",
            "4) Horizontal Flip (Mirror)",
            "5) Vertical Flip",
        ],
    )?;

    let vf = if option.contains("1)") {
        "transpose=1"
    } else if option.contains("2)") {
        "transpose=2"
    } else if option.contains("3)") {
        "transpose=2,transpose=2"
    } else if option.contains("4)") {
        "hflip"
    } else {
        "vflip"
    };

    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    let out = file.with_file_name(format!("{}_rotated.{}", stem, ext));

    println!("\n⚡ Rotating/flipping video...");
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file)
        .arg("-vf")
        .arg(vf)
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Rotation complete: {}", out.display());
    } else {
        println!("❌ Rotation failed.");
    }
    Ok(())
}

fn extract_audio() -> Result<(), Box<dyn std::error::Error>> {
    print_header("🎵 Extract Audio");
    let file = prompt_file("Select video/audio file")?;
    let fmt = prompt_select("Target audio format:", &["mp3", "aac", "wav", "flac"])?;

    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("audio");
    let out = file.with_file_name(format!("{}.{}", stem, fmt));

    println!("\n⚡ Extracting audio track as {}...", fmt.to_uppercase());
    let status = Command::new("ffmpeg").arg("-i").arg(&file).arg("-vn").arg(&out).status()?;

    if status.success() {
        println!("✅ Extracted audio to {}", out.display());
    } else {
        println!("❌ Extraction failed.");
    }
    Ok(())
}

fn mute_video() -> Result<(), Box<dyn std::error::Error>> {
    print_header("🔇 Mute Video");
    let file = prompt_file("Select video to mute")?;
    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    let out = file.with_file_name(format!("{}_muted.{}", stem, ext));

    println!("\n⚡ Removing audio track...");
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file)
        .arg("-an")
        .arg("-vcodec")
        .arg("copy")
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Muted video created: {}", out.display());
    } else {
        println!("❌ Muting failed.");
    }
    Ok(())
}

fn snapshot_video() -> Result<(), Box<dyn std::error::Error>> {
    print_header("📸 Snapshot Capture");
    let file = prompt_file("Select video file")?;
    let timestamp = prompt_text("Timestamp for snapshot (e.g. 00:00:05)")?;
    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("snapshot");
    let out = file.with_file_name(format!("{}_snapshot.jpg", stem));

    println!("\n⚡ Capturing snapshot frame at {}...", timestamp);
    let status = Command::new("ffmpeg")
        .arg("-ss")
        .arg(&timestamp)
        .arg("-i")
        .arg(&file)
        .arg("-vframes")
        .arg("1")
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Snapshot saved to {}", out.display());
    } else {
        println!("❌ Snapshot capture failed.");
    }
    Ok(())
}

fn create_gif() -> Result<(), Box<dyn std::error::Error>> {
    print_header("🎨 Pro Quality GIF Creation");
    let file = prompt_file("Select video file")?;
    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
    let out = file.with_file_name(format!("{}.gif", stem));

    println!("\n⚡ Creating high quality GIF...");
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file)
        .arg("-vf")
        .arg("fps=15,scale=480:-1:flags=lanczos")
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ GIF created: {}", out.display());
    } else {
        println!("❌ GIF creation failed.");
    }
    Ok(())
}

fn clean_metadata() -> Result<(), Box<dyn std::error::Error>> {
    print_header("🔒 Privacy Clean");
    let file = prompt_file("Select media file")?;
    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("clean");
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    let out = file.with_file_name(format!("{}_clean.{}", stem, ext));

    println!("\n⚡ Stripping EXIF/GPS metadata...");
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file)
        .arg("-map_metadata")
        .arg("-1")
        .arg("-c")
        .arg("copy")
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Metadata cleaned: {}", out.display());
    } else {
        println!("❌ Metadata cleaning failed.");
    }
    Ok(())
}

fn convert_format() -> Result<(), Box<dyn std::error::Error>> {
    print_header("🔄 Format Conversion");
    let file = prompt_file("Select media file")?;
    let target_ext = prompt_select("Select target format:", &["mp4", "mkv", "webm", "mov", "avi"])?;

    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("converted");
    let out = file.with_file_name(format!("{}.{}", stem, target_ext));

    println!("\n⚡ Converting format to {}...", target_ext.to_uppercase());
    let status = Command::new("ffmpeg").arg("-i").arg(&file).arg(&out).status()?;

    if status.success() {
        println!("✅ Format conversion complete: {}", out.display());
    } else {
        println!("❌ Format conversion failed.");
    }
    Ok(())
}

/// Detects whether we are running under a Wayland compositor.
fn is_wayland() -> bool {
    std::env::var("WAYLAND_DISPLAY").is_ok()
}

/// Returns `true` when `wf-recorder` binary is found on PATH.
#[allow(dead_code)]
fn has_wf_recorder() -> bool {
    Command::new("sh")
        .args(["-c", "command -v wf-recorder"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Returns `true` when PipeWire's `pw-cli` (or `pipewire`) is running.
fn has_pipewire() -> bool {
    Command::new("sh")
        .args(["-c", "command -v pw-cli && pw-cli info 0 >/dev/null 2>&1"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// GPU encoder types supported for hardware-accelerated recording.
#[derive(Debug, Clone, PartialEq)]
enum GpuEncoder {
    /// NVIDIA NVENC (h264_nvenc)
    Nvenc,
    /// Intel/AMD VA-API (h264_vaapi)
    Vaapi,
    /// AMD AMF (h264_amf)
    Amf,
    /// No GPU encoder found — use CPU (libx264)
    None,
}

/// Detects the best available GPU hardware encoder by probing ffmpeg.
/// Priority: NVENC > VAAPI > AMF > CPU fallback.
fn detect_gpu_encoder() -> GpuEncoder {
    let probe = |enc: &str| -> bool {
        Command::new("sh")
            .args(["-c", &format!("ffmpeg -hide_banner -encoders 2>/dev/null | grep -q {}", enc)])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    };

    // Check if NVIDIA GPU hardware device node actually exists before using NVENC
    let has_nvidia_hw = std::path::Path::new("/dev/nvidia0").exists()
                     || std::path::Path::new("/dev/nvidiactl").exists();

    if has_nvidia_hw && probe("h264_nvenc") {
        return GpuEncoder::Nvenc;
    }

    // Check Intel/AMD VA-API render node
    if std::path::Path::new("/dev/dri/renderD128").exists() && probe("h264_vaapi") {
        return GpuEncoder::Vaapi;
    }

    // Check AMD AMF hardware interface
    if std::path::Path::new("/dev/kfd").exists() && probe("h264_amf") {
        return GpuEncoder::Amf;
    }

    GpuEncoder::None
}

/// Applies the correct video encoding arguments to an ffmpeg Command
/// based on the detected GPU encoder. Falls back to CPU libx264 if none.
///
/// # GPU encoding args:
/// - NVENC  : h264_nvenc -preset p1 -rc vbr -cq <crf>  (NVIDIA)
/// - VAAPI  : h264_vaapi -qp <crf>  with hwupload filter (Intel/AMD)
/// - AMF    : h264_amf   -rc cqp -qp_i/p <crf>          (AMD)
/// - CPU    : libx264    -preset ultrafast -crf <crf>    (fallback)
#[allow(dead_code)]
fn apply_gpu_video_args(
    cmd: &mut Command,
    gpu: &GpuEncoder,
    crf: &str,
    vf_cpu: &str,
    vf_vaapi: &str,
) {
    let color_meta: &[&str] = &[
        "-color_range", "1",
        "-colorspace", "1",
        "-color_primaries", "1",
        "-color_trc", "1",
        "-movflags", "+faststart",
    ];
    match gpu {
        GpuEncoder::Nvenc => {
            // NVENC: -cq is analogous to CRF for VBR mode
            cmd.args(["-c:v", "h264_nvenc",
                      "-preset", "p1",       // fastest NVENC preset
                      "-rc", "vbr",           // variable bitrate (closest to CRF)
                      "-cq", crf,
                      "-vf", vf_cpu]);
            cmd.args(color_meta);
        }
        GpuEncoder::Vaapi => {
            // VAAPI: upload to GPU with hwupload, then encode
            cmd.args(["-c:v", "h264_vaapi",
                      "-qp", crf,
                      "-vf", vf_vaapi]);
            cmd.args(color_meta);
        }
        GpuEncoder::Amf => {
            // AMF: CQP mode with equal quality for I and P frames
            cmd.args(["-c:v", "h264_amf",
                      "-rc", "cqp",
                      "-qp_i", crf,
                      "-qp_p", crf,
                      "-vf", vf_cpu]);
            cmd.args(color_meta);
        }
        GpuEncoder::None => {
            // CPU fallback: libx264 with ultrafast preset
            cmd.args(["-c:v", "libx264",
                      "-preset", "ultrafast",
                      "-crf", crf,
                      "-vf", vf_cpu]);
            cmd.args(color_meta);
        }
    }
}

const CRF_PRESETS: &[(&str, &str, &str)] = &[
    ("Lossless",      "0",  "0% compression, 100% pixel perfect"),
    ("Near Lossless", "15", "Ultra crisp, virtually lossless"),
    ("High Quality",  "18", "Visually pristine (Recommended)"),
    ("Balanced",      "23", "Standard default balance"),
    ("Compact",       "28", "Small file size, fast export"),
];

// ── Screen Recorder TUI State ─────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum RecorderScreen {
    Settings,
    Recording,
}

struct RecorderApp {
    screen:        RecorderScreen,
    audio:         bool,
    fps:           u8,
    crf_idx:       usize,
    focused_field: usize,
    gpu:           GpuEncoder,
    // Recording state
    log_lines:     Vec<String>,
    out_file:      String,
    log_file:      String,
    pid_file:      String,   // ← stores ffmpeg PID for stop
    start_secs:    u64,
    rec_running:   bool,
    stopping:      bool,     // ← true when user pressed S to stop
}

impl RecorderApp {
    fn new(gpu: GpuEncoder) -> Self {
        Self {
            screen:        RecorderScreen::Settings,
            audio:         false,
            fps:           30,
            crf_idx:       2, // Default: High Quality (CRF 18)
            focused_field: 0,
            gpu,
            log_lines:     Vec::new(),
            out_file:      String::new(),
            log_file:      String::new(),
            pid_file:      String::new(),
            start_secs:    0,
            rec_running:   false,
            stopping:      false,
        }
    }

    fn crf(&self) -> &str {
        CRF_PRESETS[self.crf_idx].1
    }

    #[allow(dead_code)]
    fn crf_name(&self) -> &str {
        CRF_PRESETS[self.crf_idx].0
    }

    fn gpu_label(&self) -> &'static str {
        match self.gpu {
            GpuEncoder::Nvenc => "NVIDIA NVENC  (h264_nvenc)",
            GpuEncoder::Vaapi => "Intel/AMD VAAPI (h264_vaapi)",
            GpuEncoder::Amf   => "AMD AMF       (h264_amf)",
            GpuEncoder::None  => "CPU libx264   (no GPU found)",
        }
    }

    fn encoder_name(&self) -> &'static str {
        match self.gpu {
            GpuEncoder::Nvenc => "h264_nvenc",
            GpuEncoder::Vaapi => "h264_vaapi",
            GpuEncoder::Amf   => "h264_amf",
            GpuEncoder::None  => "libx264",
        }
    }

    fn elapsed_secs(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now.saturating_sub(self.start_secs)
    }

    fn format_elapsed(&self) -> String {
        let s = self.elapsed_secs();
        format!("{:02}:{:02}:{:02}", s / 3600, (s % 3600) / 60, s % 60)
    }

    /// Tail the log file and append new lines into self.log_lines
    fn refresh_logs(&mut self, max_lines: usize) {
        if self.log_file.is_empty() { return; }
        if let Ok(content) = std::fs::read_to_string(&self.log_file) {
            self.log_lines = content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.to_string())
                .collect();
            // keep only last max_lines
            if self.log_lines.len() > max_lines {
                let skip = self.log_lines.len() - max_lines;
                self.log_lines = self.log_lines.split_off(skip);
            }
        }
    }
}

// ── Settings Screen Draw ───────────────────────────────────────────────────

fn draw_recorder_settings(f: &mut Frame, app: &RecorderApp) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    // ── Banner ─────────────────────────────────────────────────────────────
    let banner = Paragraph::new(Line::from(vec![
        Span::styled("🎥  ", Style::default().fg(C_PINK)),
        Span::styled("SCREEN RECORDER", Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)),
        Span::styled("  —  Configure & Start", Style::default().fg(C_DIM)),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(banner, outer[0]);

    // ── Dual Pane ──────────────────────────────────────────────────────────
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(outer[1]);

    // Left — Settings fields
    let field_style = |idx: usize| -> Style {
        if app.focused_field == idx {
            Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(C_TEXT)
        }
    };
    let cursor = |idx: usize| -> &str {
        if app.focused_field == idx { "▶ " } else { "  " }
    };

    let audio_val = if app.audio { "[ ON  ]" } else { "[ OFF ]" };
    let audio_color = if app.audio { C_GREEN } else { C_DIM };
    let fps_val = format!("[  {}  ]", app.fps);

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(cursor(0), Style::default().fg(C_ACCENT)),
            Span::styled("Audio Capture  ", field_style(0)),
            Span::styled(audio_val, Style::default().fg(audio_color).add_modifier(Modifier::BOLD)),
            Span::styled("  ← Space", Style::default().fg(C_DIM)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(cursor(1), Style::default().fg(C_ACCENT)),
            Span::styled("Frame Rate     ", field_style(1)),
            Span::styled(&fps_val, Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("  ← ←/→", Style::default().fg(C_DIM)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(cursor(2), Style::default().fg(C_ACCENT)),
            Span::styled("Quality Preset ", field_style(2)),
            Span::styled("  ← ←/→ to select", Style::default().fg(C_DIM)),
        ]),
    ];

    // Render 5 CRF option badges (Compact & non-wrapping)
    for (i, (name, crf_val, _)) in CRF_PRESETS.iter().enumerate() {
        if i == app.crf_idx {
            lines.push(Line::from(vec![
                Span::styled("  ▶ ", Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!(" [ 🌟 {} (CRF {}) ] ", name.to_uppercase(), crf_val),
                    Style::default().fg(C_BG).bg(C_YELLOW).add_modifier(Modifier::BOLD),
                ),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("     ", Style::default()),
                Span::styled(
                    format!("  {} (CRF {})  ", name, crf_val),
                    Style::default().fg(C_DIM),
                ),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            if app.focused_field == 3 { "  ▶  [ 🔴 START RECORDING ]  ◀" }
                                      else { "     [ 🔴 START RECORDING ]   " },
            if app.focused_field == 3 {
                Style::default().fg(C_BG).bg(C_PINK).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(C_PINK).add_modifier(Modifier::BOLD)
            },
        ),
    ]));

    let left = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .title(Span::styled(
                    " ⚙  Settings ",
                    Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().bg(C_BG)),
        );
    f.render_widget(left, panes[0]);

    // Right — GPU & encoding info
    let (sel_name, sel_crf, sel_desc) = CRF_PRESETS[app.crf_idx];
    let audio_src = if has_pipewire() { "PipeWire" } else { "PulseAudio" };
    let right_lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  GPU Encoder  ", Style::default().fg(C_DIM)),
            Span::styled(app.gpu_label(), Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Audio Source ", Style::default().fg(C_DIM)),
            Span::styled(audio_src, Style::default().fg(C_CYAN)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Quality      ", Style::default().fg(C_DIM)),
            Span::styled(
                format!("{} (CRF {})", sel_name, sel_crf),
                Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("               ", Style::default()),
            Span::styled(format!("💡 {}", sel_desc), Style::default().fg(C_WHITE)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Encoder      ", Style::default().fg(C_DIM)),
            Span::styled(app.encoder_name(), Style::default().fg(C_ACCENT)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Color Space  ", Style::default().fg(C_DIM)),
            Span::styled("sRGB / BT.709 (Mobile Compatible)", Style::default().fg(C_GREEN)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Detached     ", Style::default().fg(C_DIM)),
            Span::styled("✅ Terminal close safe", Style::default().fg(C_GREEN)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  FPS          ", Style::default().fg(C_DIM)),
            Span::styled(format!("{} frames/sec", app.fps), Style::default().fg(C_CYAN)),
        ]),
    ];

    let right = Paragraph::new(right_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_DIM))
                .title(Span::styled(
                    " 🖥  Encoding Info ",
                    Style::default().fg(C_DIM).add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().bg(C_BG)),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(right, panes[1]);

    // ── Status bar ─────────────────────────────────────────────────────────
    let status = Paragraph::new(Line::from(vec![
        Span::styled("  Tab/↑↓", Style::default().fg(C_ACCENT)),
        Span::styled(" Navigate  ", Style::default().fg(C_DIM)),
        Span::styled("Space", Style::default().fg(C_ACCENT)),
        Span::styled(" Toggle Audio  ", Style::default().fg(C_DIM)),
        Span::styled("←/→", Style::default().fg(C_ACCENT)),
        Span::styled(" Select Quality/FPS  ", Style::default().fg(C_DIM)),
        Span::styled("Enter", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
        Span::styled(" Start  ", Style::default().fg(C_DIM)),
        Span::styled("Esc/q", Style::default().fg(C_PINK)),
        Span::styled(" Quit", Style::default().fg(C_DIM)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_DIM))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(status, outer[2]);
}

// ── Recording Screen Draw ──────────────────────────────────────────────────

fn draw_recorder_recording(f: &mut Frame, app: &RecorderApp) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    // ── Banner with timer ──────────────────────────────────────────────────
    let elapsed = app.format_elapsed();
    let rec_dot = if app.elapsed_secs() % 2 == 0 { "● REC" } else { "○ REC" };
    let banner = Paragraph::new(Line::from(vec![
        Span::styled("🎥  ", Style::default().fg(C_PINK)),
        Span::styled(rec_dot, Style::default().fg(C_PINK).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default()),
        Span::styled(&elapsed, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
        Span::styled("  —  Recording in progress", Style::default().fg(C_DIM)),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_PINK))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(banner, outer[0]);

    // ── Dual Pane ──────────────────────────────────────────────────────────
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(32), Constraint::Percentage(68)])
        .split(outer[1]);

    // Left — Recording info
    let info_lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  File    ", Style::default().fg(C_DIM)),
            Span::styled(&app.out_file, Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  GPU     ", Style::default().fg(C_DIM)),
            Span::styled(app.encoder_name(), Style::default().fg(C_GREEN)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  FPS     ", Style::default().fg(C_DIM)),
            Span::styled(format!("{}", app.fps), Style::default().fg(C_CYAN)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  CRF     ", Style::default().fg(C_DIM)),
            Span::styled(app.crf().to_string(), Style::default().fg(C_YELLOW)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Audio   ", Style::default().fg(C_DIM)),
            Span::styled(
                if app.audio { "ON" } else { "OFF" },
                if app.audio { Style::default().fg(C_GREEN) } else { Style::default().fg(C_DIM) },
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Elapsed ", Style::default().fg(C_DIM)),
            Span::styled(&elapsed, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [ Esc / q ] ", Style::default().fg(C_BG).bg(C_PINK).add_modifier(Modifier::BOLD)),
            Span::styled(" STOP & SAVE RECORDING", Style::default().fg(C_PINK).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let left = Paragraph::new(info_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_PINK))
                .title(Span::styled(
                    " 📹 Info ",
                    Style::default().fg(C_PINK).add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().bg(C_BG)),
        );
    f.render_widget(left, panes[0]);

    // Right — Live log
    let log_inner_height = panes[1].height.saturating_sub(2) as usize;
    let skip = app.log_lines.len().saturating_sub(log_inner_height);
    let visible_logs: Vec<ListItem> = app.log_lines[skip..]
        .iter()
        .map(|l| {
            let color = if l.contains("error") || l.contains("Error") {
                C_PINK
            } else if l.contains("fps=") || l.contains("frame=") {
                C_CYAN
            } else if l.contains("size=") || l.contains("time=") {
                C_YELLOW
            } else {
                C_DIM
            };
            ListItem::new(Span::styled(l.as_str(), Style::default().fg(color)))
        })
        .collect();

    let log_list = List::new(visible_logs).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .title(Span::styled(
                " 📡 Live FFmpeg Log ",
                Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(log_list, panes[1]);

    // ── Status bar ─────────────────────────────────────────────────────────
    let status = Paragraph::new(Line::from(vec![
        Span::styled("  [ Esc / q ] ", Style::default().fg(C_BG).bg(C_PINK).add_modifier(Modifier::BOLD)),
        Span::styled(" Stop & Save Recording  │  ", Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
        Span::styled("Press Esc, q, or Enter to stop recording and save video", Style::default().fg(C_DIM)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_DIM))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(status, outer[2]);
}

// ── Build the ffmpeg Command (detached) ───────────────────────────────────

fn build_detached_recorder(app: &RecorderApp, log_path: &str, pid_path: &str) -> Command {
    let fps    = app.fps.to_string();
    let crf    = app.crf().to_string();
    let out    = &app.out_file;
    let vf_cpu   = "scale=trunc(iw/2)*2:trunc(ih/2)*2,format=yuv420p";
    let vf_vaapi = "hwupload,scale_vaapi=format=nv12:w=trunc(iw/2)*2:h=trunc(ih/2)*2";

    let os_type = std::env::consts::OS;

    // On Linux we use `setsid` to detach from the terminal session so that
    // closing the terminal does NOT send SIGHUP to the child ffmpeg process.
    // stdout/stderr are redirected to the log file.
    let sh_args: String;

    if os_type == "linux" {
        // Build ffmpeg args string for sh -c
        let display_flag = if is_wayland() {
            std::env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string())
        } else {
            std::env::var("DISPLAY").unwrap_or_else(|_| ":0.0".to_string())
        };
        let res = get_linux_resolution();
        let audio_src = if has_pipewire() { "pipewire" } else { "pulse" };

        let video_enc = match &app.gpu {
            GpuEncoder::Nvenc => format!(
                "-c:v h264_nvenc -preset p1 -rc vbr -cq {} -vf '{}' -color_range 1 -colorspace 1 -color_primaries 1 -color_trc 1 -movflags +faststart",
                crf, vf_cpu
            ),
            GpuEncoder::Vaapi => format!(
                "-c:v h264_vaapi -qp {} -vf '{}' -color_range 1 -colorspace 1 -color_primaries 1 -color_trc 1 -movflags +faststart",
                crf, vf_vaapi
            ),
            GpuEncoder::Amf => format!(
                "-c:v h264_amf -rc cqp -qp_i {} -qp_p {} -vf '{}' -color_range 1 -colorspace 1 -color_primaries 1 -color_trc 1 -movflags +faststart",
                crf, crf, vf_cpu
            ),
            GpuEncoder::None => format!(
                "-c:v libx264 -preset ultrafast -crf {} -vf '{}' -color_range 1 -colorspace 1 -color_primaries 1 -color_trc 1 -movflags +faststart",
                crf, vf_cpu
            ),
        };

        let vaapi_device = if app.gpu == GpuEncoder::Vaapi {
            "-vaapi_device /dev/dri/renderD128 ".to_string()
        } else {
            String::new()
        };

        let audio_args = if app.audio {
            format!("-thread_queue_size 1024 -f {} -i default -c:a aac -b:a 128k", audio_src)
        } else {
            String::new()
        };

        // setsid detaches from terminal; nohup ignores SIGHUP; & runs in bg
        // `echo $! > pidfile` saves ffmpeg PID so we can kill it later
        sh_args = format!(
            "setsid nohup ffmpeg -y {vaapi}-thread_queue_size 1024 -use_wallclock_as_timestamps 1 -framerate {fps} -f x11grab -video_size {res} -i {disp} {audio} {venc} {out} >'{log}' 2>&1 & echo $! >'{pid}'",
            vaapi  = vaapi_device,
            fps    = fps,
            res    = res,
            disp   = display_flag,
            audio  = audio_args,
            venc   = video_enc,
            out    = out,
            log    = log_path,
            pid    = pid_path,
        );
    } else if os_type == "macos" {
        let audio_args = if app.audio { "-thread_queue_size 1024 -f avfoundation -i 0:0" } else { "-f avfoundation -i 0" };
        sh_args = format!(
            "nohup ffmpeg -y -thread_queue_size 1024 -use_wallclock_as_timestamps 1 -framerate {fps} {audio} -c:v libx264 -preset ultrafast -crf {crf} -vf '{vf}' -color_range 1 -colorspace 1 -color_primaries 1 -color_trc 1 -movflags +faststart {out} >'{log}' 2>&1 & echo $! >'{pid}'",
            fps=fps, audio=audio_args, crf=crf, vf=vf_cpu, out=out, log=log_path, pid=pid_path,
        );
    } else {
        // Windows fallback
        sh_args = format!(
            "start /B ffmpeg -y -thread_queue_size 1024 -use_wallclock_as_timestamps 1 -framerate {fps} -f gdigrab -i desktop -c:v libx264 -preset ultrafast -crf {crf} -vf '{vf}' -color_range 1 -colorspace 1 -color_primaries 1 -color_trc 1 -movflags +faststart {out} >{log} 2>&1",
            fps=fps, crf=crf, vf=vf_cpu, out=out, log=log_path,
        );
    }

    let mut cmd = Command::new("sh");
    cmd.args(["-c", &sh_args]);
    cmd
}

// ── Main record_screen() entry point ─────────────────────────────────────

fn record_screen() -> Result<(), Box<dyn std::error::Error>> {
    let gpu = detect_gpu_encoder();
    let mut app = RecorderApp::new(gpu);

    // ── Setup terminal ─────────────────────────────────────────────────────
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ── Settings event loop ────────────────────────────────────────────────
    'settings: loop {
        terminal.draw(|f| draw_recorder_settings(f, &app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // Cancel
                    KeyCode::Esc | KeyCode::Char('q') => break 'settings,

                    // Navigate fields
                    KeyCode::Tab | KeyCode::Down => {
                        app.focused_field = (app.focused_field + 1) % 4;
                    }
                    KeyCode::BackTab | KeyCode::Up => {
                        app.focused_field = (app.focused_field + 3) % 4;
                    }

                    // Space — toggle audio (when on audio field) OR generic toggle
                    KeyCode::Char(' ') => {
                        if app.focused_field == 0 { app.audio = !app.audio; }
                    }

                    // ←/→ — toggle fps OR navigate quality preset
                    KeyCode::Left => {
                        if app.focused_field == 1 {
                            app.fps = if app.fps == 30 { 60 } else { 30 };
                        } else if app.focused_field == 2 {
                            app.crf_idx = if app.crf_idx == 0 { CRF_PRESETS.len() - 1 } else { app.crf_idx - 1 };
                        }
                    }
                    KeyCode::Right => {
                        if app.focused_field == 1 {
                            app.fps = if app.fps == 30 { 60 } else { 30 };
                        } else if app.focused_field == 2 {
                            app.crf_idx = (app.crf_idx + 1) % CRF_PRESETS.len();
                        }
                    }



                    // Enter — start recording if on button, else next field
                    KeyCode::Enter => {
                        if app.focused_field == 3 {
                            // Launch recording
                            let ts = simple_timestamp();
                            app.out_file  = format!("screen_record_{}.mp4", ts);
                            app.log_file  = format!("/tmp/ffrecord_{}.log", ts);
                            app.pid_file  = "/tmp/ffrecord.pid".to_string();
                            app.start_secs = ts;
                            app.rec_running = true;
                            app.screen = RecorderScreen::Recording;

                            // Create empty log file so tail doesn't error
                            let _ = std::fs::write(&app.log_file, "");
                            let _ = std::fs::write(&app.pid_file, "");

                            // Launch detached ffmpeg (saves PID to pid_file)
                            let pid_f = app.pid_file.clone();
                            let log_f = app.log_file.clone();
                            let mut cmd = build_detached_recorder(&app, &log_f, &pid_f);
                            let _ = cmd.spawn();
                            // Give sh a moment to write the PID file
                            std::thread::sleep(std::time::Duration::from_millis(300));

                            break 'settings;
                        } else {
                            app.focused_field = (app.focused_field + 1) % 4;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // ── Recording event loop ───────────────────────────────────────────────
    if app.screen == RecorderScreen::Recording {
        loop {
            app.refresh_logs(200);
            terminal.draw(|f| draw_recorder_recording(f, &app))?;

            if event::poll(std::time::Duration::from_millis(500))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        // Esc / q / Enter — STOP & SAVE recording
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Enter => {
                            app.stopping = true;
                            kill_recording_by_pid(&app.pid_file);
                            std::thread::sleep(std::time::Duration::from_secs(2));
                            app.rec_running = false;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // ── Restore terminal ───────────────────────────────────────────────────
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if app.screen == RecorderScreen::Recording {
        if app.rec_running {
            kill_recording_by_pid(&app.pid_file);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("\n⏹️  Recording stopped and saved: {}", app.out_file);
    }

    Ok(())
}


/// Kill the ffmpeg recording process by reading its PID from a file.
/// Sends SIGINT (like Ctrl+C) so ffmpeg can finalize/mux the MP4 cleanly.
fn kill_recording_by_pid(pid_file: &str) {
    if let Ok(content) = std::fs::read_to_string(pid_file) {
        let pid = content.trim().to_string();
        if !pid.is_empty() {
            // SIGINT lets ffmpeg write the final moov atom & flush frames (clean MP4)
            let _ = Command::new("sh")
                .args(["-c", &format!("kill -INT {} 2>/dev/null || true", pid)])
                .status();

            // Wait until ffmpeg finishes writing buffered frames and exits (up to 3s)
            for _ in 0..30 {
                let status = Command::new("sh")
                    .args(["-c", &format!("kill -0 {} 2>/dev/null", pid)])
                    .status();
                if let Ok(st) = status {
                    if !st.success() {
                        break;
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            let _ = std::fs::remove_file(pid_file);
        }
    }
}

/// Stop any running screen recording.
/// Reads /tmp/ffrecord.pid and sends SIGINT to the ffmpeg process.
fn stop_recording() -> Result<(), Box<dyn std::error::Error>> {
    const PID_FILE: &str = "/tmp/ffrecord.pid";

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Check if pid file exists
    let pid_content = std::fs::read_to_string(PID_FILE).unwrap_or_default();
    let pid = pid_content.trim().to_string();

    // Draw a simple status screen
    let _ = terminal.draw(|f| {
        let area = f.area();
        f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(4), Constraint::Length(3)])
            .split(area);

        let banner = Paragraph::new(Line::from(vec![
            Span::styled("⏹️  ", Style::default().fg(C_PINK)),
            Span::styled("STOP SCREEN RECORDING", Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)),
        ]))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER)).style(Style::default().bg(C_BG)));
        f.render_widget(banner, chunks[0]);

        let (msg_color, msg) = if pid.is_empty() {
            (C_YELLOW, vec![
                Line::from(""),
                Line::from(Span::styled("  ⚠️  No active recording found.", Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD))),
                Line::from(""),
                Line::from(Span::styled("  PID file not found: /tmp/ffrecord.pid", Style::default().fg(C_DIM))),
                Line::from(""),
                Line::from(Span::styled("  If ffmpeg is still running, use:", Style::default().fg(C_DIM))),
                Line::from(Span::styled("    kill $(pgrep -f ffmpeg)", Style::default().fg(C_CYAN))),
            ])
        } else {
            (C_GREEN, vec![
                Line::from(""),
                Line::from(Span::styled(format!("  ✅ Stopping ffmpeg (PID {})...", pid), Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD))),
                Line::from(""),
                Line::from(Span::styled("  Sending SIGINT — ffmpeg will finalize your video.", Style::default().fg(C_TEXT))),
                Line::from(Span::styled("  Please wait 2-3 seconds for the file to be saved.", Style::default().fg(C_DIM))),
            ])
        };
        let _ = msg_color; // used via closure capture

        let body = Paragraph::new(msg)
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_DIM)).style(Style::default().bg(C_BG)));
        f.render_widget(body, chunks[1]);

        let status = Paragraph::new(Span::styled("  Press any key to continue...", Style::default().fg(C_DIM)))
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_DIM)).style(Style::default().bg(C_BG)));
        f.render_widget(status, chunks[2]);
    });

    // Actually kill the process
    if !pid.is_empty() {
        kill_recording_by_pid(PID_FILE);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    // Wait for any keypress
    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if pid.is_empty() {
        println!("\n⚠️  No active recording found.");
    } else {
        println!("\n✅ Recording stopped. Your video has been saved.");
    }

    Ok(())
}

/// Get current screen resolution on Linux.
/// Tries `xrandr` first (X11), falls back to `wlr-randr` (Wayland), then default.
fn get_linux_resolution() -> String {
    // Try xrandr (X11)
    if let Ok(out) = Command::new("sh")
        .args(["-c", "xrandr 2>/dev/null | grep ' connected primary' | grep -oP '\\d+x\\d+' | head -n1"])
        .output()
    {
        let res = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !res.is_empty() {
            return res;
        }
    }
    // Try xrandr fallback (any active output)
    if let Ok(out) = Command::new("sh")
        .args(["-c", "xrandr 2>/dev/null | grep '*' | awk '{print $1}' | head -n1"])
        .output()
    {
        let res = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !res.is_empty() {
            return res;
        }
    }
    // Try wlr-randr (Wayland wlroots compositors)
    if let Ok(out) = Command::new("sh")
        .args(["-c", "wlr-randr 2>/dev/null | grep -oP '\\d+x\\d+' | head -n1"])
        .output()
    {
        let res = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !res.is_empty() {
            return res;
        }
    }
    // Try kscreen-doctor (KDE Wayland)
    if let Ok(out) = Command::new("sh")
        .args(["-c", "kscreen-doctor -o 2>/dev/null | grep -oP '\\d+x\\d+' | head -n1"])
        .output()
    {
        let res = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !res.is_empty() {
            return res;
        }
    }
    "1920x1080".to_string()
}

fn simple_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn subtitle_burn() -> Result<(), Box<dyn std::error::Error>> {
    print_header("✍️ Subtitle Burn-in");
    let vid = prompt_file("Select video file")?;
    let sub = prompt_file("Select subtitle file (.srt/.ass)")?;

    let stem = vid.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
    let ext = vid.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    let out = vid.with_file_name(format!("{}_subtitled.{}", stem, ext));

    let sub_str = sub.to_string_lossy();
    let safe_sub = sub_str.replace('\'', "\\'").replace(':', "\\:");

    println!("\n⚡ Burning subtitle into video...");
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&vid)
        .arg("-vf")
        .arg(format!("subtitles='{}'", safe_sub))
        .arg("-c:a")
        .arg("copy")
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Subtitle burned into video: {}", out.display());
    } else {
        println!("❌ Subtitle burn-in failed.");
    }
    Ok(())
}

fn subtitle_extract() -> Result<(), Box<dyn std::error::Error>> {
    print_header("📄 Subtitle Extraction");
    let vid = prompt_file("Select video file")?;
    let stem = vid.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
    let out = vid.with_file_name(format!("{}.srt", stem));

    println!("\n⚡ Extracting subtitle track...");
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&vid)
        .arg("-map")
        .arg("0:s:0")
        .arg(&out)
        .status()?;

    if status.success() {
        println!("✅ Subtitle extracted to {}", out.display());
    } else {
        println!("❌ Subtitle extraction failed (no subtitle track found).");
    }
    Ok(())
}

// ── Helpers & Prompts ────────────────────────────────────────────────────────

fn print_header(title: &str) {
    println!("\x1b[1;35m🎬  FFMEDIA\x1b[0m — \x1b[1;36m{}\x1b[0m", title);
    println!("\x1b[90m─────────────────────────────────────────────────────────────\x1b[0m");
}

fn prompt_text(msg: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("\x1b[1;33m?\x1b[0m \x1b[1m{}\x1b[0m: ", msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_file(msg: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path_str = prompt_text(msg)?;
    let path = PathBuf::from(&path_str);
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()).into());
    }
    Ok(path)
}

fn prompt_select(msg: &str, options: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n\x1b[1;34m{}\x1b[0m", msg);
    for (i, opt) in options.iter().enumerate() {
        println!("  \x1b[36m{}\x1b[0m) {}", i + 1, opt);
    }
    print!("\x1b[1;33mSelect option\x1b[0m [1-{}]: ", options.len());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(1);
    let idx = choice.saturating_sub(1).min(options.len().saturating_sub(1));
    Ok(options[idx].to_string())
}

fn is_ffmpeg_installed() -> bool {
    crate::core::utils::cmd_exists("ffmpeg")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actions_list_non_empty() {
        assert_eq!(ALL_ACTIONS.len(), 16);
    }

    #[test]
    fn test_parse_action_arg() {
        assert_eq!(parse_action_arg("compress"), Some(1));
        assert_eq!(parse_action_arg("trim"), Some(2));
        assert_eq!(parse_action_arg("concat"), Some(3));
        assert_eq!(parse_action_arg("res"), Some(4));
        assert_eq!(parse_action_arg("speed"), Some(5));
        assert_eq!(parse_action_arg("rotate"), Some(6));
        assert_eq!(parse_action_arg("extract"), Some(7));
        assert_eq!(parse_action_arg("mute"), Some(8));
        assert_eq!(parse_action_arg("snap"), Some(9));
        assert_eq!(parse_action_arg("gif"), Some(10));
        assert_eq!(parse_action_arg("clean"), Some(11));
        assert_eq!(parse_action_arg("convert"), Some(12));
        assert_eq!(parse_action_arg("record"), Some(13));
        assert_eq!(parse_action_arg("burn"), Some(14));
        assert_eq!(parse_action_arg("sub"), Some(15));
    }
}
