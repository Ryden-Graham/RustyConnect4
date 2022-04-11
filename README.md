How to start and use the program
Requirements:
Microsoft Windows 10 or higher.
Cargo: install from https://win.rustup.rs/ 
Must have nightly Rust installed. If you don’t have nightly, run:
                “rustup override set nightly”
Yarn: install with npm: “npm i yarn”
(requires npm, install npm with: “npm install -g npm”)        
        
In a terminal in the project folder, run:
        “cargo run -p backend” to start the backend server

        In another terminal in the “frontend” folder, run:
“yarn install” initially to install dependencies.
        “yarn run dev” to start the frontend

        After starting frontend, he GUI should automatically open in a web
        browser, if not open: “http://localhost:8000/”
