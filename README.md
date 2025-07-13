# 🧠 Intelligent File Deduplicator (Minor Project)

The **Intelligent File Deduplicator** is a Rust-based tool built to identify and safely handle duplicate files across a directory. It offers fast performance, smart filtering, and safe deletion mechanisms with detailed reporting.

---

## Core Features

### Multi-Algorithm Hashing
Supports 3 hashing strategies:
- **SHA-256** – Secure and reliable
- **Blake3** – Extremely fast and modern
- **xxHash** – Optimized for speed (non-cryptographic)

### Parallel File Processing
- Uses [Rayon](https://crates.io/crates/rayon) to hash files concurrently.
- Efficiently processes large file sets with progress tracking.

### Advanced Filtering Options
- **Size ranges** – Filter files based on size thresholds
- **File types** – Include/exclude based on extensions
- **Date ranges** – Filter by modified/created dates
- **Regex matching** – Filter files using patterns (e.g., name filters)

### Safe Operations
- Built-in **quarantine system** to safely move duplicates before deletion.
- Avoids data loss by reviewing files before permanent actions.

### Report Generation
- Outputs a structured `report.json` containing:
  - Duplicate sets
  - File paths
  - File sizes and hash values
  - Estimated space savings

---

## Project Structure
Adarsh-Subhadarshi-Sahoo_24BECC16/
├── quarantine_folder/ # Stores safely moved duplicate files
├── src/
│ ├── advance_filtering.rs # Implements advanced filtering logic
│ ├── hash_algo.rs # SHA-256, Blake3, xxHash algorithms
│ ├── main.rs # Entry point of the application
│ ├── parallel_hashing.rs # Parallel processing via Rayon
│ ├── parallel_hashing/
│ │ └── hash_algo.rs
│ ├── quarantine_and_report.rs # Quarantine logic + report writing
│ └── testing_file.rs # Unit testing or test runs
├── target/ # Compiled output (auto-generated)
├── test_folder/ # Custom test files directory
├── report.json # JSON report output
├── Cargo.toml # Crate dependencies and metadata
├── Cargo.lock # Locked dependency versions
└── .gitignore # Ignored files list

---

## ⚙️ Configuration

- Set desired hashing algorithm, filters, and output format in `main.rs` (if modularized).
- Change quarantine path or report location as needed.

---

## 📦 Requirements

- [Rust](https://www.rust-lang.org/) (latest stable)

---

## 🔒 Disclaimer

The deduplicator performs file operations. Always **review reports before confirming deletion**. Use at your own risk.

### ▶️ Run the Application

# Clone the repo or navigate to the project directory
cd Adarsh-Subhadarshi-Sahoo_24BECC16(Minor_Project)

# Build and run
cargo run