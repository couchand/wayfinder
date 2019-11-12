use std::fmt;

/// A node of a trie, a prefix-keyed data structure.
pub struct Trie<K, V> {
    /// If this node has data, here it is.  Leaf nodes must.
    pub data: Option<V>,

    /// If this node has children, they are arranged in
    /// alphabetical order.
    pub children: Vec<(K, Trie<K, V>)>,
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for Trie<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Trie")
            .field("data", &self.data)
            .field("children", &self.children)
            .finish()
    }
}

impl<K: Ord + Clone, V> Trie<K, V> {
    /// Build a new, empty trie.
    pub fn new() -> Trie<K, V> {
        Trie {
            data: None,
            children: vec![],
        }
    }

/*
    /// Build a new trie initialized with the given data.
    pub fn with_data(data: V) -> Trie<K, V> {
        Trie {
            data: Some(data),
            children: vec![],
        }
    }
*/

    /// Adds the given element to the trie.  If an element already exists
    /// at that position, fails, returning the old trie and the element.
    pub fn add<P: IntoIterator<Item=K>>(
        mut self,
        path: P,
        data: V,
    ) -> Result<Trie<K, V>, (Trie<K, V>, V)> {
        let mut path = path.into_iter();
        let c = path.next();

        // If we've consumed the whole path, this is the node.
        if c.is_none() {
            if self.data.is_some() {
                return Err((self, data));
            } else {
                self.data = Some(data);
                return Ok(self);
            }
        }
        let c = c.unwrap();

        // Let's see if any of the outbound links are a candidate.
        match self.children.binary_search_by_key(&c, |p| p.0.clone()) {
            // No candidate link, so make one.
            Err(i) => {
                let new_child = Trie::new();
                new_child.add(path, data).map(|tr| {
                    self.children.insert(i, (c, tr));
                    self
                })
            },
            // We found a link.
            Ok(i) => {
                let (_, tr) = self.children.remove(i);
                tr.add(path, data).map(|tr| {
                    self.children.insert(i, (c, tr));
                    self
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let t = Trie::new();

        let t = t.add("foo".chars(), 42).map_err(|_| ()).unwrap();
        let t = t.add("foobar".chars(), 0);

        println!("{:?}", t);
        assert!(t.is_ok())
    }
}
