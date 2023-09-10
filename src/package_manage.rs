use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::execute::DownloadTp;
use crate::execute::ExecuteOption;
use crate::execute::Mission;
use crate::parser::JsonObj;
use crate::parser::PackageGraphRawData;
use crate::parser::PackageTemplete;
use crate::parser::VirtualDeps;

use petgraph::algo::toposort;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::visit::Bfs;
use petgraph::{Directed, Graph};

pub struct PackageManager {
    pub package_graph: Graph<String, (), Directed>,
    pub package_graph_raw: PackageGraphRawData,
}

impl PackageManager {
    pub fn new(packages_path: PathBuf) -> Option<PackageManager> {
        let mut manager = PackageManager {
            package_graph: Graph::new(),
            package_graph_raw: PackageGraphRawData::new(packages_path.clone())?,
        };

        //add node
        for package in &manager.package_graph_raw.package_vec {
            let node = manager.package_graph.add_node(package.package_name.clone());
            manager.package_graph_raw.package_node_vec.push(node);
        }

        //add edge
        for package in &manager.package_graph_raw.package_vec {
            for dependence in &package.depends {
                if manager
                    .package_graph_raw
                    .virtual_pacakges
                    .v_depends
                    .contains(dependence)
                {
                    continue;
                }

                let dep = match manager.package_graph_raw.node(dependence.clone()) {
                    Some(v) => v,
                    None => {
                        panic!(
                            "not find this dependence! {} {}",
                            dependence,
                            package.package_name.clone()
                        );
                    }
                };

                let pack = match manager.package_graph_raw.node(package.package_name.clone()) {
                    Some(v) => v,
                    None => panic!("not find this package! {}", dependence),
                };

                //dep->pack so base dep could be dep by every pack
                manager.package_graph.add_edge(pack, dep, ());
            }
        }

        Some(manager)
    }

    //print graph
    pub fn walk_graph(&self) {
        let mut name2_outdegree = HashMap::new();
        for start in self.package_graph.node_indices() {
            let mut bfs = Bfs::new(&self.package_graph, start);
            print!("[{}]", self.package_graph_raw.index2name(start.index()));

            while let Some(visited) = bfs.next(&self.package_graph) {
                let node = name2_outdegree.entry(start.index()).or_insert(0);
                *node += 1;
                print!("{} ", self.package_graph_raw.index2name(visited.index()));
                if visited.index() == start.index() && *node != 1 {
                    break;
                }
            }
            println!();
        }
        let mut hash_vec: Vec<(&usize, &usize)> = name2_outdegree.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        for i in hash_vec {
            if i.1 == &82 {
                println!("{}", self.package_graph_raw.index2name(i.0.clone()));
            }
        }

        for (key, value) in &name2_outdegree {
            println!("{}:{}", key, value);
        }
    }

    pub fn cycle_check(&self) {
        toposort(&self.package_graph, None).unwrap();
    }

    //    let manager=match PackageManager::new(["./new_packages/"].iter().collect()){
    //        Some(v)=>v,
    //        None=>panic!(),
    //    };
    //    manager.show();
    pub fn show(&self) {
        println!(
            "{:?}",
            Dot::with_config(&self.package_graph, &[Config::EdgeNoLabel])
        );
    }

    //fn list(package_name: Option<String>, all_package: Option<bool>, graph: Option<bool>) {}
    ////append 操作时不进行图更新，只添加node，在图更新时处理依赖关系
    //fn list_all_package(&self) {}
    //fn list_target_package() {}
    //fn package_delete(package_name: String) {}
    //fn create_dagrs_yaml() {}

    pub fn generate_download_mission(&self,target_path:PathBuf)->Mission{
        let mut mission=Mission::new();

        for package in &self.package_graph_raw.package_vec{
            let download_option=ExecuteOption{
                running_type:"download".to_string(),
                download:Some(
                    DownloadTp{
                        target_path:target_path.clone(),
                        url:package.package_url.clone(),
                        name:package.package_name.clone(),
                    }
                ),
                script:None,
                rust:None
            };
            mission.push(download_option);
        }
        mission
    }

    pub fn generate_graph_mission(&self){
        let mut name2_outdegree = HashMap::new();
        let mut pack_deps = HashMap::new();
        for start in self.package_graph.node_indices(){
            let mut bfs=Bfs::new(&self.package_graph,start);

            let mut dep_vec = Vec::new();
            while let Some(visited)=bfs.next(&self.package_graph) {
                let node=name2_outdegree.entry(start.index()).or_insert(0);
                *node+=1;
                dep_vec.push(visited.index());

                if visited.index()==start.index() && *node!=1 {
                    println!("something cycle {}",self.package_graph_raw.index2name(visited.index()));
                    break;
                }
            }
            pack_deps.insert(start.index(), dep_vec);
        }


        let mut hash_vec:Vec<(&usize,&usize)>=name2_outdegree.iter().collect();
        hash_vec.sort_by(|a,b| b.1.cmp(a.1));

        let mut wait_hash_vec:Vec<(&usize,&usize)>=Vec::new();

        let mut exec_vec=Vec::new();

        for start in hash_vec {
            if start.1==&1 {
                exec_vec.push(start);
            }
        }

        // put dep is 1 into exec list and pick up from others ,when all deps founded in exec list, put in, loop
//        for start in hash_vec {
//            if start.1==&1 {
//                continue;
//            }
//            if pack_deps.get(start.0) {
//                let mut flag=0;
//                for dep in pack_deps.get(start.0) {
//                    if !exec_vec.contains(dep) {
//                        flag=1;
//                        break;
//                    }
//                }
//                if flag {
//                    continue;
//                }else {
//                    wait_hash_vec
//                }
//            }
//        }

//            println!("index {} value {} name {}",start.0,start.1,self.package_graph_raw.index2name(*start.0));
        
    }
}
