use sha2::{Digest, Sha256};
use std::collections::HashMap;

fn leaf_hash(n: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(n);
    let result = hasher.finalize();
    let mut hash = [0; 32];
    hash.copy_from_slice(&result[..]);
    hash
}

fn parent_hash(l: &[u8], r: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(l);
    hasher.update(r);
    let result = hasher.finalize();
    let mut hash = [0; 32];
    hash.copy_from_slice(&result[..]);
    hash
}

fn foldr<F>(f: F, coll: &[&[u8]]) -> Vec<u8>
where
    F: Fn(&[u8], &[u8]) -> [u8; 32],
{
    if coll.is_empty() {
        return Vec::new();
    }
    let mut res = coll[coll.len() - 1].to_vec();
    for i in (0..coll.len() - 1).rev() {
        let folded = f(coll[i], &res);
        res = folded.to_vec();
    }
    res
}

fn insert(mut s: HashMap<i32, Vec<u8>>, v: &[u8], n: i32) -> HashMap<i32, Vec<u8>> {
    if let Some(val) = s.get(&n) {
        let p = parent_hash(val, v);
        s = del(s, n);
        return insert(s, &p, n + 1);
    }
    s.insert(n, v.to_vec());
    s
}

fn del(mut s: HashMap<i32, Vec<u8>>, n: i32) -> HashMap<i32, Vec<u8>> {
    s.remove(&n);
    s
}

fn finalize(s: HashMap<i32, Vec<u8>>) -> Vec<u8> {
    let mut keys: Vec<i32> = s.keys().cloned().collect();
    keys.sort_by(|a, b| b.cmp(a));
    let mut vals: Vec<&[u8]> = Vec::new();
    for k in keys {
        if let Some(v) = s.get(&k) {
            vals.push(v.as_slice());
        }
    }
    foldr(parent_hash, &vals)
}

pub fn merkle_root(stream: &Vec<Vec<u8>>) -> Vec<u8> {
    if stream.is_empty() {
        return Vec::new();
    }
    let mut m: HashMap<i32, Vec<u8>> = HashMap::new();
    for v in stream.iter() {
        m = insert(m, &leaf_hash(&v), 0);
    }
    finalize(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    
    #[test]
    fn test_leaf_hash() {
        let hash = leaf_hash(b"hello world");
        assert_eq!(
            hash,
            hex!("b95c1b8ce2594e9f2428143c444f0d16c8c0a8a01c3df41282162d542e3b4924")
        );
    }

    #[test]
    fn test_parent_hash() {
        let hash = parent_hash(&[1, 2, 3], &[4, 5, 6]);
        assert_eq!(
            hash,
            hex!("cf77fe174f604d47128c500dc45603671fb372032a323729191c3c84c961b75e")
        );
    }

    #[test]
    fn test_foldr() {
        let data = vec![b"foo", b"bar", b"baz"];
        let hashes: Vec<_> = data.iter().map(|&x| leaf_hash(x)).collect();
        let hash = foldr(parent_hash, hashes.iter().map(|x| x.as_ref()).collect::<Vec<_>>().as_slice());
        assert_eq!(
            hash,
            hex!("344df24d1f3ec33b876a68d37fa972cb4e2da6adbd9e8d20c38ce19367d22c")
        );
    }
    
    #[test]
    fn test_merkle_root() {
        let data = vec![vec![b"foo".to_vec()], vec![b"bar".to_vec()], vec![b"baz".to_vec()]];
        let flattened_data: Vec<_> = data.into_iter().flatten().collect();
        let root = merkle_root(&flattened_data);
        assert_eq!(
            root,
            hex!("aa6fd1e6b301a6e4566679e55e6d7c2f8bc44ca055690e15c05a1d6ec1b0a979")
        );
    }
    
    
}
