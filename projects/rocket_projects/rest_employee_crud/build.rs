// build.rs

fn main() {
    // If sqlite3.def and sqlite3.dll are in a known location
    // Make sure to change the path to where you have saved the files on your system
    println!("cargo:rerun-if-changed=E:\\Softwares\\SQLLite\\sqlite-dll-win-x64-3480000\\sqlite3.def");
    
    // Add this to link the sqlite3 library dynamically
    println!("cargo:rustc-link-search=native=E:\\Softwares\\SQLLite\\sqlite-dll-win-x64-3480000");  // Path to directory containing sqlite3.dll
    println!("cargo:rustc-link-lib=dylib=sqlite3");         // Link against sqlite3.dll
}


/*
The error `LINK : fatal error LNK1181: cannot open input file 'sqlite3.lib'` occurs because the Rust code is trying to link against SQLite using the `sqlite3.lib` file, which isn't found.

This typically happens when the SQLite development libraries are not installed on your system or are not correctly linked in your project. To fix this, follow the steps below based on your operating system:

### Windows:
1. Install SQLite Development Libraries:
   You need to install the SQLite development libraries, which include the required `sqlite3.lib` and `sqlite3.dll` files.
   
   You can download the precompiled SQLite library from [SQLite's official website](https://www.sqlite.org/download.html). Look for the "Windows" section and download the SQLite amalgamation package (`sqlite3.c` and `sqlite3.h`).

2. Set the SQLite Path:
   After downloading and extracting the SQLite development files, make sure `sqlite3.lib` is in a directory that your build toolchain can access.

   If you have `sqlite3.lib` and `sqlite3.dll` installed, ensure that the compiler can find them by setting the environment variable `LIB` to include the folder where `sqlite3.lib` is located.

   For example, in PowerShell, run:
   ```powershell
   $env:LIB="C:\path\to\sqlite\lib"
   ```

3. Set the Path to SQLite in `build.rs` (if necessary):
   If you're using `rusqlite` and it can't find the required files automatically, you may need to add a `build.rs` file to specify where to find `sqlite3.lib`:

   Create a `build.rs` file in the root of your project with the following content:
   ```rust
   fn main() {
       println!("cargo:rustc-link-lib=dylib=sqlite3");
       println!("cargo:rerun-if-changed=build.rs");
   }
   ```

4. Install the Visual Studio Build Tools (if you haven't already):
   This error may also occur if you don't have the Visual Studio Build Tools (which includes `link.exe`) installed. You can download it from the [official Visual Studio site](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

   After installation, you can try building the project again.

5. Rebuild the Project:
   After performing the above steps, try building your project again with:
   ```bash
   cargo build
   ```

---

### macOS/Linux:
If you are working on macOS or Linux, you can install SQLite using your package manager.

- macOS (with Homebrew):
  ```bash
  brew install sqlite
  ```

- Linux (on Ubuntu/Debian):
  ```bash
  sudo apt-get install libsqlite3-dev
  ```

After installing the SQLite development libraries, rebuild your project with
`
 cargo update
 cargo clean 
 cargo build
 cargo run`.

---

*/