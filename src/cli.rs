use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "minimax")]
#[command(about = "MiniMax AI CLI - Text-to-speech, video, image, music generation", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert text to audio with a given voice
    TextToAudio {
        /// The text to convert to speech
        #[arg(long)]
        text: String,
        /// Voice ID to use (default: female-shaonv)
        #[arg(long, default_value = "female-shaonv")]
        voice_id: String,
        /// Model to use (speech-2.6-hd)
        #[arg(long, default_value = "speech-2.6-hd")]
        model: String,
        /// Speech speed (0.5-2.0, default: 1.0)
        #[arg(long, default_value_t = 1.0)]
        speed: f32,
        /// Volume (0-10, default: 1.0)
        #[arg(long, default_value_t = 1.0)]
        vol: f32,
        /// Pitch (-12 to 12, default: 0)
        #[arg(long, default_value_t = 0)]
        pitch: i32,
        /// Emotion (happy, sad, angry, fearful, disgusted, surprised, neutral)
        #[arg(long, default_value = "happy")]
        emotion: String,
        /// Sample rate (8000, 16000, 22050, 24000, 32000, 44100)
        #[arg(long, default_value_t = 32000)]
        sample_rate: i32,
        /// Bitrate (32000, 64000, 128000, 256000)
        #[arg(long, default_value_t = 128000)]
        bitrate: i32,
        /// Channel (1 or 2)
        #[arg(long, default_value_t = 1)]
        channel: i32,
        /// Format (pcm, mp3, flac)
        #[arg(long, default_value = "mp3")]
        format: String,
        /// Language boost (Chinese, Chinese,Yue, English, Arabic, Russian, Spanish, French, Portuguese, German, Turkish, Dutch, Ukrainian, Vietnamese, Indonesian, Japanese, Italian, Korean, Thai, Polish, Romanian, Greek, Czech, Finnish, Hindi, auto)
        #[arg(long, default_value = "auto")]
        language_boost: String,
        /// Output directory
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    /// List all available voices
    ListVoices {
        /// Voice type filter (all, system, voice_cloning)
        #[arg(long, default_value = "all")]
        voice_type: String,
    },
    /// Clone a voice from audio file
    VoiceClone {
        /// Voice ID for the clone
        #[arg(long)]
        voice_id: String,
        /// Audio file path or URL
        #[arg(long)]
        file: String,
        /// Optional text for demo audio
        #[arg(long)]
        text: Option<String>,
        /// Is the file a URL?
        #[arg(long, default_value_t = false)]
        is_url: bool,
        /// Output directory
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    /// Generate video from text/image prompt
    GenerateVideo {
        /// The prompt to generate video from
        #[arg(long)]
        prompt: String,
        /// Model to use (T2V-01, MiniMax-Hailuo-02, etc.)
        #[arg(long, default_value = "MiniMax-Hailuo-2.3")]
        model: String,
        /// First frame image (for I2V models)
        #[arg(long)]
        first_frame_image: Option<String>,
        /// Duration (6 or 10 seconds, for Hailuo-02)
        #[arg(long)]
        duration: Option<i32>,
        /// Resolution (768P or 1080P, for Hailuo-02)
        #[arg(long)]
        resolution: Option<String>,
        /// Async mode: if true, return task_id immediately; if false, wait for completion
        #[arg(long, default_value_t = true)]
        async_mode: bool,
        /// Output directory
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    /// Query task status
    QueryTask {
        /// Task ID to query
        #[arg(long)]
        task_id: String,
        /// Output directory (if provided, will auto-download completed video to this directory)
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    /// Download completed task
    DownloadTask {
        /// Task ID to download
        #[arg(long)]
        task_id: String,
        /// Output directory
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    /// List all tasks
    ListTasks {
        /// Filter by status (pending, processing, success, fail)
        #[arg(long)]
        status: Option<String>,
        /// Maximum number of tasks to show
        #[arg(long, default_value_t = 50)]
        limit: i32,
    },
    /// Generate image from text
    TextToImage {
        /// The prompt to generate image from
        #[arg(long)]
        prompt: String,
        /// Model to use (image-01)
        #[arg(long, default_value = "image-01")]
        model: String,
        /// Aspect ratio (1:1, 16:9, 4:3, 3:2, 2:3, 3:4, 9:16, 21:9)
        #[arg(long, default_value = "1:1")]
        aspect_ratio: String,
        /// Number of images to generate (1-9)
        #[arg(long, default_value_t = 1)]
        n: i32,
        /// Enable prompt optimizer (default: true)
        #[arg(long, default_value_t = true)]
        prompt_optimizer: bool,
        /// Output directory
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    /// Generate music from prompt and lyrics
    MusicGeneration {
        /// Music style prompt
        #[arg(long)]
        prompt: String,
        /// Song lyrics (use \n for newlines)
        #[arg(long)]
        lyrics: String,
        /// Output format (mp3, wav, pcm)
        #[arg(long, default_value = "mp3")]
        format: String,
        /// Sample rate (16000, 24000, 32000, 44100)
        #[arg(long, default_value_t = 32000)]
        sample_rate: i32,
        /// Bitrate (32000, 64000, 128000, 256000)
        #[arg(long, default_value_t = 128000)]
        bitrate: i32,
        /// Output directory
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    /// Design a custom voice
    VoiceDesign {
        /// Voice description prompt
        #[arg(long)]
        prompt: String,
        /// Preview text
        #[arg(long)]
        preview_text: String,
        /// Optional voice ID
        #[arg(long)]
        voice_id: Option<String>,
        /// Output directory
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set the API key
    SetApiKey {
        #[arg(long)]
        key: String,
    },
    /// Set the API host
    SetApiHost {
        #[arg(long, default_value = "https://api.minimax.io")]
        host: String,
    },
    /// Show current configuration
    Show,
    /// Interactive configuration wizard
    Init,
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let config = crate::config::Config::load()?;

    match cli.command {
        Commands::TextToAudio {
            text,
            voice_id,
            model,
            speed,
            vol,
            pitch,
            emotion,
            sample_rate,
            bitrate,
            channel,
            format,
            language_boost,
            output_dir,
        } => {
            crate::commands::text_to_audio::run(
                &config,
                &text,
                &voice_id,
                &model,
                speed,
                vol,
                pitch,
                &emotion,
                sample_rate,
                bitrate,
                channel,
                &format,
                &language_boost,
                output_dir,
            )
            .await
        }
        Commands::ListVoices { voice_type } => {
            crate::commands::list_voices::run(&config, &voice_type).await
        }
        Commands::VoiceClone {
            voice_id,
            file,
            text,
            is_url,
            output_dir,
        } => {
            crate::commands::voice_clone::run(
                &config,
                &voice_id,
                &file,
                text.as_deref(),
                is_url,
                output_dir,
            )
            .await
        }
        Commands::GenerateVideo {
            prompt,
            model,
            first_frame_image,
            duration,
            resolution,
            async_mode,
            output_dir,
        } => {
            crate::commands::generate_video::run(
                &config,
                &prompt,
                &model,
                first_frame_image,
                duration,
                resolution,
                async_mode,
                output_dir,
            )
            .await
        }
        Commands::QueryTask {
            task_id,
            output_dir,
        } => crate::commands::query_task::run(&config, &task_id, output_dir).await,
        Commands::DownloadTask {
            task_id,
            output_dir,
        } => crate::commands::download_task::run(&config, &task_id, output_dir).await,
        Commands::ListTasks { status, limit } => {
            crate::commands::list_tasks::run(&config, status.as_deref(), limit).await
        }
        Commands::TextToImage {
            prompt,
            model,
            aspect_ratio,
            n,
            prompt_optimizer,
            output_dir,
        } => {
            crate::commands::text_to_image::run(
                &config,
                &prompt,
                &model,
                &aspect_ratio,
                n,
                prompt_optimizer,
                output_dir,
            )
            .await
        }
        Commands::MusicGeneration {
            prompt,
            lyrics,
            format,
            sample_rate,
            bitrate,
            output_dir,
        } => {
            crate::commands::music_generation::run(
                &config,
                &prompt,
                &lyrics,
                &format,
                sample_rate,
                bitrate,
                output_dir,
            )
            .await
        }
        Commands::VoiceDesign {
            prompt,
            preview_text,
            voice_id,
            output_dir,
        } => {
            crate::commands::voice_design::run(
                &config,
                &prompt,
                &preview_text,
                voice_id.as_deref(),
                output_dir,
            )
            .await
        }
        Commands::Config { command } => crate::commands::config::run(&command).await,
    }
}
