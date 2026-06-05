use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    rc::Rc,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct GraphErr {
    mess: String,
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr {
            mess: s.to_string(),
        }
    }
}

// Map Pointer Base
#[derive(Debug)]
pub struct Graph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

impl<T, E, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn new() -> Self {
        Graph {
            data: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ID, dt: T) {
        // node has no edges yet
        self.data.insert(id, (dt, Vec::new()));
    }

    pub fn add_edge(
        &mut self,
        ed_id: ID,
        from: ID,
        to: ID,
        edat: E,
    ) -> std::result::Result<(), GraphErr> {
        if !self.data.contains_key(&from) {
            return Err(GraphErr::new(" 'from' not in Nodes"));
        }

        if let Some(ref mut dt) = self.data.get_mut(&to) {
            self.edges.insert(ed_id.clone(), (edat, from.clone(), to));
            dt.1.push(ed_id.clone());
        } else {
            return Err(GraphErr::new("'to' not in nodes"));
        }

        self.data.get_mut(&from).unwrap().1.push(ed_id);
        Ok(())
    }
}

// Route Structure
#[derive(Debug)]
#[allow(dead_code)]
pub struct Route<ID> {
    pos: ID,
    path: Option<Rc<Route<ID>>>,
    len: i32,
}

impl<ID: Eq> Route<ID> {
    pub fn start_rc(pos: ID) -> Rc<Self> {
        Rc::new(Route {
            pos,
            path: None,
            len: 0,
        })
    }

    pub fn contains(&self, id: &ID) -> bool {
        if self.pos == *id {
            return true;
        }
        match self.path {
            Some(ref p) => p.contains(id),
            None => false,
        }
    }
}

impl<ID: Debug> Display for Route<ID> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(ref p) = self.path {
            write!(f, "{}-{}", p, self.len)?;
        }
        write!(f, "{:?}", self.pos)
    }
}

pub trait Weighted {
    fn weight(&self) -> i32;
}

impl Weighted for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}

// shorted path
impl<T, E: Weighted, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<Rc<Route<ID>>> {
        let mut visited = HashSet::new();
        let mut routes = Vec::new();
        routes.push(Route::start_rc(from));
        loop {
            let c_route = routes.pop()?;
            if to == c_route.pos {
                return Some(c_route);
            }
            if visited.contains(&c_route.pos) {
                // no point in searching from the same place twice
                continue;
            }
            visited.insert(c_route.pos.clone());
            let exits = self.data.get(&c_route.pos)?;

            for eid in &exits.1 {
                let edge = self.edges.get(eid)?;
                let npos = if edge.1 == c_route.pos {
                    // opposite side of the edge to current pos
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };

                let nlen = c_route.len + edge.0.weight();
                let nroute = Rc::new(Route {
                    pos: npos,
                    len: nlen,
                    path: Some(c_route.clone()),
                });
                if routes.len() == 0 {
                    routes.push(nroute);
                    continue;
                }
                // insert into the list shorted
                let mut iafter = routes.len() - 1;
                loop {
                    if routes[iafter].len > nlen {
                        // lowes element last
                        routes.insert(iafter + 1, nroute);
                        break;
                    }
                    if iafter == 0 {
                        // reached end
                        routes.insert(0, nroute);
                        break;
                    }
                    iafter -= 1;
                }
            }
        }
    }
}

fn main() -> Result<(), GraphErr> {
    let mut g = Graph::new();
    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        g.add_node(x, ());
    }
    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'B', 10)?;
    g.add_edge('d', 'H', 'A', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'H', 'G', 5)?;
    g.add_edge('g', 'G', 'A', 8)?;
    g.add_edge('h', 'A', 'F', 3)?;
    g.add_edge('i', 'F', 'E', 15)?;
    g.add_edge('j', 'C', 'E', 12)?;
    println!("Hello, graph {:?}", g);

    match g.shortest_path('A', 'D') {
        Some(route) => println!("shortest path A-D = {}", route),
        None => println!("No path from A to D"),
    }
    match g.shortest_path('H', 'B') {
        Some(route) => println!("shortest path H-B = {}", route),
        None => println!("No Path From H to B"),
    }

    match g.shortest_path('F', 'A') {
        Some(route) => println!("shortest path F-A = {}", route),
        None => println!("No Path From F to A"),
    }

    Ok(())
}
