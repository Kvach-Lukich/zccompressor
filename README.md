# Zero-Cross Compressor

**Zero-Cross Compressor** is an offline audio compressor configured via an INI file, featuring multi-threaded batch processing for any audio format supported by **FFmpeg**.

The program analyzes the entire track, automatically calculates the compression depth to match a target **Dynamic Range (DR)** value, and performs final peak normalization without destroying transients.

## How It Works

The core algorithm operates sample-by-sample using **PCM zero-crossings**:

1. **Preserving Transients and Wave Structure:** The algorithm detects the points where the audio signal crosses the zero level and linearly scales the amplitude within the intervals between adjacent zero-crossings. Because the shape of individual half-waves and transients remains intact, **drum and percussion attacks are left completely untouched**.

2. **Automatic Compression Calibration:** Dynamic range is measured according to the **TT Dynamic Range** standard (similar to the `foo_dr_meter` plugin). The compressor calculates the track's original DR, compares it against the target value specified in the configuration, and attenuates only the peak half-waves until the desired DR is reached.

3. **Normalization:** After compression, the signal is normalized to a specified peak ceiling (default is `-0.3 dB`).

## Sound Character & Technical Notes

- **Moderate Compression:** Adds no coloration and never smears transients. The introduced nonlinear distortion is comparable to a Class-B amplifier, yet so minuscule that it sits below the noise floor of standard dithering.

- **Extreme Compression:** Deep attenuation of peak half-waves generates sub-bass/infrasonic frequencies (below `20 Hz`), similar to power supply sag in a Class-B amplifier under heavy load. When using aggressive DR targets, applying an additional High-Pass Filter (HPF) is recommended.

- **Recommended DR Values:** A target of `8–9 dB` (default is `9 dB`) yields a punchy yet lively sound, avoiding the over-compressed loudness of modern masters at `5–6 dB` that can hardly be called music at all.

## Quick Start

### 1. Configuration

Open the INI file located next to the executable and adjust the parameters:

- Target **DR** level (default: `9`);

- Peak normalization ceiling in dB (default: `-0.3`);

- Output directory for processed files (if left blank, an `Output` folder is automatically created next to the executable).

### 2. Processing Files

1. Select one or more audio files (not a folder) and drag and drop them directly onto the program's executable (**Drag & Drop**).

2. A terminal window will open and begin multi-threaded processing for all queued tracks.

3. Processed files will appear in your specified output directory.

## Requirements

- **[FFmpeg DLL libraries](https://github.com/System233/ffmpeg-msvc-prebuilt/releases/download/ffmpeg-9.1-dev-3237-ga6c7695/ffmpeg-9.1-dev-3237-ga6c7695_x64-windows-shared-gpl.zip)** placed in the same directory as the executable for audio decoding and encoding.
