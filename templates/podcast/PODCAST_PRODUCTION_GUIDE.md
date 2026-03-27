---
title: "Podcast Production Guide: [Podcast Name]"
last_updated: YYYY-MM-DD
---

# Podcast Production Guide

> End-to-end workflow from recording to published episode.

## Equipment

| Item | Current | Minimum Acceptable | Notes |
|---|---|---|---|
| Microphone | [model] | [USB condenser minimum] | |
| Headphones | [model] | [closed-back, any] | Required during recording |
| Audio interface | [model or "USB direct"] | | |
| Pop filter | [yes/no] | Recommended | |
| Acoustic treatment | [description] | [quiet room minimum] | |
| Recording software | [tool] | | |
| Remote recording tool | [Riverside / Zencastr / Zoom] | | |

## Recording Workflow

### Pre-Recording (15 minutes before)
1. Close unnecessary applications (reduce CPU noise/interference)
2. Set audio input to correct microphone
3. Test audio levels — speak at normal volume, peaks at -12 to -6 dB
4. Record 10 seconds of room tone (silence)
5. Confirm backup recording is running
6. For remote guests: confirm their audio setup, help troubleshoot

### During Recording
1. Clap once at the start (sync marker if multi-track)
2. If someone makes a mistake, pause, say "pickup," and restart the sentence
3. Avoid crosstalk (especially important for remote recordings)
4. Monitor levels throughout
5. If recording locally: save every 15 minutes

### Post-Recording
1. Save all raw files immediately
2. Label: `[EP-XXX]_[topic]_raw_[date].[ext]`
3. Note timestamps of any issues, great moments, or sections to cut

## Editing Workflow

| Step | Tool | Time Estimate | Notes |
|---|---|---|---|
| 1. Import and organize tracks | [DAW] | [5 min] | Align multi-track if needed |
| 2. Remove long pauses and false starts | [DAW] | [15-30 min] | Keep natural pauses; remove >3 second gaps |
| 3. Remove filler words (optional) | [DAW / Descript] | [15-30 min] | Remove excessive "um/uh" but keep some for naturalness |
| 4. EQ and compression | [DAW] | [10 min] | Preset for voice: high-pass at 80Hz, gentle compression |
| 5. Noise reduction | [DAW / iZotope] | [5 min] | Use room tone sample |
| 6. Add intro/outro music | [DAW] | [5 min] | Fade under host voice |
| 7. Add mid-roll markers | [DAW] | [2 min] | For dynamic ad insertion if used |
| 8. Normalize loudness | [DAW] | [2 min] | Target: -16 LUFS stereo, -19 LUFS mono |
| 9. Export | [DAW] | [2 min] | MP3 128kbps stereo or 64kbps mono |
| 10. Listen to final export | — | [episode length] | Full playthrough on different device |

## Publishing Workflow

1. Upload to hosting platform
2. Set episode title, description, and show notes
3. Add episode artwork (if per-episode)
4. Set publish date/time
5. Verify RSS feed updates
6. Check episode appears on all platforms (within 24h)
7. Execute distribution checklist (see `DISTRIBUTION_PLAN.md`)

## File Organization

```
/srv/sync/<domain>/publishing/podcast/
  episodes/
    EP-001_[topic]/
      raw/                    # Raw recordings
      edit/                   # Editing project files
      final/                  # Published audio file
      assets/                 # Audiograms, clips, artwork
      show-notes.md           # Episode show notes
      episode-plan.md         # Episode plan (from template)
    EP-002_[topic]/
      ...
```

## Template Version

- **Version:** 1.0
- **Created:** 2026-03-25
- **Last Updated:** 2026-03-25
