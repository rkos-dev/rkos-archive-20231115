use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::parser::PackageGraphRaw;
use crate::parser::PackageTemplete;

use petgraph::{Directed, Graph};

struct PackageManager {
    package_graph: Graph<String, String, Directed>,
    package_graph_raw: PackageGraphRaw,
}

impl PackageManager {
    fn new() -> PackageManager {
        PackageManager {
            package_graph: Graph::new(),
            package_graph_raw: PackageGraphRaw {
                package_node: Vec::new(),
                packages: HashMap::new(),
                package_vec: Vec::new(),
            },
        }
    }
    fn build_graph(&self, packages_path: PathBuf) {
        if packages_path.is_dir() {
            let entrys = fs::read_dir(packages_path).unwrap();
            for entry in entrys {
                self.add_package(entry.path());
            }
        } else {
            return;
        }
    }

    fn add_package(&self, new_package: PathBuf) {
        let value = PackageTemplete::load(new_package);
        let node = self.package_graph.add_node(value.package_name);
        self.package_graph_raw.package_node.push(node);
        self.package_graph_raw.packages.append({value.package_name.clone(),self.package_graph_raw.index});
        
    }

    fn remove_package(package: PathBuf) {
        let package = PackageTemplete::load(package);
        let key = package.package_name;
    }

    fn create_templete(package_name: String, store_path: PathBuf) {
        //create_json_file
        //mk package dir
        //create_templete_script

        PackageTemplete::new_templete(store_path, package_name);
    }
    fn list(package_name: Option<String>, all_package: Option<bool>, graph: Option<bool>) {}
    //append 操作时不进行图更新，只添加node，在图更新时处理依赖关系
    fn list_all_package(&self) {
        let package_iter = self.package_graph.node_weights();
        for i in package_iter {}
    }
    fn list_target_package() {}
    fn package_delete(package_name: String) {}
    fn create_dagrs_yaml() {}
}
