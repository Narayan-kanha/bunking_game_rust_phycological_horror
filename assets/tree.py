import sys
from pathlib import Path

# --- Configuration ---
# Set the directory to scan. '.' means the current directory where the script is run.
TARGET_DIR = Path('.')
# List of folder names to completely ignore (including all their contents)
EXCLUDE_DIRS = {'venv', '.git', '__pycache__', 'node_modules'}

# Tree Symbols
TREE_MID = "├── "
TREE_END = "└── "
INDENT_CONT = "│   "
INDENT_SPAC = "    "


def generate_tree(dir_path: Path, prefix: str = ''):
    """
    Recursively generates and prints a directory tree structure.

    Args:
        dir_path: The Path object of the directory to scan.
        prefix: The prefix string for the current level (handles indentation 
                and connecting lines from parent directories).
    """
    try:
        # 1. Get all contents (directories and files)
        # Using sorted for consistent output order (dirs first, then files, then alphabetical)
        contents = sorted([p for p in dir_path.iterdir()])
    except Exception as e:
        print(f"Error reading directory {dir_path}: {e}")
        return
        
    # 2. Separate into directories and files
    dirs = [c for c in contents if c.is_dir() and c.name not in EXCLUDE_DIRS]
    files = [c for c in contents if c.is_file() and c.name not in EXCLUDE_DIRS]
    
    # Combined list for processing, folders come before files
    items = dirs + files

    # 3. Iterate and Print
    for i, item in enumerate(items):
        is_last = (i == len(items) - 1)
        
        # Select the correct branch symbol
        connector = TREE_END if is_last else TREE_MID

        # Print the entry with the calculated prefix and connector
        print(f"{prefix}{connector}{item.name}{'/' if item.is_dir() else ''}")

        # 4. Handle Recursion for Subdirectories
        if item.is_dir():
            # Calculate the prefix for the subdirectory's children
            # It continues the vertical line for the non-last item, and a space for the last item.
            new_prefix = prefix + (INDENT_SPAC if is_last else INDENT_CONT)
            generate_tree(item, new_prefix)

def main():
    """Main function to run the tree generation starting from the TARGET_DIR."""
    # Print the root directory (always the first line, no prefix)
    print(f"{TARGET_DIR.resolve().name}/")
    
    # Generate the tree structure
    generate_tree(TARGET_DIR)
    
# Execute the main function
if __name__ == "__main__":
    main()