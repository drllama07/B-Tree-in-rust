<h1 align="center">

  B+ Tree in Rust
  <br>
</h1>

<h4 align="center">B+ Tree implementation in Rust</h4>

<p align="center">
  <a href="#implementation-details">Key Features</a> •
  <a href="#learning-resources-and-credits">Learning Resources and Credits</a> •
  <a href="#license">License</a>
</p>


## Implementation Details
***NOTE:*** If you want to learn more about B+ Trees, you can take a look at the learning resources part.

- This project is intended for learning rather than production. There might be bugs or edge cases. If you spot one feel free to share it with me.

### Conventions used in this project:
1. > **Internal Nodes :**
   > - The internal nodes of the B+ tree use a special key, `0`, to denote values that are greater than all other keys within the same node.
   > - Every key in an internal node of the B+ tree points to a child node that contains values smaller than the key itself. `child_values < key_of_the_pointer`
   > - The only exception is `0`, because values cannot be smaller than 0. Therefore, using 0 as the key for values greater than the largest key is more efficient.
   > - You can define the minimum and maximum number of keys that an internal node can hold from here -> 
   > ```
   > const BTREE_MAX: usize = 4;
   >const MIN_KEY: usize = BTREE_MAX/2;
   >const MAX_KEY: usize = BTREE_MAX;
   >const MIN_CHILD: usize = (BTREE_MAX+1)/2;
   >  // This truncates to smaller values for odd values so adjust this for your needs.
   > 
   >const MAX_CHILD: usize = BTREE_MAX+1;
2. > ***Root Nodes :***
   > - Root node can contain at least 2 child pointers (1 real key and `0` key to denote bigger values than the first key.)
   > - Root node can both split and merge to balance the tree.

3. > ***Leaf Nodes :***
   > - Standard leaf node rules apply to them.

4. > ***Spliting :***
   > - *When a node overflows*
   > - The splitting method used in this implementation can be referred to as "Aggressive Splitting".
   > - Basically, it splits the node into two nodes from the middle and updates the tree accordingly.
   > - It is right biased which means that right node will have more keys than the left one.

5. > ***Merging (distribute_mini) :***
   > - *When a node underflows*
   > - Merging is has two options when it can merge with a sibling or it can take a value from the sibling.
   > - Merging, merges to the right node(node that contains bigger values).

- > ***Searching :*** 
  > - `search` function returns the ids of the both leaf node that the key fits and the parent of that node.
- > ***Print_tree :*** 
  > - A function that prints trees in a more readable way
  > ```
  >  Internal Node (ID: 0):
  >    - Key: 3, Points to Node: 2
  >     Leaf Node (ID: 2):
  >       - Key: 1, Value: 100
  >       - Key: 2, Value: 100
  >   - Key: 0, Points to Node: 1
  >     Leaf Node (ID: 1):
  >       - Key: 3, Value: 100
  >       - Key: 4, Value: 100
  >       - Key: 5, Value: 100  
- There are a few other functions that you can leverage. Check out [btrees.rs](src/btrees.rs).

## Learning Resources and Credits

- [Database System Concepts](https://www.db-book.com/slides-dir/PDF-dir/ch14.pdf)
- [CMU Intro to DB systems](https://15445.courses.cs.cmu.edu/fall2020/project2/)
- [SQLLite's B+ Tree](https://sqlite.org/btreemodule.html#balance_siblings)
- [geeksforgeeks.org - detailed and simple -](https://www.geeksforgeeks.org/introduction-of-b-tree/)




## You may also like...

- [A minimal database implementation in RUST]()

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


