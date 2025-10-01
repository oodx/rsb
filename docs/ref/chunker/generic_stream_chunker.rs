use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

/// Generic chunk specification for any byte stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub id: usize,
    pub start: u64,
    pub end: u64,
    pub size: u64,
}

/// Generic processing state that can be extended by consumers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamState {
    pub bytes_processed: u64,
    pub chunks_completed: Vec<usize>,
    pub custom_data: std::collections::HashMap<String, String>,
}

impl Default for StreamState {
    fn default() -> Self {
        Self {
            bytes_processed: 0,
            chunks_completed: Vec::new(),
            custom_data: std::collections::HashMap::new(),
        }
    }
}

/// Checkpoint for resumable stream processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCheckpoint {
    pub stream_source: PathBuf,
    pub current_position: u64,
    pub state: StreamState,
    pub total_size: u64,
    pub chunk_size: u64,
}

/// Generic stream chunker for any file type
pub struct StreamChunker {
    source: PathBuf,
    chunk_size: u64,
    total_size: u64,
    checkpoint_path: PathBuf,
}

impl StreamChunker {
    /// Create new stream chunker for a file
    pub fn new<P: AsRef<Path>>(source: P, chunk_size: u64) -> std::io::Result<Self> {
        let source = source.as_ref().to_path_buf();
        let total_size = std::fs::metadata(&source)?.len();
        let checkpoint_path = source.with_extension("stream_checkpoint");
        
        Ok(Self {
            source,
            chunk_size,
            total_size,
            checkpoint_path,
        })
    }
    
    /// Get all chunk specifications for the stream
    pub fn get_chunks(&self) -> Vec<StreamChunk> {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut id = 0;
        
        while start < self.total_size {
            let end = (start + self.chunk_size - 1).min(self.total_size - 1);
            let size = end - start + 1;
            
            chunks.push(StreamChunk { id, start, end, size });
            
            start = end + 1;
            id += 1;
        }
        
        chunks
    }
    
    /// Read a specific byte range from the stream
    pub fn read_range(&self, start: u64, end: u64) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(&self.source)?;
        file.seek(SeekFrom::Start(start))?;
        
        let size = (end - start + 1) as usize;
        let mut buffer = vec![0u8; size];
        let bytes_read = file.read(&mut buffer)?;
        buffer.truncate(bytes_read);
        
        Ok(buffer)
    }
    
    /// Read a specific chunk
    pub fn read_chunk(&self, chunk: &StreamChunk) -> std::io::Result<Vec<u8>> {
        self.read_range(chunk.start, chunk.end)
    }
    
    /// Process stream with custom handler and automatic checkpointing
    pub fn process_with<F>(&mut self, mut handler: F) -> std::io::Result<StreamState>
    where
        F: FnMut(&[u8], &StreamChunk, &StreamState) -> std::io::Result<StreamState>,
    {
        let mut checkpoint = self.load_checkpoint().unwrap_or_else(|_| StreamCheckpoint {
            stream_source: self.source.clone(),
            current_position: 0,
            state: StreamState::default(),
            total_size: self.total_size,
            chunk_size: self.chunk_size,
        });
        
        let chunks = self.get_chunks();
        let start_chunk = chunks.iter()
            .position(|c| c.start >= checkpoint.current_position)
            .unwrap_or(0);
        
        for chunk in &chunks[start_chunk..] {
            if checkpoint.state.chunks_completed.contains(&chunk.id) {
                continue;
            }
            
            let data = self.read_chunk(chunk)?;
            let new_state = handler(&data, chunk, &checkpoint.state)?;
            
            checkpoint.state = new_state;
            checkpoint.state.bytes_processed = chunk.end + 1;
            checkpoint.state.chunks_completed.push(chunk.id);
            checkpoint.current_position = chunk.end + 1;
            
            self.save_checkpoint(&checkpoint)?;
        }
        
        let _ = std::fs::remove_file(&self.checkpoint_path);
        Ok(checkpoint.state)
    }
    
    /// Process stream with mutable state reference for zero-copy updates
    pub fn process_streaming<F>(&mut self, mut handler: F) -> std::io::Result<StreamState>
    where
        F: FnMut(&[u8], &StreamChunk, &mut StreamState) -> std::io::Result<()>,
    {
        let mut checkpoint = self.load_checkpoint().unwrap_or_else(|_| StreamCheckpoint {
            stream_source: self.source.clone(),
            current_position: 0,
            state: StreamState::default(),
            total_size: self.total_size,
            chunk_size: self.chunk_size,
        });
        
        let chunks = self.get_chunks();
        let start_chunk = chunks.iter()
            .position(|c| c.start >= checkpoint.current_position)
            .unwrap_or(0);
        
        for chunk in &chunks[start_chunk..] {
            if checkpoint.state.chunks_completed.contains(&chunk.id) {
                continue;
            }
            
            let data = self.read_chunk(chunk)?;
            handler(&data, chunk, &mut checkpoint.state)?;
            
            checkpoint.state.bytes_processed = chunk.end + 1;
            checkpoint.state.chunks_completed.push(chunk.id);
            checkpoint.current_position = chunk.end + 1;
            
            self.save_checkpoint(&checkpoint)?;
        }
        
        let _ = std::fs::remove_file(&self.checkpoint_path);
        Ok(checkpoint.state)
    }
    
    /// Get total stream size
    pub fn size(&self) -> u64 {
        self.total_size
    }
    
    /// Get current processing progress (0.0 to 1.0)
    pub fn progress(&self) -> std::io::Result<f64> {
        if let Ok(checkpoint) = self.load_checkpoint() {
            Ok(checkpoint.current_position as f64 / self.total_size as f64)
        } else {
            Ok(0.0)
        }
    }
    
    /// Save checkpoint
    fn save_checkpoint(&self, checkpoint: &StreamCheckpoint) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.checkpoint_path)?;
        
        let mut writer = BufWriter::new(file);
        let data = serde_json::to_string_pretty(checkpoint)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        writer.write_all(data.as_bytes())?;
        writer.flush()?;
        Ok(())
    }
    
    /// Load existing checkpoint
    fn load_checkpoint(&self) -> std::io::Result<StreamCheckpoint> {
        let file = File::open(&self.checkpoint_path)?;
        let reader = BufReader::new(file);
        
        let checkpoint: StreamCheckpoint = serde_json::from_reader(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        if checkpoint.stream_source != self.source {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Checkpoint doesn't match current stream source"
            ));
        }
        
        Ok(checkpoint)
    }
}

/// Builder for configuring stream processing
pub struct StreamProcessorBuilder {
    chunk_size: Option<u64>,
    checkpoint_enabled: bool,
}

impl Default for StreamProcessorBuilder {
    fn default() -> Self {
        Self {
            chunk_size: None,
            checkpoint_enabled: true,
        }
    }
}

impl StreamProcessorBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn chunk_size(mut self, size: u64) -> Self {
        self.chunk_size = Some(size);
        self
    }
    
    pub fn checkpoint_enabled(mut self, enabled: bool) -> Self {
        self.checkpoint_enabled = enabled;
        self
    }
    
    pub fn build<P: AsRef<Path>>(self, source: P) -> std::io::Result<StreamChunker> {
        let source = source.as_ref();
        let chunk_size = self.chunk_size.unwrap_or_else(|| {
            optimal_chunk_size(std::fs::metadata(source).ok().map(|m| m.len()).unwrap_or(0))
        });
        
        StreamChunker::new(source, chunk_size)
    }
}

/// Calculate optimal chunk size based on file size
pub fn optimal_chunk_size(file_size: u64) -> u64 {
    match file_size {
        0..=1_000_000 => 256_000,              // 256KB
        1_000_001..=100_000_000 => 1_000_000,  // 1MB
        100_000_001..=1_000_000_000 => 10_000_000,  // 10MB
        _ => 100_000_000,                      // 100MB
    }
}

/// Parallel chunk processor for independent chunks
pub struct ParallelStreamProcessor {
    chunker: StreamChunker,
}

impl ParallelStreamProcessor {
    pub fn new(chunker: StreamChunker) -> Self {
        Self { chunker }
    }
    
    /// Process chunks that can be handled independently
    /// Note: Results must be manually merged by caller
    pub fn process_independent<F>(&self, handler: F) -> std::io::Result<Vec<Vec<u8>>>
    where
        F: Fn(&[u8], &StreamChunk) -> std::io::Result<Vec<u8>> + Send + Sync,
    {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let chunks = self.chunker.get_chunks();
        let results = Arc::new(Mutex::new(Vec::with_capacity(chunks.len())));
        let handler = Arc::new(handler);
        
        let mut handles = vec![];
        
        for chunk in chunks {
            let chunker = StreamChunker::new(&self.chunker.source, self.chunker.chunk_size)?;
            let results = Arc::clone(&results);
            let handler = Arc::clone(&handler);
            
            let handle = thread::spawn(move || -> std::io::Result<()> {
                let data = chunker.read_chunk(&chunk)?;
                let result = handler(&data, &chunk)?;
                
                let mut results = results.lock().unwrap();
                results.push(result);
                Ok(())
            });
            
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap()?;
        }
        
        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_basic_chunking() {
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(&vec![b'x'; 1000]).unwrap();
        
        let chunker = StreamChunker::new(temp.path(), 300).unwrap();
        let chunks = chunker.get_chunks();
        
        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0].size, 300);
        assert_eq!(chunks[3].size, 100);
    }
    
    #[test]
    fn test_read_range() {
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(b"0123456789").unwrap();
        
        let chunker = StreamChunker::new(temp.path(), 5).unwrap();
        let data = chunker.read_range(2, 7).unwrap();
        
        assert_eq!(&data, b"234567");
    }
    
    #[test]
    fn test_process_with_state() {
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(b"test data here").unwrap();
        
        let mut chunker = StreamChunker::new(temp.path(), 5).unwrap();
        
        let result = chunker.process_with(|data, _chunk, state| {
            let mut new_state = state.clone();
            new_state.custom_data.insert(
                "byte_count".to_string(),
                (state.bytes_processed + data.len() as u64).to_string()
            );
            Ok(new_state)
        }).unwrap();
        
        assert_eq!(result.bytes_processed, 14);
        assert_eq!(result.chunks_completed.len(), 3);
    }
    
    #[test]
    fn test_checkpoint_recovery() {
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(&vec![b'x'; 1000]).unwrap();
        
        let mut chunker = StreamChunker::new(temp.path(), 300).unwrap();
        
        // Process first chunk
        let mut chunk_count = 0;
        let _ = chunker.process_streaming(|_data, _chunk, state| {
            chunk_count += 1;
            state.custom_data.insert("chunks".to_string(), chunk_count.to_string());
            
            if chunk_count == 2 {
                // Simulate interruption
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Interrupted,
                    "Simulated interruption"
                ));
            }
            Ok(())
        });
        
        // Verify checkpoint exists
        assert!(chunker.checkpoint_path.exists());
        let progress = chunker.progress().unwrap();
        assert!(progress > 0.0 && progress < 1.0);
    }
    
    #[test]
    fn test_builder_pattern() {
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(&vec![b'x'; 1000]).unwrap();
        
        let chunker = StreamProcessorBuilder::new()
            .chunk_size(250)
            .checkpoint_enabled(true)
            .build(temp.path())
            .unwrap();
        
        assert_eq!(chunker.chunk_size, 250);
        assert_eq!(chunker.get_chunks().len(), 4);
    }
}