use std::env;
use std::path::{Path,MAIN_SEPARATOR as SL};
use std::sync::{Arc, Mutex, OnceLock};
use std::error::Error;
use deser::{Deserialize, Serialize};
use std::fs;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::thread;

use ez_ffmpeg::frame_export::SampleExtractor;
use ez_ffmpeg::{FfmpegContext, FfmpegScheduler, Input, Output};

#[cfg(target_os = "macos")]
use crossterm::event::{self, Event, KeyCode};
#[cfg(target_os = "macos")]
use std::io::{self, Write};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
struct Cfg{
    DEFAULT_OUTPATH: String,
    #[deser(default = 9.0)]
    DR: f32,
    #[deser(default = -0.3)]
    NORMALISE: f32,
    #[deser(default = "wav".to_string())]
    OUTPUT_CODEC: String,
    #[deser(default = "s32".to_string())]
    OUTPUT_FORMAT: String,
    #[deser(default = "pcm_f32le".to_string())]
    OUTPUT_WAVFORMAT: String,
    #[deser(default = 44100)]
    OUTPUT_SAMPLERATE: i32,
    #[deser(default = 9.0)]
    OUTPUT_LOWCUT: f32,
    #[deser(default)]
    ADD_POSTFIXDR: bool,
    #[deser(default = 0)]
    INPUT_RESAMPLE:i32,
    #[deser(default = 20.0)]
    SEARCH_FREQ:f32,
    #[deser(default = "none".to_string())]
    DITHER_METHOD: String,
    #[deser(default = 3.0)]
    DR_WINDOW: f32,
}

#[cfg(target_os = "macos")]
fn wait_any_key(message: &str) -> io::Result<()> {
    print!("{}", message);
    io::stdout().flush()?; 
    crossterm::terminal::enable_raw_mode()?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.code != KeyCode::Null {
                break;
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    println!();
    Ok(())
}

fn winstop(message:&str)->!{
    #[cfg(target_os = "windows")]
    press_btn_continue::wait(message).unwrap();
    #[cfg(target_os = "macos")]
    wait_any_key(message).unwrap();
    std::process::exit(1);
}

static CFG: OnceLock<Cfg> = OnceLock::new();

fn cfg() -> &'static Cfg {
    CFG.get().expect("Config is not initialized")
}

fn getcfg()->Cfg{
    let ini_content = fs::read_to_string(env::current_exe().unwrap().parent().unwrap().join("config.ini")).expect("Config didn't loaded");
    let mut cfg: Cfg = deser_ini::from_str(&ini_content).expect("Config didn't parsed");
    if cfg.OUTPUT_CODEC=="wavpack" { cfg.OUTPUT_CODEC="wv".to_string(); }
    cfg
}


fn getargs() -> Vec<String> {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        winstop("No input files");
    }
    paths
}

fn main() -> Result<(), Box<dyn Error>> {
    CFG.set(getcfg()).expect("Config already initialized");
    let files=getargs();
    let threads = thread::available_parallelism().map(|n| n.get()).unwrap_or(4);

    let out_dir = if cfg().DEFAULT_OUTPATH.is_empty() || !Path::new(&cfg().DEFAULT_OUTPATH).exists(){
        let dir = env::current_exe().unwrap().parent().unwrap().join("Output");
        fs::create_dir_all(&dir).expect("cannot create Output directory");
        dir.to_string_lossy().into_owned()
    } else {
        cfg().DEFAULT_OUTPATH.clone()
    };

    if files.len()>threads{
        let pool = ThreadPoolBuilder::new().num_threads(threads).build().expect("Failed to create rayon thread pool");

        pool.install(|| {
            files.par_iter().for_each(|file| {
                if let Err(e) = compressor(file, &out_dir) {
                    eprintln!("Error processing {file}: {e}");
                }
            });
        });
    }else if files.len()>1{
        files.par_iter().for_each(|file| {
            if let Err(e) = compressor(file, &out_dir) {
                eprintln!("Error processing {file}: {e}");
            }
        });
    }else if files.len()==1{
        if let Err(e) = compressor(&files[0], &out_dir) {
            eprintln!("Error processing {}: {e}", files[0]);
        }
    }
    #[cfg(target_os = "windows")]
    press_btn_continue::wait(r"Press any key...").unwrap();
    #[cfg(target_os = "macos")]
    wait_any_key("Press Enter...").unwrap();
    Ok(())
}

fn compressor(file: &String, out_dir:&String)-> Result<(), Box<dyn std::error::Error>>{
    let extractor = SampleExtractor::new(file.as_str());
    let audio_data = extractor.collect_audio()?;

    let nchannels: usize = audio_data.channels() as usize;
    let channel_layout =audio_data.channel_layout().to_string();
    let sample_rate =audio_data.sample_rate();
    let slen: usize = audio_data.as_slice().len() / nchannels;
    let sslen: usize=(sample_rate as f32/cfg().SEARCH_FREQ).ceil() as usize;
    let normalise:f32=10.0_f32.powf(cfg().NORMALISE / 20.0);

    let mut channel_streams: Vec<Vec<f32>> = vec![Vec::with_capacity(slen); nchannels];


    if nchannels>1 {
        for (i, &sample) in audio_data.as_slice().iter().enumerate() {
            let channel_idx = i % nchannels;
            channel_streams[channel_idx].push(sample);
        }
    }else{
        channel_streams[0].extend_from_slice(audio_data.as_slice());
    }
    drop(audio_data);

    let channel_dr=getdr(&channel_streams, sample_rate);
    if channel_dr.is_empty() {
        return Ok(());
    }
    let avgdr=*channel_dr.last().unwrap();
    if avgdr<=cfg().DR{
        println!("To compressed audio!");
    }

    println!("DR: {:.2}",avgdr);
    println!("Compress...");

    let newpeak:f32=10.0_f32.powf((cfg().DR-avgdr) / 20.0);

    for channel in channel_streams.iter_mut() {
        let mut i: usize = sslen;
        let rsize=channel.len()-sslen;
        while i<rsize {
                        
            if channel[i].abs()>=newpeak {
                let p: f32 = if channel[i] > 0.0 { 1.0 } else { -1.0 };
                let mut j: usize = i;
                let mut curmax:f32=0.0;
                let mut rzeroindex:usize=0;
                while j<channel.len()-1 && i+sslen>j {
                    curmax=curmax.max(channel[j].abs());
                    if  channel[j]*p>=0.0 && channel[j+1]*p<0.0{
                        rzeroindex=j;
                        break;
                    }
                    j+=1;
                }

                let mut l: usize = i;
                let mut lzeroindex:usize=0;
                while l>1 && i-sslen<l {
                    if channel[l]*p>=0.0 && channel[l-1]*p<0.0{
                        lzeroindex=l;
                        break;
                    }
                    l-=1;
                }
                if rzeroindex>0 && rzeroindex<channel.len() && lzeroindex>0 && lzeroindex<channel.len() && curmax>newpeak{
                    let rdivider:f32=newpeak/curmax;
                    channel[lzeroindex..=rzeroindex].iter_mut().for_each(|s| *s *= rdivider);
                    i=rzeroindex;
                }

            }
            i+=1;
        }
    }

    let channel_dr=getdr(&channel_streams, sample_rate);
    println!("DR: {:.2}",channel_dr.last().unwrap());

    let newpeak=normalise/newpeak;    
    let mut interleaved: Vec<f32> = Vec::with_capacity(slen * nchannels);
    for i in 0..slen {
        for ch in 0..nchannels {
            interleaved.push(channel_streams[ch][i]*newpeak);
        }
    }

    drop(channel_streams);
    println!("Convert and save...");
    let filename=Path::new(file).file_stem().unwrap_or_default().to_string_lossy();
    let mut out_path=format!("{}{SL}{}.{}",out_dir, filename, cfg().OUTPUT_CODEC);
    if cfg().ADD_POSTFIXDR || Path::new(&out_path).exists(){
        out_path=format!("{}{SL}{}_dr{}.{}",out_dir, filename,cfg().DR ,cfg().OUTPUT_CODEC);
    }
    save_pcm_f32_to_wav_in_memory(
        &interleaved,
        sample_rate,
        nchannels as u16,
        channel_layout,
        out_path.as_str(),
    )?;
    Ok(())
}

fn getdr(channel_streams: &[Vec<f32>], sample_rate: u32) ->Vec<f32>{
    let nchannels=channel_streams.len();
    
    let slen: usize = channel_streams[0].len();
    let sslen: usize=(sample_rate as f32/cfg().SEARCH_FREQ).ceil() as usize;

    if slen<2*sslen{
        println!("To short audio!");
        return Vec::new();
    }
    
    let trimin = (sample_rate as f32*cfg().DR_WINDOW).round() as usize;
    let triminslicelen: usize =(slen+trimin-1)/trimin;
    let mut channel_rms: Vec<f32> = Vec::with_capacity(nchannels);
    let mut channel_dr: Vec<f32> = Vec::with_capacity(nchannels+1);
    let mut truepeak: f32=0.0;
    let mut avgdr: f32=0.0;
    for channel in channel_streams.iter() {
        let mut rmsall: Vec<f32> = Vec::with_capacity(triminslicelen);
        let mut smax: Vec<f32> = Vec::with_capacity(triminslicelen);
        let mut rmsfinal: f32 = 0.0;
        let mut max1:f32 = 0.0;
        let mut max2:f32 = 0.0;

        for chunk in channel.chunks(trimin) {
            let mut rmsch: f32 = 0.0;
            let mut rmax: f32=0.0;
            for sample in chunk.iter() {
                rmsch+=sample*sample;
                rmax=rmax.max(sample.abs());
            }
            rmsall.push( (rmsch/ chunk.len() as f32 ).sqrt() as f32 );
            smax.push(rmax);
        }

        rmsall.sort_unstable_by(|a, b| b.total_cmp(a));
        let top_20_count = (rmsall.len() as f32 * 0.2).ceil() as usize;
        rmsall.truncate(top_20_count);
        for rmstop in rmsall.iter(){
            rmsfinal+=rmstop*rmstop;
        }
        rmsfinal=((rmsfinal/top_20_count as f32)*2.0).sqrt();
        channel_rms.push(rmsfinal);

        for &val in &smax {
            if val > max1 {
                max2 = max1;
                max1 = val;
            } else if val > max2 && val < max1 {
                max2 = val;
            }
        }
        truepeak=truepeak.max(max1);

        let dr = if rmsfinal > 0.0 && max2 > 0.0 {
            20.0 * (max2 / rmsfinal).log10()
        } else {
            0.0
        };
        channel_dr.push(dr);
        avgdr+=dr;
    }
    avgdr=avgdr/nchannels as f32;
    channel_dr.push(avgdr);
    channel_dr
}

use ffmpeg_sys_next::AVERROR_EOF;
fn save_pcm_f32_to_wav_in_memory(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    ch_layout: String,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut raw_u8 = Vec::with_capacity(samples.len() * 4);
    for &s in samples {
        raw_u8.extend_from_slice(&s.to_le_bytes());
    }
    
    // 2. Общее состояние: (буфер, текущая позиция)
    let state = Arc::new(Mutex::new((raw_u8, 0usize)));

    // 3. Read callback (без изменений, но использует общее состояние)
    let state_read = Arc::clone(&state);
    let read_callback = move |buf: &mut [u8]| -> i32 {
        let mut guard = state_read.lock().unwrap();
        let (data, pos) = &mut *guard;

        if *pos >= data.len() {
            return AVERROR_EOF;
        }

        let to_copy = buf.len().min(data.len() - *pos);
        buf[..to_copy].copy_from_slice(&data[*pos..*pos + to_copy]);
        *pos += to_copy;
        to_copy as i32
    };

    // 4. Seek callback — вот чего не хватало
    let state_seek = Arc::clone(&state);
    let seek_callback = move |offset: i64, whence: i32| -> i64 {
        let mut guard = state_seek.lock().unwrap();
        let (data, pos) = &mut *guard;

        match whence {
            // Запрос общего размера потока в байтах
            ffmpeg_sys_next::AVSEEK_SIZE => data.len() as i64,

            // Установка абсолютной позиции
            ffmpeg_sys_next::SEEK_SET => {
                if offset < 0 {
                    return ffmpeg_sys_next::AVERROR(ffmpeg_sys_next::EINVAL) as i64;
                }
                let new_pos = offset as usize;
                if new_pos > data.len() {
                    return ffmpeg_sys_next::AVERROR(ffmpeg_sys_next::EINVAL) as i64;
                }
                *pos = new_pos;
                new_pos as i64
            }

            // Смещение относительно текущей позиции
            ffmpeg_sys_next::SEEK_CUR => {
                let current = *pos as i64;
                let new_pos = current.saturating_add(offset);
                if new_pos < 0 || new_pos as usize > data.len() {
                    return ffmpeg_sys_next::AVERROR(ffmpeg_sys_next::EINVAL) as i64;
                }
                *pos = new_pos as usize;
                new_pos
            }

            // Смещение относительно конца
            ffmpeg_sys_next::SEEK_END => {
                let end = data.len() as i64;
                let new_pos = end.saturating_add(offset);
                if new_pos < 0 || new_pos as usize > data.len() {
                    return ffmpeg_sys_next::AVERROR(ffmpeg_sys_next::EINVAL) as i64;
                }
                *pos = new_pos as usize;
                new_pos
            }

            _ => ffmpeg_sys_next::AVERROR(ffmpeg_sys_next::EINVAL) as i64,
        }
    };

    // 5. Собираем Input с обоими callback-ами
    let input = Input::new_by_read_callback(read_callback)
        .set_seek_callback(seek_callback) //Important
        .set_format("f32le")
        .set_format_opt("sample_rate", sample_rate.to_string())
        .set_format_opt("channels", channels.to_string())
        .set_format_opt("ch_layout", ch_layout);

    let mut output = Output::new(output_path).set_format(&cfg().OUTPUT_CODEC);
    
    let mut filter:String=String::new();
    if cfg().OUTPUT_LOWCUT>0.0 {
        let f=cfg().OUTPUT_LOWCUT;
        filter=format!("highpass=f={f}, areverse,highpass=f={f}, areverse,highpass=f={f}, areverse,highpass=f={f}, areverse");
    }
    if cfg().OUTPUT_SAMPLERATE>0{
        if !filter.is_empty() { filter.push(','); }
        filter=format!("{} aresample=resampler=soxr:osr={}:precision=28:dither_method={}", filter, cfg().OUTPUT_SAMPLERATE, cfg().DITHER_METHOD);
        output=output.set_audio_sample_rate(cfg().OUTPUT_SAMPLERATE);
    }
    if !filter.is_empty() {
        output=output.set_audio_filter(filter);
    }
    match cfg().OUTPUT_CODEC.as_str(){
        "wav" => output=output.set_audio_codec(&cfg().OUTPUT_WAVFORMAT),
        "flac" => {
            match cfg().OUTPUT_FORMAT.as_str(){
                "s24" => output=output.set_audio_codec("flac").set_audio_sample_fmt("s32"),
                "s32" => output=output.set_audio_codec("flac").set_audio_sample_fmt("s32").set_audio_codec_opt("bits_per_raw_sample", "32").set_audio_codec_opt("strict", "experimental"),
                "s16" => output=output.set_audio_codec("flac").set_audio_sample_fmt("s16"),
                _=> winstop("Unsupported bit depth"),
            }
           
        },
        "wv" =>{
            match cfg().OUTPUT_FORMAT.as_str(){
                "s24" => output=output.set_audio_codec("wavpack").set_audio_sample_fmt("s32p").set_bits_per_raw_sample(24).set_audio_codec_opt("bits_per_raw_sample", "24").set_audio_codec_opt("compression_level", "3"),
                "s32" => output=output.set_audio_codec("wavpack").set_audio_sample_fmt("s32p").set_audio_codec_opt("compression_level", "3"),
                "f32" => output=output.set_audio_codec("wavpack").set_audio_sample_fmt("fltp").set_audio_codec_opt("compression_level", "3"),
                "s16" => output=output.set_audio_codec("wavpack").set_audio_sample_fmt("s16p").set_audio_codec_opt("compression_level", "3"),
                _=> winstop("Unsupported bit depth"),
            }
        },
        _=> winstop("Wrong codec"),
        
    }

    let context = FfmpegContext::builder()
        .input(input)
        .output(output)
        .build()?;

    FfmpegScheduler::new(context).start()?.wait()?;
    Ok(())
}