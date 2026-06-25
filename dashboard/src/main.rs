use std::collections::HashMap;
use std::thread;

// Find newline-aligned chunk boundaries (same as before)
fn find_chunk_boundaries(data: &[u8], num_chunks: usize) -> Vec<(usize, usize)> {
    let len = data.len();
    let chunk_size = len / num_chunks;
    let mut boundaries = Vec::with_capacity(num_chunks);
    let mut start = 0;

    for i in 0..num_chunks {
        if start >= len {
            break;
        }
        if i == num_chunks - 1 {
            boundaries.push((start, len));
            break;
        }
        let naive_end = (start + chunk_size).min(len);
        // Scan forward for next newline
        let actual_end = data[naive_end..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|off| naive_end + off + 1)
            .unwrap_or(len);
        boundaries.push((start, actual_end));
        start = actual_end;
    }
    boundaries
}

// Count words in a byte slice chunk.
// Returns HashMap<word_bytes, count>.
// Keys are &[u8] slices pointing into `chunk`.
fn count_words<'a>(chunk: &'a [u8]) -> HashMap<&'a [u8], u64> {
    let mut counts: HashMap<&'a [u8], u64> = HashMap::with_capacity(1024);
    let mut pos = 0;

    while pos < chunk.len() {
        // Skip spaces and newlines (word separators)
        while pos < chunk.len() && (chunk[pos] == b' ' || chunk[pos] == b'\n') {
            pos += 1;
        }
        if pos >= chunk.len() {
            break;
        }

        // Find end of current word
        let word_start = pos;
        while pos < chunk.len() && chunk[pos] != b' ' && chunk[pos] != b'\n' {
            pos += 1;
        }
        let word = &chunk[word_start..pos]; // zero-copy slice reference

        // Increment count for this word
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let text = b"the quick brown fox jumps over the lazy dog\n\
                 the dog barked at the fox\n\
                 the fox ran away quickly\n\
                 the quick brown dog sat down quickly the \n";

    let boundaries = find_chunk_boundaries(text, 8);

    // Parallel counting: each thread counts words in its chunk
    let local_counts: Vec<HashMap<&[u8], u64>> = thread::scope(|s| {
        let handles: Vec<_> = boundaries
            .iter()
            .map(|&(start, end)| {
                let chunk = &text[start..end];
                // Borrow of `chunk` (which borrows from `text`) is safe
                // because scoped threads are joined before text can be dropped.
                s.spawn(move || {
                    let counts = count_words(chunk);
                    println!("Chunk {start}-{end}: {} unique words", counts.len());
                    counts
                })
            })
            .collect();

        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    // Single-threaded merge
    let mut final_counts: HashMap<&[u8], u64> = HashMap::with_capacity(1024);
    for local in local_counts {
        for (word, count) in local {
            *final_counts.entry(word).or_insert(0) += count;
        }
    }

    // Sort by frequency descending, then alphabetically
    let mut words: Vec<(&&[u8], &u64)> = final_counts.iter().collect();
    words.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    println!("\nTop words:");
    for (word, count) in words.iter().take(5) {
        println!("  {:?}: {}", std::str::from_utf8(word).unwrap(), count);
    }

    // Expected output:
    // Chunk 0-...: N unique words   (for each chunk)
    // ...
    // Top words:
    //   "the": 7
    //   "fox": 3
    //   "dog": 3
    //   "brown": 2
    //   "quick": 2
}
