/*
 * BSD 3-Clause License
 *
 * Copyright 2021 Stanislav Yakovlev.
 */

fn apply_to_all<T, R>(v: Vec<T>, f: fn(T) -> R) -> Vec<R> {
    v.into_iter().map(|x| f(x)).collect()
}

fn apply_in_threads<T, R>(v: Vec<T>, f: fn(T) -> R, bsize: usize) -> Vec<R>
where
    T: Clone + Send + 'static,
    R: Send + 'static,
{
    let blocks: Vec<_> = v.chunks(bsize).map(|x| x.to_vec()).collect();

    let handles: Vec<_> = blocks
        .into_iter()
        .map(|x| std::thread::spawn(move || apply_to_all(x, f)))
        .collect();

    let output: Vec<_> = handles.into_iter().map(|x| x.join().unwrap()).collect();

    output.into_iter().fold(vec![], |mut acc, mut x| {
        acc.append(&mut x);
        acc
    })
}

fn calculate_block_size(len: usize, min_size: usize, max_blocks: usize) -> usize {
    let mut bsize = len / max_blocks;

    if bsize < min_size {
        bsize = min_size;
    } else {
        if len % max_blocks != 0 {
            bsize += 1
        }
    }
    bsize
}

fn compute<T, R>(v: Vec<T>, f: fn(T) -> R, threshold: usize, max_threads: usize) -> Vec<R>
where
    T: Clone + Send + 'static,
    R: Send + 'static,
{
    let len = v.len();
    if len < threshold {
        return apply_to_all(v, f);
    }

    let bsize = calculate_block_size(len, threshold, max_threads);
    apply_in_threads(v, f, bsize)
}
