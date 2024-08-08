<h1>📝 Todo Project</h1>

<p>Welcome to the <strong>Todo Project</strong> – a simple yet effective CLI-based task manager written in Rust. This project is designed to help you manage your tasks while honing your Rust programming skills.</p>

<h2>🌟 Features</h2>
<ul>
  <li><strong>Add Tasks:</strong> Quickly add tasks to your list.</li>
  <li><strong>Mark as Done:</strong> Easily mark tasks as completed.</li>
  <li><strong>View Tasks:</strong> Display all pending and completed tasks.</li>
  <li><strong>Delete Tasks:</strong> Remove tasks that are no longer needed.</li>
</ul>

<h2>🚀 Getting Started</h2>

<h3>Prerequisites</h3>
<p>Ensure you have Rust installed. If not, you can install it via <a href="https://rustup.rs/" target="_blank">rustup</a>.</p>
<pre><code>
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
</code></pre>

<h3>Installation</h3>
<p>Clone the repository and navigate into the project directory:</p>
<pre><code>
git clone https://github.com/yourusername/todo-project.git
cd todo-project
</code></pre>

<p>Build the project:</p>
<pre><code>
cargo build --release
</code></pre>

<p>Run the project:</p>
<pre><code>
cargo run
</code></pre>

<h3>Usage</h3>

<p>Add a new task:</p>
<pre><code>
cargo run -- add "Your task here"
</code></pre>

<p>View all tasks:</p>
<pre><code>
cargo run -- list
</code></pre>

<p>Mark a task as done:</p>
<pre><code>
cargo run -- done 1
</code></pre>

<p>Delete a task:</p>
<pre><code>
cargo run -- delete 1
</code></pre>

<h2>🛠️ Built With</h2>
<ul>
  <li><a href="https://www.rust-lang.org/" target="_blank">Rust</a> - The programming language used.</li>
  <li><a href="https://doc.rust-lang.org/cargo/" target="_blank">Cargo</a> - Rust's package manager and build system.</li>
</ul>

<h2>📜 License</h2>
<p>This project is licensed under the MIT License - see the <a href="LICENSE">LICENSE</a> file for details.</p>

<h2>🤝 Contributing</h2>
<p>Contributions are welcome! Please feel free to submit a Pull Request.</p>

<h2>💬 Contact</h2>
<p>Feel free to reach out via <a href="mailto:your.email@example.com">Email</a> or <a href="https://github.com/yourusername" target="_blank">GitHub</a>.</p>

<hr>

<p align="center">
  Made with ❤️ in Rust
</p>

