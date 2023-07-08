use std::path::PathBuf;

struct PackageManager {}

pub trait Manager {
    fn create_templete(package_name: String, store_path: PathBuf);
    fn package_append(package_path: PathBuf);
    fn package_delete(package_name: String);
    fn list_all_package();
    fn create_graph();
    fn create_dagrs_yaml();
}

impl Manager for PackageManager {
    fn create_templete(package_name: String, store_path: PathBuf) {
        //create_json_file
        //mk package dir
        //create_templete_script
    }
    fn package_append(package_path: PathBuf) {}
    fn list_all_package() {}
    fn create_graph() {}
    fn package_delete(package_name: String) {}
    fn create_dagrs_yaml() {}
}
