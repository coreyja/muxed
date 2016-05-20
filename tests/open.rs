//! The integration suite for inspecting sessions.

extern crate libc;
extern crate rand;

mod helpers;

mod open {
    use rand::{random};
    use std::fs::File;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::io::prelude::*;
    use helpers::*;

    fn setup(contents: &'static [u8]) -> (String, PathBuf) {
        let project_name = format!("muxed_int_test_{}", random::<u16>());
        let project_file = format!("/tmp/muxed_{}/{}.yml", random::<u16>(), project_name);
        let project_path = PathBuf::from(&project_file);

        let muxed_path = project_path.parent().unwrap();
        if !muxed_path.exists() { println!("{:?}", fs::create_dir(muxed_path)) };

        let mut buffer = File::create(&project_path).unwrap();
        let _ = buffer.write(contents);

        (project_name, project_path.clone())
    }

    fn cleanup(project_name: &String, config_path: &PathBuf) -> () {
        let _ = fs::remove_file(config_path);
        let _ = fs::remove_dir(config_path.parent().unwrap());
        kill_session(&project_name);
    }

    fn test_with_contents(contents: &'static [u8]) -> TmuxSession {
        let (project_name, config_path) = setup(contents);
        open_muxed(&project_name, config_path.parent().unwrap());
        let session = session_object(&list_windows(&project_name.to_string()));
        cleanup(&project_name, &config_path);
        session
    }

    #[test]
    fn opens_3_windows_from_array() {
        let contents = b"---
windows: ['cargo', 'vim', 'git']
";
        let session = test_with_contents(contents);
        assert_eq!(session.num_of_windows, 3)
    }

    #[test]
    fn opens_2_windows() {
        let contents = b"---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
  - stuff: ''
";
        let session = test_with_contents(contents);
        assert_eq!(session.num_of_windows, 2)
    }

    #[test]
    fn opens_3_windows_with_integer_names() {
        let contents = b"---
windows: [1, 'vim', 3]
";
        let session = test_with_contents(contents);
        assert_eq!(session.num_of_windows, 3)
    }
}
