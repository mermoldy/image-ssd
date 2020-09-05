use crate::error;
use std::fs;
use std::io;
use std::io::Write;
use std::path;

// TODO: add multiple mirrors
const SSD_V2_SOURCE_URL: &str = "http://download.tensorflow.org/models/object_detection/ssd_mobilenet_v2_coco_2018_03_29.tar.gz";
const SSD_V2_SOURCE_SHA256: &str =
    "b9380178b2e35333f1a735e39745928488bdabeb9ed20bc6fa07af8172cb5adc";

fn ssd_mobilenet_v2_files() -> Result<(path::PathBuf, path::PathBuf, path::PathBuf), error::Error> {
    let home_dir = dirs::home_dir().ok_or_else(|| "Impossible to get your home dir!")?;

    let archive_file_name = path::Path::new(SSD_V2_SOURCE_URL)
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Failed to parse file name")?;
    let archive_file_stem = archive_file_name
        .split('.')
        .next()
        .ok_or_else(|| "Failed to parse file stem")?;

    let ssd_base_dir = home_dir.join(".cache/image_ssd/models");
    let ssd_archive = ssd_base_dir.join(archive_file_name);
    let ssd_graph = ssd_base_dir
        .join(archive_file_stem)
        .join("frozen_inference_graph.pb");

    Ok((ssd_base_dir, ssd_archive, ssd_graph))
}

// Returns a path to the `frozen_inference_graph.pb` file located in the cache.
//
// The models cache is located inside of the `~/.cache/image_ssd/models` directory.
pub fn get_ssd_mobilenet_v2_graph() -> Result<path::PathBuf, error::Error> {
    let (_, _, ssd_graph) = ssd_mobilenet_v2_files()?;

    if !ssd_graph.exists() {
        Err(error::Error::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "MobileNet SSD graph file not found by path \"{}\". Please consider to download it manually from \"{}\".",
                ssd_graph.display(), SSD_V2_SOURCE_URL
            ),
        )))
    } else {
        Ok(ssd_graph)
    }
}

// Returns a path to the `frozen_inference_graph.pb` file and download it if a file not exist.
pub fn get_or_load_ssd_mobilenet_v2_graph() -> Result<path::PathBuf, error::Error> {
    let (ssd_base_dir, ssd_archive, ssd_graph) = ssd_mobilenet_v2_files()?;

    if !ssd_graph.exists() {
        fs::create_dir_all(&ssd_base_dir)?;

        info!("Downloading {}...", ssd_archive.display());
        download_file(&ssd_archive, SSD_V2_SOURCE_URL, SSD_V2_SOURCE_SHA256)?;

        info!("Unpacking {}...", ssd_archive.display());
        unpack_tar(&ssd_archive, &ssd_base_dir)?;

        info!("Download complete.");
        fs::remove_file(ssd_archive)?;
    }

    if !ssd_graph.exists() {
        Err(error::Error::IoError(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not find \"{}\" file.", ssd_graph.display()),
        )))
    } else {
        Ok(ssd_graph)
    }
}

// Downloads file into `file_path`.
fn download_file(
    file_path: &path::PathBuf,
    url: &str,
    sha265sum: &str,
) -> Result<(), error::Error> {
    let response = minreq::get(url).send()?;
    let mut file = fs::File::create(&file_path)?;
    file.write_all(response.as_bytes())?;

    let ssd_archive_sha256 =
        checksums::hash_file(&file_path, checksums::Algorithm::SHA2256).to_lowercase();
    if ssd_archive_sha256 != sha265sum {
        return Err(error::Error::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "The checksum of the file ({}) does not match the expected checksum value ({}).",
                ssd_archive_sha256, sha265sum
            ),
        )));
    }

    Ok(())
}

// Unpacks Tar archive into `base_dir`.
fn unpack_tar(file_path: &path::PathBuf, base_dir: &path::Path) -> Result<(), error::Error> {
    let tar_decoder = flate2::read::GzDecoder::new(fs::File::open(&file_path)?);
    let mut tar_archive = tar::Archive::new(tar_decoder);
    tar_archive.unpack(base_dir)?;

    Ok(())
}
