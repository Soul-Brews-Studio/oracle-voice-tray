# Voice Tray v2 - Bugs and Fixes Documentation

This document captures the bugs encountered and fixes applied during Voice Tray v2 development, researched from claude-mem historical observations.

---

## Bug #1: Duplicate Tray Icon Bug

**Observation IDs:** 5241, 5242, 5243, 5245, 5246, 5250
**Type:** bugfix
**Date Fixed:** 2026-01-02

### Symptoms
- Two tray icons appeared in the macOS menu bar
- One icon was transparent/ghost icon that handled click events
- The other icon was non-functional

### Root Cause
**Dual tray initialization** - both `tauri.conf.json` configuration AND programmatic `TrayIconBuilder` code were creating separate tray instances.

This is a known Tauri 2.0 issue tracked as [tauri-apps/tauri#8982](https://github.com/tauri-apps/tauri/issues/8982).

The problematic configuration in `tauri.conf.json`:
```json
"trayIcon": {
  "iconPath": "icons/32x32.png",
  "iconAsTemplate": false
}
```

Combined with code-based initialization in `lib.rs`:
```rust
TrayIconBuilder::new()
```

### Fix Applied
**Remove the `trayIcon` section from `tauri.conf.json`** and keep only the code-based `TrayIconBuilder` initialization.

This same fix was previously applied to oracle-status-tray in commit `a6bc658b` (Issue #74).

**Rule:** Use either config OR code for tray initialization, never both.

---

## Bug #2: White Square / Black Square / Invisible Icon Bug

**Observation IDs:** 5390, 5404, 5420, 5423, 5431, 5435, 8491
**Type:** bugfix
**Date Fixed:** 2026-01-03

### Symptoms
- Tray icon appeared as a black square in the menu bar
- Or icon was completely invisible
- Or icon showed with a dark background instead of transparency

### Root Causes (Three Hidden Traps)

1. **JPEG disguised as PNG**: AI-generated icons from Antigravity/Gemini output JPEG image data with `.png` extension - the files claim to be PNG but contain JPEG data with baked-in dark backgrounds

2. **Checkered background illusion**: Image viewers show checkered patterns for "transparency" but the actual dark background was baked into the image data

3. **Palette-based PNG format**: PNG files using `PaletteAlpha` format fail to decode correctly in Tauri - requires `TrueColorAlpha` format

4. **Inverted alpha channel**: Initial ImageMagick brightness-to-alpha conversion was inverted - dark pixels became opaque while white lines became transparent (alpha mean 2.3/255, nearly invisible)

5. **Rust `png` crate limitation**: The `png` crate cannot automatically convert palette-based PNG to RGBA format

### Fix Applied

**ImageMagick Conversion Workflow:**
```bash
magick source.png \
  -fuzz 30% \
  -trim \
  -transparent "rgb(45,46,45)" \
  -resize 22x22 \
  -level 0%,40% \
  PNG32:output.png
```

Key elements:
- `-fuzz 30-40%`: Fuzzy matching to remove variations of dark background
- `-transparent "rgb(45,45,45)"`: Remove the dark background color
- `-level 0%,50%`: Brighten remaining pixels to white
- `PNG32:`: Force TrueColorAlpha format output
- `-trim`: Remove AI-generated padding

**Rust Code Changes:**
- Switched from `png` crate to `image` crate
- Use `to_rgba8()` for automatic format conversion
- Icons embedded at compile time using `include_bytes!` macro

**Icon Sizing:**
- macOS menu bar requires 22x22 pixel dimensions
- Multiple resolutions tested: 16x16 -> 32x32 -> 44x44 -> 64x64

### Verification
Use `file` command to verify actual image format:
```bash
file your-icon.png
```

Check alpha channel statistics:
```bash
identify -verbose icon.png | grep -A5 "Alpha:"
```

---

## Bug #3: Build Failures

**Observation IDs:** 5068, 5228, 5266
**Type:** bugfix
**Date:** 2026-01-02

### Issues Encountered

1. **Proc Macro Panic** (ID: 5266)
   - Build fails with "proc macro panicked" error
   - Compilation stopped preventing library build

2. **Permission Denied Error** (ID: 5228)
   - Shell evaluation error during rebuild
   - Occurred after tray icon configuration changes

3. **Compilation Error in lib.rs** (ID: 5068)
   - Unused variable warning in mqtt.rs
   - Unnecessary mut in lib.rs

### Resolution
- Fix code issues identified in warnings
- Ensure clean process termination before rebuild
- Use `bun tauri build` not just `cargo build` when icons change (include_bytes! caches at compile time)

---

## Bug #4: Icons Too Small in Menu Bar

**Observation IDs:** 5495, 5538, 5539, 5569
**Type:** change (iterative improvement)
**Date:** 2026-01-03

### Issue
Icons appeared too small or sparse in the macOS menu bar, especially on retina displays.

### Resolution
Progressive icon resolution increases:
1. 16x16 -> 32x32 (initial improvement)
2. 32x32 -> 44x44 (retina adjustment)
3. 44x44 -> 64x64 (final increase)

**Final recommendation:** Use 22x22 for macOS menu bar icons, or provide @2x versions.

---

## Related Projects

The duplicate tray icon bug was first encountered and fixed in **oracle-status-tray**:
- Commit: `a6bc658b`
- Issue: #74
- Date: 2025-12-30

The `/trace` command successfully located this previous fix in ~20 seconds via Oracle's context-finder.

---

## Key Learnings

1. **Tauri 2.0 tray initialization**: Use either config OR code, never both
2. **AI-generated icons**: Always verify actual format with `file` command
3. **PNG transparency**: Use PNG32 format flag for guaranteed TrueColorAlpha
4. **Rust image handling**: Use `image` crate over `png` crate for automatic format conversion
5. **macOS menu bar**: Icons should be 22x22 pixels for standard displays
6. **Compile-time assets**: Changes to `include_bytes!` assets require full Tauri rebuild

---

*Document generated from claude-mem observations on 2026-01-04*
