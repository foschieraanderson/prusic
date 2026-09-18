use crate::player::AudioPlayer;
use base64::{Engine, engine::general_purpose::STANDARD};
use crossterm::terminal;
use image::{ImageFormat, imageops::FilterType};
use ratatui::layout::Rect;
use std::{
    io::{self, Cursor, Write},
    path::Path,
    time::Duration,
};

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();

    let minutes = seconds / 60;
    let seconds = seconds % 60;

    format!("{minutes:02}:{seconds:02}")
}

pub fn print_help() {
    println!();
    println!("Controles:");
    println!("  p       Play / Pause");
    println!("  s       Stop");
    println!("  n       Próxima");
    println!("  b       Anterior");
    println!("  +       Aumentar volume");
    println!("  -       Diminuir volume");
    println!("  h       Ajuda");
    println!("  q       Sair");
    println!();
}

fn jpeg_to_png(data: &[u8]) -> io::Result<Vec<u8>> {
    let image =
        image::load_from_memory(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut png = Cursor::new(Vec::new());

    image
        .write_to(&mut png, ImageFormat::Png)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(png.into_inner())
}

pub fn show_image(data: &[u8], area: Rect) -> io::Result<()> {
    clear_image()?;

    let png = jpeg_to_png(data)?;

    let image = image::load_from_memory(&png).map_err(io::Error::other)?;

    // Tamanho real da janela do terminal.
    let window = terminal::window_size()?;

    let cell_width = window.width as u32 / window.columns as u32;

    let cell_height = window.height as u32 / window.rows as u32;

    // Tamanho disponível para a imagem em pixels.
    let available_width = area.width as u32 * cell_width;

    let available_height = area.height as u32 * cell_height;

    // A capa será sempre quadrada.
    let size = available_width.min(available_height);

    if size == 0 {
        return Ok(());
    }

    // Redimensiona preservando a proporção.
    let image = image.resize_to_fill(size, size, FilterType::Lanczos3);

    // Converte novamente para PNG.
    let mut png_data = Vec::new();

    {
        let mut cursor = std::io::Cursor::new(&mut png_data);

        image
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(io::Error::other)?;
    }

    // Quantas células a imagem ocupará.
    let image_columns = (size as f32 / cell_width as f32).ceil() as u16;

    let image_rows = (size as f32 / cell_height as f32).ceil() as u16;

    // Centraliza dentro do Rect.
    let x = area.x + area.width.saturating_sub(image_columns) / 2;

    let y = area.y + area.height.saturating_sub(image_rows) / 2;

    let mut stdout = io::stdout();

    // Move o cursor para o canto superior esquerdo da imagem.
    write!(stdout, "\x1b[{};{}H", y + 1, x + 1)?;

    // Kitty Graphics Protocol.
    let encoded = STANDARD.encode(&png_data);

    let chunk_size = 4096;

    for (i, chunk) in encoded.as_bytes().chunks(chunk_size).enumerate() {
        let more = if i + 1 < encoded.len().div_ceil(chunk_size) {
            1
        } else {
            0
        };

        write!(
            stdout,
            "\x1b_Ga=T,f=100,t=d,s={},v={},m={};{}\x1b\\",
            size,
            size,
            more,
            std::str::from_utf8(chunk).map_err(io::Error::other)?
        )?;
    }

    stdout.flush()?;

    Ok(())
}

pub fn clear_image() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Kitty Graphics Protocol:
    // a=d -> delete
    // d=A -> delete all images
    write!(stdout, "\x1b_Ga=d,d=A\x1b\\")?;

    stdout.flush()?;

    Ok(())
}

pub fn is_audio_file(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
        return false;
    };

    matches!(
        extension.to_lowercase().as_str(),
        "mp3" | "wav" | "flac" | "ogg" | "oga" | "m4a" | "aac"
    )
}
