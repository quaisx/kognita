/*
 _        _______  _______  _       __________________ _______
| \    /\(  ___  )(  ____ \( (    /|\__   __/\__   __/(  ___  )
|  \  / /| (   ) || (    \/|  \  ( |   ) (      ) (   | (   ) |
|  (_/ / | |   | || |      |   \ | |   | |      | |   | (___) |
|   _ (  | |   | || | ____ | (\ \) |   | |      | |   |  ___  |
|  ( \ \ | |   | || | \_  )| | \   |   | |      | |   | (   ) |
|  /  \ \| (___) || (___) || )  \  |___) (___   | |   | )   ( |
|_/    \/(_______)(_______)|/    )_)\_______/   )_(   |/     \|

@authors: free thinkers of the world
    1. Qua Is X (Ukraine) qua.is.kyiv.ua@gmail.com
    /add your name here.../

 */

use crate::tx::Transaction;
use time::timestamp;

/// Node - a tx placeholder inside DAG
///    - id: hash of the node with cum. weight = 0; this value will most likely keep changing
///            hence, there is no need to keep recalculating hash of each block after
///            each cum.weight update.
///    - timestamp: a time, this node has been generated. it's value value will always be
///            either equal or later than the tx timestamp
///    - cumweight: a number of references to this node from the nodes past this node
///            For explanation, see [https://wiki.iota.org/learn/glossary/#:~:text=Cumulative%20Weight,additional%20transaction%20that%20references%20it.]
///            Also, cumweight.md contains a brief explanation of cumweight.
///    - tx: a BSV transaction this nodes wraps around
///    - et: Edges Target a vec of RC<NODE> to the target nodes fo which we are the source
///    - es: Edges Source a vec of RC<NODE> to the source nodes fo which we are the target
///    Na <--Es-- Nb --Et--> Nc: Node b references Na as its source node, while references
///                          Nc as its target node.
/// Node: we limit the number of edges to the source and target nodes to max 2.
pub struct Node {
    id: u32,
    timestamp: time::timestamp,
    cumweight: u32,
    tx: Box<Transaction>,
    et: Vec<Rc<Node>>,
    es: Vec<Rc<Node>>,
}
