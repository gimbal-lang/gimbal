

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct IndexTree<T> 
{
    nodes: Vec<IndexNode<T>>
}

impl<T> IndexTree<T>
{
    /*
    pub fn new(root_val: T) -> IndexTree<T> {
        let idx = 0;
        IndexTree{nodes: vec![IndexNode::new(idx, None, root_val)]}
    }
    */

    pub fn new() -> IndexTree<T> {
        IndexTree{nodes: Vec::new()}
    }

    pub fn add_node(&mut self, parent_idx: Option<usize>, val: T) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(IndexNode::new(idx, parent_idx, val));
        self.add_child(parent_idx, idx);
        idx
    }

    fn add_child(&mut self, parent_idx: Option<usize>, child_idx: usize)  {
        match parent_idx {
            Some(i) => self.nodes[i].children.push(child_idx),
            None => ()
        }
    }

    pub fn root(&self) -> &IndexNode<T> {
        &self.nodes[0]
    }

    pub fn node(&self, idx: usize) -> &IndexNode<T> {
        &self.nodes[idx]
    }

    pub fn children(&self, idx: usize) -> Vec<&IndexNode<T>> {
        self.nodes[idx].children.iter().map(|i| {
            &self.nodes[*i]
        }).collect::<Vec<&IndexNode<T>>>()
    }

    pub fn children_idx(&self, idx: usize) -> Vec<usize> {
        self.children(idx).iter().map(|n| n.idx()).collect::<Vec<usize>>()
    }

    pub fn children_val(&self, idx: usize) -> Vec<&T> {
        self.children(idx).iter().map(|n| n.val()).collect::<Vec<&T>>()
    }

    pub fn node_val(&self, idx: usize) -> &T {
        self.nodes[idx].val()
    }

    pub fn node_idx(&self, idx: usize) -> usize {
        self.nodes[idx].idx()
    }
}

#[derive(Debug)]
struct IndexNode<T>
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>
}

impl<T> IndexNode<T>
{
    fn new(idx: usize, parent_idx: Option<usize>, val: T) -> Self {
        Self {
            idx,
            val,
            parent: parent_idx,
            children: vec![]
        }
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn idx(&self) -> usize {
        self.idx
    }
}




#[derive(Debug)]
pub enum NameMapError<T: Named> {
    NameAlreadyExists(T)
}

#[derive(Debug)]
pub struct NameMap<T: Named> {
    map: HashMap<String, T>
}

pub trait Named {
    fn name(&self) -> String;
}

impl<T: Named> NameMap<T> {
    pub fn new() -> Self {
        NameMap{map: HashMap::new()}
    }

    pub fn insert(&mut self, item: T) -> Result<Option<T>, NameMapError<T>> {
        match self.map.get(&item.name()) {
            Some(v) => Err(NameMapError::NameAlreadyExists(item)),
            None => Ok(self.map.insert(item.name().clone(), item)),
        }
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        self.map.get(name)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_index_tree() {
      let mut tree = IndexTree::new();
      tree.add_node(None, "Root".to_string());
      assert_eq!(tree.root().val, "Root");
      let child01 = tree.add_node(Some(0), "Child01".to_string());
      let _ = tree.add_node(Some(0), "Child02".to_string());
      assert_eq!(tree.children_val(0), vec!["Child01", "Child02"]);
      let _ = tree.add_node(Some(child01), "GChild01".to_string());
      let _ = tree.add_node(Some(child01), "GChild02".to_string());
      assert_eq!(tree.children_val(child01), vec!["GChild01", "GChild02"]);
  }
}