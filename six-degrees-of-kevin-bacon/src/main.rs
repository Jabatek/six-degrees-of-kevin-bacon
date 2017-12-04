use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::VecDeque;


struct Node {
    name: String,
    enodes: Vec<usize>,
    checked: bool,
    parent: usize,
}


struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn search_node(&mut self, name: &str) -> usize {
        for i in 0..self.nodes.len() {
            if self.nodes[i].name == name
                {
                    return i;
                }
        }
        return <usize>::max_value();
    }


    fn init(path: &str) -> Graph {
        println!("start init");
        let mut graph = Graph { nodes: vec![] };
        let f = File::open(path).unwrap();
        let file = BufReader::new(&f);

        for (_num, line) in file.lines().enumerate() {
            let l = line.unwrap();
            let s: Vec<&str> = l.split(";").collect();


            let mut aind = graph.search_node(s[0]);

            if aind == <usize>::max_value() {
                graph.nodes.push(Node { name: s[0].to_string(), enodes:vec![], checked: false, parent: <usize>::max_value() });
                aind = graph.nodes.len() - 1
            }

            let mut mind = graph.search_node(s[1]);

            if mind == <usize>::max_value() {
                graph.nodes.push(Node { name: s[1].to_string(), enodes: vec![], checked: false, parent: <usize>::max_value() });
                let len=graph.nodes.len() - 1;
                graph.nodes[len].enodes.push(aind);
                mind = graph.nodes.len() - 1
            } else {
                graph.nodes[mind].enodes.push(aind);
            }

            if aind == <usize>::max_value() {
                let len=graph.nodes.len() - 1;
                graph.nodes[len].enodes.push(mind);
            } else {
                graph.nodes[aind].enodes.push(mind);
            }
        }
        println!("end init");
        return graph;
    }

    fn search(&mut self, start: &str, end: &str) {

        println!();
        println!("search for connection between '{}' and '{}' ",start,end);
        println!();
        let mut queue: VecDeque<usize> = VecDeque::new();

        let istart = self.search_node(start);
        let mut icurrent=istart;
        let iend = self.search_node(end);
        self.nodes[icurrent].checked = true;
        queue.push_back(icurrent);


        while queue.len() > 0 {
            icurrent = queue.pop_front().unwrap();

            if icurrent == iend {
                break;
            }

            let len = self.nodes[icurrent].enodes.len() - 1;

            for i in 0..len {
                let  inode = self.nodes[icurrent].enodes[i];
                if !self.nodes[inode].checked {
                    self.nodes[inode].checked = true;
                    self.nodes[inode].parent = icurrent;
                    queue.push_back(inode);
                }
            }
        }


        let mut mora=0;
        loop {
            println!("{}", self.nodes[icurrent].name);


            if icurrent==istart {
                break
            }

            if mora%2==0{
                print!("        Movie: ")
            }
            mora+=1;

            icurrent = self.nodes[icurrent].parent;
        }
    }


}


fn main() {
    let mut graph = Graph::init("data/movies-demo.csv");
    graph.search("Martin Sheen", "Elarica Gallacher");
}
