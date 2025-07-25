use winreg::RegKey;
use winreg::enums::*;

pub fn register_file_associations() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;
    let exe_path = current_exe.to_string_lossy().to_string();

    // Register .newick and .tre extensions
    register_file_type(
        "newick",
        "TreeHouse.NewickFile",
        "Newick Phylogenetic Tree",
        &exe_path,
        &format!("{},1", exe_path),
    )?;

    register_file_type(
        "tre",
        "TreeHouse.NewickFile",
        "Newick Phylogenetic Tree",
        &exe_path,
        &format!("{},1", exe_path),
    )?;

    // Register .nexus, .nex, .tree, and .trees extensions
    register_file_type(
        "nexus",
        "TreeHouse.NexusFile",
        "Nexus Phylogenetic Tree",
        &exe_path,
        &format!("{},2", exe_path),
    )?;

    register_file_type(
        "nex",
        "TreeHouse.NexusFile",
        "Nexus Phylogenetic Tree",
        &exe_path,
        &format!("{},2", exe_path),
    )?;

    register_file_type(
        "tree",
        "TreeHouse.NexusFile",
        "Nexus Phylogenetic Tree",
        &exe_path,
        &format!("{},2", exe_path),
    )?;

    register_file_type(
        "trees",
        "TreeHouse.NexusFile",
        "Nexus Phylogenetic Tree",
        &exe_path,
        &format!("{},2", exe_path),
    )?;

    notify_shell_change();

    Ok(())
}

fn register_file_type(
    extension: &str,
    prog_id: &str,
    description: &str,
    exe_path: &str,
    icon_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Create/open the file extension key
    let (ext_key, _) =
        hkcu.create_subkey(format!("Software\\Classes\\.{}", extension))?;
    ext_key.set_value("", &prog_id)?;

    // Create/open the ProgID key
    let (prog_key, _) =
        hkcu.create_subkey(format!("Software\\Classes\\{}", prog_id))?;
    prog_key.set_value("", &description)?;

    // Set the icon (using embedded resource icon)
    let (icon_key, _) = prog_key.create_subkey("DefaultIcon")?;
    icon_key.set_value("", &icon_path)?;

    // Set the open command
    let (shell_key, _) = prog_key.create_subkey("shell")?;
    let (open_key, _) = shell_key.create_subkey("open")?;
    let (command_key, _) = open_key.create_subkey("command")?;
    command_key.set_value("", &format!("\"{}\" \"%1\"", exe_path))?;

    Ok(())
}

pub fn unregister_file_associations() -> Result<(), Box<dyn std::error::Error>>
{
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes_key = hkcu.open_subkey("Software\\Classes")?;

    // Remove file extensions
    let extensions = ["newick", "tre", "nexus", "nex", "tree", "trees"];
    for ext in &extensions {
        let _ = classes_key.delete_subkey_all(format!(".{}", ext));
    }

    // Remove ProgIDs
    let _ = classes_key.delete_subkey_all("TreeHouse.NewickFile");
    let _ = classes_key.delete_subkey_all("TreeHouse.NexusFile");

    // Notify the shell of the changes
    notify_shell_change();

    Ok(())
}

pub fn are_file_associations_registered() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Check if at least one of our extensions is registered
    if let Ok(classes_key) = hkcu.open_subkey("Software\\Classes") {
        if let Ok(ext_key) = classes_key.open_subkey(".newick") {
            if let Ok(prog_id) = ext_key.get_value::<String, _>("") {
                return prog_id == "TreeHouse.NewickFile";
            }
        }
    }

    false
}

fn notify_shell_change() {
    use windows::Win32::UI::Shell::SHChangeNotify;
    use windows::Win32::UI::Shell::{SHCNE_ASSOCCHANGED, SHCNF_IDLIST};

    unsafe {
        SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None);
    }
}

pub fn setup_file_handling() -> Result<(), Box<dyn std::error::Error>> {
    if !are_file_associations_registered() {
        println!("Registering file type associations...");
        register_file_associations()?;
        println!("File type associations registered successfully!");
    }

    Ok(())
}
