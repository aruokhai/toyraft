


#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let path = config::data_dir_absolute_path(opt.data_dir.clone());
    let conf_file_path = path.join("teos.toml");
    // Load conf (from file or defaults) and patch it with the command line parameters received (if any)
    let mut conf = config::from_file::<Config>(&conf_file_path);
    
    let server_api = Arc::new(ServerApi::new())




}
