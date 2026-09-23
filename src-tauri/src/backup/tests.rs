use crate::backup::{create_local_backup, list_local_backups};
use std::fs;

#[test]
fn test_create_and_list_local_backup() {
    let test_dir = std::env::temp_dir().join("sb_integration_source");
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).unwrap();

    let file1 = test_dir.join("file1.txt");
    let file2 = test_dir.join("sub").join("file2.txt");
    fs::create_dir_all(test_dir.join("sub")).unwrap();

    fs::write(&file1, b"first content").unwrap();
    fs::write(&file2, b"second nested content").unwrap();

    let backup_result = create_local_backup(&test_dir).unwrap();
    assert_eq!(backup_result.manifest.total_files, 2);
    assert_eq!(backup_result.manifest.files.len(), 2);

    // Verify manifest JSON exists on disk
    let manifest_path = std::path::Path::new(&backup_result.target_directory).join("manifest.json");
    assert!(manifest_path.exists());

    // Verify backups can be listed
    let backups = list_local_backups().unwrap();
    assert!(!backups.is_empty());
    assert!(backups.iter().any(|b| b.id == backup_result.backup_id));

    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
    let _ = fs::remove_dir_all(&backup_result.target_directory);
}
