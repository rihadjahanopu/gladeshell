# 🎬 Multimedia Suite (`ffmedia` & `video_player`)

- **Primary Sources:**
  - FFmpeg Suite: [`src/tools/ffmedia.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/ffmedia.rs) (2,469 LoC)
  - Video Player: [`src/tools/video_player.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/video_player.rs) (556 LoC)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Role

`ffmedia` provides a **24-in-1 multimedia processing engine** directly from the terminal, wrapping FFmpeg with an intuitive, interactive TUI and command dispatcher.

### Key Capabilities:
- **Video Compression:** Target file size (e.g., 25MB for Discord/email) with two-pass H.264/H.265 bitrate calculation.
- **Audio Extraction:** High-speed audio stream ripping (`mp3`, `aac`, `flac`, `wav`, `ogg`).
- **GIF Generation:** High-fidelity animated GIFs with custom palette generation (`palettegen` / `paletteuse`).
- **Media Trimming & Cutting:** Lossless stream-copy trimming (`-c copy`) without re-encoding delays.
- **Metadata Sanitization:** Strips camera EXIF, GPS location coordinates, and creation timestamps for privacy.

---

## 2. 📺 Terminal Video Player (`video_player.rs`)

`video_player` allows playing audio and video directly inside the terminal:
- Renders video frames into high-speed ANSI true-color blocks (`\x1b[48;2;R;G;Bm `) or braille characters.
- Synchronizes audio output via cross-platform system audio backends.

---

## 3. 💻 CLI Usage

```bash
# Launch interactive FFmpeg 24-in-1 wizard
ffmedia

# Compress video to under 25MB (Discord/Slack limit)
ffmedia compress input.mp4 25M

# Extract audio losslessly
ffmedia extract-audio video.mkv audio.mp3

# Convert video segment into high-quality GIF
ffmedia gif video.mp4 --start 00:01:20 --duration 5 output.gif

# Strip metadata for privacy
ffmedia strip-metadata photo_or_video.mp4
```
