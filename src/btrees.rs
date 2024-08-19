use std::cmp::min;
use std::{collections::HashMap, u16};
use std::mem;


const BTREE_MAX: usize = 4;
const MIN_KEY: usize = BTREE_MAX/2;
const MAX_KEY: usize = BTREE_MAX;
const MIN_CHILD: usize = (BTREE_MAX+1)/2;
const MAX_CHILD: usize = BTREE_MAX+1;

#[derive(Clone, Debug, PartialEq)]
pub struct KeyValue {
    pub key: u16,
    pub value: u16,
}

#[derive(Clone, Debug)]
pub enum  NodeType {
    Internal(Vec<KeyValue>),
    Leaf(Vec<KeyValue>)
}
#[derive(Clone, Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub is_root: bool,
}
impl Node {
    pub fn new(is_root: bool) -> Self {
        Node{node_type: NodeType::Internal(Vec::new()), is_root: is_root}
    }
    pub fn get_child(&self, key: u16)-> u16 {
        match &self.node_type {
            NodeType::Internal(keys) => {
                for i in 1..keys.len() {
                    if keys[i].key > key {
                        return keys[i].value;
                    }
                }
                return keys[0].value;
            },
            NodeType::Leaf(_) => panic!("There are no children of leaf nodes"),
            _ => panic!("---")
        }
    }

    fn extract(&mut self) -> &mut Vec<KeyValue> {
        match &mut self.node_type {
             NodeType::Internal(kv) => {
                return kv
             },
             NodeType::Leaf(kv) => {
                return kv
             }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BPlusTree {
    root: Node,
    leaf_tree: Node,
    unique_id: u16,
    nodes: HashMap<u16, Node>
}

impl BPlusTree {
    pub fn new() -> Self {
        BPlusTree{root: Node::new(true), leaf_tree: Node { node_type: NodeType::Leaf(Vec::new()), is_root: true },unique_id: 1, nodes: HashMap::new()}
    }
    pub fn print_tree(&self, node_key: u16, level: usize) {
        if self.is_leaf_root() {
            println!("-> {:?}", self.leaf_tree)
        }
        else {
            if let Some(node) = self.nodes.get(&node_key) {
                // Print the current node with indentation
                let indent = "    ".repeat(level);
                match &node.node_type {
                    NodeType::Internal(kvs) => {
                        println!("{}Internal Node (ID: {}):", indent, node_key);
                        for kv in kvs {
                            println!("{}  - Key: {}, Points to Node: {}", indent, kv.key, kv.value);
                            // Recursively print the child nodes
                            self.print_tree(kv.value, level + 1);
                        }
                    }
                    NodeType::Leaf(kvs) => {
                        println!("{}Leaf Node (ID: {}):", indent, node_key);
                        for kv in kvs {
                            println!("{}  - Key: {}, Value: {}", indent, kv.key, kv.value);
                        }
                    }
                }
            }
         }
       
    }

    
    pub fn search(&self, k: u16) -> (u16, u16){
        let root = self.root.clone();
        self.search_tree(root, k, None, None)
    }
    fn search_tree(&self, node:Node, key: u16, leaf_id: Option<u16>,parent_id: Option<u16>) -> (u16, u16){
       match node.node_type {
          NodeType::Leaf(keyvalue) => {
               return (leaf_id.unwrap(), parent_id.unwrap())
          },
          NodeType::Internal(keys) => {
                  let mut pointer: u16 = keys[keys.len()-1].value;
                  for i in 0..keys.len()-1{
                      if key < keys[i].key {
                        pointer = keys[i].value;
                        break;
                      }
                  };
                  let child = self.nodes.get(&pointer).unwrap().clone();
                  let mut parent:u16;
                  match &child.node_type {
                      NodeType::Internal(_) => parent = pointer,
                      NodeType::Leaf(_) => {
                        if parent_id != None {
                            parent = parent_id.unwrap();
                        } else {
                            parent = 0;
                        }
                      }
                  }
                  self.search_tree(child, key, Some(pointer), Some(parent))

          },
          _ => panic!("Undefined structure")
       }
    }



    pub fn get_node(&self, key: u16) -> Node {
        let (node_id, parent_id) = self.search(key);
        self.nodes.get(&node_id).unwrap().clone()
    }

    pub fn mut_node(&mut self, key: u16) -> &mut Node {
        let (node_id, parent_id) = self.search(key);
        self.nodes.get_mut(&node_id).unwrap()
    }



    fn is_underflow(&self, node:&Node) -> bool{
        match &node.node_type {
            NodeType::Internal(keys) => {
                if keys.len() < MIN_CHILD + 1 {
                    
                    return true
                } else {
                    return false 
                }
            },
            NodeType::Leaf(KeyValue) => {
                if KeyValue.len() < MIN_KEY {
                    return true
                } else {
                    return false
                }
            }
        }
    }

    fn is_overflow(&self, node:&Node) -> bool{
        match &node.node_type {
            NodeType::Internal(keys) => {
                if keys.len() > MAX_CHILD {
                    return true
                } else {
                    return false 
                }
            },
            NodeType::Leaf(KeyValue) => {
                if KeyValue.len() > MAX_KEY {
                    return true
                } else {
                    return false
                }
            }
        }
    }
    
    fn is_leaf_root(&self) -> bool {
        match &self.leaf_tree.node_type {
            NodeType::Leaf(kvs) => {
                kvs.len() < MAX_KEY + 1
            },
            _ => panic!()
        }
    }

    fn insert_leaf_tree(&mut self, new_kv: KeyValue) {
       match &mut self.leaf_tree.node_type {
         NodeType::Leaf(kvs) => {
            let mut idx = kvs.len();
            for i in 0..kvs.len(){
                if new_kv.key < kvs[i].key {
                    idx = i
                }
            }
            kvs.insert(idx, new_kv);
            let cells = kvs.clone();
            if !self.is_leaf_root() {
                self.root = Node {node_type: NodeType::Internal(vec![KeyValue {key: 0, value: self.unique_id}]), is_root: true};
                self.nodes.insert(0, self.root.clone());
                self.nodes.insert(self.unique_id, Node { node_type: NodeType::Leaf(cells), is_root: false });
                self.unique_id += 1;
                self.split(self.unique_id -1, 0);
                self.root = self.nodes.get(&0).unwrap().clone();
            }
         },
         _ => panic!()
       }
    }
    
    pub fn insert(&mut self, new_kv: KeyValue) -> bool {
        if self.is_leaf_root() {
            self.insert_leaf_tree(new_kv);
            return true;
        }
        let root = self.root.clone();
        let mut parents = vec![0];
        let mut next_node_id;
        match root.node_type {
            NodeType::Internal(pkvs) => {
              next_node_id = pkvs[pkvs.len() -1].value;
              for i in 0..pkvs.len() - 1 {
                  if  new_kv.key < pkvs[i].key {
                      next_node_id = pkvs[i].value;
                      break;
                  }
              }
            },
            _ => panic!("__")
        }
        self.insert_recursive(new_kv, next_node_id, &mut parents);
        while parents.len() > 1 {
            let node_id = parents.pop().unwrap();
            if self.is_overflow(self.nodes.get(&node_id).unwrap()) {
                self.split(node_id, parents[parents.len() -1]);
            } 
        }
        let root_id = parents.pop().unwrap();
        if self.is_overflow(self.nodes.get(&root_id).unwrap()) {
            self.split_root(root_id);
        }
        self.root = self.nodes.get(&0).unwrap().clone();
        return true

    }

    fn insert_recursive(&mut self,new_kv: KeyValue,current: u16, parents: &mut Vec<u16>){
       let mut node = self.nodes.get_mut(&current).unwrap();
       match &mut node.node_type {
        NodeType::Leaf(kvs) => {
            let mut insertion_idx = kvs.len();
            for i in 0..kvs.len() {
                if new_kv.key < kvs[i].key {
                   insertion_idx = i;
                   break;
                }
            }
            kvs.insert(insertion_idx, new_kv);
            parents.push(current);
            
        },
        NodeType::Internal(kvs) => {
            parents.push(current);
            let mut next_node_id;
            next_node_id = kvs[kvs.len() -1].value;
            for i in 0..kvs.len() - 1 {
                if  new_kv.key < kvs[i].key {
                    next_node_id = kvs[i].value;
                    break;
                }
            }

            self.insert_recursive(new_kv, next_node_id, parents)
        }
       }

    }

    fn split_root(&mut self, root_id: u16) {
        let new_node_id = self.unique_id;
        let root = self.nodes.get_mut(&root_id).unwrap();
        let pkvs = root.extract();
        let new_node = Node {node_type: NodeType::Internal(mem::replace(pkvs, Vec::new())), is_root: false};
        pkvs.push(KeyValue { key: 0, value: new_node_id});
        self.nodes.insert(new_node_id, new_node);
        self.unique_id += 1;
        self.split(new_node_id, root_id);
    }
    
    fn split(&mut self, current: u16, parent: u16) -> bool {
        let node = self.nodes.get(&current).unwrap();
        let mut cells: Vec<KeyValue> = Vec::new();
        let mut is_internal = false;
        match &node.node_type {
          NodeType::Internal(kvs) => {
            cells = kvs.clone();
            is_internal = true;
          },
          NodeType::Leaf(kvs) => {
            cells = kvs.clone();
          }
        };

        let middle_index = cells.len() / 2;

        let new_node_id = self.unique_id;
        let mut new_node_vec: Vec<KeyValue> = Vec::new();
        for i in 0..middle_index {
            new_node_vec.push(cells[i].clone())
        }
        let mut divider = cells[middle_index].key;
        if is_internal {
            divider = new_node_vec[middle_index-1].key
        }
        if is_internal{
            new_node_vec[middle_index-1].key = 0;
        }
        let  node = self.nodes.get_mut(&current).unwrap();
        match &mut node.node_type {
            NodeType::Internal(kvs) => {
                for i in 0..middle_index {
                    kvs.remove(0);
                }
                self.nodes.insert(new_node_id, Node { node_type: NodeType::Internal(new_node_vec), is_root: false });
                self.unique_id += 1;
            },
            NodeType::Leaf(kvs) => {
                for i in 0..middle_index {
                    kvs.remove(0);
                }
                self.nodes.insert(new_node_id, Node { node_type: NodeType::Leaf(new_node_vec), is_root: false });
                self.unique_id += 1;
            }
        };

        

        let mut parent_node = self.nodes.get_mut(&parent).unwrap();
        match &mut parent_node.node_type {
          NodeType::Internal(pkvs) => {
            let mut current_index: usize = 0;
            for i in 0..pkvs.len() {
               if &pkvs[i].value == &current {
                  current_index = i;  
               }   
            };

            pkvs.insert(current_index, KeyValue { key: divider, value: new_node_id });

          },
          _ => panic!("No non-internal parent")
        }
       
       


       return  true
    }

    pub fn delete(&mut self, key_d: u16) -> bool {
        let root = self.root.clone();
        let mut parents = vec![0];
        let mut next_node_id;
        match root.node_type {
            NodeType::Internal(pkvs) => {
              next_node_id = pkvs[pkvs.len() -1].value;
              for i in 0..pkvs.len() - 1 {
                  if  key_d < pkvs[i].key {
                      next_node_id = pkvs[i].value;
                      break;
                  }
              }
            },
            _ => panic!("__")
        }
        let exists = self.delete_recursive(key_d, next_node_id, &mut parents);
        if exists {
            while parents.len() > 1 {
                let node_id = parents.pop().unwrap();
                if self.is_underflow(self.nodes.get(&node_id).unwrap()) {
                    self.distribute_mini(node_id, parents[parents.len() -1]);
                } 
            }

            self.merge_root(parents.pop().unwrap());
        }
        self.root = self.nodes.get(&0).unwrap().clone();
        return true

    }

    
    fn get_sibling(&self , current: u16, parent: u16) -> Vec<KeyValue>{
       let parent = self.nodes.get(&parent).unwrap();
       match &parent.node_type {
        NodeType::Internal(pkvs) => {
            let mut return_vec = Vec::new();
            let mut index_current = 0;
            for i in 0..pkvs.len() {
                if current == pkvs[i].value {
                   index_current = i;
                   break;
                }
            }
            return_vec.push(KeyValue { key: current, value: index_current as u16});
            if index_current == 0 {
                return_vec.push(KeyValue { key: pkvs[1].value, value: 1 });
            } else {
                return_vec.push(KeyValue { key: pkvs[index_current -1].value, value: (index_current -1) as u16 });
            }
            return return_vec;
        },
        _ => panic!("_+_")
       } 
    }

    fn delete_recursive(&mut self, key_d: u16, current: u16, parents: &mut Vec<u16>) -> bool {
        let mut node = self.nodes.get_mut(&current).unwrap();
        match &mut node.node_type {
            NodeType::Leaf(kvs) => {
                let mut exists = false;
                for i in 0..kvs.len() {
                    if key_d == kvs[i].key {
                       kvs.remove(i);
                       parents.push(current); 
                       exists= true;
                       break;
                    }
                }
                if !exists {
                    println!("No key found as {}", key_d);
                }

                return exists;
            
            },
            NodeType::Internal(kvs) => {
                  parents.push(current);
                  let mut next_node_id;
                  next_node_id = kvs[kvs.len() -1].value;
                  for i in 0..kvs.len() - 1 {
                     if  key_d < kvs[i].key {
                         next_node_id = kvs[i].value;
                         break;
                     }
                  }
                  self.delete_recursive(key_d, next_node_id, parents)
            }
       }
    }

    fn distribute_mini(&mut self, current: u16, parent: u16) {
       let siblings = self.get_sibling(current, parent);
       
       let mut cells: Vec<KeyValue>= Vec::new();
       let mut reverse = false;
       let mut internal_divider: u16  = 0;
       if siblings[0].value < siblings[1].value {
          let node1 = self.nodes.get(&siblings[0].key).unwrap();
          let node2 = self.nodes.get(&siblings[1].key).unwrap();
          match &node1.node_type {
            NodeType::Internal(kvs) => {
                cells.extend(kvs.iter().cloned());
                let parent = self.nodes.get(&parent).unwrap();
                match &parent.node_type {
                    NodeType::Internal(pkvs) => {
                        internal_divider = pkvs[siblings[0].value as usize].key;
                    },
                    _ => panic!("-")
                }
            },
            NodeType::Leaf(kvs) => {
                cells.extend(kvs.iter().cloned());
            },  
          };
          match &node2.node_type {
            NodeType::Internal(kvs) => {
                cells.extend(kvs.iter().cloned());
            },
            NodeType::Leaf(kvs) => {
                cells.extend(kvs.iter().cloned());
            },
        }
       } else {
            reverse = true;
            let node1 = self.nodes.get(&siblings[0].key).unwrap();
            let node2 = self.nodes.get(&siblings[1].key).unwrap();
            match &node2.node_type {
                NodeType::Internal(kvs) => {
                    cells.extend(kvs.iter().cloned());
                    let parent = self.nodes.get(&parent).unwrap();
                    match &parent.node_type {
                        NodeType::Internal(pkvs) => {
                            internal_divider = pkvs[siblings[1].value as usize].key;
                        },
                        _ => panic!("-")
                    }
                    
                },
                NodeType::Leaf(kvs) => {
                    cells.extend(kvs.iter().cloned());
                },  
            };
            match &node1.node_type {
                NodeType::Internal(kvs) => {
                    cells.extend(kvs.iter().cloned());
                },
                NodeType::Leaf(kvs) => {
                    cells.extend(kvs.iter().cloned());
                },
            }
       }
       
       
       
       if reverse {
        let node2 = self.nodes.get_mut(&siblings[1].key).unwrap();
        match &mut node2.node_type {
           NodeType::Leaf(kvs) => {
             if cells.len() <= MAX_KEY {
                let node1 = self.nodes.get_mut(&siblings[0].key).unwrap();
                 match &mut node1.node_type {
                     NodeType::Leaf(kvs1) => {
                        let _ = mem::replace(kvs1, cells);
                     },
                     _ => panic!("___")
                 }
                 self.nodes.remove(&siblings[1].key);
                 let parent_node = self.nodes.get_mut(&parent).unwrap();
                 match &mut parent_node.node_type {
                     NodeType::Internal(pkvs) => {
                         pkvs.remove(siblings[1].value as usize);
                     },
                     _ => panic!("___")
                 }
             } else {
                 let mut moved_value = kvs.pop().unwrap();
                 let last_idx = &kvs.len() -1;
                 let new_bound = moved_value.key.clone();
                 let node1 = self.nodes.get_mut(&siblings[0].key).unwrap();
                 match &mut node1.node_type {
                     NodeType::Leaf(kvs1) => {
                         kvs1.insert(0,moved_value);
                     },
                     _ => panic!("___")
                 }
                 let parent_node = self.nodes.get_mut(&parent).unwrap();
                 match &mut parent_node.node_type {
                     NodeType::Internal(pkvs) => {
                         pkvs[siblings[1].value as usize].key = new_bound;
                     },
                     _ => panic!("___")
                 }
             }
           },
           NodeType::Internal(kvs) => {
             if cells.len() <= MAX_CHILD {
                 for i in 0..cells.len() {
                     if cells[i].key == 0 {
                         cells[i].key = internal_divider;
                         break;
        
                     }
                 }
                 let node1 = self.nodes.get_mut(&siblings[0].key).unwrap();
                 match &mut node1.node_type {
                     NodeType::Internal(kvs1) => {
                        let _ = mem::replace(kvs1, cells);
                     },
                     _ => panic!("{:?}", node1)
                 }
                 self.nodes.remove(&siblings[1].key);
                 let parent_node = self.nodes.get_mut(&parent).unwrap();
                 match &mut parent_node.node_type {
                     NodeType::Internal(pkvs) => {
                         pkvs.remove(siblings[1].value as usize);
                     },
                     _ => panic!("___")
                 }
                 
             } else {
                 let mut moved_value = kvs.pop().unwrap();
                 let last_idx = &kvs.len() -1;
                 let new_bound = kvs[last_idx].key.clone();
                 kvs[last_idx].key = 0;
                 moved_value.key = internal_divider;
                 let node1 = self.nodes.get_mut(&siblings[0].key).unwrap();
                 match &mut node1.node_type {
                     NodeType::Internal(kvs1) => {
                         kvs1.insert(0,moved_value);
                     },
                     _ => panic!("___")
                 }
                 let parent_node = self.nodes.get_mut(&parent).unwrap();
                 match &mut parent_node.node_type {
                     NodeType::Internal(pkvs) => {
                         pkvs[siblings[1].value as usize].key = new_bound;
                     },
                     _ => panic!("___")
                 }
             }
           }
        }
       } else {
           let node2 = self.nodes.get_mut(&siblings[1].key).unwrap();
           match &mut node2.node_type {
              NodeType::Leaf(kvs) => {
                if cells.len() <= MAX_KEY {
                    let _ = mem::replace(kvs, cells);
                    self.nodes.remove(&siblings[0].key);
                    let parent_node = self.nodes.get_mut(&parent).unwrap();
                    match &mut parent_node.node_type {
                        NodeType::Internal(pkvs) => {
                            pkvs.remove(siblings[0].value as usize);
                        },
                        _ => panic!("___")
                    }
                } else {
                    let moved_value = kvs.remove(0);
                    let new_bound = kvs[0].key.clone();
                    let node1 = self.nodes.get_mut(&siblings[0].key).unwrap();
                    match &mut node1.node_type {
                        NodeType::Leaf(kvs1) => {
                            kvs1.push(moved_value);
                        },
                        _ => panic!("___")
                    }
                    let parent_node = self.nodes.get_mut(&parent).unwrap();
                    match &mut parent_node.node_type {
                        NodeType::Internal(pkvs) => {
                            pkvs[siblings[0].value as usize].key = new_bound;
                        },
                        _ => panic!("___")
                    }
                }
              },
              NodeType::Internal(kvs) => {
                if cells.len() <= MAX_CHILD {
                    for i in 0..cells.len() {
                        if cells[i].key == 0 {
                            
                            cells[i].key = internal_divider;
                            break;
           
                        }
                    }
                    let _ = mem::replace(kvs, cells);
                    self.nodes.remove(&siblings[0].key);
                    let parent_node = self.nodes.get_mut(&parent).unwrap();
                    match &mut parent_node.node_type {
                        NodeType::Internal(pkvs) => {
                            pkvs.remove(siblings[0].value as usize);
                            
                        },
                        _ => panic!("___")
                    }
                    
                } else {
                    let mut moved_value = kvs.remove(0);
                    let new_bound = moved_value.key.clone();
                    moved_value.key = 0;
                    let node1 = self.nodes.get_mut(&siblings[0].key).unwrap();
                    match &mut node1.node_type {
                        NodeType::Internal(kvs1) => {
                            let last_idx = &kvs1.len() -1;
                            kvs1[last_idx].key = internal_divider;
                            kvs1.push(moved_value);
                        },
                        _ => panic!("___")
                    }
                    let parent_node = self.nodes.get_mut(&parent).unwrap();
                    match &mut parent_node.node_type {
                        NodeType::Internal(pkvs) => {
                            println!("{:?}", pkvs);
                            pkvs[siblings[0].value as usize].key = new_bound;
                        },
                        _ => panic!("___")
                    }
                }
              }
           }
       }
       
       
     }
       
     fn merge_root(&mut self, root_id: u16) -> bool{
        let root = self.nodes.get(&root_id).unwrap();
        let mut child_id = 0;
        let mut cells: Vec<KeyValue> = Vec::new();
        match &root.node_type {
         NodeType::Internal(pkvs) => {
            if pkvs.len() > 1 {
                return false
            }
             child_id = pkvs[0].value;
         },
         _ => panic!("__")
        };
        match &self.nodes.get(&child_id).unwrap().node_type {
          NodeType::Internal(kvs) => {
             cells.extend(kvs.clone().into_iter())
          },
          NodeType::Leaf(kvs) => {
             todo!()
          }
        };
 
        self.nodes.remove(&child_id);
 
        let root = self.nodes.get_mut(&root_id).unwrap();
        match &mut root.node_type {
          NodeType::Internal(pkvs) => {
             mem::replace(pkvs, cells);
          },
          _ => panic!("{{{{}}}}")
        }
        true
     }

}

/* These function were inspired by https://github.com/antoniosarosi/mkdb/blob/master/src/storage/btree.rs -> SQLLite's B-tree.
But after some time I realized that this might be a bit more complicated than what I wanted to make so but later on if ever need a better version like this I might come back and finish it.
// NOT completed, this function will not work as expected do not use it
    fn merge(&mut self,current: u16, parent: u16) -> bool {
        let siblings = self.siblings(current, parent);
        
        let mut cells = Vec::new();
 
        for  kvs in siblings.clone().into_iter() {
             let node = self.nodes.get(&kvs.key).unwrap().clone();
             match &node.node_type {
                 NodeType::Internal(keychild) => {
                     for skv in keychild.into_iter() {
                         
                         cells.push(skv.clone())
                     }
                 },
                 NodeType::Leaf(keyvalue) => {
                     for skv in keyvalue.into_iter() {
                         cells.push(skv.clone())
                     }
                 }
             }
        }
        
        
        
        if cells.len() > siblings.len() * MAX_KEY {
          let new_node_id = self.nodes.len() as u16;
          let mut is_end = false;
          let mut plusser = 0;
          for sibling in siblings.clone().into_iter() {
             let mut node = self.nodes.get_mut(&sibling.key).unwrap();
             match &mut node.node_type {
                 NodeType::Internal(kvs) => {
                 *kvs = Vec::new();
                 for i in  (sibling.value * 4 ) as usize..min((((sibling.value +1) * 4)) as usize, cells.len()){ 
                     kvs.push(cells[i].clone());
                     plusser += 1;
                 }
                 
                 
                 let mut parent = self.nodes.get_mut(&parent).unwrap();
                 match &mut parent.node_type{
                     NodeType::Internal(pkvs) => {
                     if pkvs.len() - 1 == sibling.value as usize {
                         is_end = true
                     }
                     pkvs[sibling.value as usize].key = cells[plusser].clone().key
                     },
                     _ => panic!("______")
                 }
                 self.nodes.insert(new_node_id, Node { node_type: NodeType::Internal(vec![ cells[plusser].clone() ]), is_root: false });
                 },
                 NodeType::Leaf(kvs) => {
                 let mut kvs_index = 0;
                 *kvs = Vec::new();
                 for i in  (sibling.value * 4 ) as usize..min((((sibling.value +1) * 4)) as usize, cells.len()){ 
                     kvs.push(cells[i].clone());
                     plusser += 1;
                 }
                 
                 let mut parent = self.nodes.get_mut(&parent).unwrap();
                 match &mut parent.node_type{
                     NodeType::Internal(pkvs) => {
                     if pkvs.len() - 1 == sibling.value as usize {
                         is_end = true
                     }
                     pkvs[sibling.value as usize].key = cells[plusser].key
                     
                     },
                     _ => panic!("______")
                 }  
 
                 self.nodes.insert(new_node_id, Node { node_type: NodeType::Leaf(vec![ cells[plusser].clone() ]), is_root: false });
                 }
             }
          }
          
          if is_end {
             let mut parent = self.nodes.get_mut(&parent).unwrap();
             match &mut parent.node_type{
                     NodeType::Internal(pkvs) => {
                     pkvs.push(KeyValue { key: 0, value: cells[plusser].key })
                     
                     },
                     _ => panic!("______")
             }
          } else {
             let mut parent = self.nodes.get_mut(&parent).unwrap();
             match &mut parent.node_type{
                     NodeType::Internal(pkvs) => {
                     pkvs.insert(siblings[siblings.len() - 1].value as usize + 1,KeyValue { key: cells[plusser].key, value: new_node_id });
                     
                     },
                     _ => panic!("______")
             }
          }
 
        } else {
          let mut plusser = 0;
          let last_sibling = &siblings[&siblings.len()-1].clone();
          for sibling in siblings.into_iter() {
             let mut node = self.nodes.get_mut(&sibling.key).unwrap();
             match &mut node.node_type {
                 NodeType::Internal(kvs) => {
                 *kvs = Vec::new();
                 for i in  (sibling.value * 4 ) as usize..min((((sibling.value +1) * 4)) as usize, cells.len()){ 
                     kvs.push(cells[i].clone());
                     plusser += 1;
                 }
                 let mut parent = self.nodes.get_mut(&parent).unwrap();
                 match &mut parent.node_type{
                     NodeType::Internal(pkvs) => {
                     if sibling.value == (pkvs.len() -1) as u16 {
                         pkvs[sibling.value as usize].key = 0;
                     }
                     else {
                         if last_sibling == &sibling {
                             continue;
                          }
                         pkvs[sibling.value as usize].key = cells[plusser].clone().key
                     }
                     },
                     _ => panic!("______")
                 }
                 },
                 NodeType::Leaf(kvs) => {
                 *kvs = Vec::new();
                 for i in  (sibling.value * 4 ) as usize..min((((sibling.value +1) * 4)) as usize, cells.len()){ 
                     kvs.push(cells[i].clone());
                     plusser += 1;
                    
                 }
                 let mut parent = self.nodes.get_mut(&parent).unwrap();
                 match &mut parent.node_type{
                     NodeType::Internal(pkvs) => {
                     if sibling.value ==  (pkvs.len() -1) as u16 {
                         pkvs[sibling.value as usize].key = 0;
                     }
                     else {
                         if last_sibling == &sibling {
                            continue;
                         }
                         pkvs[sibling.value as usize].key = cells[plusser].key
                     }
                     },
                     _ => panic!("______")
                 }  
                 }
             }
          }
        }
 
       return true 
         
     }

     // NOT used for the working b+tree implementation
     fn siblings(&mut self, node_id: u16, parent: u16) -> Vec<KeyValue> {
        let parent_node = self.nodes.get(&parent).unwrap();
        
        let mut index;
        let mut num_siblings = 1;
        match &parent_node.node_type {
            NodeType::Internal(keys) => {
                
                for i in 0..keys.len() {
                    if keys[i].value == node_id {
                        index = i;
                        if index == 0 || index == keys.len() {
                           num_siblings *= 2;
                        }

                        let left_siblings = if index == 0 {
                            0..0 
                        } else {
                             index.saturating_sub(num_siblings)..index
                        };
                        let right_siblings = if index == keys.len() - 1 {
                            index + 1..index + 1 // No right siblings if it's the last node
                        } else {
                            (index + 1)..min(index + num_siblings + 1, keys.len())
                        };
                        
                        let get_siblings = |index: usize| KeyValue{key: keys[index].value, value: index as u16};
                        
                        return left_siblings
                                .map(get_siblings)
                                .chain(std::iter::once(get_siblings(index)))
                                .chain(right_siblings.map(get_siblings)).collect();
                    }
                }
            },
            _ => panic!("siblings fn")
        };
        
      return Vec::new();
    }
     */