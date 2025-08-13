use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about = "Efficiently delete node_modules directories")]
struct Args {
    /// Directory to start scanning from (default: current)
    #[arg(default_value = ".")]
    dir: PathBuf,

    /// Simulate deletion without actually deleting
    #[arg(long)]
    dry_run: bool,

    /// Verbose output
    #[arg(long)]
    verbose: bool,

    /// Maximum recursion depth
    #[arg(long)]
    depth: Option<usize>,

    /// Paths to exclude (comma-separated)
    #[arg(long, value_delimiter = ',')]
    exclude: Vec<PathBuf>,
}

#[derive(Default, Clone)]
struct Stats {
    #[allow(dead_code)]
    folders_found: usize,
    #[allow(dead_code)]
    total_size: u64,
    folders_processed: usize,
    size_freed: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let exclude: HashSet<PathBuf> = args.exclude.into_iter().collect();

    // Scan phase
    let scan_start = Instant::now();
    let node_modules = scan_node_modules(&args.dir, args.depth, &exclude, args.verbose)?;
    let scan_duration = scan_start.elapsed();

    if node_modules.is_empty() {
        println!("No node_modules directories found.");
        return Ok(());
    }

    // Calculate total size in parallel
    let pb = ProgressBar::new(node_modules.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} {bar:40.cyan/blue} {pos}/{len} [{elapsed_precise}]")?
            .progress_chars("##-"),
    );
    pb.set_message("Calculating sizes...");

    let total_size: u64 = node_modules
        .par_iter()
        .progress_with(pb)
        .map(|path| calculate_dir_size(path).unwrap_or(0))
        .sum();

    let size_str = format_size(total_size);
    println!(
        "\nScan complete in {:.2}s:\n- Folders found: {}\n- Total size: {}\n- Estimated savings: {}",
        scan_duration.as_secs_f64(),
        node_modules.len(),
        size_str,
        size_str
    );

    if args.dry_run {
        println!("Dry run: No deletions performed.");
        return Ok(());
    }

    // Confirmation
    print!("Proceed with deletion? (yes/no): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    if input.trim().to_lowercase() != "yes" {
        println!("Deletion cancelled.");
        return Ok(());
    }

    // Deletion phase
    let deletion_start = Instant::now();
    let node_modules_len = node_modules.len();
    let stats = Arc::new(Mutex::new(Stats {
        folders_found: node_modules_len,
        total_size,
        ..Default::default()
    }));

    let pb = ProgressBar::new(node_modules_len as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{msg} {bar:40.cyan/blue} {pos}/{len} {eta} [{elapsed_precise}] Freed: {wide_msg}",
            )?
            .progress_chars("##-"),
    );

    let avg_time_per_folder = Arc::new(Mutex::new(Duration::ZERO));
    let _start_time = Instant::now();

    node_modules
        .into_par_iter()
        .progress_with(pb.clone())
        .try_for_each(|path: PathBuf| -> Result<()> {
            let folder_start = Instant::now();

            if args.verbose {
                println!("Processing: {:?}", path);
            }

            let size = calculate_dir_size(&path)?;
            fs::remove_dir_all(&path).with_context(|| format!("Failed to delete {:?}", path))?;

            let duration = folder_start.elapsed();
            {
                let mut stats = stats.lock().unwrap();
                stats.folders_processed += 1;
                stats.size_freed += size;

                let mut avg = avg_time_per_folder.lock().unwrap();
                *avg = (*avg * (stats.folders_processed as u32 - 1) + duration)
                    / stats.folders_processed as u32;

                let remaining = node_modules_len - stats.folders_processed;
                let eta = *avg * remaining as u32;

                pb.set_message(format!("Deleting... ETA: {:.2}s", eta.as_secs_f64()));
                pb.set_message(format!("Freed: {}", format_size(stats.size_freed)));
                // Note: wide_msg for freed
            }

            Ok(())
        })?;

    let deletion_duration = deletion_start.elapsed();
    let stats = stats.lock().unwrap();
    let size_freed_str = format_size(stats.size_freed);

    println!(
        "\nDeletion complete in {:.2}s:\n- Folders deleted: {}\n- Space freed: {}",
        deletion_duration.as_secs_f64(),
        stats.folders_processed,
        size_freed_str
    );

    Ok(())
}

fn scan_node_modules(
    root: &Path,
    max_depth: Option<usize>,
    exclude: &HashSet<PathBuf>,
    verbose: bool,
) -> Result<Vec<PathBuf>> {
    let mut node_modules = Vec::new();

    for entry in WalkDir::new(root)
        .max_depth(max_depth.unwrap_or(usize::MAX))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir() && e.file_name() == "node_modules")
    {
        let path = entry.path().to_path_buf();
        if exclude.contains(&path) {
            if verbose {
                println!("Excluding: {:?}", path);
            }
            continue;
        }
        node_modules.push(path);
    }

    Ok(node_modules)
}

fn calculate_dir_size(path: &Path) -> Result<u64> {
    let mut total_size = 0u64;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            total_size += entry.metadata()?.len();
        }
    }
    Ok(total_size)
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
