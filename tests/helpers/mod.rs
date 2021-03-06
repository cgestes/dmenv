extern crate structopt;
use self::structopt::StructOpt;

pub struct TestApp {
    tmp_path: std::path::PathBuf,
    cfg_path: std::path::PathBuf,
}

///
/// An instance of dmenv::App designed for testing
///
/// By default, contains a copy of all the Python files in
/// demo/
impl TestApp {
    pub fn new(tmp_path: std::path::PathBuf) -> Self {
        let cfg_path = tmp_path.join("dmenv.toml");
        let test_app = TestApp { tmp_path, cfg_path };
        test_app.ensure_cfg();
        test_app.copy_demo_files();
        test_app
    }

    fn ensure_cfg(&self) {
        let python_binary = std::env::var("PYTHON_BINARY")
            .expect("PYTHON_BINARY environment variable should be set to a python3 binary");
        let to_write = format!(
            r#"
            [pythons]
            default = "{}"
            "#,
            python_binary
        );
        std::fs::write(&self.cfg_path, to_write).expect("");
    }

    fn copy_demo_files(&self) {
        std::fs::write(
            self.tmp_path.join("demo.py"),
            include_str!("../../demo/demo.py"),
        ).expect("");
        std::fs::write(
            self.tmp_path.join("test_demo.py"),
            include_str!("../../demo/test_demo.py"),
        ).expect("");
        std::fs::write(
            self.tmp_path.join("setup.py"),
            include_str!("../../demo/setup.py"),
        ).expect("");
    }

    pub fn remove_cfg(&self) {
        self.remove_file(&self.cfg_path.to_string_lossy());
    }

    pub fn remove_setup_py(&self) {
        self.remove_file("setup.py");
    }

    pub fn run(&self, args: Vec<String>) -> Result<(), dmenv::Error> {
        let mut cmd = vec![];
        cmd.extend(vec!["dmenv".to_string()]);
        let tmp_path: String = self.tmp_path.to_string_lossy().into();
        cmd.extend(vec!["--cwd".to_string(), tmp_path]);

        let cfg_path: String = self.tmp_path.join("dmenv.toml").to_string_lossy().into();
        cmd.extend(vec!["--cfg-path".to_string(), cfg_path]);
        cmd.extend(args);
        let options = dmenv::Options::from_iter_safe(cmd).expect("");
        dmenv::run(options)
    }

    pub fn assert_run_ok(&self, args: Vec<&str>) {
        let args = to_string_args(args);
        self.run(args).expect("");
    }

    pub fn assert_lock(&self) {
        self.assert_file(dmenv::LOCK_FILE_NAME);
    }

    pub fn assert_setup_py(&self) {
        self.assert_file("setup.py");
    }

    pub fn assert_file(&self, name: &str) {
        assert!(self.tmp_path.join(name).exists());
    }

    pub fn assert_run_error(&self, args: Vec<&str>) -> String {
        let args = to_string_args(args);
        let res = self.run(args);
        res.unwrap_err().to_string()
    }

    pub fn write_lock(&self, contents: &str) {
        self.write_file(dmenv::LOCK_FILE_NAME, contents);
    }

    pub fn write_file(&self, name: &str, contents: &str) {
        let path = self.tmp_path.join(name);
        std::fs::write(path, &contents).expect("");
    }

    pub fn remove_file(&self, name: &str) {
        let path = self.tmp_path.join(name);
        std::fs::remove_file(path).expect("");
    }

    pub fn assert_python(&self, version: &str, expected_path: &str) {
        let cfg_path = self.cfg_path.to_string_lossy();
        let config_handler = dmenv::ConfigHandler::new(Some(cfg_path.into())).expect("");
        let actual = config_handler.get_python(version).expect("");
        assert_eq!(actual, expected_path);
    }

    pub fn assert_no_python(&self, version: &str) {
        let cfg_path = self.cfg_path.to_string_lossy();
        let config_handler = dmenv::ConfigHandler::new(Some(cfg_path.into())).expect("");
        let err = config_handler.get_python(version).unwrap_err();
        assert!(err.to_string().contains("No python found"));
    }
}

pub fn to_string_args(args: Vec<&str>) -> Vec<String> {
    args.iter().map(|x| x.to_string()).collect()
}
