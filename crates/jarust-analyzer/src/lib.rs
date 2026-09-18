use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum JvmHotspotCategory {
    GcAllocationChurn,
    LockContention,
    BlockingThreadModel,
    HeapFragmentation,
}

impl JvmHotspotCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::GcAllocationChurn => "GC Allocation Churn",
            Self::LockContention => "Lock & Monitor Contention",
            Self::BlockingThreadModel => "Blocking Thread Overhead",
            Self::HeapFragmentation => "Heap Fragmentation & Indirection",
        }
    }

    pub fn severity_weight(&self) -> u32 {
        match self {
            Self::GcAllocationChurn => 15,
            Self::LockContention => 25,
            Self::BlockingThreadModel => 20,
            Self::HeapFragmentation => 10,
        }
    }

    pub fn rust_remedy(&self) -> &'static str {
        match self {
            Self::GcAllocationChurn => "Zero-copy slices (`&[u8]`), `bytes::Bytes`",
            Self::LockContention => "Lock-free atomics, `tokio::sync::RwLock`",
            Self::BlockingThreadModel => "Tokio multi-threaded async runtime",
            Self::HeapFragmentation => "Contiguous memory layouts (`Vec<T>`)",
        }
    }
}

pub struct DiagnosticRule {
    pub pattern: &'static str,
    pub category: JvmHotspotCategory,
    pub detail: &'static str,
}

pub struct ScanReport {
    pub total_java_files: usize,
    pub total_lines_scanned: usize,
    pub hotspot_counts: HashMap<JvmHotspotCategory, usize>,
    pub file_hotspots: HashMap<PathBuf, Vec<(&'static str, usize)>>,
}

// FIX: Added Default implementation for ScanReport
impl Default for ScanReport {
    fn default() -> Self {
        Self::new()
    }
}

impl ScanReport {
    pub fn new() -> Self {
        Self {
            total_java_files: 0,
            total_lines_scanned: 0,
            hotspot_counts: HashMap::new(),
            file_hotspots: HashMap::new(),
        }
    }

    pub fn calculate_migration_score(&self) -> u32 {
        let mut raw_score = 0;
        for (cat, count) in &self.hotspot_counts {
            raw_score += (*count as u32) * cat.severity_weight();
        }
        if self.total_java_files == 0 {
            0
        } else {
            (raw_score / (self.total_java_files as u32)).min(100)
        }
    }
}

pub struct RepositoryScanner {
    rules: Vec<DiagnosticRule>,
}

impl Default for RepositoryScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl RepositoryScanner {
    pub fn new() -> Self {
        Self {
            rules: vec![
                DiagnosticRule { pattern: "new byte[", category: JvmHotspotCategory::GcAllocationChurn, detail: "Heap-allocated byte buffers" },
                DiagnosticRule { pattern: "synchronized", category: JvmHotspotCategory::LockContention, detail: "JVM object monitor acquisition" },
                DiagnosticRule { pattern: "CompletableFuture", category: JvmHotspotCategory::BlockingThreadModel, detail: "JVM thread scheduling" },
                DiagnosticRule { pattern: "List<", category: JvmHotspotCategory::HeapFragmentation, detail: "Pointer-chasing node graphs" },
            ],
        }
    }

    pub fn scan_path(&self, root: &Path) -> io::Result<ScanReport> {
        let mut report = ScanReport::new();
        self.walk_dir(root, &mut report)?;
        Ok(report)
    }

    fn walk_dir(&self, dir: &Path, report: &mut ScanReport) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() && !path.to_string_lossy().contains("target") {
                    self.walk_dir(&path, report)?;
                // FIX: Replaced map_or with is_some_and
                } else if path.extension().is_some_and(|ext| ext == "java") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        report.total_java_files += 1;
                        report.total_lines_scanned += content.lines().count();
                        for rule in &self.rules {
                            let matches = content.matches(rule.pattern).count();
                            if matches > 0 {
                                *report.hotspot_counts.entry(rule.category).or_insert(0) += matches;
                                report.file_hotspots.entry(path.clone()).or_default().push((rule.pattern, matches));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
